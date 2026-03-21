use std::{convert::Infallible, env, sync::Arc};

use axum::{
    Json, Router, body,
    extract::{Extension, FromRequestParts, OptionalFromRequestParts, Query},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use axum_oidc::{
    EmptyAdditionalClaims, OidcAuthLayer, OidcClaims, OidcLoginLayer, OidcRpInitiatedLogout,
    error::MiddlewareError,
};
use entity::{prelude::User, user};
use http::Uri;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Database, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use urlencoding::encode;

#[derive(Clone)]
struct AuthState {
    db: DatabaseConnection,
    app_base_url: String,
}

#[derive(Clone)]
struct OidcConfig {
    app_base_url: String,
    issuer: String,
    client_id: String,
    client_secret: String,
}

impl OidcConfig {
    fn from_env() -> Option<Self> {
        Some(Self {
            app_base_url: env::var("APP_BASE_URL").ok()?,
            issuer: env::var("OIDC_ISSUER").ok()?,
            client_id: env::var("OIDC_CLIENT_ID").ok()?,
            client_secret: env::var("OIDC_CLIENT_SECRET").ok()?,
        })
    }
}

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
    service: &'static str,
    oidc_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct LoginQuery {
    redirect_uri: Option<String>,
}

#[derive(Debug, Serialize)]
struct AuthStatusResponse {
    logged_in: bool,
    user_id: Option<i32>,
}

#[derive(Clone)]
struct SyncedUser(Arc<user::Model>);

impl<S> FromRequestParts<S> for SyncedUser
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<SyncedUser>()
            .cloned()
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)
    }
}

impl<S> OptionalFromRequestParts<S> for SyncedUser
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(parts.extensions.get::<SyncedUser>().cloned())
    }
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        ok: true,
        service: "auth",
        oidc_enabled: OidcConfig::from_env().is_some(),
    })
}

async fn login(
    Extension(state): Extension<AuthState>,
    Query(params): Query<LoginQuery>,
) -> impl IntoResponse {
    let callback = format!(
        "{}/api/auth/callback",
        state.app_base_url.trim_end_matches('/')
    );

    if let Some(redirect_uri) = params
        .redirect_uri
        .filter(|uri| uri.starts_with(&state.app_base_url))
    {
        let target = format!("{}?redirect_uri={}", callback, encode(&redirect_uri));
        return Redirect::to(&target);
    }

    Redirect::to(&callback)
}

async fn callback(
    _claims: OidcClaims<EmptyAdditionalClaims>,
    user: SyncedUser,
    Extension(state): Extension<AuthState>,
    Query(params): Query<LoginQuery>,
) -> impl IntoResponse {
    let _user_id = user.0.id;

    let redirect_to = params
        .redirect_uri
        .filter(|uri| uri.starts_with(&state.app_base_url))
        .unwrap_or_else(|| state.app_base_url.clone());

    Redirect::to(&redirect_to)
}

async fn logout(
    logout: OidcRpInitiatedLogout,
    Extension(state): Extension<AuthState>,
) -> impl IntoResponse {
    logout
        .with_post_logout_redirect(
            Uri::from_maybe_shared(state.app_base_url.clone()).expect("valid APP_BASE_URL"),
        )
        .into_response()
}

async fn auth_status(
    user: Option<OidcClaims<EmptyAdditionalClaims>>,
    synced_user: Option<SyncedUser>,
) -> impl IntoResponse {
    Json(AuthStatusResponse {
        logged_in: user.is_some(),
        user_id: synced_user.map(|u| u.0.id),
    })
}

async fn sync_user_middleware(
    claims: OidcClaims<EmptyAdditionalClaims>,
    mut request: http::Request<body::Body>,
    next: Next,
) -> Response {
    let Some(state) = request.extensions().get::<AuthState>().cloned() else {
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "auth state missing",
        )
            .into_response();
    };

    if request.extensions().get::<SyncedUser>().is_some() {
        return next.run(request).await;
    }

    let oidc_sub = claims.subject().to_string();
    let existing = User::find()
        .filter(user::Column::Name.eq(&oidc_sub))
        .one(&state.db)
        .await
        .ok()
        .flatten();

    match existing {
        Some(found_user) => {
            request
                .extensions_mut()
                .insert(SyncedUser(Arc::new(found_user)));
        }
        None => {
            let new_user = user::ActiveModel {
                name: Set(oidc_sub),
                ..Default::default()
            };
            match new_user.insert(&state.db).await {
                Ok(created_user) => {
                    request
                        .extensions_mut()
                        .insert(SyncedUser(Arc::new(created_user)));
                }
                Err(error) => {
                    tracing::error!("failed to create user from oidc claims: {:?}", error);
                }
            }
        }
    }

    next.run(request).await
}

pub async fn router() -> Router {
    let base_router = Router::new().route("/health", get(health));

    let Some(config) = OidcConfig::from_env() else {
        tracing::warn!(
            "OIDC env vars missing; auth router will expose only /api/auth/health for now"
        );
        return base_router;
    };

    let database_url = match env::var("DATABASE_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!(
                "DATABASE_URL missing; auth router will expose only /api/auth/health for now"
            );
            return base_router;
        }
    };

    let db = match Database::connect(&database_url).await {
        Ok(connection) => connection,
        Err(error) => {
            tracing::error!("failed to connect DB for auth router: {:?}", error);
            return base_router;
        }
    };

    let state = AuthState {
        db,
        app_base_url: config.app_base_url.clone(),
    };

    let session_layer = SessionManagerLayer::new(tower_sessions::MemoryStore::default())
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)));

    let oidc_auth_layer = match OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
        Uri::try_from(config.app_base_url.clone()).expect("valid APP_BASE_URL"),
        config.issuer,
        config.client_id,
        Some(config.client_secret),
        vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
        ],
    )
    .await
    {
        Ok(layer) => layer,
        Err(error) => {
            tracing::error!("failed to discover OIDC provider: {:?}", error);
            return base_router;
        }
    };

    let oidc_login_service = ServiceBuilder::new()
        .layer(axum::error_handling::HandleErrorLayer::new(
            |err: MiddlewareError| async move {
                tracing::error!("OIDC login error: {:?}", err);
                err.into_response()
            },
        ))
        .layer(OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let oidc_auth_service = ServiceBuilder::new()
        .layer(axum::error_handling::HandleErrorLayer::new(
            |err: MiddlewareError| async move {
                tracing::error!("OIDC auth error: {:?}", err);
                err.into_response()
            },
        ))
        .layer(oidc_auth_layer);

    let protected_auth_router = Router::new()
        .route("/callback", get(callback))
        .route("/logout", get(logout))
        .layer(middleware::from_fn(sync_user_middleware))
        .layer(oidc_login_service.clone());

    base_router
        .route("/login", get(login))
        .merge(protected_auth_router)
        .route("/status", get(auth_status))
        .layer(Extension(state))
        .layer(oidc_auth_service)
        .layer(session_layer)
}

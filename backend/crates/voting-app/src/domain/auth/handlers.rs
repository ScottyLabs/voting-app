use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims, OidcRpInitiatedLogout};
use http::Uri;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthStatusResponse {
    pub logged_in: bool,
    pub user_id: Option<i32>,
}

pub async fn login(
    _claims: OidcClaims<EmptyAdditionalClaims>,
    user: SyncedUser,
    State(state): State<AppState>,
    Query(params): Query<LoginQuery>,
) -> impl IntoResponse {
    let _user_id = user.0.id;

    let redirect_to = params
        .redirect_uri
        .filter(|uri| uri.starts_with(&state.config.app_base_url))
        .unwrap_or_else(|| state.config.app_base_url.clone());

    Redirect::to(&redirect_to)
}

pub async fn logout(
    logout: OidcRpInitiatedLogout,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let post_logout_redirect = format!("{}/", state.config.app_base_url.trim_end_matches('/'));

    logout
        .with_post_logout_redirect(
            Uri::from_maybe_shared(post_logout_redirect).expect("valid APP_BASE_URL"),
        )
        .into_response()
}

pub async fn auth_status(user: Option<OidcClaims<EmptyAdditionalClaims>>) -> impl IntoResponse {
    let payload = AuthStatusResponse {
        logged_in: user.is_some(),
        user_id: None,
    };
    Json(payload)
}

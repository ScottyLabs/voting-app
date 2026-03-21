use axum::{
    body,
    extract::{FromRequestParts, OptionalFromRequestParts, State},
    middleware::Next,
    response::Response,
};
use axum_oidc::{EmptyAdditionalClaims, OidcClaims};
use entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use std::convert::Infallible;
use std::sync::Arc;

use crate::AppState;

#[derive(Clone)]
pub struct SyncedUser(pub Arc<user::Model>);

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

pub async fn sync_user_middleware(
    State(state): State<AppState>,
    claims: OidcClaims<EmptyAdditionalClaims>,
    mut request: http::Request<body::Body>,
    next: Next,
) -> Response {
    if request.extensions().get::<SyncedUser>().is_some() {
        return next.run(request).await;
    }

    let oidc_sub = claims.subject().to_string();
    let user = User::find()
        .filter(user::Column::Name.eq(&oidc_sub))
        .one(&state.db)
        .await
        .ok()
        .flatten();

    match user {
        Some(user) => {
            request.extensions_mut().insert(SyncedUser(Arc::new(user)));
        }
        None => {
            let new_user = user::ActiveModel {
                // Current schema only has `name`, so we store OIDC subject as a stable identity key.
                name: Set(oidc_sub.clone()),
                ..Default::default()
            };

            match new_user.insert(&state.db).await {
                Ok(created) => {
                    request.extensions_mut().insert(SyncedUser(Arc::new(created)));
                }
                Err(err) => {
                    tracing::error!("failed to create user from oidc claims: {:?}", err);
                }
            }
        }
    }

    next.run(request).await
}

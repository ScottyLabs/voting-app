use axum::{Router, routing::post};
use sea_orm::DbErr;

use super::{
    handler::{create, delete},
    state::{AppState, build_state},
};

pub fn router_with_state(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/delete", post(delete))
        .with_state(state)
}

pub async fn router() -> Result<Router, DbErr> {
    let state = build_state().await?;
    Ok(router_with_state(state))
}

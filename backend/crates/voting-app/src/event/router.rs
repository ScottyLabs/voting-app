use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DbErr;

use super::{
    handler::{create, get_by_id, remove},
    state::{AppState, build_state},
};

pub fn router_with_state(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/get", get(get_by_id))
        .route("/delete", post(remove))
        .with_state(state)
}

pub async fn router() -> Result<Router, DbErr> {
    let state = build_state().await?;
    Ok(router_with_state(state))
}

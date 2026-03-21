use axum::{
    Router,
    routing::{get, patch, post},
};
use sea_orm::DbErr;

use super::{
    handler::{close, create, display_timer, set_end_time},
    state::{AppState, build_state},
};

pub fn router_with_state(state: AppState) -> Router {
    Router::new()
        .route("/create_motion", post(create))
        .route("/end_motion", post(close))
        .route("/set_end_time", patch(set_end_time))
        .route("/display_timer", get(display_timer))
        .with_state(state)
}

pub async fn router() -> Result<Router, DbErr> {
    let state = build_state().await?;
    Ok(router_with_state(state))
}

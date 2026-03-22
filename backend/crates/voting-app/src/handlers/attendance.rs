use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use entity::organization_member;

#[axum::debug_handler]
pub async fn join(
    State(state): State<AppState>,
    Path(session_code): Path<String>,
) -> Result<Json<organization_member::Model>, (StatusCode, String)> {
    // TODO: replace with real auth middleware
    let user_id = 1;

    let event = state
        .store
        .events()
        .find_active_by_session_code(&session_code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("No active event for session code '{session_code}'"),
            )
        })?;

    let member = state
        .store
        .organization_members()
        .find_by_organization_and_user(event.organization_id, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| {
            (
                StatusCode::FORBIDDEN,
                format!("User {user_id} is not a member of this organization"),
            )
        })?;

    Ok(Json(member))
}

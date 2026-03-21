use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, FixedOffset, Utc};
use entity::{prelude::User, user};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use serde::{Deserialize, Serialize};

use super::state::AppState;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    Database(DbErr),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self::Database(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: message }),
            )
                .into_response(),
            Self::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse { error: message }),
            )
                .into_response(),
            Self::Database(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("database error: {error}"),
                }),
            )
                .into_response(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IdQuery {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct RecordResponse {
    pub id: i32,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub id: i32,
    pub deleted: bool,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> Result<(StatusCode, Json<RecordResponse>), ApiError> {
    if payload.name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "name is required and cannot be empty".to_owned(),
        ));
    }

    let created_at = match payload.created_at.as_deref() {
        Some(value) => parse_timestamp(value)?,
        None => Utc::now().fixed_offset(),
    };

    let model = user::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(payload.name),
        created_at: ActiveValue::Set(created_at),
    };

    let created = model.insert(&state.db).await?;
    Ok((
        StatusCode::CREATED,
        Json(RecordResponse {
            id: created.id,
            name: created.name,
            created_at: created.created_at.to_rfc3339(),
        }),
    ))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Query(query): Query<IdQuery>,
) -> Result<Json<RecordResponse>, ApiError> {
    let model = User::find_by_id(query.id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("user not found".to_owned()))?;

    Ok(Json(RecordResponse {
        id: model.id,
        name: model.name,
        created_at: model.created_at.to_rfc3339(),
    }))
}

pub async fn remove(
    State(state): State<AppState>,
    Query(query): Query<IdQuery>,
) -> Result<Json<DeleteResponse>, ApiError> {
    let existing = User::find_by_id(query.id).one(&state.db).await?;
    if existing.is_none() {
        return Err(ApiError::NotFound("user not found".to_owned()));
    }

    User::delete_by_id(query.id).exec(&state.db).await?;
    Ok(Json(DeleteResponse {
        id: query.id,
        deleted: true,
    }))
}

fn parse_timestamp(raw: &str) -> Result<DateTime<FixedOffset>, ApiError> {
    DateTime::parse_from_rfc3339(raw).map_err(|_| {
        ApiError::BadRequest("timestamp must be RFC3339, e.g. 2026-03-20T20:30:00Z".to_owned())
    })
}

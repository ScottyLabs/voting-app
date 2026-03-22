use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use entity::{organization, prelude::Organization};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

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
    pub data: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub id: i32,
    pub name: String,
    pub data: Value,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub id: i32,
    pub deleted: bool,
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> Result<(StatusCode, Json<CreateResponse>), ApiError> {
    if payload.name.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "name is required and cannot be empty".to_owned(),
        ));
    }

    let model = organization::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(payload.name),
        data: ActiveValue::Set(payload.data.unwrap_or_else(|| json!({}))),
    };

    let created = model.insert(&state.db).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateResponse {
            id: created.id,
            name: created.name,
            data: created.data,
        }),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    Json(payload): Json<DeleteRequest>,
) -> Result<Json<DeleteResponse>, ApiError> {
    let existing = Organization::find_by_id(payload.id).one(&state.db).await?;
    if existing.is_none() {
        return Err(ApiError::NotFound("organization not found".to_owned()));
    }

    Organization::delete_by_id(payload.id)
        .exec(&state.db)
        .await?;

    Ok(Json(DeleteResponse {
        id: payload.id,
        deleted: true,
    }))
}

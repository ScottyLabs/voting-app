use std::time::SystemTime;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::state::{AppState, StoredRecord};

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
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
            Self::Internal(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: message }),
            )
                .into_response(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub table: String,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Actor {
    User { user_id: i32, role: String },
    System(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Changes {
    pub before: Value,
    pub after: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogData {
    pub action: String,
    pub target: Target,
    pub actor: Actor,
    pub event_id: i32,
    pub changes: Changes,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub data: LogData,
}

#[derive(Debug, Deserialize)]
pub struct IdQuery {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct RecordResponse {
    pub id: i32,
    pub created_at: String,
    pub data: LogData,
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
    validate_data(&payload.data)?;

    let id = state.allocate_id();
    let created_at = SystemTime::now();
    let serialized = serde_json::to_value(payload.data.clone())
        .map_err(|_| ApiError::BadRequest("log.data is invalid".to_owned()))?;

    let mut records = state
        .records
        .lock()
        .map_err(|_| ApiError::Internal("failed to lock log records".to_owned()))?;
    records.push(StoredRecord {
        id,
        created_at,
        data: serialized,
    });

    Ok((
        StatusCode::CREATED,
        Json(RecordResponse {
            id,
            created_at: format_time(created_at),
            data: payload.data,
        }),
    ))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Query(query): Query<IdQuery>,
) -> Result<Json<RecordResponse>, ApiError> {
    let records = state
        .records
        .lock()
        .map_err(|_| ApiError::Internal("failed to lock log records".to_owned()))?;

    let record = records
        .iter()
        .find(|record| record.id == query.id)
        .ok_or_else(|| ApiError::NotFound("log record not found".to_owned()))?;

    let data: LogData = serde_json::from_value(record.data.clone())
        .map_err(|_| ApiError::Internal("stored log data is malformed".to_owned()))?;

    Ok(Json(RecordResponse {
        id: record.id,
        created_at: format_time(record.created_at),
        data,
    }))
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<RecordResponse>>, ApiError> {
    let records = state
        .records
        .lock()
        .map_err(|_| ApiError::Internal("failed to lock log records".to_owned()))?;

    let mut output = Vec::with_capacity(records.len());
    for record in records.iter() {
        let data: LogData = serde_json::from_value(record.data.clone())
            .map_err(|_| ApiError::Internal("stored log data is malformed".to_owned()))?;
        output.push(RecordResponse {
            id: record.id,
            created_at: format_time(record.created_at),
            data,
        });
    }

    Ok(Json(output))
}

pub async fn remove(
    State(state): State<AppState>,
    Query(query): Query<IdQuery>,
) -> Result<Json<DeleteResponse>, ApiError> {
    let mut records = state
        .records
        .lock()
        .map_err(|_| ApiError::Internal("failed to lock log records".to_owned()))?;

    let before_len = records.len();
    records.retain(|record| record.id != query.id);
    if records.len() == before_len {
        return Err(ApiError::NotFound("log record not found".to_owned()));
    }

    Ok(Json(DeleteResponse {
        id: query.id,
        deleted: true,
    }))
}

fn validate_data(data: &LogData) -> Result<(), ApiError> {
    if data.action.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "log.data.action cannot be empty".to_owned(),
        ));
    }
    if data.target.table.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "log.data.target.table cannot be empty".to_owned(),
        ));
    }
    match &data.actor {
        Actor::User { role, .. } if role.trim().is_empty() => {
            return Err(ApiError::BadRequest(
                "log.data.actor.role cannot be empty".to_owned(),
            ));
        }
        Actor::System(value) if value != "system" => {
            return Err(ApiError::BadRequest(
                "log.data.actor string must be \"system\"".to_owned(),
            ));
        }
        _ => {}
    }
    Ok(())
}

fn format_time(value: SystemTime) -> String {
    let timestamp: DateTime<Utc> = value.into();
    timestamp.to_rfc3339()
}

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, FixedOffset, Utc};
use entity::{event, prelude::Event};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use serde::{Deserialize, Serialize};

use super::{
    state::AppState,
    types::{EventData, default_event_data, is_supported_event_type, validate_event_data},
};

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
    pub event_type: String,
    pub name: String,
    pub status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub data: Option<EventData>,
    pub created_by_user_id: i32,
    pub organization_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct IdQuery {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct RecordResponse {
    pub id: i32,
    pub event_type: String,
    pub name: String,
    pub status: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub data: EventData,
    pub created_by_user_id: i32,
    pub organization_id: i32,
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
    if payload.event_type.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "event_type is required and cannot be empty".to_owned(),
        ));
    }

    let event_type = payload.event_type.trim().to_owned();
    if !is_supported_event_type(&event_type) {
        return Err(ApiError::BadRequest(
            "event_type must be one of attendance, motion, or vote".to_owned(),
        ));
    }

    let start_time = match payload.start_time.as_deref() {
        Some(value) => parse_timestamp(value)?,
        None => Utc::now().fixed_offset(),
    };

    let end_time = match payload.end_time.as_deref() {
        Some(value) => Some(parse_timestamp(value)?),
        None => None,
    };

    if let Some(value) = end_time.as_ref() {
        if value <= &start_time {
            return Err(ApiError::BadRequest(
                "end_time must be later than start_time".to_owned(),
            ));
        }
    }

    let data = payload
        .data
        .unwrap_or_else(|| default_event_data(&event_type));
    validate_event_data(&event_type, &data).map_err(ApiError::BadRequest)?;

    let model = event::ActiveModel {
        id: ActiveValue::NotSet,
        event_type: ActiveValue::Set(event_type),
        name: ActiveValue::Set(payload.name),
        status: ActiveValue::Set(payload.status.unwrap_or_else(|| "OPEN".to_owned())),
        start_time: ActiveValue::Set(start_time),
        end_time: ActiveValue::Set(end_time),
        data: ActiveValue::Set(
            serde_json::to_value(&data)
                .map_err(|_| ApiError::BadRequest("event.data is invalid".to_owned()))?,
        ),
        created_by_user_id: ActiveValue::Set(payload.created_by_user_id),
        organization_id: ActiveValue::Set(payload.organization_id),
    };

    let created = model.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(map_response(created)?)))
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Query(query): Query<IdQuery>,
) -> Result<Json<RecordResponse>, ApiError> {
    let model = Event::find_by_id(query.id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("event not found".to_owned()))?;

    Ok(Json(map_response(model)?))
}

pub async fn remove(
    State(state): State<AppState>,
    Query(query): Query<IdQuery>,
) -> Result<Json<DeleteResponse>, ApiError> {
    let existing = Event::find_by_id(query.id).one(&state.db).await?;
    if existing.is_none() {
        return Err(ApiError::NotFound("event not found".to_owned()));
    }

    Event::delete_by_id(query.id).exec(&state.db).await?;
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

fn map_response(model: event::Model) -> Result<RecordResponse, ApiError> {
    let data: EventData = serde_json::from_value(model.data)
        .unwrap_or_else(|_| default_event_data(&model.event_type));

    Ok(RecordResponse {
        id: model.id,
        event_type: model.event_type,
        name: model.name,
        status: model.status,
        start_time: model.start_time.to_rfc3339(),
        end_time: model.end_time.map(|value| value.to_rfc3339()),
        data,
        created_by_user_id: model.created_by_user_id,
        organization_id: model.organization_id,
    })
}

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, FixedOffset, Utc};
use entity::{
    event,
    prelude::{Event, Organization, OrganizationMember, User},
};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::state::AppState;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Forbidden(String),
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
            Self::Forbidden(message) => (
                StatusCode::FORBIDDEN,
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
    pub organization_id: i32,
    pub created_by_user_id: i32,
    pub name: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub event_type: Option<String>,
    pub data: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct CloseRequest {
    pub record_id: i32,
    pub ended_by_user_id: i32,
    pub status: Option<String>,
    pub close_data: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct SetEndTimeRequest {
    pub record_id: i32,
    pub updated_by_user_id: i32,
    pub end_time: String,
}

#[derive(Debug, Deserialize)]
pub struct TimerQuery {
    pub record_id: i32,
}

#[derive(Debug, Serialize)]
pub struct RecordResponse {
    pub record_id: i32,
    pub event_type: String,
    pub name: String,
    pub status: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub created_by_user_id: i32,
    pub organization_id: i32,
    pub data: Value,
}

#[derive(Debug, Serialize)]
pub struct TimerResponse {
    pub record_id: i32,
    pub status: String,
    pub now: String,
    pub end_time: Option<String>,
    pub seconds_remaining: Option<i64>,
    pub expired: bool,
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

    Organization::find_by_id(payload.organization_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("organization not found".to_owned()))?;

    User::find_by_id(payload.created_by_user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("user not found".to_owned()))?;

    let membership =
        OrganizationMember::find_by_id((payload.organization_id, payload.created_by_user_id))
            .one(&state.db)
            .await?;
    if membership.is_none() {
        return Err(ApiError::Forbidden(
            "user must be a member of the organization to create a motion".to_owned(),
        ));
    }

    let start_time = match payload.start_time.as_deref() {
        Some(input) => parse_timestamp(input)?,
        None => Utc::now().fixed_offset(),
    };

    let end_time = match payload.end_time.as_deref() {
        Some(input) => Some(parse_timestamp(input)?),
        None => None,
    };

    if let Some(end_time_value) = end_time.as_ref() {
        if end_time_value <= &start_time {
            return Err(ApiError::BadRequest(
                "end_time must be later than start_time".to_owned(),
            ));
        }
    }

    let event_type = payload
        .event_type
        .unwrap_or_else(|| state.default_type.clone());

    let model = event::ActiveModel {
        id: ActiveValue::NotSet,
        event_type: ActiveValue::Set(event_type),
        name: ActiveValue::Set(payload.name),
        status: ActiveValue::Set(state.open_status.clone()),
        start_time: ActiveValue::Set(start_time),
        end_time: ActiveValue::Set(end_time),
        data: ActiveValue::Set(payload.data.unwrap_or_else(|| json!({}))),
        created_by_user_id: ActiveValue::Set(payload.created_by_user_id),
        organization_id: ActiveValue::Set(payload.organization_id),
    };

    let created_record = model.insert(&state.db).await?;

    Ok((StatusCode::CREATED, Json(map_response(created_record))))
}

pub async fn close(
    State(state): State<AppState>,
    Json(payload): Json<CloseRequest>,
) -> Result<Json<RecordResponse>, ApiError> {
    let existing_record = Event::find_by_id(payload.record_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("record not found".to_owned()))?;

    validate_user_membership(
        &state,
        existing_record.organization_id,
        payload.ended_by_user_id,
    )
    .await?;

    let ended_at = Utc::now().fixed_offset();
    let next_status = payload
        .status
        .unwrap_or_else(|| state.closed_status.clone());

    let updated_data = append_metadata(
        existing_record.data.clone(),
        payload.close_data,
        payload.ended_by_user_id,
        ended_at,
    );

    let mut active_model: event::ActiveModel = existing_record.into();
    active_model.status = ActiveValue::Set(next_status);
    active_model.end_time = ActiveValue::Set(Some(ended_at));
    active_model.data = ActiveValue::Set(updated_data);

    let updated_record = active_model.update(&state.db).await?;
    Ok(Json(map_response(updated_record)))
}

pub async fn set_end_time(
    State(state): State<AppState>,
    Json(payload): Json<SetEndTimeRequest>,
) -> Result<Json<RecordResponse>, ApiError> {
    let existing_record = Event::find_by_id(payload.record_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("record not found".to_owned()))?;

    validate_user_membership(
        &state,
        existing_record.organization_id,
        payload.updated_by_user_id,
    )
    .await?;

    let next_end_time = parse_timestamp(&payload.end_time)?;
    if next_end_time <= existing_record.start_time {
        return Err(ApiError::BadRequest(
            "end_time must be later than start_time".to_owned(),
        ));
    }

    let metadata = append_end_time_update_metadata(
        existing_record.data.clone(),
        payload.updated_by_user_id,
        next_end_time,
    );

    let mut active_model: event::ActiveModel = existing_record.into();
    active_model.end_time = ActiveValue::Set(Some(next_end_time));
    active_model.data = ActiveValue::Set(metadata);

    let updated_record = active_model.update(&state.db).await?;
    Ok(Json(map_response(updated_record)))
}

pub async fn display_timer(
    State(state): State<AppState>,
    Query(query): Query<TimerQuery>,
) -> Result<Json<TimerResponse>, ApiError> {
    let record = Event::find_by_id(query.record_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("record not found".to_owned()))?;

    let now = Utc::now().fixed_offset();
    let end_time = record.end_time;
    let seconds_remaining = end_time
        .as_ref()
        .map(|value| value.signed_duration_since(now).num_seconds().max(0));
    let expired_from_time = end_time
        .as_ref()
        .map(|value| value <= &now)
        .unwrap_or(false);
    let expired_from_status = record.status == state.closed_status;

    Ok(Json(TimerResponse {
        record_id: record.id,
        status: record.status,
        now: now.to_rfc3339(),
        end_time: end_time.map(|value| value.to_rfc3339()),
        seconds_remaining,
        expired: expired_from_time || expired_from_status,
    }))
}

fn map_response(model: event::Model) -> RecordResponse {
    RecordResponse {
        record_id: model.id,
        event_type: model.event_type,
        name: model.name,
        status: model.status,
        start_time: model.start_time.to_rfc3339(),
        end_time: model.end_time.map(|value| value.to_rfc3339()),
        created_by_user_id: model.created_by_user_id,
        organization_id: model.organization_id,
        data: model.data,
    }
}

async fn validate_user_membership(
    state: &AppState,
    organization_id: i32,
    user_id: i32,
) -> Result<(), ApiError> {
    User::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("user not found".to_owned()))?;

    let membership = OrganizationMember::find_by_id((organization_id, user_id))
        .one(&state.db)
        .await?;

    if membership.is_none() {
        return Err(ApiError::Forbidden(
            "user must be a member of the organization".to_owned(),
        ));
    }

    Ok(())
}

fn parse_timestamp(raw: &str) -> Result<DateTime<FixedOffset>, ApiError> {
    DateTime::parse_from_rfc3339(raw).map_err(|_| {
        ApiError::BadRequest("timestamp must be RFC3339, e.g. 2026-03-20T20:30:00Z".to_owned())
    })
}

fn append_metadata(
    current_data: Value,
    close_data: Option<Value>,
    ended_by_user_id: i32,
    ended_at: DateTime<FixedOffset>,
) -> Value {
    let mut base = if current_data.is_object() {
        current_data
    } else {
        json!({ "payload": current_data })
    };

    if let Some(map) = base.as_object_mut() {
        if let Some(extra) = close_data {
            map.insert("close_data".to_owned(), extra);
        }
        map.insert("ended_by_user_id".to_owned(), json!(ended_by_user_id));
        map.insert("ended_at".to_owned(), json!(ended_at.to_rfc3339()));
    }

    base
}

fn append_end_time_update_metadata(
    current_data: Value,
    updated_by_user_id: i32,
    new_end_time: DateTime<FixedOffset>,
) -> Value {
    let mut base = if current_data.is_object() {
        current_data
    } else {
        json!({ "payload": current_data })
    };

    if let Some(map) = base.as_object_mut() {
        map.insert(
            "end_time_updated_by_user_id".to_owned(),
            json!(updated_by_user_id),
        );
        map.insert(
            "end_time_updated_to".to_owned(),
            json!(new_end_time.to_rfc3339()),
        );
    }

    base
}

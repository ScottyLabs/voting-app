use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use entity::vote;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use serde_json::json;
use voting_app_store::Store;

use crate::AppState;
use crate::core::auth::middleware::SyncedUser;

#[derive(Debug, Deserialize)]
pub struct CastVoteRequest {
    pub vote_response: Vec<String>,
    pub proxy: bool,
}

#[derive(Debug, Serialize)]
pub struct MotionResults {
    pub pass: u32,
    pub reject: u32,
    pub abstain: u32,
    pub total: u32,
    pub threshold: f64,
    pub passed: bool,
}

pub async fn cast_vote(
    user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
    Json(body): Json<CastVoteRequest>,
) -> impl IntoResponse {
    let store = Store::new(state.db.clone());

    let event = match store.events().find_by_id(event_id).await {
        Ok(Some(e)) => e,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"})))
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response()
        }
    };

    let event_data = event.data.clone();
    let vote_type = event_data["vote_type"].as_str().unwrap_or("");

    if (vote_type != "motion" || vote_type != "election") {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Event is not a motion"})),
        )
            .into_response();
    }

    if event.status != "open" { //CHANGE THIS WHEN YIYOUNG CHANGES TO ENUM  
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Event is not open"})),
        )
            .into_response();
    }

    if body.proxy && !event_data["proxy"].as_bool().unwrap_or(false) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Proxy voting is not allowed for this event"})),
        )
            .into_response();
    }

    if body.vote_response.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "vote_response cannot be empty"})),
        )
            .into_response();
    }

    let vote_options: Vec<String> = event_data["vote_options"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    if !vote_options.contains(&body.vote_response[0]) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid vote option"})),
        )
            .into_response();
    }

    let new_vote = vote::ActiveModel {
        event_id: Set(event_id),
        voter_id: Set(user.0.id),
        cast_time: Set(Utc::now().into()),
        proxy: Set(body.proxy),
        data: Set(json!({
            "vote_type": "motion",
            "vote_response": body.vote_response,
        })),
        ..Default::default()
    };

    match store.votes().create(new_vote).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({"message": "Vote cast successfully"})),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to cast vote"})),
        )
            .into_response(),
    }
}

pub async fn get_motion_results(
    _user: SyncedUser,
    State(state): State<AppState>,
    Path(event_id): Path<i32>,
) -> impl IntoResponse {
    let store = Store::new(state.db.clone());

    let event = match store.events().find_by_id(event_id).await {
        Ok(Some(e)) => e,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"})))
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response()
        }
    };

    let event_data = event.data.clone();
    let vote_type = event_data["vote_type"].as_str().unwrap_or("");

    if vote_type != "motion" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Event is not a motion"})),
        )
            .into_response();
    }

    //Place holder for when we figure the visibility out
    let visibility = event_data["visibility"]["participants"].as_str().unwrap_or("");
    if visibility == "hidden_until_release" && event.status != "closed" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Results are not yet available"})),
        )
            .into_response();
    }

    let votes = match store.votes().find_by_event_id(event_id).await {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
                .into_response()
        }
    };

    let mut pass = 0u32;
    let mut reject = 0u32;
    let mut abstain = 0u32;

    for vote in &votes {
        match vote.data["vote_response"][0]
            .as_str()
            .unwrap_or("")
            .to_lowercase()
            .as_str()
        {
            "pass" => pass += 1,
            "reject" => reject += 1,
            "abstain" => abstain += 1,
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Unrecognized vote option in database"})),
                )
                    .into_response()
            }
        }
    }

    let total = pass + reject + abstain;
    let threshold = event_data["threshold"].as_f64().unwrap_or(0.5);
    let passed = total > 0 && (pass as f64 / (pass + reject) as f64) >= threshold;

    (
        StatusCode::OK,
        Json(MotionResults {
            pass,
            reject,
            abstain,
            total,
            threshold,
            passed,
        }),
    )
        .into_response()
}

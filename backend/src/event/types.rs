use serde::{Deserialize, Serialize};

pub const EVENT_TYPE_ATTENDANCE: &str = "attendance";
pub const EVENT_TYPE_MOTION: &str = "motion";
pub const EVENT_TYPE_VOTE: &str = "vote";

pub const SUPPORTED_EVENT_TYPES: [&str; 3] = [
    EVENT_TYPE_ATTENDANCE,
    EVENT_TYPE_MOTION,
    EVENT_TYPE_VOTE,
];

pub fn is_supported_event_type(event_type: &str) -> bool {
    SUPPORTED_EVENT_TYPES.contains(&event_type)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visibility {
    pub participants: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceEventData {
    pub description: String,
    pub session_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionEventData {
    pub description: String,
    pub threshold: f64,
    pub visibility: Visibility,
    pub proxy: bool,
    pub vote_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteEventData {
    pub description: String,
    pub vote_type: String,
    pub threshold: f64,
    pub visibility: Visibility,
    pub proxy: bool,
    pub vote_options: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendance: Option<AttendanceEventData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motion: Option<MotionEventData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vote: Option<VoteEventData>,
}

pub fn default_event_data(event_type: &str) -> EventData {
    match event_type {
        EVENT_TYPE_ATTENDANCE => EventData {
            attendance: Some(AttendanceEventData {
                description: "Attendance session".to_owned(),
                session_code: "attendance-session".to_owned(),
            }),
            motion: None,
            vote: None,
        },
        EVENT_TYPE_MOTION => EventData {
            attendance: None,
            motion: Some(MotionEventData {
                description: "Motion event".to_owned(),
                threshold: 0.5,
                visibility: Visibility {
                    participants: "hidden_until_release".to_owned(),
                },
                proxy: false,
                vote_options: vec!["yes".to_owned(), "no".to_owned()],
            }),
            vote: None,
        },
        EVENT_TYPE_VOTE => EventData {
            attendance: None,
            motion: None,
            vote: Some(VoteEventData {
                description: "Vote event".to_owned(),
                vote_type: "motion".to_owned(),
                threshold: 0.5,
                visibility: Visibility {
                    participants: "hidden_until_release".to_owned(),
                },
                proxy: false,
                vote_options: vec!["option1".to_owned(), "option2".to_owned()],
            }),
        },
        _ => EventData::default(),
    }
}

pub fn validate_event_data(event_type: &str, data: &EventData) -> Result<(), String> {
    let provided_fields = usize::from(data.attendance.is_some())
        + usize::from(data.motion.is_some())
        + usize::from(data.vote.is_some());

    if provided_fields != 1 {
        return Err("event.data must include exactly one of attendance, motion, or vote".to_owned());
    }

    match event_type {
        EVENT_TYPE_ATTENDANCE => {
            let attendance = data.attendance.as_ref().ok_or_else(|| {
                "event.data.attendance is required for attendance".to_owned()
            })?;
            validate_attendance_data(attendance)
        }
        EVENT_TYPE_MOTION => {
            let motion = data
                .motion
                .as_ref()
                .ok_or_else(|| "event.data.motion is required for motion".to_owned())?;
            validate_motion_data(motion)
        }
        EVENT_TYPE_VOTE => {
            let vote = data
                .vote
                .as_ref()
                .ok_or_else(|| "event.data.vote is required for vote".to_owned())?;
            validate_vote_data(vote)
        }
        _ => Err("event_type must be one of attendance, motion, or vote".to_owned()),
    }
}

fn validate_attendance_data(data: &AttendanceEventData) -> Result<(), String> {
    validate_non_empty(
        &data.session_code,
        "event.data.attendance.session_code cannot be empty",
    )
}

fn validate_motion_data(data: &MotionEventData) -> Result<(), String> {
    validate_threshold(data.threshold, "event.data.motion.threshold")?;
    validate_non_empty(
        &data.visibility.participants,
        "event.data.motion.visibility.participants cannot be empty",
    )?;
    validate_vote_options(&data.vote_options, "event.data.motion.vote_options")
}

fn validate_vote_data(data: &VoteEventData) -> Result<(), String> {
    validate_non_empty(&data.description, "event.data.vote.description cannot be empty")?;
    validate_non_empty(&data.vote_type, "event.data.vote.vote_type cannot be empty")?;
    validate_threshold(data.threshold, "event.data.vote.threshold")?;
    validate_non_empty(
        &data.visibility.participants,
        "event.data.vote.visibility.participants cannot be empty",
    )?;
    validate_vote_options(&data.vote_options, "event.data.vote.vote_options")
}

fn validate_non_empty(value: &str, message: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(message.to_owned());
    }
    Ok(())
}

fn validate_threshold(value: f64, field_name: &str) -> Result<(), String> {
    if !(0.0..=1.0).contains(&value) {
        return Err(format!("{field_name} must be between 0 and 1"));
    }
    Ok(())
}

fn validate_vote_options(options: &[String], field_name: &str) -> Result<(), String> {
    if options.is_empty() {
        return Err(format!(
            "{field_name} must include at least one option"
        ));
    }
    if options.iter().any(|option| option.trim().is_empty()) {
        return Err(format!("{field_name} cannot include empty values"));
    }
    Ok(())
}

// Harana Actions - Zoom Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

// create_meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMeetingOutput {
    pub join_url: String,
    pub meeting_id: String,
    pub password: Option<String>,
    pub start_url: String,
    pub success: bool,
}

// get_meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeetingOutput {
    pub agenda: Option<String>,
    pub duration: i32,
    pub host_id: String,
    pub join_url: String,
    pub meeting_id: String,
    pub password: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub status: String,
    pub timezone: Option<String>,
    pub topic: String,
    #[serde(rename = "type")]
    pub meeting_type: i32,
}

// update_meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMeetingOutput {
    pub success: bool,
}

// delete_meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMeetingOutput {
    pub success: bool,
}

// list_meetings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMeetingsOutput {
    pub meetings: Vec<ZoomMeeting>,
    pub next_page_token: Option<String>,
    pub total_records: i32,
}

// add_registrant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRegistrantOutput {
    pub join_url: String,
    pub registrant_id: String,
    pub success: bool,
}

// list_registrants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRegistrantsOutput {
    pub next_page_token: Option<String>,
    pub registrants: Vec<ZoomRegistrant>,
    pub total_records: i32,
}

// update_registrant_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRegistrantStatusOutput {
    pub success: bool,
}

// get_meeting_recordings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeetingRecordingsOutput {
    pub duration: i32,
    pub recording_files: Vec<ZoomRecordingFile>,
    pub share_url: Option<String>,
    pub total_size: i64,
}

// delete_recording
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRecordingOutput {
    pub success: bool,
}

// get_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    pub created_at: DateTime<Utc>,
    pub email: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub role_id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub user_type: i32,
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub next_page_token: Option<String>,
    pub total_records: i32,
    pub users: Vec<ZoomUser>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomMeeting {
    pub id: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub meeting_type: i32,
    pub start_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub timezone: Option<String>,
    pub join_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomRegistrant {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub status: String,
    pub create_time: DateTime<Utc>,
    pub join_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomRecordingFile {
    pub id: String,
    pub file_type: String,
    pub file_size: i64,
    pub download_url: String,
    pub play_url: Option<String>,
    pub recording_start: DateTime<Utc>,
    pub recording_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomUser {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(rename = "type")]
    pub user_type: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomMeetingSettings {
    pub host_video: Option<bool>,
    pub participant_video: Option<bool>,
    pub join_before_host: Option<bool>,
    pub mute_upon_entry: Option<bool>,
    pub waiting_room: Option<bool>,
    pub approval_type: Option<i32>,
    pub auto_recording: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomRecurrence {
    #[serde(rename = "type")]
    pub recurrence_type: i32,
    pub repeat_interval: Option<i32>,
    pub weekly_days: Option<String>,
    pub monthly_day: Option<i32>,
    pub monthly_week: Option<i32>,
    pub monthly_week_day: Option<i32>,
    pub end_times: Option<i32>,
    pub end_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomCustomQuestion {
    pub title: String,
    pub value: String,
}

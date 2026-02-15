// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomMeeting {
    pub agenda: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration: i64,
    pub host_id: Option<String>,
    pub join_url: Option<String>,
    pub meeting_id: String,
    pub password: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub start_url: Option<String>,
    pub status: String,
    pub timezone: Option<String>,
    pub topic: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ZoomMeeting {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomMeetingRegistrant {
    pub email: String,
    pub first_name: String,
    pub join_url: Option<String>,
    pub last_name: Option<String>,
    pub meeting_id: String,
    pub phone: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub registered_at: chrono::DateTime<chrono::Utc>,
    pub registrant_id: String,
    pub status: String,
}

impl ZoomMeetingRegistrant {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomMeetingParticipant {
    pub duration_seconds: Option<i64>,
    pub email: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub join_time: chrono::DateTime<chrono::Utc>,
    pub leave_time: Option<chrono::DateTime<chrono::Utc>>,
    pub meeting_id: String,
    pub name: Option<String>,
    pub user_id: Option<String>,
}

impl ZoomMeetingParticipant {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomWebinar {
    pub agenda: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration: i64,
    pub host_id: Option<String>,
    pub join_url: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub timezone: Option<String>,
    pub topic: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub webinar_id: String,
}

impl ZoomWebinar {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomUser {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: String,
    pub timezone: Option<String>,
    pub type: i64,
    pub user_id: String,
}

impl ZoomUser {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomRecording {
    pub download_url: Option<String>,
    pub duration_seconds: Option<i64>,
    pub file_size: Option<i64>,
    pub file_type: String,
    pub meeting_id: String,
    pub recording_id: String,
    pub share_url: Option<String>,
    pub status: String,
}

impl ZoomRecording {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomRecurrence {
    pub type: String,
    pub repeat_interval: i64,
    pub weekly_days: String,
    pub monthly_day: i64,
    pub monthly_week: i64,
    pub monthly_week_day: i64,
    pub end_times: i64,
    pub end_date_time: chrono::DateTime<chrono::Utc>,
}

impl ZoomRecurrence {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomMeetingSettings {
    pub host_video: bool,
    pub participant_video: bool,
    pub join_before_host: bool,
    pub mute_upon_entry: bool,
    pub watermark: bool,
    pub use_pmi: bool,
    pub approval_type: i64,
    pub registration_type: i64,
    pub audio: String,
    pub auto_recording: String,
    pub waiting_room: bool,
}

impl ZoomMeetingSettings {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomWebinarSettings {
    pub host_video: bool,
    pub panelists_video: bool,
    pub practice_session: bool,
    pub hd_video: bool,
    pub approval_type: i64,
    pub registration_type: i64,
    pub audio: String,
    pub auto_recording: String,
    pub close_registration: bool,
    pub show_share_button: bool,
    pub allow_multiple_devices: bool,
}

impl ZoomWebinarSettings {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomCustomQuestion {
    pub title: String,
    pub value: String,
}

impl ZoomCustomQuestion {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomRegistrant {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub join_url: String,
}

impl ZoomRegistrant {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomRegistrantId {
    pub id: String,
    pub email: String,
}

impl ZoomRegistrantId {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomParticipant {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub join_time: chrono::DateTime<chrono::Utc>,
    pub leave_time: chrono::DateTime<chrono::Utc>,
    pub duration: i64,
    pub attentiveness_score: String,
    pub status: String,
}

impl ZoomParticipant {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomRecordingFile {
    pub id: String,
    pub meeting_id: String,
    pub recording_start: chrono::DateTime<chrono::Utc>,
    pub recording_end: chrono::DateTime<chrono::Utc>,
    pub file_type: String,
    pub file_size: i64,
    pub download_url: String,
    pub play_url: String,
    pub status: String,
}

impl ZoomRecordingFile {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


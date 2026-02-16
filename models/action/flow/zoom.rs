// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMeetingOutput {
    pub join_url: String,
    pub meeting_id: String,
    pub password: String,
    pub start_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMeetingOutput {
    pub agenda: String,
    pub duration: i64,
    pub host_id: String,
    pub join_url: String,
    pub meeting_id: String,
    pub password: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub timezone: String,
    pub topic: String,
    pub type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMeetingsOutput {
    pub meetings: Vec<String>,
    pub next_page_token: String,
    pub total_records: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddRegistrantOutput {
    pub join_url: String,
    pub registrant_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRegistrantsOutput {
    pub next_page_token: String,
    pub registrants: Vec<String>,
    pub total_records: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetParticipantsOutput {
    pub next_page_token: String,
    pub participants: Vec<String>,
    pub total_records: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateWebinarOutput {
    pub join_url: String,
    pub password: String,
    pub start_url: String,
    pub webinar_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWebinarOutput {
    pub agenda: String,
    pub duration: i64,
    pub host_id: String,
    pub join_url: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub topic: String,
    pub type: i64,
    pub webinar_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
    pub timezone: String,
    pub type: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersOutput {
    pub next_page_token: String,
    pub total_records: i64,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRecordingOutput {
    pub download_url: String,
    pub duration: i64,
    pub recording_files: Vec<String>,
    pub share_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRecordingsOutput {
    pub next_page_token: String,
    pub recordings: Vec<String>,
    pub total_records: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomMeeting {
    pub meeting_id: String,
    pub topic: String,
    pub type: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration: i64,
    pub timezone: String,
    pub password: String,
    pub join_url: String,
    pub start_url: String,
    pub host_id: String,
    pub status: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomCustomQuestion {
    pub title: String,
    pub value: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomRegistrantId {
    pub id: String,
    pub email: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomUser {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub type: i64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomRecording {
    pub id: String,
    pub meeting_id: String,
    pub topic: String,
    pub type: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration: i64,
    pub total_size: i64,
    pub recording_count: i64,
    pub recording_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomMeetingRegistrant {
    pub email: String,
    pub first_name: String,
    pub join_url: String,
    pub last_name: String,
    pub meeting_id: String,
    pub phone: String,
    #[serde(default = "chrono::Utc::now")]
    pub registered_at: chrono::DateTime<chrono::Utc>,
    pub registrant_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomMeetingParticipant {
    pub duration_seconds: i64,
    pub email: String,
    #[serde(default = "chrono::Utc::now")]
    pub join_time: chrono::DateTime<chrono::Utc>,
    pub leave_time: chrono::DateTime<chrono::Utc>,
    pub meeting_id: String,
    pub name: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZoomWebinar {
    pub agenda: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration: i64,
    pub host_id: String,
    pub join_url: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub topic: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub webinar_id: String,
}

#[async_trait]
pub trait ZoomAction {
    async fn create_meeting(&self, agenda: String, duration: i64, password: String, recurrence: String, settings: String, start_time: chrono::DateTime<chrono::Utc>, timezone: String, topic: String, type: String) -> Result<CreateMeetingOutput, Box<dyn std::error::Error>>;
    async fn get_meeting(&self, meeting_id: String) -> Result<GetMeetingOutput, Box<dyn std::error::Error>>;
    async fn update_meeting(&self, agenda: String, duration: i64, meeting_id: String, password: String, settings: String, start_time: chrono::DateTime<chrono::Utc>, timezone: String, topic: String, type: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_meeting(&self, meeting_id: String, occurrence_id: String, schedule_for_reminder: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_meetings(&self, next_page_token: String, page_size: i64, type: String, user_id: String) -> Result<ListMeetingsOutput, Box<dyn std::error::Error>>;
    async fn add_registrant(&self, custom_questions: Vec<String>, email: String, first_name: String, last_name: String, meeting_id: String, phone: String) -> Result<AddRegistrantOutput, Box<dyn std::error::Error>>;
    async fn list_registrants(&self, meeting_id: String, next_page_token: String, page_size: i64, status: String) -> Result<ListRegistrantsOutput, Box<dyn std::error::Error>>;
    async fn update_registrant_status(&self, method: String, meeting_id: String, registrants: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_participants(&self, meeting_id: String, next_page_token: String, page_size: i64, type: String) -> Result<GetParticipantsOutput, Box<dyn std::error::Error>>;
    async fn create_webinar(&self, agenda: String, duration: i64, password: String, settings: String, start_time: chrono::DateTime<chrono::Utc>, timezone: String, topic: String, type: String) -> Result<CreateWebinarOutput, Box<dyn std::error::Error>>;
    async fn get_webinar(&self, webinar_id: String) -> Result<GetWebinarOutput, Box<dyn std::error::Error>>;
    async fn delete_webinar(&self, occurrence_id: String, webinar_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_user(&self, user_id: String) -> Result<GetUserOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, next_page_token: String, page_size: i64, role_id: String, status: String) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
    async fn get_recording(&self, meeting_id: String) -> Result<GetRecordingOutput, Box<dyn std::error::Error>>;
    async fn delete_recording(&self, method: String, meeting_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_recordings(&self, from_date: chrono::DateTime<chrono::Utc>, next_page_token: String, page_size: i64, to_date: chrono::DateTime<chrono::Utc>, user_id: String) -> Result<ListRecordingsOutput, Box<dyn std::error::Error>>;
    async fn end_meeting(&self, meeting_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_invitation(&self, emails: Vec<String>, meeting_id: String) -> Result<(), Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMeetingInput {
    pub agenda: String,
    pub duration: i64,
    pub password: String,
    pub recurrence: String,
    pub settings: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub topic: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMeetingOutput {
    pub join_url: String,
    pub meeting_id: String,
    pub password: String,
    pub start_url: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMeetingInput {
    pub meeting_id: String,
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
pub struct UpdateMeetingInput {
    pub agenda: String,
    pub duration: i64,
    pub meeting_id: String,
    pub password: String,
    pub settings: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub topic: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMeetingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMeetingInput {
    pub meeting_id: String,
    pub occurrence_id: String,
    #[serde(default)]
    pub schedule_for_reminder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMeetingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMeetingsInput {
    pub next_page_token: String,
    pub page_size: i64,
    pub type: String,
    pub user_id: String,
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
pub struct AddRegistrantInput {
    pub custom_questions: Vec<String>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub meeting_id: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddRegistrantOutput {
    pub join_url: String,
    pub registrant_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRegistrantsInput {
    pub meeting_id: String,
    pub next_page_token: String,
    pub page_size: i64,
    pub status: String,
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
pub struct UpdateRegistrantStatusInput {
    pub method: String,
    pub meeting_id: String,
    pub registrants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateRegistrantStatusOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetParticipantsInput {
    pub meeting_id: String,
    pub next_page_token: String,
    pub page_size: i64,
    pub type: String,
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
pub struct CreateWebinarInput {
    pub agenda: String,
    pub duration: i64,
    pub password: String,
    pub settings: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub topic: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateWebinarOutput {
    pub join_url: String,
    pub password: String,
    pub start_url: String,
    pub success: bool,
    pub webinar_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWebinarInput {
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
pub struct DeleteWebinarInput {
    pub occurrence_id: String,
    pub webinar_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteWebinarOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserInput {
    pub user_id: String,
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
pub struct ListUsersInput {
    pub next_page_token: String,
    pub page_size: i64,
    pub role_id: String,
    pub status: String,
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
pub struct GetRecordingInput {
    pub meeting_id: String,
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
pub struct DeleteRecordingInput {
    pub method: String,
    pub meeting_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRecordingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRecordingsInput {
    pub from_date: chrono::DateTime<chrono::Utc>,
    pub next_page_token: String,
    pub page_size: i64,
    pub to_date: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
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
pub struct EndMeetingInput {
    pub meeting_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EndMeetingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendInvitationInput {
    pub emails: Vec<String>,
    pub meeting_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendInvitationOutput {
    pub success: bool,
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

#[async_trait]
pub trait ZoomAction {
    async fn create_meeting(&self, input: CreateMeetingInput) -> Result<CreateMeetingOutput, Box<dyn std::error::Error>>;
    async fn get_meeting(&self, input: GetMeetingInput) -> Result<GetMeetingOutput, Box<dyn std::error::Error>>;
    async fn update_meeting(&self, input: UpdateMeetingInput) -> Result<UpdateMeetingOutput, Box<dyn std::error::Error>>;
    async fn delete_meeting(&self, input: DeleteMeetingInput) -> Result<DeleteMeetingOutput, Box<dyn std::error::Error>>;
    async fn list_meetings(&self, input: ListMeetingsInput) -> Result<ListMeetingsOutput, Box<dyn std::error::Error>>;
    async fn add_registrant(&self, input: AddRegistrantInput) -> Result<AddRegistrantOutput, Box<dyn std::error::Error>>;
    async fn list_registrants(&self, input: ListRegistrantsInput) -> Result<ListRegistrantsOutput, Box<dyn std::error::Error>>;
    async fn update_registrant_status(&self, input: UpdateRegistrantStatusInput) -> Result<UpdateRegistrantStatusOutput, Box<dyn std::error::Error>>;
    async fn get_participants(&self, input: GetParticipantsInput) -> Result<GetParticipantsOutput, Box<dyn std::error::Error>>;
    async fn create_webinar(&self, input: CreateWebinarInput) -> Result<CreateWebinarOutput, Box<dyn std::error::Error>>;
    async fn get_webinar(&self, input: GetWebinarInput) -> Result<GetWebinarOutput, Box<dyn std::error::Error>>;
    async fn delete_webinar(&self, input: DeleteWebinarInput) -> Result<DeleteWebinarOutput, Box<dyn std::error::Error>>;
    async fn get_user(&self, input: GetUserInput) -> Result<GetUserOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, input: ListUsersInput) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
    async fn get_recording(&self, input: GetRecordingInput) -> Result<GetRecordingOutput, Box<dyn std::error::Error>>;
    async fn delete_recording(&self, input: DeleteRecordingInput) -> Result<DeleteRecordingOutput, Box<dyn std::error::Error>>;
    async fn list_recordings(&self, input: ListRecordingsInput) -> Result<ListRecordingsOutput, Box<dyn std::error::Error>>;
    async fn end_meeting(&self, input: EndMeetingInput) -> Result<EndMeetingOutput, Box<dyn std::error::Error>>;
    async fn send_invitation(&self, input: SendInvitationInput) -> Result<SendInvitationOutput, Box<dyn std::error::Error>>;
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// === Action Outputs ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMeetingOutput {
    pub meeting_id: String,
    pub join_url: String,
    pub start_url: String,
    pub password: Option<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeetingOutput {
    pub meeting_id: String,
    pub topic: String,
    pub r#type: i32,
    pub start_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub timezone: String,
    pub agenda: Option<String>,
    pub password: Option<String>,
    pub join_url: String,
    pub host_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMeetingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMeetingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMeetingsOutput {
    pub meetings: Vec<ZoomMeeting>,
    pub total_records: i32,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRegistrantOutput {
    pub registrant_id: String,
    pub join_url: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRegistrantsOutput {
    pub registrants: Vec<ZoomRegistrant>,
    pub total_records: i32,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRegistrantStatusOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParticipantsOutput {
    pub participants: Vec<ZoomParticipant>,
    pub total_records: i32,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebinarOutput {
    pub webinar_id: String,
    pub join_url: String,
    pub start_url: String,
    pub password: Option<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebinarOutput {
    pub webinar_id: String,
    pub topic: String,
    pub r#type: i32,
    pub start_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub timezone: String,
    pub agenda: Option<String>,
    pub join_url: String,
    pub host_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebinarOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    pub user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub r#type: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub users: Vec<ZoomUser>,
    pub total_records: i32,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecordingOutput {
    pub download_url: Option<String>,
    pub share_url: Option<String>,
    pub duration: i32,
    pub recording_files: Vec<ZoomRecordingFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRecordingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRecordingsOutput {
    pub recordings: Vec<ZoomRecording>,
    pub total_records: i32,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndMeetingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInvitationOutput {
    pub success: bool,
}

// === Class Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomMeeting {
    pub meeting_id: String,
    pub topic: String,
    pub r#type: i32,
    pub start_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub timezone: String,
    pub password: Option<String>,
    pub join_url: String,
    pub start_url: String,
    pub host_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZoomRecurrence {
    pub r#type: Option<RecurrenceType>,
    pub repeat_interval: Option<i32>,
    pub weekly_days: Option<String>,
    pub monthly_day: Option<i32>,
    pub monthly_week: Option<i32>,
    pub monthly_week_day: Option<i32>,
    pub end_times: Option<i32>,
    pub end_date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZoomMeetingSettings {
    pub host_video: Option<bool>,
    pub participant_video: Option<bool>,
    pub join_before_host: Option<bool>,
    pub mute_upon_entry: Option<bool>,
    pub watermark: Option<bool>,
    pub use_pmi: Option<bool>,
    pub approval_type: Option<i32>,
    pub registration_type: Option<i32>,
    pub audio: Option<String>,
    pub auto_recording: Option<String>,
    pub waiting_room: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZoomWebinarSettings {
    pub host_video: Option<bool>,
    pub panelists_video: Option<bool>,
    pub practice_session: Option<bool>,
    pub hd_video: Option<bool>,
    pub approval_type: Option<i32>,
    pub registration_type: Option<i32>,
    pub audio: Option<String>,
    pub auto_recording: Option<String>,
    pub close_registration: Option<bool>,
    pub show_share_button: Option<bool>,
    pub allow_multiple_devices: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomCustomQuestion {
    pub title: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomRegistrant {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub status: String,
    pub create_time: DateTime<Utc>,
    pub join_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomRegistrantId {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomParticipant {
    pub id: String,
    pub user_id: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub join_time: DateTime<Utc>,
    pub leave_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub attentiveness_score: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomUser {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub r#type: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomRecordingFile {
    pub id: String,
    pub meeting_id: String,
    pub recording_start: DateTime<Utc>,
    pub recording_end: DateTime<Utc>,
    pub file_type: String,
    pub file_size: i64,
    pub download_url: String,
    pub play_url: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomRecording {
    pub id: String,
    pub meeting_id: String,
    pub topic: String,
    pub r#type: i32,
    pub start_time: DateTime<Utc>,
    pub duration: i32,
    pub total_size: i64,
    pub recording_count: i32,
    pub recording_files: Vec<ZoomRecordingFile>,
}

// === Enums ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeetingType {
    Instant = 1,
    Scheduled = 2,
    RecurringUnfixed = 3,
    RecurringFixed = 8,
}

impl Default for MeetingType {
    fn default() -> Self {
        MeetingType::Scheduled
    }
}

impl MeetingType {
    pub fn as_i32(&self) -> i32 {
        match self {
            MeetingType::Instant => 1,
            MeetingType::Scheduled => 2,
            MeetingType::RecurringUnfixed => 3,
            MeetingType::RecurringFixed => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebinarType {
    Webinar = 5,
    RecurringWebinarUnfixed = 6,
    RecurringWebinarFixed = 9,
}

impl Default for WebinarType {
    fn default() -> Self {
        WebinarType::Webinar
    }
}

impl WebinarType {
    pub fn as_i32(&self) -> i32 {
        match self {
            WebinarType::Webinar => 5,
            WebinarType::RecurringWebinarUnfixed => 6,
            WebinarType::RecurringWebinarFixed => 9,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrenceType {
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListMeetingsType {
    Scheduled,
    Live,
    Upcoming,
    UpcomingMeetings,
    PreviousMeetings,
}

impl Default for ListMeetingsType {
    fn default() -> Self {
        ListMeetingsType::Scheduled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrantStatus {
    Pending,
    Approved,
    Denied,
}

impl Default for RegistrantStatus {
    fn default() -> Self {
        RegistrantStatus::Approved
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrantAction {
    Approve,
    Deny,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticipantType {
    Past,
    PastOne,
    Live,
}

impl Default for ParticipantType {
    fn default() -> Self {
        ParticipantType::Live
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Inactive,
    Pending,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Active
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingDeleteAction {
    Trash,
    Delete,
}

impl Default for RecordingDeleteAction {
    fn default() -> Self {
        RecordingDeleteAction::Trash
    }
}

// === Internal Storage Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMeeting {
    pub meeting_id: String,
    pub topic: String,
    pub meeting_type: MeetingType,
    pub start_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub timezone: String,
    pub agenda: Option<String>,
    pub password: Option<String>,
    pub join_url: String,
    pub start_url: String,
    pub host_id: String,
    pub status: String,
    pub settings: ZoomMeetingSettings,
    pub recurrence: Option<ZoomRecurrence>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredWebinar {
    pub webinar_id: String,
    pub topic: String,
    pub webinar_type: WebinarType,
    pub start_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub timezone: String,
    pub agenda: Option<String>,
    pub password: Option<String>,
    pub join_url: String,
    pub start_url: String,
    pub host_id: String,
    pub settings: ZoomWebinarSettings,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredRegistrant {
    pub id: String,
    pub meeting_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub custom_questions: Vec<ZoomCustomQuestion>,
    pub status: RegistrantStatus,
    pub join_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredUser {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_type: i32,
    pub status: UserStatus,
    pub role_id: Option<String>,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredRecording {
    pub id: String,
    pub meeting_id: String,
    pub topic: String,
    pub recording_type: i32,
    pub start_time: DateTime<Utc>,
    pub duration: i32,
    pub total_size: i64,
    pub files: Vec<StoredRecordingFile>,
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredRecordingFile {
    pub id: String,
    pub recording_start: DateTime<Utc>,
    pub recording_end: DateTime<Utc>,
    pub file_type: String,
    pub file_size: i64,
    pub download_url: String,
    pub play_url: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredParticipant {
    pub id: String,
    pub meeting_id: String,
    pub user_id: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub join_time: DateTime<Utc>,
    pub leave_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub status: String,
}

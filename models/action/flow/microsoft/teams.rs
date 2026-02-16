// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateChannelOutput {
    pub channel_id: String,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListChannelsOutput {
    pub channels: Vec<String>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetChannelMessagesOutput {
    pub messages: Vec<String>,
    pub next_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMeetingOutput {
    pub join_url: String,
    pub meeting_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMeetingOutput {
    pub attendees: Vec<String>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub join_url: String,
    pub meeting_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTeamsOutput {
    pub count: i64,
    pub teams: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTeamOutput {
    pub description: String,
    pub display_name: String,
    pub team_id: String,
    pub visibility: String,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTeamMembersOutput {
    pub count: i64,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadFileOutput {
    pub file_id: String,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMessage {
    pub message_id: String,
    pub channel_id: String,
    pub team_id: String,
    pub content: String,
    pub content_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub from_user: String,
    pub importance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsUser {
    pub user_id: String,
    pub display_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsAttachment {
    pub id: String,
    pub content_type: String,
    pub content_url: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMention {
    pub id: String,
    pub mentioned_id: String,
    pub mentioned_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsChannel {
    pub channel_id: String,
    pub display_name: String,
    pub description: String,
    pub membership_type: String,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsAttendee {
    pub email: String,
    pub display_name: String,
    pub response_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsTeam {
    pub team_id: String,
    pub display_name: String,
    pub description: String,
    pub visibility: String,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMember {
    pub membership_id: String,
    pub user_id: String,
    pub display_name: String,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsAdaptiveCard {
    pub type: String,
    pub version: String,
    pub body: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMeeting {
    pub body: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_online: bool,
    pub join_url: String,
    pub location: String,
    pub meeting_id: String,
    pub organizer_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub subject: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait TeamsAction {
    async fn send_message(&self, attachments: Vec<String>, channel_id: String, content: String, content_type: String, importance: String, mentions: Vec<String>, team_id: String) -> Result<SendMessageOutput, Box<dyn std::error::Error>>;
    async fn reply_to_message(&self, attachments: Vec<String>, channel_id: String, content: String, content_type: String, message_id: String, team_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn update_message(&self, channel_id: String, content: String, content_type: String, message_id: String, team_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_message(&self, channel_id: String, message_id: String, team_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_chat_message(&self, attachments: Vec<String>, chat_id: String, content: String, content_type: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_channel(&self, description: String, display_name: String, membership_type: String, team_id: String) -> Result<CreateChannelOutput, Box<dyn std::error::Error>>;
    async fn delete_channel(&self, channel_id: String, team_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_channels(&self, filter: String, team_id: String) -> Result<ListChannelsOutput, Box<dyn std::error::Error>>;
    async fn get_channel_messages(&self, channel_id: String, order_by: String, team_id: String, top: i64) -> Result<GetChannelMessagesOutput, Box<dyn std::error::Error>>;
    async fn create_meeting(&self, allow_new_time_proposals: bool, attendees: Vec<String>, body: String, end_time: chrono::DateTime<chrono::Utc>, is_online_meeting: bool, location: String, start_time: chrono::DateTime<chrono::Utc>, subject: String) -> Result<CreateMeetingOutput, Box<dyn std::error::Error>>;
    async fn get_meeting(&self, meeting_id: String) -> Result<GetMeetingOutput, Box<dyn std::error::Error>>;
    async fn cancel_meeting(&self, comment: String, meeting_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_teams(&self, filter: String, top: i64) -> Result<ListTeamsOutput, Box<dyn std::error::Error>>;
    async fn get_team(&self, team_id: String) -> Result<GetTeamOutput, Box<dyn std::error::Error>>;
    async fn list_team_members(&self, role: String, team_id: String) -> Result<ListTeamMembersOutput, Box<dyn std::error::Error>>;
    async fn add_team_member(&self, roles: Vec<String>, team_id: String, user_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn remove_team_member(&self, membership_id: String, team_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_adaptive_card(&self, card: String, channel_id: String, team_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn upload_file(&self, channel_id: String, content: String, file_name: String, folder_path: String, team_id: String) -> Result<UploadFileOutput, Box<dyn std::error::Error>>;
}

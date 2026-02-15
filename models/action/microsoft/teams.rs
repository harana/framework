// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageInput {
    pub attachments: Vec<String>,
    pub channel_id: String,
    pub content: String,
    pub content_type: String,
    pub importance: String,
    pub mentions: Vec<String>,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplyToMessageInput {
    pub attachments: Vec<String>,
    pub channel_id: String,
    pub content: String,
    pub content_type: String,
    pub message_id: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplyToMessageOutput {
    pub reply_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMessageInput {
    pub channel_id: String,
    pub content: String,
    pub content_type: String,
    pub message_id: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMessageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageInput {
    pub channel_id: String,
    pub message_id: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendChatMessageInput {
    pub attachments: Vec<String>,
    pub chat_id: String,
    pub content: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendChatMessageOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateChannelInput {
    pub description: String,
    pub display_name: String,
    pub membership_type: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateChannelOutput {
    pub channel_id: String,
    pub success: bool,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteChannelInput {
    pub channel_id: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteChannelOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListChannelsInput {
    pub filter: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListChannelsOutput {
    pub channels: Vec<String>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetChannelMessagesInput {
    pub channel_id: String,
    pub order_by: String,
    pub team_id: String,
    pub top: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetChannelMessagesOutput {
    pub messages: Vec<String>,
    pub next_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMeetingInput {
    #[serde(default)]
    pub allow_new_time_proposals: bool,
    pub attendees: Vec<String>,
    pub body: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_online_meeting: bool,
    pub location: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMeetingOutput {
    pub join_url: String,
    pub meeting_id: String,
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
    pub attendees: Vec<String>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub join_url: String,
    pub meeting_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelMeetingInput {
    pub comment: String,
    pub meeting_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelMeetingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTeamsInput {
    pub filter: String,
    pub top: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTeamsOutput {
    pub count: i64,
    pub teams: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTeamInput {
    pub team_id: String,
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
pub struct ListTeamMembersInput {
    pub role: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTeamMembersOutput {
    pub count: i64,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTeamMemberInput {
    pub roles: Vec<String>,
    pub team_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTeamMemberOutput {
    pub membership_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTeamMemberInput {
    pub membership_id: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTeamMemberOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendAdaptiveCardInput {
    pub card: String,
    pub channel_id: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendAdaptiveCardOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadFileInput {
    pub channel_id: String,
    pub content: String,
    pub file_name: String,
    pub folder_path: String,
    pub team_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadFileOutput {
    pub file_id: String,
    pub success: bool,
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

#[async_trait]
pub trait TeamsAction {
    async fn send_message(&self, input: SendMessageInput) -> Result<SendMessageOutput, Box<dyn std::error::Error>>;
    async fn reply_to_message(&self, input: ReplyToMessageInput) -> Result<ReplyToMessageOutput, Box<dyn std::error::Error>>;
    async fn update_message(&self, input: UpdateMessageInput) -> Result<UpdateMessageOutput, Box<dyn std::error::Error>>;
    async fn delete_message(&self, input: DeleteMessageInput) -> Result<DeleteMessageOutput, Box<dyn std::error::Error>>;
    async fn send_chat_message(&self, input: SendChatMessageInput) -> Result<SendChatMessageOutput, Box<dyn std::error::Error>>;
    async fn create_channel(&self, input: CreateChannelInput) -> Result<CreateChannelOutput, Box<dyn std::error::Error>>;
    async fn delete_channel(&self, input: DeleteChannelInput) -> Result<DeleteChannelOutput, Box<dyn std::error::Error>>;
    async fn list_channels(&self, input: ListChannelsInput) -> Result<ListChannelsOutput, Box<dyn std::error::Error>>;
    async fn get_channel_messages(&self, input: GetChannelMessagesInput) -> Result<GetChannelMessagesOutput, Box<dyn std::error::Error>>;
    async fn create_meeting(&self, input: CreateMeetingInput) -> Result<CreateMeetingOutput, Box<dyn std::error::Error>>;
    async fn get_meeting(&self, input: GetMeetingInput) -> Result<GetMeetingOutput, Box<dyn std::error::Error>>;
    async fn cancel_meeting(&self, input: CancelMeetingInput) -> Result<CancelMeetingOutput, Box<dyn std::error::Error>>;
    async fn list_teams(&self, input: ListTeamsInput) -> Result<ListTeamsOutput, Box<dyn std::error::Error>>;
    async fn get_team(&self, input: GetTeamInput) -> Result<GetTeamOutput, Box<dyn std::error::Error>>;
    async fn list_team_members(&self, input: ListTeamMembersInput) -> Result<ListTeamMembersOutput, Box<dyn std::error::Error>>;
    async fn add_team_member(&self, input: AddTeamMemberInput) -> Result<AddTeamMemberOutput, Box<dyn std::error::Error>>;
    async fn remove_team_member(&self, input: RemoveTeamMemberInput) -> Result<RemoveTeamMemberOutput, Box<dyn std::error::Error>>;
    async fn send_adaptive_card(&self, input: SendAdaptiveCardInput) -> Result<SendAdaptiveCardOutput, Box<dyn std::error::Error>>;
    async fn upload_file(&self, input: UploadFileInput) -> Result<UploadFileOutput, Box<dyn std::error::Error>>;
}

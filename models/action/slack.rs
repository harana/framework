// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageInput {
    pub attachments: Vec<String>,
    pub blocks: Vec<String>,
    pub channel_id: String,
    pub icon_emoji: String,
    pub icon_url: String,
    pub text: String,
    pub thread_ts: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageOutput {
    pub channel_id: String,
    pub message_ts: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMessageInput {
    pub attachments: Vec<String>,
    pub blocks: Vec<String>,
    pub channel_id: String,
    pub text: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMessageOutput {
    pub message_ts: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageInput {
    pub channel_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendDmInput {
    pub attachments: Vec<String>,
    pub blocks: Vec<String>,
    pub text: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendDmOutput {
    pub channel_id: String,
    pub message_ts: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateChannelInput {
    pub description: String,
    #[serde(default)]
    pub is_private: bool,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateChannelOutput {
    pub channel_id: String,
    pub name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveChannelInput {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveChannelOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnarchiveChannelInput {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnarchiveChannelOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InviteUsersInput {
    pub channel_id: String,
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InviteUsersOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KickUserInput {
    pub channel_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KickUserOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetChannelInfoInput {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetChannelInfoOutput {
    pub channel_id: String,
    pub is_archived: bool,
    pub is_private: bool,
    pub name: String,
    pub num_members: i64,
    pub purpose: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListChannelsInput {
    #[serde(default)]
    pub exclude_archived: bool,
    pub limit: i64,
    pub types: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListChannelsOutput {
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserInfoInput {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserInfoOutput {
    pub email: String,
    pub is_admin: bool,
    pub is_bot: bool,
    pub name: String,
    pub real_name: String,
    pub timezone: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersInput {
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersOutput {
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddReactionInput {
    pub channel_id: String,
    pub emoji: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddReactionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveReactionInput {
    pub channel_id: String,
    pub emoji: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveReactionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadFileInput {
    pub channels: Vec<String>,
    pub content: String,
    pub file: String,
    pub filename: String,
    pub initial_comment: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadFileOutput {
    pub file_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PinMessageInput {
    pub channel_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PinMessageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnpinMessageInput {
    pub channel_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnpinMessageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetChannelTopicInput {
    pub channel_id: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetChannelTopicOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetChannelPurposeInput {
    pub channel_id: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetChannelPurposeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackMessage {
    pub message_ts: String,
    pub channel_id: String,
    pub text: String,
    pub user_id: String,
    pub thread_ts: String,
    pub blocks: Vec<String>,
    pub attachments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackBlock {
    pub type: String,
    pub block_id: String,
    pub text: String,
    pub elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackTextObject {
    pub type: String,
    pub text: String,
    pub emoji: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackAttachment {
    pub fallback: String,
    pub color: String,
    pub pretext: String,
    pub title: String,
    pub title_link: String,
    pub text: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackAttachmentField {
    pub title: String,
    pub value: String,
    pub short: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackChannel {
    pub channel_id: String,
    pub name: String,
    pub is_private: bool,
    pub is_archived: bool,
    pub num_members: i64,
    pub topic: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackUser {
    pub user_id: String,
    pub name: String,
    pub real_name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_bot: bool,
    pub timezone: String,
}

#[async_trait]
pub trait SlackAction {
    async fn send_message(&self, input: SendMessageInput) -> Result<SendMessageOutput, Box<dyn std::error::Error>>;
    async fn update_message(&self, input: UpdateMessageInput) -> Result<UpdateMessageOutput, Box<dyn std::error::Error>>;
    async fn delete_message(&self, input: DeleteMessageInput) -> Result<DeleteMessageOutput, Box<dyn std::error::Error>>;
    async fn send_dm(&self, input: SendDmInput) -> Result<SendDmOutput, Box<dyn std::error::Error>>;
    async fn create_channel(&self, input: CreateChannelInput) -> Result<CreateChannelOutput, Box<dyn std::error::Error>>;
    async fn archive_channel(&self, input: ArchiveChannelInput) -> Result<ArchiveChannelOutput, Box<dyn std::error::Error>>;
    async fn unarchive_channel(&self, input: UnarchiveChannelInput) -> Result<UnarchiveChannelOutput, Box<dyn std::error::Error>>;
    async fn invite_users(&self, input: InviteUsersInput) -> Result<InviteUsersOutput, Box<dyn std::error::Error>>;
    async fn kick_user(&self, input: KickUserInput) -> Result<KickUserOutput, Box<dyn std::error::Error>>;
    async fn get_channel_info(&self, input: GetChannelInfoInput) -> Result<GetChannelInfoOutput, Box<dyn std::error::Error>>;
    async fn list_channels(&self, input: ListChannelsInput) -> Result<ListChannelsOutput, Box<dyn std::error::Error>>;
    async fn get_user_info(&self, input: GetUserInfoInput) -> Result<GetUserInfoOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, input: ListUsersInput) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
    async fn add_reaction(&self, input: AddReactionInput) -> Result<AddReactionOutput, Box<dyn std::error::Error>>;
    async fn remove_reaction(&self, input: RemoveReactionInput) -> Result<RemoveReactionOutput, Box<dyn std::error::Error>>;
    async fn upload_file(&self, input: UploadFileInput) -> Result<UploadFileOutput, Box<dyn std::error::Error>>;
    async fn pin_message(&self, input: PinMessageInput) -> Result<PinMessageOutput, Box<dyn std::error::Error>>;
    async fn unpin_message(&self, input: UnpinMessageInput) -> Result<UnpinMessageOutput, Box<dyn std::error::Error>>;
    async fn set_channel_topic(&self, input: SetChannelTopicInput) -> Result<SetChannelTopicOutput, Box<dyn std::error::Error>>;
    async fn set_channel_purpose(&self, input: SetChannelPurposeInput) -> Result<SetChannelPurposeOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageOutput {
    pub channel_id: String,
    pub message_ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendDmOutput {
    pub channel_id: String,
    pub message_ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateChannelOutput {
    pub channel_id: String,
    pub name: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackWorkspace {
    pub access_token: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub team_id: String,
    pub team_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackMessageLog {
    pub channel_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message_ts: String,
    pub status: String,
    pub text: String,
    pub thread_ts: String,
    pub user_id: String,
}

#[async_trait]
pub trait SlackAction {
    async fn send_message(&self, attachments: Vec<String>, blocks: Vec<String>, channel_id: String, icon_emoji: String, icon_url: String, text: String, thread_ts: String, username: String) -> Result<SendMessageOutput, Box<dyn std::error::Error>>;
    async fn update_message(&self, attachments: Vec<String>, blocks: Vec<String>, channel_id: String, text: String, timestamp: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_message(&self, channel_id: String, timestamp: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_dm(&self, attachments: Vec<String>, blocks: Vec<String>, text: String, user_id: String) -> Result<SendDmOutput, Box<dyn std::error::Error>>;
    async fn create_channel(&self, description: String, is_private: bool, name: String) -> Result<CreateChannelOutput, Box<dyn std::error::Error>>;
    async fn archive_channel(&self, channel_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn unarchive_channel(&self, channel_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn invite_users(&self, channel_id: String, user_ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn kick_user(&self, channel_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_channel_info(&self, channel_id: String) -> Result<GetChannelInfoOutput, Box<dyn std::error::Error>>;
    async fn list_channels(&self, exclude_archived: bool, limit: i64, types: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn get_user_info(&self, user_id: String) -> Result<GetUserInfoOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, limit: i64) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn add_reaction(&self, channel_id: String, emoji: String, timestamp: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_reaction(&self, channel_id: String, emoji: String, timestamp: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn upload_file(&self, channels: Vec<String>, content: String, file: String, filename: String, initial_comment: String, title: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn pin_message(&self, channel_id: String, timestamp: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn unpin_message(&self, channel_id: String, timestamp: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_channel_topic(&self, channel_id: String, topic: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_channel_purpose(&self, channel_id: String, purpose: String) -> Result<(), Box<dyn std::error::Error>>;
}

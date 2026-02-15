// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// slack_workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackWorkspace {
    pub access_token: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub team_id: String,
    pub team_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl SlackWorkspace {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackChannel {
    pub channel_id: String,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_private: bool,
    pub name: String,
    pub num_members: i64,
    pub purpose: Option<String>,
    pub topic: Option<String>,
    /// Reference: slack_workspace.id
    pub workspace_id: String,
}

impl SlackChannel {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackUser {
    pub email: Option<String>,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub is_bot: bool,
    pub name: Option<String>,
    pub real_name: Option<String>,
    pub timezone: Option<String>,
    pub user_id: String,
    /// Reference: slack_workspace.id
    pub workspace_id: String,
}

impl SlackUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_message_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackMessageLog {
    pub channel_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message_ts: String,
    pub status: String,
    pub text: Option<String>,
    pub thread_ts: Option<String>,
    pub user_id: Option<String>,
}

impl SlackMessageLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_message
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

impl SlackMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackBlock {
    pub type: String,
    pub block_id: String,
    pub text: String,
    pub elements: Vec<String>,
}

impl SlackBlock {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_text_object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackTextObject {
    pub type: String,
    pub text: String,
    pub emoji: bool,
}

impl SlackTextObject {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_attachment
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

impl SlackAttachment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// slack_attachment_field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlackAttachmentField {
    pub title: String,
    pub value: String,
    pub short: bool,
}

impl SlackAttachmentField {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


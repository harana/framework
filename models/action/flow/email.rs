// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateEmailOutput {
    pub reason: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusOutput {
    pub clicked_at: chrono::DateTime<chrono::Utc>,
    pub delivered_at: chrono::DateTime<chrono::Utc>,
    pub error: String,
    pub opened_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBulkOutput {
    pub failed: i64,
    pub message_ids: Vec<String>,
    pub successful: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTemplatesOutput {
    pub templates: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailMessage {
    pub message_id: String,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub content_type: String,
    pub attachments: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    pub content: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailRecipient {
    pub email: String,
    pub name: String,
    pub variables: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailTemplate {
    pub template_id: String,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub content_type: String,
    pub variables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Email {
    pub bcc: String,
    pub body_html: String,
    pub body_text: String,
    pub cc: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub from_address: String,
    pub reply_to: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub subject: String,
    pub template_id: String,
    pub to_address: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait EmailAction {
    async fn send(&self, attachments: Vec<String>, bcc: Vec<String>, body: String, cc: Vec<String>, content_type: String, from: String, reply_to: String, subject: String, to: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_template(&self, attachments: Vec<String>, bcc: Vec<String>, cc: Vec<String>, from: String, reply_to: String, template_id: String, to: Vec<String>, variables: std::collections::HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn validate_email(&self, check_mx: bool, check_smtp: bool, email: String) -> Result<ValidateEmailOutput, Box<dyn std::error::Error>>;
    async fn status(&self, message_id: String) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn send_bulk(&self, body: String, content_type: String, from: String, recipients: Vec<String>, subject: String) -> Result<SendBulkOutput, Box<dyn std::error::Error>>;
    async fn create_template(&self, body: String, content_type: String, name: String, subject: String, variables: Vec<String>, template_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_template(&self, template_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_templates(&self, limit: i64, offset: i64) -> Result<ListTemplatesOutput, Box<dyn std::error::Error>>;
}

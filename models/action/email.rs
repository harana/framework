// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendInput {
    pub attachments: Vec<String>,
    pub bcc: Vec<String>,
    pub body: String,
    pub cc: Vec<String>,
    pub content_type: String,
    pub from: String,
    pub reply_to: String,
    pub subject: String,
    pub to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendTemplateInput {
    pub attachments: Vec<String>,
    pub bcc: Vec<String>,
    pub cc: Vec<String>,
    pub from: String,
    pub reply_to: String,
    pub template_id: String,
    pub to: Vec<String>,
    pub variables: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendTemplateOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateEmailInput {
    #[serde(default)]
    pub check_mx: bool,
    #[serde(default)]
    pub check_smtp: bool,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateEmailOutput {
    pub reason: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusInput {
    pub message_id: String,
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
pub struct SendBulkInput {
    pub body: String,
    pub content_type: String,
    pub from: String,
    pub recipients: Vec<String>,
    pub subject: String,
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
pub struct CreateTemplateInput {
    pub body: String,
    pub content_type: String,
    pub name: String,
    pub subject: String,
    pub variables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTemplateOutput {
    pub success: bool,
    pub template_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTemplateInput {
    pub template_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTemplateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTemplatesInput {
    pub limit: i64,
    pub offset: i64,
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

#[async_trait]
pub trait EmailAction {
    async fn send(&self, input: SendInput) -> Result<SendOutput, Box<dyn std::error::Error>>;
    async fn send_template(&self, input: SendTemplateInput) -> Result<SendTemplateOutput, Box<dyn std::error::Error>>;
    async fn validate_email(&self, input: ValidateEmailInput) -> Result<ValidateEmailOutput, Box<dyn std::error::Error>>;
    async fn status(&self, input: StatusInput) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn send_bulk(&self, input: SendBulkInput) -> Result<SendBulkOutput, Box<dyn std::error::Error>>;
    async fn create_template(&self, input: CreateTemplateInput) -> Result<CreateTemplateOutput, Box<dyn std::error::Error>>;
    async fn delete_template(&self, input: DeleteTemplateInput) -> Result<DeleteTemplateOutput, Box<dyn std::error::Error>>;
    async fn list_templates(&self, input: ListTemplatesInput) -> Result<ListTemplatesOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailTemplate {
    pub body_html: String,
    pub body_text: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub subject: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl EmailTemplate {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Email {
    pub bcc: Option<String>,
    pub body_html: Option<String>,
    pub body_text: Option<String>,
    pub cc: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub from_address: String,
    pub reply_to: Option<String>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub subject: String,
    pub template_id: Option<String>,
    pub to_address: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Email {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailAttachment {
    pub content_type: String,
    pub email_id: String,
}

impl EmailAttachment {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl EmailMessage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailRecipient {
    pub email: String,
    pub name: String,
    pub variables: std::collections::HashMap<String, String>,
}

impl EmailRecipient {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


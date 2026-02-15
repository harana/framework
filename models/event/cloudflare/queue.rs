// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessageSent {
    pub queue: String,
    pub message_id: Option<String>,
    pub content_type: String,
    pub delay_seconds: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessageSent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessageBatchSent {
    pub queue: String,
    pub message_count: i64,
    pub failed_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessageBatchSent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessageArrived {
    pub queue: String,
    pub message_id: String,
    pub body: Option<String>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub arrived_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessageArrived {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessageBatchArrived {
    pub queue: String,
    pub message_count: i64,
    pub max_batch_size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub arrived_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessageBatchArrived {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessageAcknowledged {
    pub queue: String,
    pub message_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub acknowledged_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessageAcknowledged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueAllMessagesAcknowledged {
    pub queue: String,
    #[serde(default = "chrono::Utc::now")]
    pub acknowledged_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueAllMessagesAcknowledged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessageRetried {
    pub queue: String,
    pub message_id: String,
    pub delay_seconds: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub retried_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessageRetried {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueAllMessagesRetried {
    pub queue: String,
    pub delay_seconds: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub retried_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueAllMessagesRetried {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessageProcessingFailed {
    pub queue: String,
    pub message_id: Option<String>,
    pub error_message: Option<String>,
    pub retry_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessageProcessingFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


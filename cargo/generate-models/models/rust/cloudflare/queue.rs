// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cloudflare_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueue {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub queue_id: Option<String>,
    pub queue_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueue {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_queue_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueMessage {
    pub body: String,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delay_seconds: Option<i64>,
    pub message_id: String,
    /// Reference: cf_queue.id
    pub queue_id: String,
    pub receive_count: i64,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_queue_consumer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareQueueConsumer {
    pub consumer_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub max_batch_size: i64,
    pub max_batch_timeout: i64,
    pub max_retries: i64,
    /// Reference: cf_queue.id
    pub queue_id: String,
    pub script_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareQueueConsumer {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


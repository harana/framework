// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Queue {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_queue_id: Option<String>,
    #[serde(default)]
    pub is_fifo: bool,
    pub max_message_size: i64,
    pub message_retention_seconds: i64,
    pub receive_wait_time_seconds: i64,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub visibility_timeout_seconds: i64,
}

impl Queue {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueMessage {
    pub attributes: Option<String>,
    pub body: String,
    pub message_group_id: Option<String>,
    pub queue_id: String,
    pub receipt_handle: Option<String>,
    pub receive_count: i64,
    pub sequence_number: Option<i64>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub visible_at: chrono::DateTime<chrono::Utc>,
}

impl QueueMessage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueSubscription {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub endpoint: String,
    #[serde(default)]
    pub is_active: bool,
    pub protocol: String,
    pub queue_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl QueueSubscription {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueMetric {
    pub approximate_age_oldest_message: Option<i64>,
    pub approximate_messages_delayed: i64,
    pub queue_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl QueueMetric {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueuePeekMessage {
    pub message_id: String,
    pub message: String,
    pub enqueued_at: chrono::DateTime<chrono::Utc>,
}

impl QueuePeekMessage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


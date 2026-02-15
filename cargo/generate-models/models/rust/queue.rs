// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// queue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Queue {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: queue.id
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// queue_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueMessage {
    pub attributes: Option<String>,
    pub body: String,
    pub message_group_id: Option<String>,
    /// Reference: queue.id
    pub queue_id: String,
    pub receipt_handle: Option<String>,
    pub receive_count: i64,
    pub sequence_number: Option<i64>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub visible_at: chrono::DateTime<chrono::Utc>,
}

impl QueueMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// queue_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueSubscription {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub endpoint: String,
    #[serde(default)]
    pub is_active: bool,
    pub protocol: String,
    /// Reference: queue.id
    pub queue_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl QueueSubscription {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// queue_metric
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueMetric {
    pub approximate_age_oldest_message: Option<i64>,
    pub approximate_messages_delayed: i64,
    /// Reference: queue.id
    pub queue_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl QueueMetric {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// queue_peek_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueuePeekMessage {
    pub message_id: String,
    pub message: String,
    pub enqueued_at: chrono::DateTime<chrono::Utc>,
}

impl QueuePeekMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfQueue {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub queue_id: String,
    pub queue_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfQueueMessage {
    pub body: String,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delay_seconds: i64,
    pub message_id: String,
    pub queue_id: String,
    pub receive_count: i64,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfQueueConsumer {
    pub consumer_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub max_batch_size: i64,
    pub max_batch_timeout: i64,
    pub max_retries: i64,
    pub queue_id: String,
    pub script_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait QueueAction {
    async fn send(&self, content_type: String, delay_seconds: i64, message: String, queue: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_batch(&self, messages: Vec<String>, queue: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

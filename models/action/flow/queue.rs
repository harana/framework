// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DequeueOutput {
    pub found: bool,
    pub message: String,
    pub message_id: String,
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueueStatsOutput {
    pub delayed_count: i64,
    pub in_flight_count: i64,
    pub message_count: i64,
    pub oldest_message_age: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueuesOutput {
    pub queues: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueMessage {
    pub message_id: String,
    pub queue_name: String,
    pub message: String,
    pub priority: i64,
    pub delay_seconds: i64,
    pub receipt_handle: String,
    pub visible_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueuePeekMessage {
    pub message_id: String,
    pub message: String,
    pub enqueued_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Queue {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_queue_id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueMetric {
    pub approximate_age_oldest_message: i64,
    pub approximate_messages_delayed: i64,
    pub queue_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait QueueAction {
    async fn enqueue(&self, deduplication_id: String, delay_seconds: i64, message: String, priority: i64, queue_name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn dequeue(&self, queue_name: String, visibility_timeout: i64, wait_time_seconds: i64) -> Result<DequeueOutput, Box<dyn std::error::Error>>;
    async fn peek(&self, count: i64, queue_name: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn acknowledge(&self, queue_name: String, receipt_handle: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn nack(&self, delay_seconds: i64, queue_name: String, receipt_handle: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_queue_stats(&self, queue_name: String) -> Result<GetQueueStatsOutput, Box<dyn std::error::Error>>;
    async fn purge_queue(&self, queue_name: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn create_queue(&self, max_message_size: i64, message_retention: i64, queue_name: String, visibility_timeout: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_queue(&self, queue_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_queues(&self, prefix: String) -> Result<ListQueuesOutput, Box<dyn std::error::Error>>;
    async fn move_to_dlq(&self, queue_name: String, reason: String, receipt_handle: String) -> Result<(), Box<dyn std::error::Error>>;
}

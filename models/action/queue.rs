// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnqueueInput {
    pub deduplication_id: String,
    pub delay_seconds: i64,
    pub message: String,
    pub priority: i64,
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnqueueOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DequeueInput {
    pub queue_name: String,
    pub visibility_timeout: i64,
    pub wait_time_seconds: i64,
}

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
pub struct PeekInput {
    pub count: i64,
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PeekOutput {
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcknowledgeInput {
    pub queue_name: String,
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcknowledgeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NackInput {
    pub delay_seconds: i64,
    pub queue_name: String,
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NackOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueueStatsInput {
    pub queue_name: String,
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
pub struct PurgeQueueInput {
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PurgeQueueOutput {
    pub messages_deleted: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateQueueInput {
    pub max_message_size: i64,
    pub message_retention: i64,
    pub queue_name: String,
    pub visibility_timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateQueueOutput {
    pub queue_url: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteQueueInput {
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteQueueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueuesInput {
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueuesOutput {
    pub queues: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MoveToDlqInput {
    pub queue_name: String,
    pub reason: String,
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MoveToDlqOutput {
    pub success: bool,
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

#[async_trait]
pub trait QueueAction {
    async fn enqueue(&self, input: EnqueueInput) -> Result<EnqueueOutput, Box<dyn std::error::Error>>;
    async fn dequeue(&self, input: DequeueInput) -> Result<DequeueOutput, Box<dyn std::error::Error>>;
    async fn peek(&self, input: PeekInput) -> Result<PeekOutput, Box<dyn std::error::Error>>;
    async fn acknowledge(&self, input: AcknowledgeInput) -> Result<AcknowledgeOutput, Box<dyn std::error::Error>>;
    async fn nack(&self, input: NackInput) -> Result<NackOutput, Box<dyn std::error::Error>>;
    async fn get_queue_stats(&self, input: GetQueueStatsInput) -> Result<GetQueueStatsOutput, Box<dyn std::error::Error>>;
    async fn purge_queue(&self, input: PurgeQueueInput) -> Result<PurgeQueueOutput, Box<dyn std::error::Error>>;
    async fn create_queue(&self, input: CreateQueueInput) -> Result<CreateQueueOutput, Box<dyn std::error::Error>>;
    async fn delete_queue(&self, input: DeleteQueueInput) -> Result<DeleteQueueOutput, Box<dyn std::error::Error>>;
    async fn list_queues(&self, input: ListQueuesInput) -> Result<ListQueuesOutput, Box<dyn std::error::Error>>;
    async fn move_to_dlq(&self, input: MoveToDlqInput) -> Result<MoveToDlqOutput, Box<dyn std::error::Error>>;
}

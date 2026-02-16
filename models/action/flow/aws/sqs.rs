// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueuesOutput {
    pub next_token: String,
    pub queue_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageOutput {
    pub md5_of_message_attributes: String,
    pub md5_of_message_body: String,
    pub message_id: String,
    pub sequence_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageBatchOutput {
    pub failed: Vec<String>,
    pub successful: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageBatchOutput {
    pub failed: Vec<String>,
    pub successful: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityBatchOutput {
    pub failed: Vec<String>,
    pub successful: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDeadLetterSourceQueuesOutput {
    pub next_token: String,
    pub queue_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSqsQueue {
    pub account_id: String,
    #[serde(default)]
    pub content_based_deduplication: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_target_arn: String,
    pub delay_seconds: i64,
    #[serde(default)]
    pub is_fifo: bool,
    pub max_message_size: i64,
    pub message_retention_seconds: i64,
    pub queue_arn: String,
    pub queue_name: String,
    pub queue_url: String,
    pub receive_wait_time_seconds: i64,
    pub redrive_max_receive_count: i64,
    pub region: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub visibility_timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSqsMessage {
    pub body: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delay_seconds: i64,
    pub message_attributes: String,
    pub message_deduplication_id: String,
    pub message_group_id: String,
    pub message_id: String,
    pub queue_id: String,
    pub receipt_handle: String,
    pub receive_count: i64,
    pub sequence_number: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub visible_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSqsQueuePermission {
    pub actions: String,
    pub aws_account_ids: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    pub queue_id: String,
}

#[async_trait]
pub trait SqsAction {
    async fn create_queue(&self, attributes: String, queue_name: String, region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_queue(&self, queue_url: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_queue_url(&self, queue_name: String, queue_owner_aws_account_id: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_queues(&self, max_results: i64, next_token: String, queue_name_prefix: String, region: String) -> Result<ListQueuesOutput, Box<dyn std::error::Error>>;
    async fn send_message(&self, delay_seconds: i64, message_attributes: String, message_body: String, message_deduplication_id: String, message_group_id: String, queue_url: String, region: String) -> Result<SendMessageOutput, Box<dyn std::error::Error>>;
    async fn send_message_batch(&self, entries: Vec<String>, queue_url: String, region: String) -> Result<SendMessageBatchOutput, Box<dyn std::error::Error>>;
    async fn receive_message(&self, attribute_names: Vec<String>, max_number_of_messages: i64, message_attribute_names: Vec<String>, queue_url: String, receive_request_attempt_id: String, region: String, visibility_timeout: i64, wait_time_seconds: i64) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn delete_message(&self, queue_url: String, receipt_handle: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_message_batch(&self, entries: Vec<String>, queue_url: String, region: String) -> Result<DeleteMessageBatchOutput, Box<dyn std::error::Error>>;
    async fn change_message_visibility(&self, queue_url: String, receipt_handle: String, region: String, visibility_timeout: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn change_message_visibility_batch(&self, entries: Vec<String>, queue_url: String, region: String) -> Result<ChangeMessageVisibilityBatchOutput, Box<dyn std::error::Error>>;
    async fn get_queue_attributes(&self, attribute_names: Vec<String>, queue_url: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn set_queue_attributes(&self, attributes: String, queue_url: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn purge_queue(&self, queue_url: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn add_permission(&self, aws_account_ids: Vec<String>, actions: Vec<String>, label: String, queue_url: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_permission(&self, label: String, queue_url: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn tag_queue(&self, queue_url: String, region: String, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn untag_queue(&self, queue_url: String, region: String, tag_keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_queue_tags(&self, queue_url: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_dead_letter_source_queues(&self, max_results: i64, next_token: String, queue_url: String, region: String) -> Result<ListDeadLetterSourceQueuesOutput, Box<dyn std::error::Error>>;
}

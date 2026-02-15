// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateQueueInput {
    pub attributes: String,
    pub queue_name: String,
    pub region: String,
    pub tags: String,
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
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteQueueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueueUrlInput {
    pub queue_name: String,
    pub queue_owner_aws_account_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueueUrlOutput {
    pub queue_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueuesInput {
    pub max_results: i64,
    pub next_token: String,
    pub queue_name_prefix: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueuesOutput {
    pub next_token: String,
    pub queue_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageInput {
    pub delay_seconds: i64,
    pub message_attributes: String,
    pub message_body: String,
    pub message_deduplication_id: String,
    pub message_group_id: String,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageOutput {
    pub md5_of_message_attributes: String,
    pub md5_of_message_body: String,
    pub message_id: String,
    pub sequence_number: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageBatchInput {
    pub entries: Vec<String>,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageBatchOutput {
    pub failed: Vec<String>,
    pub successful: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReceiveMessageInput {
    pub attribute_names: Vec<String>,
    pub max_number_of_messages: i64,
    pub message_attribute_names: Vec<String>,
    pub queue_url: String,
    pub receive_request_attempt_id: String,
    pub region: String,
    pub visibility_timeout: i64,
    pub wait_time_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReceiveMessageOutput {
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageInput {
    pub queue_url: String,
    pub receipt_handle: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageBatchInput {
    pub entries: Vec<String>,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageBatchOutput {
    pub failed: Vec<String>,
    pub successful: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityInput {
    pub queue_url: String,
    pub receipt_handle: String,
    pub region: String,
    pub visibility_timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityBatchInput {
    pub entries: Vec<String>,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityBatchOutput {
    pub failed: Vec<String>,
    pub successful: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueueAttributesInput {
    pub attribute_names: Vec<String>,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueueAttributesOutput {
    pub attributes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetQueueAttributesInput {
    pub attributes: String,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetQueueAttributesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PurgeQueueInput {
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PurgeQueueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddPermissionInput {
    pub aws_account_ids: Vec<String>,
    pub actions: Vec<String>,
    pub label: String,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddPermissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemovePermissionInput {
    pub label: String,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemovePermissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagQueueInput {
    pub queue_url: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagQueueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntagQueueInput {
    pub queue_url: String,
    pub region: String,
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntagQueueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueueTagsInput {
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueueTagsOutput {
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDeadLetterSourceQueuesInput {
    pub max_results: i64,
    pub next_token: String,
    pub queue_url: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDeadLetterSourceQueuesOutput {
    pub next_token: String,
    pub queue_urls: Vec<String>,
}

#[async_trait]
pub trait SqsAction {
    async fn create_queue(&self, input: CreateQueueInput) -> Result<CreateQueueOutput, Box<dyn std::error::Error>>;
    async fn delete_queue(&self, input: DeleteQueueInput) -> Result<DeleteQueueOutput, Box<dyn std::error::Error>>;
    async fn get_queue_url(&self, input: GetQueueUrlInput) -> Result<GetQueueUrlOutput, Box<dyn std::error::Error>>;
    async fn list_queues(&self, input: ListQueuesInput) -> Result<ListQueuesOutput, Box<dyn std::error::Error>>;
    async fn send_message(&self, input: SendMessageInput) -> Result<SendMessageOutput, Box<dyn std::error::Error>>;
    async fn send_message_batch(&self, input: SendMessageBatchInput) -> Result<SendMessageBatchOutput, Box<dyn std::error::Error>>;
    async fn receive_message(&self, input: ReceiveMessageInput) -> Result<ReceiveMessageOutput, Box<dyn std::error::Error>>;
    async fn delete_message(&self, input: DeleteMessageInput) -> Result<DeleteMessageOutput, Box<dyn std::error::Error>>;
    async fn delete_message_batch(&self, input: DeleteMessageBatchInput) -> Result<DeleteMessageBatchOutput, Box<dyn std::error::Error>>;
    async fn change_message_visibility(&self, input: ChangeMessageVisibilityInput) -> Result<ChangeMessageVisibilityOutput, Box<dyn std::error::Error>>;
    async fn change_message_visibility_batch(&self, input: ChangeMessageVisibilityBatchInput) -> Result<ChangeMessageVisibilityBatchOutput, Box<dyn std::error::Error>>;
    async fn get_queue_attributes(&self, input: GetQueueAttributesInput) -> Result<GetQueueAttributesOutput, Box<dyn std::error::Error>>;
    async fn set_queue_attributes(&self, input: SetQueueAttributesInput) -> Result<SetQueueAttributesOutput, Box<dyn std::error::Error>>;
    async fn purge_queue(&self, input: PurgeQueueInput) -> Result<PurgeQueueOutput, Box<dyn std::error::Error>>;
    async fn add_permission(&self, input: AddPermissionInput) -> Result<AddPermissionOutput, Box<dyn std::error::Error>>;
    async fn remove_permission(&self, input: RemovePermissionInput) -> Result<RemovePermissionOutput, Box<dyn std::error::Error>>;
    async fn tag_queue(&self, input: TagQueueInput) -> Result<TagQueueOutput, Box<dyn std::error::Error>>;
    async fn untag_queue(&self, input: UntagQueueInput) -> Result<UntagQueueOutput, Box<dyn std::error::Error>>;
    async fn list_queue_tags(&self, input: ListQueueTagsInput) -> Result<ListQueueTagsOutput, Box<dyn std::error::Error>>;
    async fn list_dead_letter_source_queues(&self, input: ListDeadLetterSourceQueuesInput) -> Result<ListDeadLetterSourceQueuesOutput, Box<dyn std::error::Error>>;
}

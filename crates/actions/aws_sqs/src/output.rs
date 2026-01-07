// Harana Actions - Aws Sqs Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPermissionOutput {
    pub success: bool
}

// change_message_visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityOutput {
    pub success: bool
}

// change_message_visibility_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityBatchOutput {
    pub successful: Vec<HashMap<String, Value>>,
    pub failed: Vec<HashMap<String, Value>>,
    pub success: bool
}

// create_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueueOutput {
    pub queue_url: String,
    pub success: bool
}

// delete_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageOutput {
    pub success: bool
}

// delete_message_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageBatchOutput {
    pub successful: Vec<HashMap<String, Value>>,
    pub failed: Vec<HashMap<String, Value>>,
    pub success: bool
}

// delete_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQueueOutput {
    pub success: bool
}

// get_queue_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueAttributesOutput {
    pub attributes: HashMap<String, Value>
}

// get_queue_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueUrlOutput {
    pub queue_url: String
}

// list_dead_letter_source_queues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeadLetterSourceQueuesOutput {
    pub queue_urls: Vec<String>,
    pub next_token: String
}

// list_queue_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueueTagsOutput {
    pub tags: HashMap<String, Value>
}

// list_queues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueuesOutput {
    pub next_token: String,
    pub queue_urls: Vec<String>
}

// purge_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeQueueOutput {
    pub success: bool
}

// receive_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveMessageOutput {
    pub messages: Vec<HashMap<String, Value>>
}

// remove_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePermissionOutput {
    pub success: bool
}

// send_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    pub sequence_number: String,
    pub success: bool,
    pub md5_of_message_body: String,
    pub md5_of_message_attributes: String,
    pub message_id: String
}

// send_message_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageBatchOutput {
    pub failed: Vec<HashMap<String, Value>>,
    pub successful: Vec<HashMap<String, Value>>,
    pub success: bool
}

// set_queue_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetQueueAttributesOutput {
    pub success: bool
}

// tag_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagQueueOutput {
    pub success: bool
}

// untag_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntagQueueOutput {
    pub success: bool
}

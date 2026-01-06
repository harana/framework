// Harana Actions - AWS SQS Module Output Types
// Auto-generated output structs for AWS SQS action methods.

use serde::{Deserialize, Serialize};

// create_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueueOutput {
    pub queue_url: String,
    pub success: bool,
}

// delete_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQueueOutput {
    pub success: bool,
}

// send_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    pub md5_of_message_attributes: String,
    pub md5_of_message_body: String,
    pub message_id: String,
    pub sequence_number: String,
    pub success: bool,
}


// get_queue_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueUrlOutput {
    pub queue_url: String
}

// list_queues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueuesOutput {
    pub queue_urls: Vec<String>,
    pub next_token: String
}

// send_message_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageBatchOutput {
    pub success: bool,
    pub failed: Vec<HashMap<String, Value>>,
    pub successful: Vec<HashMap<String, Value>>
}

// receive_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveMessageOutput {
    pub messages: Vec<HashMap<String, Value>>
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
    pub success: bool,
    pub failed: Vec<HashMap<String, Value>>
}

// change_message_visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityOutput {
    pub success: bool
}

// change_message_visibility_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityBatchOutput {
    pub failed: Vec<HashMap<String, Value>>,
    pub successful: Vec<HashMap<String, Value>>,
    pub success: bool
}

// get_queue_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueAttributesOutput {
    pub attributes: HashMap<String, Value>
}

// set_queue_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetQueueAttributesOutput {
    pub success: bool
}

// purge_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeQueueOutput {
    pub success: bool
}

// add_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPermissionOutput {
    pub success: bool
}

// remove_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePermissionOutput {
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

// list_queue_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueueTagsOutput {
    pub tags: HashMap<String, Value>
}

// list_dead_letter_source_queues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeadLetterSourceQueuesOutput {
    pub queue_urls: Vec<String>,
    pub next_token: String
}
// TODO: Add remaining output types - see core/schema/actions/aws_sqs.yml

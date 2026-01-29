//! Output types for AWS SQS actions
//!
//! This module contains all the output structs and helper types used by the AWS SQS actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output for add_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPermissionOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for change_message_visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for change_message_visibility_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityBatchOutput {
    /// List of successfully changed messages
    pub successful: Vec<HashMap<String, Value>>,
    /// List of failed messages
    pub failed: Vec<HashMap<String, Value>>,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for create_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueueOutput {
    /// The URL of the created Amazon SQS queue
    pub queue_url: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_message_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageBatchOutput {
    /// List of successfully deleted messages
    pub successful: Vec<HashMap<String, Value>>,
    /// List of failed messages
    pub failed: Vec<HashMap<String, Value>>,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQueueOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_queue_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueAttributesOutput {
    /// A map of attributes to their respective values
    pub attributes: HashMap<String, Value>,
}

/// Output for get_queue_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueUrlOutput {
    /// The URL of the queue
    pub queue_url: String,
}

/// Output for list_dead_letter_source_queues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeadLetterSourceQueuesOutput {
    /// A list of source queue URLs that have the RedrivePolicy queue attribute configured with a dead-letter queue
    pub queue_urls: Vec<String>,
    /// Pagination token to include in the next request
    pub next_token: String,
}

/// Output for list_queue_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueueTagsOutput {
    /// The list of tags associated with the specified queue
    pub tags: HashMap<String, Value>,
}

/// Output for list_queues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueuesOutput {
    /// Pagination token to include in the next request
    pub next_token: String,
    /// A list of queue URLs, up to 1,000 entries
    pub queue_urls: Vec<String>,
}

/// Output for purge_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeQueueOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for receive_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveMessageOutput {
    /// A list of messages
    pub messages: Vec<HashMap<String, Value>>,
}

/// Output for remove_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePermissionOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for send_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    /// This parameter applies only to FIFO queues. The large, non-consecutive number that Amazon SQS assigns to each message
    pub sequence_number: String,
    /// Whether the operation was successful
    pub success: bool,
    /// An MD5 digest of the message body
    pub md5_of_message_body: String,
    /// An MD5 digest of the message attributes
    pub md5_of_message_attributes: String,
    /// An identifier for the message
    pub message_id: String,
}

/// Output for send_message_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageBatchOutput {
    /// List of failed messages
    pub failed: Vec<HashMap<String, Value>>,
    /// List of successfully sent messages
    pub successful: Vec<HashMap<String, Value>>,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for set_queue_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetQueueAttributesOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for tag_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagQueueOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for untag_queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntagQueueOutput {
    /// Whether the operation was successful
    pub success: bool,
}

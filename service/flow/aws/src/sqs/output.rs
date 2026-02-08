//! Output types for AWS SQS actions
//!
//! This module contains all the output structs and helper types used by the AWS SQS actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPermissionOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMessageVisibilityBatchOutput {
        pub successful: Vec<HashMap<String, Value>>,
        pub failed: Vec<HashMap<String, Value>>,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueueOutput {
        pub queue_url: String,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageBatchOutput {
        pub successful: Vec<HashMap<String, Value>>,
        pub failed: Vec<HashMap<String, Value>>,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteQueueOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueAttributesOutput {
        pub attributes: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQueueUrlOutput {
        pub queue_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeadLetterSourceQueuesOutput {
        pub queue_urls: Vec<String>,
        pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueueTagsOutput {
        pub tags: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueuesOutput {
        pub next_token: String,
        pub queue_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeQueueOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveMessageOutput {
        pub messages: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePermissionOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
        pub sequence_number: String,
        pub success: bool,
        pub md5_of_message_body: String,
        pub md5_of_message_attributes: String,
        pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageBatchOutput {
        pub failed: Vec<HashMap<String, Value>>,
        pub successful: Vec<HashMap<String, Value>>,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetQueueAttributesOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagQueueOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntagQueueOutput {
        pub success: bool,
}

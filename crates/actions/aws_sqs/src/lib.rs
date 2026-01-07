// Harana Actions - Aws Sqs Module
// This module provides aws sqs actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Add SQS Permission
pub async fn add_permission(
    aws_account_ids: Vec<String>,
    actions: Vec<String>,
    label: &str,
    queue_url: &str,
    region: Option<&str>,
) -> Result<AddPermissionOutput, String> {
    unimplemented!("add_permission")
}

/// Change Message Visibility
pub async fn change_message_visibility(
    queue_url: &str,
    receipt_handle: &str,
    visibility_timeout: i32,
    region: Option<&str>,
) -> Result<ChangeMessageVisibilityOutput, String> {
    unimplemented!("change_message_visibility")
}

/// Change Message Visibility Batch
pub async fn change_message_visibility_batch(
    entries: Vec<HashMap<String, Value>>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<ChangeMessageVisibilityBatchOutput, String> {
    unimplemented!("change_message_visibility_batch")
}

/// Create SQS Queue
pub async fn create_queue(
    queue_name: &str,
    attributes: Option<HashMap<String, Value>>,
    region: Option<&str>,
    tags: Option<HashMap<String, Value>>,
) -> Result<CreateQueueOutput, String> {
    unimplemented!("create_queue")
}

/// Delete SQS Message
pub async fn delete_message(
    queue_url: &str,
    receipt_handle: &str,
    region: Option<&str>,
) -> Result<DeleteMessageOutput, String> {
    unimplemented!("delete_message")
}

/// Delete SQS Message Batch
pub async fn delete_message_batch(
    entries: Vec<HashMap<String, Value>>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<DeleteMessageBatchOutput, String> {
    unimplemented!("delete_message_batch")
}

/// Delete SQS Queue
pub async fn delete_queue(
    queue_url: &str,
    region: Option<&str>,
) -> Result<DeleteQueueOutput, String> {
    unimplemented!("delete_queue")
}

/// Get SQS Queue Attributes
pub async fn get_queue_attributes(
    queue_url: &str,
    attribute_names: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<GetQueueAttributesOutput, String> {
    unimplemented!("get_queue_attributes")
}

/// Get SQS Queue URL
pub async fn get_queue_url(
    queue_name: &str,
    region: Option<&str>,
    queue_owner_aws_account_id: Option<&str>,
) -> Result<GetQueueUrlOutput, String> {
    unimplemented!("get_queue_url")
}

/// List SQS Dead Letter Source Queues
pub async fn list_dead_letter_source_queues(
    queue_url: &str,
    max_results: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListDeadLetterSourceQueuesOutput, String> {
    unimplemented!("list_dead_letter_source_queues")
}

/// List SQS Queue Tags
pub async fn list_queue_tags(
    queue_url: &str,
    region: Option<&str>,
) -> Result<ListQueueTagsOutput, String> {
    unimplemented!("list_queue_tags")
}

/// List SQS Queues
pub async fn list_queues(
    max_results: Option<i32>,
    region: Option<&str>,
    next_token: Option<&str>,
    queue_name_prefix: Option<&str>,
) -> Result<ListQueuesOutput, String> {
    unimplemented!("list_queues")
}

/// Purge SQS Queue
pub async fn purge_queue(
    queue_url: &str,
    region: Option<&str>,
) -> Result<PurgeQueueOutput, String> {
    unimplemented!("purge_queue")
}

/// Receive SQS Messages
pub async fn receive_message(
    queue_url: &str,
    max_number_of_messages: Option<i32>,
    visibility_timeout: Option<i32>,
    wait_time_seconds: Option<i32>,
    receive_request_attempt_id: Option<&str>,
    attribute_names: Option<Vec<String>>,
    message_attribute_names: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<ReceiveMessageOutput, String> {
    unimplemented!("receive_message")
}

/// Remove SQS Permission
pub async fn remove_permission(
    label: &str,
    queue_url: &str,
    region: Option<&str>,
) -> Result<RemovePermissionOutput, String> {
    unimplemented!("remove_permission")
}

/// Send SQS Message
pub async fn send_message(
    queue_url: &str,
    message_body: &str,
    message_group_id: Option<&str>,
    message_attributes: Option<HashMap<String, Value>>,
    message_deduplication_id: Option<&str>,
    delay_seconds: Option<i32>,
    region: Option<&str>,
) -> Result<SendMessageOutput, String> {
    unimplemented!("send_message")
}

/// Send SQS Message Batch
pub async fn send_message_batch(
    entries: Vec<HashMap<String, Value>>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<SendMessageBatchOutput, String> {
    unimplemented!("send_message_batch")
}

/// Set SQS Queue Attributes
pub async fn set_queue_attributes(
    attributes: HashMap<String, Value>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<SetQueueAttributesOutput, String> {
    unimplemented!("set_queue_attributes")
}

/// Tag SQS Queue
pub async fn tag_queue(
    tags: HashMap<String, Value>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<TagQueueOutput, String> {
    unimplemented!("tag_queue")
}

/// Untag SQS Queue
pub async fn untag_queue(
    tag_keys: Vec<String>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<UntagQueueOutput, String> {
    unimplemented!("untag_queue")
}

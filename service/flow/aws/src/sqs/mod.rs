// Harana Actions - AWS SQS Module
//
// This module provides AWS Simple Queue Service (SQS) actions for managing queues
// and messages.

pub mod output;


use aws_config::BehaviorVersion;
use aws_sdk_sqs::{
    config::Region,
    types::{
        ChangeMessageVisibilityBatchRequestEntry, DeleteMessageBatchRequestEntry,
        MessageAttributeValue, MessageSystemAttributeName, QueueAttributeName,
        SendMessageBatchRequestEntry,
    },
    Client,
};
use output::*;
use serde_json::Value;
use std::collections::HashMap;

/// Creates an SQS client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let sqs_config = if let Some(region_str) = region {
        aws_sdk_sqs::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_sqs::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(sqs_config))
}

/// Add SQS Permission
///
/// Adds a permission to a queue for a specific principal.
///
pub async fn add_permission(
    aws_account_ids: Vec<String>,
    actions: Vec<String>,
    label: &str,
    queue_url: &str,
    region: Option<&str>,
) -> Result<AddPermissionOutput, String> {
    let client = create_client(region).await?;

    client
        .add_permission()
        .queue_url(queue_url)
        .label(label)
        .set_aws_account_ids(Some(aws_account_ids))
        .set_actions(Some(actions))
        .send()
        .await
        .map_err(|e| format!("Failed to add permission: {}", e))?;

    Ok(AddPermissionOutput { success: true })
}

/// Change Message Visibility
///
/// Changes the visibility timeout of a specified message in a queue.
///
pub async fn change_message_visibility(
    queue_url: &str,
    receipt_handle: &str,
    visibility_timeout: i32,
    region: Option<&str>,
) -> Result<ChangeMessageVisibilityOutput, String> {
    let client = create_client(region).await?;

    client
        .change_message_visibility()
        .queue_url(queue_url)
        .receipt_handle(receipt_handle)
        .visibility_timeout(visibility_timeout)
        .send()
        .await
        .map_err(|e| format!("Failed to change message visibility: {}", e))?;

    Ok(ChangeMessageVisibilityOutput { success: true })
}

/// Change Message Visibility Batch
///
/// Changes the visibility timeout of multiple messages in a queue.
///
pub async fn change_message_visibility_batch(
    entries: Vec<HashMap<String, Value>>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<ChangeMessageVisibilityBatchOutput, String> {
    let client = create_client(region).await?;

    let batch_entries: Vec<ChangeMessageVisibilityBatchRequestEntry> = entries
        .iter()
        .filter_map(|entry| {
            let id = entry.get("id")?.as_str()?;
            let receipt_handle = entry.get("receipt_handle")?.as_str()?;
            let visibility_timeout = entry.get("visibility_timeout")?.as_i64()? as i32;

            Some(
                ChangeMessageVisibilityBatchRequestEntry::builder()
                    .id(id)
                    .receipt_handle(receipt_handle)
                    .visibility_timeout(visibility_timeout)
                    .build()
                    .ok()?,
            )
        })
        .collect();

    let response = client
        .change_message_visibility_batch()
        .queue_url(queue_url)
        .set_entries(Some(batch_entries))
        .send()
        .await
        .map_err(|e| format!("Failed to change message visibility batch: {}", e))?;

    let successful: Vec<HashMap<String, Value>> = response
        .successful()
        .iter()
        .map(|s| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::String(s.id().to_string()));
            map
        })
        .collect();

    let failed: Vec<HashMap<String, Value>> = response
        .failed()
        .iter()
        .map(|f| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::String(f.id().to_string()));
            map.insert("code".to_string(), Value::String(f.code().to_string()));
            if let Some(msg) = f.message() {
                map.insert("message".to_string(), Value::String(msg.to_string()));
            }
            map.insert(
                "sender_fault".to_string(),
                Value::Bool(f.sender_fault()),
            );
            map
        })
        .collect();

    Ok(ChangeMessageVisibilityBatchOutput {
        successful,
        failed,
        success: true,
    })
}

/// Create SQS Queue
///
/// Creates a new standard or FIFO queue.
///
pub async fn create_queue(
    queue_name: &str,
    attributes: Option<HashMap<String, Value>>,
    region: Option<&str>,
    tags: Option<HashMap<String, Value>>,
) -> Result<CreateQueueOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_queue().queue_name(queue_name);

    if let Some(attrs) = attributes {
        let queue_attrs: HashMap<QueueAttributeName, String> = attrs
            .iter()
            .filter_map(|(k, v)| {
                let attr_name = parse_queue_attribute_name(k)?;
                let attr_value = v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string());
                Some((attr_name, attr_value))
            })
            .collect();
        request = request.set_attributes(Some(queue_attrs));
    }

    if let Some(tag_map) = tags {
        let string_tags: HashMap<String, String> = tag_map
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string()),
                )
            })
            .collect();
        request = request.set_tags(Some(string_tags));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create queue: {}", e))?;

    Ok(CreateQueueOutput {
        queue_url: response.queue_url().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete SQS Message
///
/// Deletes the specified message from the specified queue.
///
pub async fn delete_message(
    queue_url: &str,
    receipt_handle: &str,
    region: Option<&str>,
) -> Result<DeleteMessageOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_message()
        .queue_url(queue_url)
        .receipt_handle(receipt_handle)
        .send()
        .await
        .map_err(|e| format!("Failed to delete message: {}", e))?;

    Ok(DeleteMessageOutput { success: true })
}

/// Delete SQS Message Batch
///
/// Deletes up to ten messages from the specified queue.
///
pub async fn delete_message_batch(
    entries: Vec<HashMap<String, Value>>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<DeleteMessageBatchOutput, String> {
    let client = create_client(region).await?;

    let batch_entries: Vec<DeleteMessageBatchRequestEntry> = entries
        .iter()
        .filter_map(|entry| {
            let id = entry.get("id")?.as_str()?;
            let receipt_handle = entry.get("receipt_handle")?.as_str()?;

            Some(
                DeleteMessageBatchRequestEntry::builder()
                    .id(id)
                    .receipt_handle(receipt_handle)
                    .build()
                    .ok()?,
            )
        })
        .collect();

    let response = client
        .delete_message_batch()
        .queue_url(queue_url)
        .set_entries(Some(batch_entries))
        .send()
        .await
        .map_err(|e| format!("Failed to delete message batch: {}", e))?;

    let successful: Vec<HashMap<String, Value>> = response
        .successful()
        .iter()
        .map(|s| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::String(s.id().to_string()));
            map
        })
        .collect();

    let failed: Vec<HashMap<String, Value>> = response
        .failed()
        .iter()
        .map(|f| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::String(f.id().to_string()));
            map.insert("code".to_string(), Value::String(f.code().to_string()));
            if let Some(msg) = f.message() {
                map.insert("message".to_string(), Value::String(msg.to_string()));
            }
            map.insert(
                "sender_fault".to_string(),
                Value::Bool(f.sender_fault()),
            );
            map
        })
        .collect();

    Ok(DeleteMessageBatchOutput {
        successful,
        failed,
        success: true,
    })
}

/// Delete SQS Queue
///
/// Deletes the queue specified by the QueueUrl.
///
pub async fn delete_queue(queue_url: &str, region: Option<&str>) -> Result<DeleteQueueOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_queue()
        .queue_url(queue_url)
        .send()
        .await
        .map_err(|e| format!("Failed to delete queue: {}", e))?;

    Ok(DeleteQueueOutput { success: true })
}

/// Get SQS Queue Attributes
///
/// Gets attributes for the specified queue.
///
pub async fn get_queue_attributes(
    queue_url: &str,
    attribute_names: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<GetQueueAttributesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_queue_attributes().queue_url(queue_url);

    if let Some(names) = attribute_names {
        let attr_names: Vec<QueueAttributeName> = names
            .iter()
            .filter_map(|n| parse_queue_attribute_name(n))
            .collect();
        request = request.set_attribute_names(Some(attr_names));
    } else {
        request = request.attribute_names(QueueAttributeName::All);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get queue attributes: {}", e))?;

    let attributes: HashMap<String, Value> = response
        .attributes()
        .map(|attrs| {
            attrs
                .iter()
                .map(|(k, v)| (k.as_str().to_string(), Value::String(v.clone())))
                .collect()
        })
        .unwrap_or_default();

    Ok(GetQueueAttributesOutput { attributes })
}

/// Get SQS Queue URL
///
/// Returns the URL of an existing Amazon SQS queue.
///
pub async fn get_queue_url(
    queue_name: &str,
    region: Option<&str>,
    queue_owner_aws_account_id: Option<&str>,
) -> Result<GetQueueUrlOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_queue_url().queue_name(queue_name);

    if let Some(owner_id) = queue_owner_aws_account_id {
        request = request.queue_owner_aws_account_id(owner_id);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get queue URL: {}", e))?;

    Ok(GetQueueUrlOutput {
        queue_url: response.queue_url().unwrap_or_default().to_string(),
    })
}

/// List SQS Dead Letter Source Queues
///
/// Returns a list of your queues that have the RedrivePolicy queue attribute configured
/// with a dead-letter queue.
///
pub async fn list_dead_letter_source_queues(
    queue_url: &str,
    max_results: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListDeadLetterSourceQueuesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .list_dead_letter_source_queues()
        .queue_url(queue_url);

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list dead letter source queues: {}", e))?;

    Ok(ListDeadLetterSourceQueuesOutput {
        queue_urls: response.queue_urls().to_vec(),
        next_token: response.next_token().unwrap_or_default().to_string(),
    })
}

/// List SQS Queue Tags
///
/// Lists the tags for the specified queue.
///
pub async fn list_queue_tags(
    queue_url: &str,
    region: Option<&str>,
) -> Result<ListQueueTagsOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .list_queue_tags()
        .queue_url(queue_url)
        .send()
        .await
        .map_err(|e| format!("Failed to list queue tags: {}", e))?;

    let tags: HashMap<String, Value> = response
        .tags()
        .map(|t| {
            t.iter()
                .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                .collect()
        })
        .unwrap_or_default();

    Ok(ListQueueTagsOutput { tags })
}

/// List SQS Queues
///
/// Returns a list of your queues in the current region.
///
pub async fn list_queues(
    max_results: Option<i32>,
    region: Option<&str>,
    next_token: Option<&str>,
    queue_name_prefix: Option<&str>,
) -> Result<ListQueuesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_queues();

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    if let Some(prefix) = queue_name_prefix {
        request = request.queue_name_prefix(prefix);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list queues: {}", e))?;

    Ok(ListQueuesOutput {
        next_token: response.next_token().unwrap_or_default().to_string(),
        queue_urls: response.queue_urls().to_vec(),
    })
}

/// Purge SQS Queue
///
/// Deletes all messages in a queue.
///
pub async fn purge_queue(queue_url: &str, region: Option<&str>) -> Result<PurgeQueueOutput, String> {
    let client = create_client(region).await?;

    client
        .purge_queue()
        .queue_url(queue_url)
        .send()
        .await
        .map_err(|e| format!("Failed to purge queue: {}", e))?;

    Ok(PurgeQueueOutput { success: true })
}

/// Receive SQS Messages
///
/// Retrieves one or more messages from the specified queue.
///
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
    let client = create_client(region).await?;

    let mut request = client.receive_message().queue_url(queue_url);

    if let Some(max) = max_number_of_messages {
        request = request.max_number_of_messages(max);
    }

    if let Some(timeout) = visibility_timeout {
        request = request.visibility_timeout(timeout);
    }

    if let Some(wait) = wait_time_seconds {
        request = request.wait_time_seconds(wait);
    }

    if let Some(attempt_id) = receive_request_attempt_id {
        request = request.receive_request_attempt_id(attempt_id);
    }

    if let Some(names) = attribute_names {
        let attr_names: Vec<MessageSystemAttributeName> = names
            .iter()
            .filter_map(|n| parse_message_system_attribute_name(n))
            .collect();
        request = request.set_message_system_attribute_names(Some(attr_names));
    }

    if let Some(msg_attr_names) = message_attribute_names {
        request = request.set_message_attribute_names(Some(msg_attr_names));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to receive messages: {}", e))?;

    let messages: Vec<HashMap<String, Value>> = response
        .messages()
        .iter()
        .map(|msg| {
            let mut map = HashMap::new();

            if let Some(id) = msg.message_id() {
                map.insert("message_id".to_string(), Value::String(id.to_string()));
            }
            if let Some(handle) = msg.receipt_handle() {
                map.insert("receipt_handle".to_string(), Value::String(handle.to_string()));
            }
            if let Some(body) = msg.body() {
                map.insert("body".to_string(), Value::String(body.to_string()));
            }
            if let Some(md5) = msg.md5_of_body() {
                map.insert("md5_of_body".to_string(), Value::String(md5.to_string()));
            }
            if let Some(md5_attrs) = msg.md5_of_message_attributes() {
                map.insert(
                    "md5_of_message_attributes".to_string(),
                    Value::String(md5_attrs.to_string()),
                );
            }

            // Include system attributes
            let attrs: HashMap<String, Value> = msg
                .attributes()
                .map(|a| {
                    a.iter()
                        .map(|(k, v)| (k.as_str().to_string(), Value::String(v.clone())))
                        .collect()
                })
                .unwrap_or_default();
            if !attrs.is_empty() {
                map.insert("attributes".to_string(), serde_json::to_value(attrs).unwrap_or(Value::Null));
            }

            // Include message attributes
            if let Some(message_attributes_map) = msg.message_attributes() {
                if !message_attributes_map.is_empty() {
                    let msg_attrs: HashMap<String, Value> = message_attributes_map
                        .iter()
                        .map(|(k, v): (&String, &MessageAttributeValue)| {
                            let mut attr_map: HashMap<String, Value> = HashMap::new();
                            attr_map.insert(
                                "data_type".to_string(),
                                Value::String(v.data_type().to_string()),
                            );
                            if let Some(sv) = v.string_value() {
                                attr_map.insert("string_value".to_string(), Value::String(sv.to_string()));
                            }
                            (k.clone(), serde_json::to_value(attr_map).unwrap_or(Value::Null))
                        })
                        .collect();
                    map.insert(
                        "message_attributes".to_string(),
                        serde_json::to_value(msg_attrs).unwrap_or(Value::Null),
                    );
                }
            }

            map
        })
        .collect();

    Ok(ReceiveMessageOutput { messages })
}

/// Remove SQS Permission
///
/// Revokes any permissions in the queue policy that matches the specified Label.
///
pub async fn remove_permission(
    label: &str,
    queue_url: &str,
    region: Option<&str>,
) -> Result<RemovePermissionOutput, String> {
    let client = create_client(region).await?;

    client
        .remove_permission()
        .queue_url(queue_url)
        .label(label)
        .send()
        .await
        .map_err(|e| format!("Failed to remove permission: {}", e))?;

    Ok(RemovePermissionOutput { success: true })
}

/// Send SQS Message
///
/// Delivers a message to the specified queue.
///
pub async fn send_message(
    queue_url: &str,
    message_body: &str,
    message_group_id: Option<&str>,
    message_attributes: Option<HashMap<String, Value>>,
    message_deduplication_id: Option<&str>,
    delay_seconds: Option<i32>,
    region: Option<&str>,
) -> Result<SendMessageOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .send_message()
        .queue_url(queue_url)
        .message_body(message_body);

    if let Some(group_id) = message_group_id {
        request = request.message_group_id(group_id);
    }

    if let Some(dedup_id) = message_deduplication_id {
        request = request.message_deduplication_id(dedup_id);
    }

    if let Some(delay) = delay_seconds {
        request = request.delay_seconds(delay);
    }

    if let Some(attrs) = message_attributes {
        let msg_attrs: HashMap<String, MessageAttributeValue> = attrs
            .iter()
            .filter_map(|(k, v)| {
                let attr_obj = v.as_object()?;
                let data_type = attr_obj.get("data_type")?.as_str()?;
                let string_value = attr_obj.get("string_value").and_then(|v| v.as_str());

                let mut builder = MessageAttributeValue::builder().data_type(data_type);
                if let Some(sv) = string_value {
                    builder = builder.string_value(sv);
                }

                Some((k.clone(), builder.build().ok()?))
            })
            .collect();
        request = request.set_message_attributes(Some(msg_attrs));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to send message: {}", e))?;

    Ok(SendMessageOutput {
        sequence_number: response.sequence_number().unwrap_or_default().to_string(),
        success: true,
        md5_of_message_body: response.md5_of_message_body().unwrap_or_default().to_string(),
        md5_of_message_attributes: response
            .md5_of_message_attributes()
            .unwrap_or_default()
            .to_string(),
        message_id: response.message_id().unwrap_or_default().to_string(),
    })
}

/// Send SQS Message Batch
///
/// Delivers up to ten messages to the specified queue.
///
pub async fn send_message_batch(
    entries: Vec<HashMap<String, Value>>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<SendMessageBatchOutput, String> {
    let client = create_client(region).await?;

    let batch_entries: Vec<SendMessageBatchRequestEntry> = entries
        .iter()
        .filter_map(|entry| {
            let id = entry.get("id")?.as_str()?;
            let message_body = entry.get("message_body")?.as_str()?;

            let mut builder = SendMessageBatchRequestEntry::builder()
                .id(id)
                .message_body(message_body);

            if let Some(group_id) = entry.get("message_group_id").and_then(|v| v.as_str()) {
                builder = builder.message_group_id(group_id);
            }

            if let Some(dedup_id) = entry.get("message_deduplication_id").and_then(|v| v.as_str()) {
                builder = builder.message_deduplication_id(dedup_id);
            }

            if let Some(delay) = entry.get("delay_seconds").and_then(|v| v.as_i64()) {
                builder = builder.delay_seconds(delay as i32);
            }

            Some(builder.build().ok()?)
        })
        .collect();

    let response = client
        .send_message_batch()
        .queue_url(queue_url)
        .set_entries(Some(batch_entries))
        .send()
        .await
        .map_err(|e| format!("Failed to send message batch: {}", e))?;

    let successful: Vec<HashMap<String, Value>> = response
        .successful()
        .iter()
        .map(|s| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::String(s.id().to_string()));
            map.insert(
                "message_id".to_string(),
                Value::String(s.message_id().to_string()),
            );
            map.insert(
                "md5_of_message_body".to_string(),
                Value::String(s.md5_of_message_body().to_string()),
            );
            if let Some(md5_attrs) = s.md5_of_message_attributes() {
                map.insert(
                    "md5_of_message_attributes".to_string(),
                    Value::String(md5_attrs.to_string()),
                );
            }
            if let Some(seq) = s.sequence_number() {
                map.insert("sequence_number".to_string(), Value::String(seq.to_string()));
            }
            map
        })
        .collect();

    let failed: Vec<HashMap<String, Value>> = response
        .failed()
        .iter()
        .map(|f| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::String(f.id().to_string()));
            map.insert("code".to_string(), Value::String(f.code().to_string()));
            if let Some(msg) = f.message() {
                map.insert("message".to_string(), Value::String(msg.to_string()));
            }
            map.insert(
                "sender_fault".to_string(),
                Value::Bool(f.sender_fault()),
            );
            map
        })
        .collect();

    Ok(SendMessageBatchOutput {
        failed,
        successful,
        success: true,
    })
}

/// Set SQS Queue Attributes
///
/// Sets the value of one or more queue attributes.
///
pub async fn set_queue_attributes(
    attributes: HashMap<String, Value>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<SetQueueAttributesOutput, String> {
    let client = create_client(region).await?;

    let queue_attrs: HashMap<QueueAttributeName, String> = attributes
        .iter()
        .filter_map(|(k, v)| {
            let attr_name = parse_queue_attribute_name(k)?;
            let attr_value = v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string());
            Some((attr_name, attr_value))
        })
        .collect();

    client
        .set_queue_attributes()
        .queue_url(queue_url)
        .set_attributes(Some(queue_attrs))
        .send()
        .await
        .map_err(|e| format!("Failed to set queue attributes: {}", e))?;

    Ok(SetQueueAttributesOutput { success: true })
}

/// Tag SQS Queue
///
/// Add cost allocation tags to the specified Amazon SQS queue.
///
pub async fn tag_queue(
    tags: HashMap<String, Value>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<TagQueueOutput, String> {
    let client = create_client(region).await?;

    let string_tags: HashMap<String, String> = tags
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string()),
            )
        })
        .collect();

    client
        .tag_queue()
        .queue_url(queue_url)
        .set_tags(Some(string_tags))
        .send()
        .await
        .map_err(|e| format!("Failed to tag queue: {}", e))?;

    Ok(TagQueueOutput { success: true })
}

/// Untag SQS Queue
///
/// Remove cost allocation tags from the specified Amazon SQS queue.
///
pub async fn untag_queue(
    tag_keys: Vec<String>,
    queue_url: &str,
    region: Option<&str>,
) -> Result<UntagQueueOutput, String> {
    let client = create_client(region).await?;

    client
        .untag_queue()
        .queue_url(queue_url)
        .set_tag_keys(Some(tag_keys))
        .send()
        .await
        .map_err(|e| format!("Failed to untag queue: {}", e))?;

    Ok(UntagQueueOutput { success: true })
}

/// Helper function to parse queue attribute names from strings
fn parse_queue_attribute_name(name: &str) -> Option<QueueAttributeName> {
    match name.to_lowercase().replace('_', "").as_str() {
        "all" => Some(QueueAttributeName::All),
        "approximatenumberofmessages" => Some(QueueAttributeName::ApproximateNumberOfMessages),
        "approximatenumberofmessagesdelayed" => {
            Some(QueueAttributeName::ApproximateNumberOfMessagesDelayed)
        }
        "approximatenumberofmessagesnotvisible" => {
            Some(QueueAttributeName::ApproximateNumberOfMessagesNotVisible)
        }
        "contentbaseddeduplication" => Some(QueueAttributeName::ContentBasedDeduplication),
        "createdtimestamp" => Some(QueueAttributeName::CreatedTimestamp),
        "deduplicationscope" => Some(QueueAttributeName::DeduplicationScope),
        "delayseconds" => Some(QueueAttributeName::DelaySeconds),
        "fifoqueue" => Some(QueueAttributeName::FifoQueue),
        "fifothroughputlimit" => Some(QueueAttributeName::FifoThroughputLimit),
        "kmsdatakeyreuseperiodseconds" => Some(QueueAttributeName::KmsDataKeyReusePeriodSeconds),
        "kmsmasterkeyid" => Some(QueueAttributeName::KmsMasterKeyId),
        "lastmodifiedtimestamp" => Some(QueueAttributeName::LastModifiedTimestamp),
        "maximummessagesize" => Some(QueueAttributeName::MaximumMessageSize),
        "messageretentionperiod" => Some(QueueAttributeName::MessageRetentionPeriod),
        "policy" => Some(QueueAttributeName::Policy),
        "queuearn" => Some(QueueAttributeName::QueueArn),
        "receivemessagewaittimeseconds" => Some(QueueAttributeName::ReceiveMessageWaitTimeSeconds),
        "redrivepolicy" => Some(QueueAttributeName::RedrivePolicy),
        "redriveallowpolicy" => Some(QueueAttributeName::RedriveAllowPolicy),
        "sqsmanagedsseenabled" => Some(QueueAttributeName::SqsManagedSseEnabled),
        "visibilitytimeout" => Some(QueueAttributeName::VisibilityTimeout),
        _ => None,
    }
}

/// Helper function to parse message system attribute names from strings
fn parse_message_system_attribute_name(name: &str) -> Option<MessageSystemAttributeName> {
    match name.to_lowercase().replace('_', "").as_str() {
        "all" => Some(MessageSystemAttributeName::All),
        "approximatefirstreceivetimestamp" => {
            Some(MessageSystemAttributeName::ApproximateFirstReceiveTimestamp)
        }
        "approximatereceivecount" => Some(MessageSystemAttributeName::ApproximateReceiveCount),
        "awstraceparent" | "awstraceheader" => Some(MessageSystemAttributeName::AwsTraceHeader),
        "deadletterqueuesourcearn" => Some(MessageSystemAttributeName::DeadLetterQueueSourceArn),
        "messagededuplicationid" => Some(MessageSystemAttributeName::MessageDeduplicationId),
        "messagegroupid" => Some(MessageSystemAttributeName::MessageGroupId),
        "senderid" => Some(MessageSystemAttributeName::SenderId),
        "senttimestamp" => Some(MessageSystemAttributeName::SentTimestamp),
        "sequencenumber" => Some(MessageSystemAttributeName::SequenceNumber),
        _ => None,
    }
}

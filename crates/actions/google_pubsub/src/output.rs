// Harana Actions - Google Pub/Sub Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTopicOutput {
    pub success: bool,
    pub topic_name: String,
}

// delete_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTopicOutput {
    pub success: bool,
}

// get_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTopicOutput {
    pub labels: Option<PubSubLabels>,
    pub message_retention_duration: Option<String>,
    pub schema_settings: Option<PubSubSchemaSettings>,
    pub topic_name: String,
}

// list_topics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTopicsOutput {
    pub next_page_token: Option<String>,
    pub topics: Vec<PubSubTopic>,
}

// publish
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOutput {
    pub message_id: String,
    pub success: bool,
}

// publish_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBatchOutput {
    pub failure_count: i32,
    pub message_ids: Vec<String>,
    pub success_count: i32,
}

// create_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionOutput {
    pub subscription_name: String,
    pub success: bool,
}

// delete_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscriptionOutput {
    pub success: bool,
}

// get_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionOutput {
    pub ack_deadline_seconds: i32,
    pub labels: Option<PubSubLabels>,
    pub message_retention_duration: Option<String>,
    pub push_config: Option<PushConfig>,
    pub subscription_name: String,
    pub topic_name: String,
}

// list_subscriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubscriptionsOutput {
    pub next_page_token: Option<String>,
    pub subscriptions: Vec<PubSubSubscription>,
}

// pull
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullOutput {
    pub messages: Vec<PubSubReceivedMessage>,
}

// acknowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledgeOutput {
    pub success: bool,
}

// modify_ack_deadline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyAckDeadlineOutput {
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubLabels {
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubSchemaSettings {
    pub encoding: Option<String>,
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubTopic {
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubSubscription {
    pub name: String,
    pub topic: String,
    pub ack_deadline_seconds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubPublishMessage {
    pub data: String,
    pub attributes: Option<HashMap<String, String>>,
    pub ordering_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubReceivedMessage {
    pub ack_id: String,
    pub message: PubSubMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubMessage {
    pub data: String,
    pub attributes: Option<HashMap<String, String>>,
    pub message_id: String,
    pub publish_time: String,
    pub ordering_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushConfig {
    pub push_endpoint: String,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterPolicy {
    pub dead_letter_topic: String,
    pub max_delivery_attempts: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub minimum_backoff: Option<String>,
    pub maximum_backoff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubMessageAttributes {
    pub attributes: HashMap<String, String>,
}

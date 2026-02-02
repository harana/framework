use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// === Action Outputs ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTopicOutput {
    pub topic_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTopicOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTopicOutput {
    pub topic_name: String,
    pub labels: PubSubLabels,
    pub message_retention_duration: Option<String>,
    pub schema_settings: Option<PubSubSchemaSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTopicsOutput {
    pub topics: Vec<PubSubTopic>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishBatchOutput {
    pub message_ids: Vec<String>,
    pub success_count: i32,
    pub failure_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionOutput {
    pub subscription_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscriptionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionOutput {
    pub subscription_name: String,
    pub topic: String,
    pub ack_deadline_seconds: i32,
    pub message_retention_duration: Option<String>,
    pub labels: PubSubLabels,
    pub push_config: Option<PushConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubscriptionsOutput {
    pub subscriptions: Vec<PubSubSubscription>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullOutput {
    pub messages: Vec<PubSubReceivedMessage>,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledgeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyAckDeadlineOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeekOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotOutput {
    pub snapshot_name: String,
    pub topic: String,
    pub expire_time: DateTime<Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSnapshotOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTopicSubscriptionsOutput {
    pub subscriptions: Vec<String>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTopicOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionOutput {
    pub success: bool,
}

// === Class Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubMessage {
    pub message_id: String,
    pub data: String,
    pub attributes: PubSubMessageAttributes,
    pub publish_time: DateTime<Utc>,
    pub ordering_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PubSubLabels {
    pub labels: Vec<PubSubLabel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubLabel {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubSchemaSettings {
    pub schema: String,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubTopic {
    pub name: String,
    pub labels: PubSubLabels,
    pub message_retention_duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PubSubMessageAttributes {
    pub attributes: Vec<PubSubMessageAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubMessageAttribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubPublishMessage {
    pub data: String,
    pub attributes: Option<PubSubMessageAttributes>,
    pub ordering_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterPolicy {
    pub dead_letter_topic: String,
    pub max_delivery_attempts: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushConfig {
    pub push_endpoint: String,
    pub attributes: Option<PubSubMessageAttributes>,
    pub oidc_token: Option<OidcToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcToken {
    pub service_account_email: String,
    pub audience: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub minimum_backoff: String,
    pub maximum_backoff: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubSubscription {
    pub name: String,
    pub topic: String,
    pub ack_deadline_seconds: i32,
    pub message_retention_duration: Option<String>,
    pub labels: PubSubLabels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubReceivedMessage {
    pub ack_id: String,
    pub message: PubSubMessage,
    pub delivery_attempt: i32,
}

// === Internal Storage Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredTopic {
    pub name: String,
    pub project_id: String,
    pub topic_id: String,
    pub labels: PubSubLabels,
    pub message_retention_duration: Option<String>,
    pub schema_settings: Option<PubSubSchemaSettings>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSubscription {
    pub name: String,
    pub project_id: String,
    pub subscription_id: String,
    pub topic_id: String,
    pub ack_deadline_seconds: i32,
    pub message_retention_duration: Option<String>,
    pub labels: PubSubLabels,
    pub push_config: Option<PushConfig>,
    pub dead_letter_policy: Option<DeadLetterPolicy>,
    pub retry_policy: Option<RetryPolicy>,
    pub enable_message_ordering: bool,
    pub retain_acked_messages: bool,
    pub filter: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub message_id: String,
    pub topic_name: String,
    pub data: String,
    pub attributes: PubSubMessageAttributes,
    pub ordering_key: Option<String>,
    pub publish_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPendingMessage {
    pub ack_id: String,
    pub message: StoredMessage,
    pub subscription_name: String,
    pub delivery_attempt: i32,
    pub ack_deadline: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSnapshot {
    pub name: String,
    pub project_id: String,
    pub snapshot_id: String,
    pub subscription_name: String,
    pub topic: String,
    pub labels: PubSubLabels,
    pub expire_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTopicOutput {
    pub labels: String,
    pub message_retention_duration: String,
    pub schema_settings: String,
    pub topic_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTopicsOutput {
    pub next_page_token: String,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishBatchOutput {
    pub failure_count: i64,
    pub message_ids: Vec<String>,
    pub success_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSubscriptionOutput {
    pub ack_deadline_seconds: i64,
    pub labels: String,
    pub message_retention_duration: String,
    pub push_config: String,
    pub subscription_name: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSubscriptionsOutput {
    pub next_page_token: String,
    pub subscriptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullOutput {
    pub count: i64,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSnapshotOutput {
    pub expire_time: chrono::DateTime<chrono::Utc>,
    pub snapshot_name: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTopicSubscriptionsOutput {
    pub next_page_token: String,
    pub subscriptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubMessage {
    pub message_id: String,
    pub data: String,
    pub attributes: String,
    pub publish_time: chrono::DateTime<chrono::Utc>,
    pub ordering_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubLabels {
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubLabel {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubSchemaSettings {
    pub schema: String,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubTopic {
    pub name: String,
    pub labels: String,
    pub message_retention_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubMessageAttributes {
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubMessageAttribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubPublishMessage {
    pub data: String,
    pub attributes: String,
    pub ordering_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeadLetterPolicy {
    pub dead_letter_topic: String,
    pub max_delivery_attempts: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushConfig {
    pub push_endpoint: String,
    pub attributes: String,
    pub oidc_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OidcToken {
    pub service_account_email: String,
    pub audience: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryPolicy {
    pub minimum_backoff: String,
    pub maximum_backoff: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubSubscription {
    pub name: String,
    pub topic: String,
    pub ack_deadline_seconds: i64,
    pub message_retention_duration: String,
    pub labels: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubReceivedMessage {
    pub ack_id: String,
    pub message: String,
    pub delivery_attempt: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubTopic {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub labels: String,
    pub message_retention_duration: String,
    pub project_id: String,
    pub schema_encoding: String,
    pub schema_name: String,
    pub topic_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubSubscription {
    pub ack_deadline_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_max_delivery_attempts: i64,
    pub dead_letter_topic: String,
    #[serde(default)]
    pub enable_message_ordering: bool,
    pub filter: String,
    pub labels: String,
    pub message_retention_duration: String,
    pub project_id: String,
    pub push_endpoint: String,
    #[serde(default)]
    pub retain_acked_messages: bool,
    pub retry_max_backoff: String,
    pub retry_min_backoff: String,
    pub subscription_id: String,
    pub topic_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubMessage {
    pub attributes: String,
    pub data: String,
    pub message_id: String,
    pub ordering_key: String,
    pub publish_time: chrono::DateTime<chrono::Utc>,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubReceivedMessage {
    pub ack_id: String,
    pub delivery_attempt: i64,
    pub message_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubSnapshot {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expire_time: chrono::DateTime<chrono::Utc>,
    pub labels: String,
    pub project_id: String,
    pub snapshot_id: String,
    pub subscription_id: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubLabel {
    pub key: String,
    pub resource_id: String,
    pub resource_type: String,
    pub value: String,
}

#[async_trait]
pub trait PubsubAction {
    async fn create_topic(&self, labels: String, message_retention_duration: String, project_id: String, schema_settings: String, topic_id: String, topic_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_topic(&self, project_id: String, topic_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_topic(&self, project_id: String, topic_id: String) -> Result<GetTopicOutput, Box<dyn std::error::Error>>;
    async fn list_topics(&self, page_size: i64, page_token: String, project_id: String) -> Result<ListTopicsOutput, Box<dyn std::error::Error>>;
    async fn publish(&self, attributes: String, data: String, ordering_key: String, project_id: String, topic_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn publish_batch(&self, messages: Vec<String>, project_id: String, topic_id: String) -> Result<PublishBatchOutput, Box<dyn std::error::Error>>;
    async fn create_subscription(&self, ack_deadline_seconds: i64, dead_letter_policy: String, enable_message_ordering: bool, filter: String, message_retention_duration: String, project_id: String, push_config: String, retain_acked_messages: bool, retry_policy: String, subscription_id: String, topic_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_subscription(&self, project_id: String, subscription_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_subscription(&self, project_id: String, subscription_id: String) -> Result<GetSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn list_subscriptions(&self, page_size: i64, page_token: String, project_id: String) -> Result<ListSubscriptionsOutput, Box<dyn std::error::Error>>;
    async fn pull(&self, max_messages: i64, project_id: String, return_immediately: bool, subscription_id: String) -> Result<PullOutput, Box<dyn std::error::Error>>;
    async fn acknowledge(&self, ack_ids: Vec<String>, project_id: String, subscription_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn modify_ack_deadline(&self, ack_deadline_seconds: i64, ack_ids: Vec<String>, project_id: String, subscription_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn seek(&self, project_id: String, snapshot: String, subscription_id: String, time: chrono::DateTime<chrono::Utc>) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_snapshot(&self, labels: String, project_id: String, snapshot_id: String, subscription_id: String) -> Result<CreateSnapshotOutput, Box<dyn std::error::Error>>;
    async fn delete_snapshot(&self, project_id: String, snapshot_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_topic_subscriptions(&self, page_size: i64, page_token: String, project_id: String, topic_id: String) -> Result<ListTopicSubscriptionsOutput, Box<dyn std::error::Error>>;
    async fn update_topic(&self, labels: String, message_retention_duration: String, project_id: String, topic_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_subscription(&self, ack_deadline_seconds: i64, labels: String, message_retention_duration: String, project_id: String, push_config: String, subscription_id: String) -> Result<(), Box<dyn std::error::Error>>;
}

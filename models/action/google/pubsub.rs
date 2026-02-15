// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTopicInput {
    pub labels: String,
    pub message_retention_duration: String,
    pub project_id: String,
    pub schema_settings: String,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTopicOutput {
    pub success: bool,
    pub topic_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTopicInput {
    pub project_id: String,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTopicOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTopicInput {
    pub project_id: String,
    pub topic_id: String,
}

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
pub struct ListTopicsInput {
    pub page_size: i64,
    pub page_token: String,
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTopicsOutput {
    pub next_page_token: String,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishInput {
    pub attributes: String,
    pub data: String,
    pub ordering_key: String,
    pub project_id: String,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishBatchInput {
    pub messages: Vec<String>,
    pub project_id: String,
    pub topic_id: String,
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
pub struct CreateSubscriptionInput {
    pub ack_deadline_seconds: i64,
    pub dead_letter_policy: String,
    #[serde(default)]
    pub enable_message_ordering: bool,
    pub filter: String,
    pub message_retention_duration: String,
    pub project_id: String,
    pub push_config: String,
    #[serde(default)]
    pub retain_acked_messages: bool,
    pub retry_policy: String,
    pub subscription_id: String,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSubscriptionOutput {
    pub subscription_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSubscriptionInput {
    pub project_id: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSubscriptionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSubscriptionInput {
    pub project_id: String,
    pub subscription_id: String,
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
pub struct ListSubscriptionsInput {
    pub page_size: i64,
    pub page_token: String,
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSubscriptionsOutput {
    pub next_page_token: String,
    pub subscriptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullInput {
    pub max_messages: i64,
    pub project_id: String,
    #[serde(default)]
    pub return_immediately: bool,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullOutput {
    pub count: i64,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcknowledgeInput {
    pub ack_ids: Vec<String>,
    pub project_id: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcknowledgeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyAckDeadlineInput {
    pub ack_deadline_seconds: i64,
    pub ack_ids: Vec<String>,
    pub project_id: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyAckDeadlineOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SeekInput {
    pub project_id: String,
    pub snapshot: String,
    pub subscription_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SeekOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSnapshotInput {
    pub labels: String,
    pub project_id: String,
    pub snapshot_id: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSnapshotOutput {
    pub expire_time: chrono::DateTime<chrono::Utc>,
    pub snapshot_name: String,
    pub success: bool,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSnapshotInput {
    pub project_id: String,
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSnapshotOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTopicSubscriptionsInput {
    pub page_size: i64,
    pub page_token: String,
    pub project_id: String,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTopicSubscriptionsOutput {
    pub next_page_token: String,
    pub subscriptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTopicInput {
    pub labels: String,
    pub message_retention_duration: String,
    pub project_id: String,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTopicOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateSubscriptionInput {
    pub ack_deadline_seconds: i64,
    pub labels: String,
    pub message_retention_duration: String,
    pub project_id: String,
    pub push_config: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateSubscriptionOutput {
    pub success: bool,
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

#[async_trait]
pub trait PubsubAction {
    async fn create_topic(&self, input: CreateTopicInput) -> Result<CreateTopicOutput, Box<dyn std::error::Error>>;
    async fn delete_topic(&self, input: DeleteTopicInput) -> Result<DeleteTopicOutput, Box<dyn std::error::Error>>;
    async fn get_topic(&self, input: GetTopicInput) -> Result<GetTopicOutput, Box<dyn std::error::Error>>;
    async fn list_topics(&self, input: ListTopicsInput) -> Result<ListTopicsOutput, Box<dyn std::error::Error>>;
    async fn publish(&self, input: PublishInput) -> Result<PublishOutput, Box<dyn std::error::Error>>;
    async fn publish_batch(&self, input: PublishBatchInput) -> Result<PublishBatchOutput, Box<dyn std::error::Error>>;
    async fn create_subscription(&self, input: CreateSubscriptionInput) -> Result<CreateSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn delete_subscription(&self, input: DeleteSubscriptionInput) -> Result<DeleteSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn get_subscription(&self, input: GetSubscriptionInput) -> Result<GetSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn list_subscriptions(&self, input: ListSubscriptionsInput) -> Result<ListSubscriptionsOutput, Box<dyn std::error::Error>>;
    async fn pull(&self, input: PullInput) -> Result<PullOutput, Box<dyn std::error::Error>>;
    async fn acknowledge(&self, input: AcknowledgeInput) -> Result<AcknowledgeOutput, Box<dyn std::error::Error>>;
    async fn modify_ack_deadline(&self, input: ModifyAckDeadlineInput) -> Result<ModifyAckDeadlineOutput, Box<dyn std::error::Error>>;
    async fn seek(&self, input: SeekInput) -> Result<SeekOutput, Box<dyn std::error::Error>>;
    async fn create_snapshot(&self, input: CreateSnapshotInput) -> Result<CreateSnapshotOutput, Box<dyn std::error::Error>>;
    async fn delete_snapshot(&self, input: DeleteSnapshotInput) -> Result<DeleteSnapshotOutput, Box<dyn std::error::Error>>;
    async fn list_topic_subscriptions(&self, input: ListTopicSubscriptionsInput) -> Result<ListTopicSubscriptionsOutput, Box<dyn std::error::Error>>;
    async fn update_topic(&self, input: UpdateTopicInput) -> Result<UpdateTopicOutput, Box<dyn std::error::Error>>;
    async fn update_subscription(&self, input: UpdateSubscriptionInput) -> Result<UpdateSubscriptionOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// google_pubsub_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubTopic {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub labels: Option<String>,
    pub message_retention_duration: Option<String>,
    pub project_id: String,
    pub schema_encoding: Option<String>,
    pub schema_name: Option<String>,
    pub topic_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl GooglePubsubTopic {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_pubsub_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubSubscription {
    pub ack_deadline_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dead_letter_max_delivery_attempts: Option<i64>,
    pub dead_letter_topic: Option<String>,
    #[serde(default)]
    pub enable_message_ordering: bool,
    pub filter: Option<String>,
    pub labels: Option<String>,
    pub message_retention_duration: Option<String>,
    pub project_id: String,
    pub push_endpoint: Option<String>,
    #[serde(default)]
    pub retain_acked_messages: bool,
    pub retry_max_backoff: Option<String>,
    pub retry_min_backoff: Option<String>,
    pub subscription_id: String,
    /// Reference: google_pubsub_topic.id
    pub topic_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl GooglePubsubSubscription {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_pubsub_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubMessage {
    pub attributes: Option<String>,
    pub data: String,
    pub message_id: String,
    pub ordering_key: Option<String>,
    pub publish_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: google_pubsub_topic.id
    pub topic_id: String,
}

impl GooglePubsubMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_pubsub_received_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubReceivedMessage {
    pub ack_id: String,
    pub delivery_attempt: Option<i64>,
    /// Reference: google_pubsub_message.id
    pub message_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub received_at: chrono::DateTime<chrono::Utc>,
    /// Reference: google_pubsub_subscription.id
    pub subscription_id: String,
}

impl GooglePubsubReceivedMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_pubsub_snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubSnapshot {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expire_time: Option<chrono::DateTime<chrono::Utc>>,
    pub labels: Option<String>,
    pub project_id: String,
    pub snapshot_id: String,
    /// Reference: google_pubsub_subscription.id
    pub subscription_id: String,
    pub topic: Option<String>,
}

impl GooglePubsubSnapshot {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_pubsub_label
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GooglePubsubLabel {
    pub key: String,
    pub resource_id: String,
    pub resource_type: String,
    pub value: Option<String>,
}

impl GooglePubsubLabel {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubMessage {
    pub message_id: String,
    pub data: String,
    pub attributes: String,
    pub publish_time: chrono::DateTime<chrono::Utc>,
    pub ordering_key: String,
}

impl PubSubMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_labels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubLabels {
    pub labels: Vec<String>,
}

impl PubSubLabels {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_label
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubLabel {
    pub key: String,
    pub value: String,
}

impl PubSubLabel {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_schema_settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubSchemaSettings {
    pub schema: String,
    pub encoding: String,
}

impl PubSubSchemaSettings {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubTopic {
    pub name: String,
    pub labels: String,
    pub message_retention_duration: String,
}

impl PubSubTopic {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_message_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubMessageAttributes {
    pub attributes: Vec<String>,
}

impl PubSubMessageAttributes {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_message_attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubMessageAttribute {
    pub key: String,
    pub value: String,
}

impl PubSubMessageAttribute {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_publish_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubPublishMessage {
    pub data: String,
    pub attributes: String,
    pub ordering_key: String,
}

impl PubSubPublishMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// dead_letter_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeadLetterPolicy {
    pub dead_letter_topic: String,
    pub max_delivery_attempts: i64,
}

impl DeadLetterPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// push_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushConfig {
    pub push_endpoint: String,
    pub attributes: String,
    pub oidc_token: String,
}

impl PushConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// oidc_token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OidcToken {
    pub service_account_email: String,
    pub audience: String,
}

impl OidcToken {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// retry_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryPolicy {
    pub minimum_backoff: String,
    pub maximum_backoff: String,
}

impl RetryPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubSubscription {
    pub name: String,
    pub topic: String,
    pub ack_deadline_seconds: i64,
    pub message_retention_duration: String,
    pub labels: String,
}

impl PubSubSubscription {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pub_sub_received_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PubSubReceivedMessage {
    pub ack_id: String,
    pub message: String,
    pub delivery_attempt: i64,
}

impl PubSubReceivedMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


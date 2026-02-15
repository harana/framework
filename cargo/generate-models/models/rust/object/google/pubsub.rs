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


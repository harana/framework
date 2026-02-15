// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// push_device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushDevice {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub device_token: String,
    #[serde(default)]
    pub is_active: bool,
    pub platform: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_agent: Option<String>,
    /// Reference: user.id
    pub user_id: String,
}

impl PushDevice {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// push_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushTopic {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub platform: String,
}

impl PushTopic {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// push_topic_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushTopicSubscription {
    /// Reference: push_device.id
    pub device_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub subscribed_at: chrono::DateTime<chrono::Utc>,
    /// Reference: push_topic.id
    pub topic_id: String,
}

impl PushTopicSubscription {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// push_notification_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushNotificationLog {
    pub body: String,
    /// Reference: push_device.id
    pub device_id: Option<String>,
    pub error_message: Option<String>,
    pub message_id: Option<String>,
    pub platform: String,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub title: String,
    /// Reference: push_topic.id
    pub topic_id: Option<String>,
}

impl PushNotificationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


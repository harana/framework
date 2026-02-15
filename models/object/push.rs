// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub user_id: String,
}

impl PushDevice {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushTopicSubscription {
    pub device_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub subscribed_at: chrono::DateTime<chrono::Utc>,
    pub topic_id: String,
}

impl PushTopicSubscription {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushNotificationLog {
    pub body: String,
    pub device_id: Option<String>,
    pub error_message: Option<String>,
    pub message_id: Option<String>,
    pub platform: String,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub title: String,
    pub topic_id: Option<String>,
}

impl PushNotificationLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushNotification {
    pub title: String,
    pub body: String,
    pub platform: String,
    pub token: String,
    pub data: std::collections::HashMap<String, String>,
    pub badge: i64,
    pub sound: String,
}

impl PushNotification {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushAction {
    pub method: String,
    pub title: String,
    pub icon: String,
}

impl PushAction {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushSubscription {
    pub endpoint: String,
    pub keys: String,
}

impl PushSubscription {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushSubscriptionKeys {
    pub p256dh: String,
    pub auth: String,
}

impl PushSubscriptionKeys {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushFailure {
    pub token: String,
    pub error: String,
}

impl PushFailure {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


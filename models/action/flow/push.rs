// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendApnsOutput {
    pub apns_id: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendFcmOutput {
    pub error: String,
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMulticastPushOutput {
    pub failed: i64,
    pub failures: Vec<String>,
    pub successful: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidatePushTokenOutput {
    pub error: String,
    pub valid: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushAction {
    pub method: String,
    pub title: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushSubscription {
    pub endpoint: String,
    pub keys: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushSubscriptionKeys {
    pub p256dh: String,
    pub auth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushFailure {
    pub token: String,
    pub error: String,
}

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
    pub user_agent: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushTopic {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushTopicSubscription {
    pub device_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub subscribed_at: chrono::DateTime<chrono::Utc>,
    pub topic_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushNotificationLog {
    pub body: String,
    pub device_id: String,
    pub error_message: String,
    pub message_id: String,
    pub platform: String,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub title: String,
    pub topic_id: String,
}

#[async_trait]
pub trait PushAction {
    async fn send_apns(&self, alert: String, badge: i64, body: String, category: String, collapse_id: String, custom_data: std::collections::HashMap<String, String>, device_token: String, expiration: i64, priority: String, sound: String, subtitle: String, thread_id: String, title: String, topic: String) -> Result<SendApnsOutput, Box<dyn std::error::Error>>;
    async fn send_web_push(&self, actions: Vec<String>, badge: String, body: String, data: std::collections::HashMap<String, String>, dir: String, icon: String, image: String, lang: String, renotify: bool, require_intermethod: bool, silent: bool, subscription: String, tag: String, timestamp: i64, title: String, vibrate: Vec<i64>) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_fcm(&self, body: String, channel_id: String, click_method: String, collapse_key: String, color: String, data: std::collections::HashMap<String, String>, icon: String, image: String, priority: String, registration_token: String, sound: String, tag: String, time_to_live: i64, title: String) -> Result<SendFcmOutput, Box<dyn std::error::Error>>;
    async fn send_multicast_push(&self, body: String, data: std::collections::HashMap<String, String>, platform: String, title: String, tokens: Vec<String>) -> Result<SendMulticastPushOutput, Box<dyn std::error::Error>>;
    async fn send_topic_push(&self, body: String, data: std::collections::HashMap<String, String>, platform: String, title: String, topic: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn subscribe_to_topic(&self, platform: String, tokens: Vec<String>, topic: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn unsubscribe_from_topic(&self, platform: String, tokens: Vec<String>, topic: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn validate_push_token(&self, platform: String, token: String) -> Result<ValidatePushTokenOutput, Box<dyn std::error::Error>>;
}

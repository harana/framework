// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendApnsInput {
    pub alert: String,
    pub badge: i64,
    pub body: String,
    pub category: String,
    pub collapse_id: String,
    pub custom_data: std::collections::HashMap<String, String>,
    pub device_token: String,
    pub expiration: i64,
    pub priority: String,
    pub sound: String,
    pub subtitle: String,
    pub thread_id: String,
    pub title: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendApnsOutput {
    pub apns_id: String,
    pub error: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendWebPushInput {
    pub actions: Vec<String>,
    pub badge: String,
    pub body: String,
    pub data: std::collections::HashMap<String, String>,
    pub dir: String,
    pub icon: String,
    pub image: String,
    pub lang: String,
    #[serde(default)]
    pub renotify: bool,
    #[serde(default)]
    pub require_intermethod: bool,
    #[serde(default)]
    pub silent: bool,
    pub subscription: String,
    pub tag: String,
    pub timestamp: i64,
    pub title: String,
    pub vibrate: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendWebPushOutput {
    pub error: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendFcmInput {
    pub body: String,
    pub channel_id: String,
    pub click_method: String,
    pub collapse_key: String,
    pub color: String,
    pub data: std::collections::HashMap<String, String>,
    pub icon: String,
    pub image: String,
    pub priority: String,
    pub registration_token: String,
    pub sound: String,
    pub tag: String,
    pub time_to_live: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendFcmOutput {
    pub error: String,
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMulticastPushInput {
    pub body: String,
    pub data: std::collections::HashMap<String, String>,
    pub platform: String,
    pub title: String,
    pub tokens: Vec<String>,
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
pub struct SendTopicPushInput {
    pub body: String,
    pub data: std::collections::HashMap<String, String>,
    pub platform: String,
    pub title: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendTopicPushOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubscribeToTopicInput {
    pub platform: String,
    pub tokens: Vec<String>,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubscribeToTopicOutput {
    pub failed_tokens: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsubscribeFromTopicInput {
    pub platform: String,
    pub tokens: Vec<String>,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsubscribeFromTopicOutput {
    pub failed_tokens: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidatePushTokenInput {
    pub platform: String,
    pub token: String,
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

#[async_trait]
pub trait PushAction {
    async fn send_apns(&self, input: SendApnsInput) -> Result<SendApnsOutput, Box<dyn std::error::Error>>;
    async fn send_web_push(&self, input: SendWebPushInput) -> Result<SendWebPushOutput, Box<dyn std::error::Error>>;
    async fn send_fcm(&self, input: SendFcmInput) -> Result<SendFcmOutput, Box<dyn std::error::Error>>;
    async fn send_multicast_push(&self, input: SendMulticastPushInput) -> Result<SendMulticastPushOutput, Box<dyn std::error::Error>>;
    async fn send_topic_push(&self, input: SendTopicPushInput) -> Result<SendTopicPushOutput, Box<dyn std::error::Error>>;
    async fn subscribe_to_topic(&self, input: SubscribeToTopicInput) -> Result<SubscribeToTopicOutput, Box<dyn std::error::Error>>;
    async fn unsubscribe_from_topic(&self, input: UnsubscribeFromTopicInput) -> Result<UnsubscribeFromTopicOutput, Box<dyn std::error::Error>>;
    async fn validate_push_token(&self, input: ValidatePushTokenInput) -> Result<ValidatePushTokenOutput, Box<dyn std::error::Error>>;
}

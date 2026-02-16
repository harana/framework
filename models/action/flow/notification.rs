// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBulkOutput {
    pub total: i64,
    pub successful: i64,
    pub failed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusOutput {
    pub status: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub delivered_at: chrono::DateTime<chrono::Utc>,
    pub read_at: chrono::DateTime<chrono::Utc>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListOutput {
    pub notifications: Vec<String>,
    pub total: i64,
    pub unread_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Notification {
    pub notification_id: String,
    pub user_id: String,
    pub title: String,
    pub body: String,
    pub notification_type: String,
    pub channel: String,
    pub status: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
    pub delivered_at: chrono::DateTime<chrono::Utc>,
    pub read_at: chrono::DateTime<chrono::Utc>,
    pub action_url: String,
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeviceRegistration {
    pub device_token: String,
    pub user_id: String,
    pub platform: String,
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationPreference {
    pub channel: String,
    #[serde(default)]
    pub is_enabled: bool,
    pub notification_type: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushSubscription {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub device_token: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[async_trait]
pub trait NotificationAction {
    async fn send_push(&self, user_id: String, title: String, body: String, data: std::collections::HashMap<String, String>, badge: i64, sound: String, priority: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_sms(&self, phone_number: String, message: String, sender_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_in_app(&self, user_id: String, title: String, message: String, notification_type: String, action_url: String, data: std::collections::HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_bulk(&self, user_ids: Vec<String>, title: String, body: String, channel: String, data: std::collections::HashMap<String, String>) -> Result<SendBulkOutput, Box<dyn std::error::Error>>;
    async fn status(&self, notification_id: String) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn mark_as_read(&self, notification_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list(&self, user_id: String, unread_only: bool, limit: i64, offset: i64) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, notification_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn register_device(&self, user_id: String, device_token: String, platform: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn unregister_device(&self, device_token: String) -> Result<(), Box<dyn std::error::Error>>;
}

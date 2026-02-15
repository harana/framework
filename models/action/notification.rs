// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendPushInput {
    pub user_id: String,
    pub title: String,
    pub body: String,
    pub data: std::collections::HashMap<String, String>,
    pub badge: i64,
    pub sound: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendPushOutput {
    pub notification_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendSmsInput {
    pub phone_number: String,
    pub message: String,
    pub sender_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendSmsOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendInAppInput {
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub action_url: String,
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendInAppOutput {
    pub notification_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBulkInput {
    pub user_ids: Vec<String>,
    pub title: String,
    pub body: String,
    pub channel: String,
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBulkOutput {
    pub total: i64,
    pub successful: i64,
    pub failed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusInput {
    pub notification_id: String,
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
pub struct MarkAsReadInput {
    pub notification_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkAsReadOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListInput {
    pub user_id: String,
    #[serde(default)]
    pub unread_only: bool,
    pub limit: i64,
    pub offset: i64,
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
pub struct DeleteInput {
    pub notification_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterDeviceInput {
    pub user_id: String,
    pub device_token: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterDeviceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnregisterDeviceInput {
    pub device_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnregisterDeviceOutput {
    pub success: bool,
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

#[async_trait]
pub trait NotificationAction {
    async fn send_push(&self, input: SendPushInput) -> Result<SendPushOutput, Box<dyn std::error::Error>>;
    async fn send_sms(&self, input: SendSmsInput) -> Result<SendSmsOutput, Box<dyn std::error::Error>>;
    async fn send_in_app(&self, input: SendInAppInput) -> Result<SendInAppOutput, Box<dyn std::error::Error>>;
    async fn send_bulk(&self, input: SendBulkInput) -> Result<SendBulkOutput, Box<dyn std::error::Error>>;
    async fn status(&self, input: StatusInput) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn mark_as_read(&self, input: MarkAsReadInput) -> Result<MarkAsReadOutput, Box<dyn std::error::Error>>;
    async fn list(&self, input: ListInput) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn register_device(&self, input: RegisterDeviceInput) -> Result<RegisterDeviceOutput, Box<dyn std::error::Error>>;
    async fn unregister_device(&self, input: UnregisterDeviceInput) -> Result<UnregisterDeviceOutput, Box<dyn std::error::Error>>;
}

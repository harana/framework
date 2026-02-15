// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Notification {
    pub action_url: Option<String>,
    pub body: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_read: bool,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub type: String,
    pub user_id: String,
}

impl Notification {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl NotificationPreference {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl PushSubscription {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeviceRegistration {
    pub device_token: String,
    pub user_id: String,
    pub platform: String,
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

impl DeviceRegistration {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


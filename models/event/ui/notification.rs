// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationShown {
    pub notification_id: String,
    pub notification_type: String,
    pub notification_title: Option<String>,
    pub notification_message: Option<String>,
    pub duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub shown_at: chrono::DateTime<chrono::Utc>,
}

impl NotificationShown {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationDismissed {
    pub notification_id: String,
    pub notification_type: String,
    pub dismiss_method: String,
    pub time_visible_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub dismissed_at: chrono::DateTime<chrono::Utc>,
}

impl NotificationDismissed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationClicked {
    pub notification_id: String,
    pub notification_type: String,
    pub notification_title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl NotificationClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationActionClicked {
    pub notification_id: String,
    pub action_id: String,
    pub action_label: Option<String>,
    pub action_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl NotificationActionClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToastExpired {
    pub toast_id: String,
    pub toast_type: String,
    pub toast_message: Option<String>,
    pub duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

impl ToastExpired {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModalOpened {
    pub modal_id: String,
    pub modal_name: Option<String>,
    pub modal_type: String,
    pub trigger_source: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

impl ModalOpened {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModalClosed {
    pub modal_id: String,
    pub modal_name: Option<String>,
    pub close_reason: String,
    pub time_open_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl ModalClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModalDismissed {
    pub modal_id: String,
    pub modal_name: Option<String>,
    pub dismiss_method: String,
    #[serde(default = "chrono::Utc::now")]
    pub dismissed_at: chrono::DateTime<chrono::Utc>,
}

impl ModalDismissed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfirmationAccepted {
    pub modal_id: String,
    pub modal_name: Option<String>,
    pub confirmation_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub accepted_at: chrono::DateTime<chrono::Utc>,
}

impl ConfirmationAccepted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfirmationRejected {
    pub modal_id: String,
    pub modal_name: Option<String>,
    pub confirmation_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub rejected_at: chrono::DateTime<chrono::Utc>,
}

impl ConfirmationRejected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModalBackdropClicked {
    pub modal_id: String,
    pub modal_name: Option<String>,
    #[serde(default)]
    pub closes_modal: bool,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl ModalBackdropClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


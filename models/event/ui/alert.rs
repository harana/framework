// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlertOpened {
    pub alert_id: String,
    pub alert_name: Option<String>,
    pub alert_title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

impl AlertOpened {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlertClosed {
    pub alert_id: String,
    pub alert_name: Option<String>,
    pub close_reason: String,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl AlertClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlertActionClicked {
    pub alert_id: String,
    pub action_id: String,
    pub action_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl AlertActionClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlertDismissed {
    pub alert_id: String,
    pub alert_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub dismissed_at: chrono::DateTime<chrono::Utc>,
}

impl AlertDismissed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


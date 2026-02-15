// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DialogOpened {
    pub dialog_id: String,
    pub dialog_name: Option<String>,
    pub dialog_title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

impl DialogOpened {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DialogClosed {
    pub dialog_id: String,
    pub dialog_name: Option<String>,
    pub close_reason: String,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl DialogClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DialogActionClicked {
    pub dialog_id: String,
    pub action_id: String,
    pub action_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl DialogActionClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DialogSubmitted {
    pub dialog_id: String,
    pub dialog_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

impl DialogSubmitted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DialogCancelled {
    pub dialog_id: String,
    pub dialog_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub cancelled_at: chrono::DateTime<chrono::Utc>,
}

impl DialogCancelled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


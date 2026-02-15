// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CardClicked {
    pub card_id: String,
    pub card_title: Option<String>,
    pub card_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl CardClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CardExpanded {
    pub card_id: String,
    pub card_title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub expanded_at: chrono::DateTime<chrono::Utc>,
}

impl CardExpanded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CardCollapsed {
    pub card_id: String,
    pub card_title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub collapsed_at: chrono::DateTime<chrono::Utc>,
}

impl CardCollapsed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CardActionClicked {
    pub card_id: String,
    pub action_id: String,
    pub action_name: Option<String>,
    pub action_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl CardActionClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CardDismissed {
    pub card_id: String,
    pub card_title: Option<String>,
    pub dismiss_method: String,
    #[serde(default = "chrono::Utc::now")]
    pub dismissed_at: chrono::DateTime<chrono::Utc>,
}

impl CardDismissed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


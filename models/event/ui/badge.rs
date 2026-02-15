// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BadgeButtonClicked {
    pub badge_id: String,
    pub badge_label: Option<String>,
    pub badge_color: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl BadgeButtonClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BadgeHovered {
    pub badge_id: String,
    pub badge_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub hovered_at: chrono::DateTime<chrono::Utc>,
}

impl BadgeHovered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


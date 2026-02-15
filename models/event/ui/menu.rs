// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MenuOpened {
    pub menu_id: String,
    pub menu_name: Option<String>,
    pub menu_type: String,
    pub trigger_element: Option<String>,
    pub items_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

impl MenuOpened {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MenuClosed {
    pub menu_id: String,
    pub menu_name: Option<String>,
    pub close_reason: String,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl MenuClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmenuExpanded {
    pub menu_id: String,
    pub submenu_id: String,
    pub submenu_label: Option<String>,
    pub parent_item_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub expanded_at: chrono::DateTime<chrono::Utc>,
}

impl SubmenuExpanded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


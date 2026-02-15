// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MenuItem {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub href: Option<String>,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub label: String,
    pub menu_id: String,
    pub order: i64,
    pub parent_id: Option<String>,
    pub slug: String,
    pub target: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MenuItem {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MenuItemAction {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub href: Option<String>,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub label: String,
    pub menu_item_id: String,
    pub order: i64,
    pub slug: String,
    pub target: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MenuItemAction {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// menu_item
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
    /// Reference: menu_item.id
    pub parent_id: Option<String>,
    pub slug: String,
    pub target: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MenuItem {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// menu_item_action
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
    /// Reference: menu_item.id
    pub menu_item_id: String,
    pub order: i64,
    pub slug: String,
    pub target: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MenuItemAction {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


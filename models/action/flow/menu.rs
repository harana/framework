// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub href: String,
    pub icon: String,
    #[serde(default)]
    pub is_active: bool,
    pub label: String,
    pub menu_id: String,
    pub order: i64,
    pub parent_id: String,
    pub slug: String,
    pub target: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemAction {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub href: String,
    pub icon: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MenuItem {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub href: String,
    pub icon: String,
    #[serde(default)]
    pub is_active: bool,
    pub label: String,
    pub menu_id: String,
    pub order: i64,
    pub parent_id: String,
    pub slug: String,
    pub target: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MenuItemAction {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub href: String,
    pub icon: String,
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


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SidebarItemEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SidebarItem {
    pub href: Option<String>,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_current: bool,
    pub label: String,
    pub order: i64,
    pub section_id: Option<String>,
    pub sidebar_id: String,
}


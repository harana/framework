// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TabItemEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TabItem {
    pub badge: Option<String>,
    pub href: Option<String>,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_current: bool,
    pub label: String,
    pub order: i64,
    pub tabs_id: String,
}


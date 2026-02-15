// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavbarItemEvent {
    Click,
    Hover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NavbarItem {
    pub aria_label: Option<String>,
    pub href: Option<String>,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_current: bool,
    pub label: Option<String>,
    pub navbar_id: String,
    pub order: i64,
    pub section_id: Option<String>,
}


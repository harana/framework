// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DropdownButtonEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropdownButton {
    pub color: String,
    pub dropdown_id: String,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_outline: bool,
    #[serde(default)]
    pub is_plain: bool,
    pub label: String,
}


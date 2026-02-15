// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitchEvent {
    Change,
    ToggleOff,
    ToggleOn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Switch {
    pub color: String,
    #[serde(default)]
    pub default_checked: bool,
    pub description: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    pub label: Option<String>,
    pub value: Option<String>,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonEvent {
    Click,
    DoubleClick,
    LongPress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Button {
    pub color: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub is_outline: bool,
    #[serde(default)]
    pub is_plain: bool,
    pub label: String,
    pub type: String,
}


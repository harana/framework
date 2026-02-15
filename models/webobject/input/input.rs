// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputEvent {
    Blur,
    Change,
    Clear,
    Focus,
    Input,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Input {
    pub aria_label: Option<String>,
    #[serde(default)]
    pub auto_focus: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub error_message: Option<String>,
    pub icon: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub is_invalid: bool,
    pub label: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub type: String,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextareaEvent {
    Blur,
    Change,
    CharacterLimitReach,
    Focus,
    Input,
    Resize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Textarea {
    pub aria_label: Option<String>,
    #[serde(default)]
    pub auto_focus: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub error_message: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub is_invalid: bool,
    #[serde(default)]
    pub is_resizable: bool,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub rows: Option<i64>,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectEvent {
    Blur,
    Change,
    Close,
    Focus,
    Open,
    OptionSelect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Select {
    pub aria_label: Option<String>,
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub error_message: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub is_invalid: bool,
    pub label: Option<String>,
}


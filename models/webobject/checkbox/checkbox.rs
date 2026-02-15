// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckboxEvent {
    Change,
    Checked,
    IndeterminateChange,
    Unchecked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Checkbox {
    pub color: String,
    #[serde(default)]
    pub default_checked: bool,
    pub description: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    #[serde(default)]
    pub is_indeterminate: bool,
    pub label: Option<String>,
    pub value: Option<String>,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RadioGroupEvent {
    Change,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RadioGroup {
    pub aria_label: Option<String>,
    pub default_value: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckboxGroupEvent {
    Change,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckboxGroup {
    pub aria_label: Option<String>,
}


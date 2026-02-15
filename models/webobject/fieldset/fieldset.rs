// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldsetEvent {
    Disable,
    Enable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Fieldset {
    pub aria_label: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    pub legend: Option<String>,
}


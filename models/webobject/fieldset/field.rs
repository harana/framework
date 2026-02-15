// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldEvent {
    Blur,
    ErrorClear,
    ErrorShow,
    Focus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Field {
    pub description: Option<String>,
    pub error_message: Option<String>,
    pub field_group_id: Option<String>,
    #[serde(default)]
    pub is_disabled: bool,
    pub label: Option<String>,
}


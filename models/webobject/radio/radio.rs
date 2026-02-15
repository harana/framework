// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RadioEvent {
    Blur,
    Focus,
    Select,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Radio {
    pub color: String,
    pub description: Option<String>,
    pub group_id: String,
    #[serde(default)]
    pub is_disabled: bool,
    pub label: String,
    pub order: i64,
    pub value: String,
}


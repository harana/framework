// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SelectOption {
    #[serde(default)]
    pub is_disabled: bool,
    pub label: String,
    pub order: i64,
    pub select_id: String,
    pub value: String,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogActionEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DialogAction {
    pub color: String,
    pub dialog_id: String,
    #[serde(default)]
    pub is_plain: bool,
    pub label: String,
    pub order: i64,
}


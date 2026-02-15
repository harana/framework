// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertEvent {
    Close,
    Dismiss,
    Open,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Alert {
    pub description: Option<String>,
    #[serde(default)]
    pub is_open: bool,
    pub size: String,
    pub title: String,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyoutMenuEvent {
    Close,
    Open,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlyoutMenu {
    pub anchor: String,
    #[serde(default)]
    pub is_open: bool,
    pub label: String,
    pub size: String,
}


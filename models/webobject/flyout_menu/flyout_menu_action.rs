// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyoutMenuActionEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlyoutMenuAction {
    pub flyout_menu_id: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub label: String,
    pub order: i64,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationEvent {
    Close,
    Dismiss,
    Open,
    PrimaryAction,
    SecondaryAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Notification {
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub icon: String,
    #[serde(default)]
    pub is_open: bool,
    pub primary_action_label: Option<String>,
    pub secondary_action_label: Option<String>,
    #[serde(default)]
    pub show_close_button: bool,
    pub title: String,
    pub variant: String,
}


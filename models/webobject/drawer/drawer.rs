// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawerEvent {
    Close,
    Open,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Drawer {
    #[serde(default)]
    pub is_open: bool,
    pub position: String,
    #[serde(default)]
    pub show_backdrop: bool,
    #[serde(default)]
    pub show_footer: bool,
    pub size: String,
    pub title: String,
}


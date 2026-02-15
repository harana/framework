// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SidebarLayoutInitialized {
    pub layout_id: String,
    pub layout_name: Option<String>,
    pub sidebar_id: String,
    pub navbar_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub initialized_at: chrono::DateTime<chrono::Utc>,
}

impl SidebarLayoutInitialized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SidebarLayoutResized {
    pub layout_id: String,
    pub layout_name: Option<String>,
    pub old_sidebar_width: Option<i64>,
    pub new_sidebar_width: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub resized_at: chrono::DateTime<chrono::Utc>,
}

impl SidebarLayoutResized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


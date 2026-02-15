// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StackedLayoutInitialized {
    pub layout_id: String,
    pub layout_name: Option<String>,
    pub navbar_id: String,
    pub sidebar_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub initialized_at: chrono::DateTime<chrono::Utc>,
}

impl StackedLayoutInitialized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StackedLayoutViewChanged {
    pub layout_id: String,
    pub layout_name: Option<String>,
    pub old_view: Option<String>,
    pub new_view: String,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl StackedLayoutViewChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


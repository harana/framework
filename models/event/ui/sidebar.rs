// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SidebarToggled {
    pub sidebar_id: String,
    pub sidebar_name: Option<String>,
    pub new_state: String,
    #[serde(default = "chrono::Utc::now")]
    pub toggled_at: chrono::DateTime<chrono::Utc>,
}

impl SidebarToggled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SidebarCollapsed {
    pub sidebar_id: String,
    pub sidebar_name: Option<String>,
    pub collapse_trigger: String,
    #[serde(default = "chrono::Utc::now")]
    pub collapsed_at: chrono::DateTime<chrono::Utc>,
}

impl SidebarCollapsed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SidebarExpanded {
    pub sidebar_id: String,
    pub sidebar_name: Option<String>,
    pub expand_trigger: String,
    #[serde(default = "chrono::Utc::now")]
    pub expanded_at: chrono::DateTime<chrono::Utc>,
}

impl SidebarExpanded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


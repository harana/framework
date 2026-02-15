// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NavItemClicked {
    pub nav_id: String,
    pub nav_item_id: String,
    pub nav_item_label: Option<String>,
    pub nav_item_path: Option<String>,
    pub nav_level: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl NavItemClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TabSwitched {
    pub tab_group_id: String,
    pub from_tab_id: Option<String>,
    pub to_tab_id: String,
    pub from_tab_label: Option<String>,
    pub to_tab_label: Option<String>,
    pub tab_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub switched_at: chrono::DateTime<chrono::Utc>,
}

impl TabSwitched {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BreadcrumbClicked {
    pub breadcrumb_id: String,
    pub breadcrumb_label: Option<String>,
    pub breadcrumb_path: Option<String>,
    pub breadcrumb_level: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl BreadcrumbClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


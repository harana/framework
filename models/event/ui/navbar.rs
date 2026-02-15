// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NavbarItemClicked {
    pub navbar_id: String,
    pub item_id: String,
    pub item_label: Option<String>,
    pub item_href: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl NavbarItemClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NavbarItemHovered {
    pub navbar_id: String,
    pub item_id: String,
    pub item_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub hovered_at: chrono::DateTime<chrono::Utc>,
}

impl NavbarItemHovered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NavbarSectionExpanded {
    pub navbar_id: String,
    pub section_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub expanded_at: chrono::DateTime<chrono::Utc>,
}

impl NavbarSectionExpanded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NavbarSectionCollapsed {
    pub navbar_id: String,
    pub section_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub collapsed_at: chrono::DateTime<chrono::Utc>,
}

impl NavbarSectionCollapsed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NavbarCurrentItemChanged {
    pub navbar_id: String,
    pub old_item_id: Option<String>,
    pub new_item_id: String,
    pub new_item_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl NavbarCurrentItemChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


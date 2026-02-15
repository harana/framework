// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LinkClicked {
    pub link_id: String,
    pub link_label: Option<String>,
    pub link_href: Option<String>,
    pub link_target: String,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl LinkClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LinkHovered {
    pub link_id: String,
    pub link_label: Option<String>,
    pub link_href: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub hovered_at: chrono::DateTime<chrono::Utc>,
}

impl LinkHovered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LinkFocused {
    pub link_id: String,
    pub link_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub focused_at: chrono::DateTime<chrono::Utc>,
}

impl LinkFocused {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


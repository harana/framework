// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommandPaletteOpened {
    pub palette_id: String,
    pub trigger_method: String,
    #[serde(default = "chrono::Utc::now")]
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

impl CommandPaletteOpened {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommandPaletteClosed {
    pub palette_id: String,
    pub close_reason: String,
    pub time_open_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl CommandPaletteClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommandSelected {
    pub palette_id: String,
    pub command_id: String,
    pub command_name: Option<String>,
    pub command_category: Option<String>,
    pub selection_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl CommandSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommandSearched {
    pub palette_id: String,
    pub search_query: String,
    pub results_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub searched_at: chrono::DateTime<chrono::Utc>,
}

impl CommandSearched {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


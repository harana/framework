// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListboxOpened {
    pub listbox_id: String,
    pub listbox_name: Option<String>,
    pub options_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

impl ListboxOpened {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListboxClosed {
    pub listbox_id: String,
    pub listbox_name: Option<String>,
    pub close_reason: String,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl ListboxClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListboxOptionSelected {
    pub listbox_id: String,
    pub listbox_name: Option<String>,
    pub option_value: String,
    pub option_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl ListboxOptionSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListboxOptionHighlighted {
    pub listbox_id: String,
    pub option_value: Option<String>,
    pub option_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub highlighted_at: chrono::DateTime<chrono::Utc>,
}

impl ListboxOptionHighlighted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListboxSearched {
    pub listbox_id: String,
    pub listbox_name: Option<String>,
    pub search_query: Option<String>,
    pub results_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub searched_at: chrono::DateTime<chrono::Utc>,
}

impl ListboxSearched {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


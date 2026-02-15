// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropdownOpened {
    pub dropdown_id: String,
    pub dropdown_name: Option<String>,
    pub options_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

impl DropdownOpened {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropdownClosed {
    pub dropdown_id: String,
    pub dropdown_name: Option<String>,
    pub close_reason: String,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl DropdownClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OptionSelected {
    pub dropdown_id: String,
    pub dropdown_name: Option<String>,
    pub option_value: String,
    pub option_label: Option<String>,
    pub option_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl OptionSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OptionDeselected {
    pub dropdown_id: String,
    pub dropdown_name: Option<String>,
    pub option_value: String,
    pub option_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub deselected_at: chrono::DateTime<chrono::Utc>,
}

impl OptionDeselected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFiltered {
    pub dropdown_id: String,
    pub dropdown_name: Option<String>,
    pub search_query: Option<String>,
    pub results_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub filtered_at: chrono::DateTime<chrono::Utc>,
}

impl SearchFiltered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ComboboxTyped {
    pub combobox_id: String,
    pub combobox_name: Option<String>,
    pub typed_value: Option<String>,
    pub suggestions_shown: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub typed_at: chrono::DateTime<chrono::Utc>,
}

impl ComboboxTyped {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


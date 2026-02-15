// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckboxChecked {
    pub checkbox_id: String,
    pub checkbox_name: Option<String>,
    pub checkbox_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
}

impl CheckboxChecked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckboxUnchecked {
    pub checkbox_id: String,
    pub checkbox_name: Option<String>,
    pub checkbox_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub unchecked_at: chrono::DateTime<chrono::Utc>,
}

impl CheckboxUnchecked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckboxIndeterminateChanged {
    pub checkbox_id: String,
    pub checkbox_name: Option<String>,
    pub is_indeterminate: bool,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl CheckboxIndeterminateChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckboxGroupChanged {
    pub group_id: String,
    pub group_name: Option<String>,
    pub selected_values: Option<String>,
    pub selected_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl CheckboxGroupChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


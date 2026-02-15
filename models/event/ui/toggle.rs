// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToggleSwitched {
    pub toggle_id: String,
    pub toggle_name: Option<String>,
    pub old_state: bool,
    pub new_state: bool,
    #[serde(default = "chrono::Utc::now")]
    pub switched_at: chrono::DateTime<chrono::Utc>,
}

impl ToggleSwitched {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckboxChecked {
    pub checkbox_id: String,
    pub checkbox_name: Option<String>,
    pub checkbox_label: Option<String>,
    pub form_id: Option<String>,
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
    pub checkbox_label: Option<String>,
    pub form_id: Option<String>,
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
pub struct RadioSelected {
    pub radio_group_id: String,
    pub radio_id: String,
    pub radio_name: Option<String>,
    pub radio_value: Option<String>,
    pub previous_value: Option<String>,
    pub form_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl RadioSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SwitchEnabled {
    pub switch_id: String,
    pub switch_name: Option<String>,
    pub switch_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub enabled_at: chrono::DateTime<chrono::Utc>,
}

impl SwitchEnabled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SwitchDisabled {
    pub switch_id: String,
    pub switch_name: Option<String>,
    pub switch_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub disabled_at: chrono::DateTime<chrono::Utc>,
}

impl SwitchDisabled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


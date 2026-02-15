// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RadioSelected {
    pub group_id: String,
    pub radio_value: String,
    pub radio_label: Option<String>,
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
pub struct RadioGroupChanged {
    pub group_id: String,
    pub group_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: String,
    pub new_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl RadioGroupChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RadioFocused {
    pub group_id: String,
    pub radio_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub focused_at: chrono::DateTime<chrono::Utc>,
}

impl RadioFocused {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RadioBlurred {
    pub group_id: String,
    pub radio_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub blurred_at: chrono::DateTime<chrono::Utc>,
}

impl RadioBlurred {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


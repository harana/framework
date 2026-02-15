// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldFocused {
    pub field_id: String,
    pub field_name: Option<String>,
    pub field_type: String,
    pub form_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub focused_at: chrono::DateTime<chrono::Utc>,
}

impl FieldFocused {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldBlurred {
    pub field_id: String,
    pub field_name: Option<String>,
    pub field_type: String,
    pub form_id: Option<String>,
    #[serde(default)]
    pub had_value: bool,
    #[serde(default = "chrono::Utc::now")]
    pub blurred_at: chrono::DateTime<chrono::Utc>,
}

impl FieldBlurred {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldChanged {
    pub field_id: String,
    pub field_name: Option<String>,
    pub field_type: String,
    pub form_id: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl FieldChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldsetEnabled {
    pub fieldset_id: String,
    pub fieldset_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub enabled_at: chrono::DateTime<chrono::Utc>,
}

impl FieldsetEnabled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldsetDisabled {
    pub fieldset_id: String,
    pub fieldset_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub disabled_at: chrono::DateTime<chrono::Utc>,
}

impl FieldsetDisabled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldFocused {
    pub field_id: String,
    pub field_name: Option<String>,
    pub fieldset_id: Option<String>,
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
    pub fieldset_id: Option<String>,
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
pub struct FieldErrorShown {
    pub field_id: String,
    pub field_name: Option<String>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub shown_at: chrono::DateTime<chrono::Utc>,
}

impl FieldErrorShown {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldErrorCleared {
    pub field_id: String,
    pub field_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub cleared_at: chrono::DateTime<chrono::Utc>,
}

impl FieldErrorCleared {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


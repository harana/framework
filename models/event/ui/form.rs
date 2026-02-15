// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormSubmitted {
    pub form_id: String,
    pub form_name: Option<String>,
    pub fields_count: Option<i64>,
    pub submission_time_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

impl FormSubmitted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormValidationFailed {
    pub form_id: String,
    pub form_name: Option<String>,
    pub field_name: Option<String>,
    pub error_message: Option<String>,
    pub error_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl FormValidationFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormReset {
    pub form_id: String,
    pub form_name: Option<String>,
    pub fields_cleared: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub reset_at: chrono::DateTime<chrono::Utc>,
}

impl FormReset {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


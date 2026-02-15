// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Form {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub slug: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Form {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormField {
    pub default_value: Option<String>,
    pub form_id: String,
    #[serde(default)]
    pub is_required: bool,
    pub label: String,
    pub options: Option<String>,
    pub placeholder: Option<String>,
    pub sort_order: i64,
    pub type: String,
    pub validation_rules: Option<String>,
}

impl FormField {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormSubmission {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub data: String,
    pub ip_address: Option<String>,
    pub status: String,
    pub submitted_by: Option<String>,
    pub user_agent: Option<String>,
}

impl FormSubmission {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormData {
    pub values: std::collections::HashMap<String, String>,
}

impl FormData {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl FormValidationError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormMetadata {
    pub title: String,
    pub description: String,
    pub version: String,
}

impl FormMetadata {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormValidationRules {
    pub rules: std::collections::HashMap<String, String>,
}

impl FormValidationRules {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormSubmissionInfo {
    pub submission_id: String,
    pub form_id: String,
    pub status: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

impl FormSubmissionInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


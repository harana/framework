// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRule {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub pattern: Option<String>,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub value: Option<String>,
}

impl ValidationRule {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRuleSet {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ValidationRuleSet {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRuleSetAssignment {
    pub rule_id: String,
    pub rule_set_id: String,
    pub sort_order: i64,
}

impl ValidationRuleSetAssignment {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationErrorLog {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub field_name: String,
    pub input_value: Option<String>,
    pub rule_id: Option<String>,
    pub rule_set_id: Option<String>,
    pub violation_message: String,
}

impl ValidationErrorLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub field_name: String,
    pub value: String,
}

impl ValidationResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationSchema {
    pub type: String,
    pub properties: std::collections::HashMap<String, String>,
    pub required: Vec<String>,
}

impl ValidationSchema {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub code: String,
}

impl ValidationError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlAllowedAttributes {
    pub global: Vec<String>,
    pub per_tag: std::collections::HashMap<String, String>,
}

impl HtmlAllowedAttributes {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


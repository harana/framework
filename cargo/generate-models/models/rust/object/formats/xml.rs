// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// xml_document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlDocument {
    pub charset: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub root_element: Option<String>,
    pub size: i64,
    pub source_path: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl XmlDocument {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// xml_schema
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlSchema {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub schema_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

impl XmlSchema {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// xml_validation_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlValidationResult {
    /// Reference: xml_document.id
    pub document_id: String,
    pub error_count: i64,
    pub errors: Option<String>,
    #[serde(default)]
    pub is_valid: bool,
    /// Reference: xml_schema.id
    pub schema_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

impl XmlValidationResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


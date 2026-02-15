// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// csv_document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvDocument {
    pub column_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub delimiter: String,
    #[serde(default)]
    pub has_headers: bool,
    pub row_count: i64,
    pub size: i64,
    pub source_path: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CsvDocument {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// csv_column_schema
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvColumnSchema {
    /// Reference: csv_document.id
    pub document_id: String,
    #[serde(default)]
    pub is_required: bool,
    pub name: String,
    pub pattern: Option<String>,
    pub sort_order: i64,
    pub type: String,
}

impl CsvColumnSchema {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// csv_validation_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvValidationResult {
    /// Reference: csv_document.id
    pub document_id: String,
    pub error_count: i64,
    pub errors: Option<String>,
    #[serde(default)]
    pub is_valid: bool,
    #[serde(default = "chrono::Utc::now")]
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

impl CsvValidationResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvDocument {
    pub column_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvColumnSchema {
    pub document_id: String,
    #[serde(default)]
    pub is_required: bool,
    pub name: String,
    pub pattern: Option<String>,
    pub sort_order: i64,
    pub type: String,
}

impl CsvColumnSchema {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvValidationResult {
    pub document_id: String,
    pub error_count: i64,
    pub errors: Option<String>,
    #[serde(default)]
    pub is_valid: bool,
    #[serde(default = "chrono::Utc::now")]
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

impl CsvValidationResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvRow {
    pub values: std::collections::HashMap<String, String>,
}

impl CsvRow {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvTransformOperation {
    pub operation: String,
    pub column: String,
    pub parameters: std::collections::HashMap<String, String>,
}

impl CsvTransformOperation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvSchema {
    pub columns: Vec<String>,
    pub allow_extra_columns: bool,
}

impl CsvSchema {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvValidationError {
    pub row: i64,
    pub column: String,
    pub message: String,
}

impl CsvValidationError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


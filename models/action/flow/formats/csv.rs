// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvDocument {
    pub headers: Vec<String>,
    pub rows: Vec<String>,
    pub delimiter: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvRow {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvTransformOperation {
    pub operation: String,
    pub column: String,
    pub parameters: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvSchema {
    pub columns: Vec<String>,
    pub allow_extra_columns: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvColumnSchema {
    pub name: String,
    pub type: String,
    pub required: bool,
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvValidationError {
    pub row: i64,
    pub column: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvValidationResult {
    pub document_id: String,
    pub error_count: i64,
    pub errors: String,
    #[serde(default)]
    pub is_valid: bool,
    #[serde(default = "chrono::Utc::now")]
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait CsvAction {
    async fn parse(&self, data: String, delimiter: String, has_headers: bool, headers: Vec<String>, skip_empty_lines: bool) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn generate(&self, data: Vec<String>, delimiter: String, headers: Vec<String>, include_headers: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn transform(&self, data: String, delimiter: String, operations: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn validate(&self, data: String, delimiter: String, schema: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}

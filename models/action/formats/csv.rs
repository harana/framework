// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub data: String,
    pub delimiter: String,
    #[serde(default)]
    pub has_headers: bool,
    pub headers: Vec<String>,
    #[serde(default)]
    pub skip_empty_lines: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateInput {
    pub data: Vec<String>,
    pub delimiter: String,
    pub headers: Vec<String>,
    #[serde(default)]
    pub include_headers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateOutput {
    pub csv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformInput {
    pub data: String,
    pub delimiter: String,
    pub operations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformOutput {
    pub csv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateInput {
    pub data: String,
    pub delimiter: String,
    pub schema: String,
}

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

#[async_trait]
pub trait CsvAction {
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn generate(&self, input: GenerateInput) -> Result<GenerateOutput, Box<dyn std::error::Error>>;
    async fn transform(&self, input: TransformInput) -> Result<TransformOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}

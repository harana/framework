// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReadInput {
    pub file: String,
    #[serde(default)]
    pub has_headers: bool,
    pub range: String,
    pub sheet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReadOutput {
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WriteInput {
    pub data: Vec<String>,
    pub file: String,
    pub headers: Vec<String>,
    #[serde(default)]
    pub include_headers: bool,
    pub sheet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WriteOutput {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSheetsInput {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSheetsOutput {
    pub sheets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReadSheetInput {
    pub file: String,
    #[serde(default)]
    pub has_headers: bool,
    pub range: String,
    pub sheet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReadSheetOutput {
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExcelWorkbook {
    pub path: String,
    pub sheets: Vec<String>,
    pub active_sheet: String,
    pub row_count: i64,
    pub column_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExcelRow {
    pub values: std::collections::HashMap<String, String>,
}

#[async_trait]
pub trait ExcelAction {
    async fn read(&self, input: ReadInput) -> Result<ReadOutput, Box<dyn std::error::Error>>;
    async fn write(&self, input: WriteInput) -> Result<WriteOutput, Box<dyn std::error::Error>>;
    async fn get_sheets(&self, input: GetSheetsInput) -> Result<GetSheetsOutput, Box<dyn std::error::Error>>;
    async fn read_sheet(&self, input: ReadSheetInput) -> Result<ReadSheetOutput, Box<dyn std::error::Error>>;
}

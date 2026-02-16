// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExcelSheet {
    pub column_count: i64,
    pub name: String,
    pub row_count: i64,
    pub workbook_id: String,
}

#[async_trait]
pub trait ExcelAction {
    async fn read(&self, file: String, has_headers: bool, range: String, sheet: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn write(&self, data: Vec<String>, file: String, headers: Vec<String>, include_headers: bool, sheet: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_sheets(&self, file: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn read_sheet(&self, file: String, has_headers: bool, range: String, sheet: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

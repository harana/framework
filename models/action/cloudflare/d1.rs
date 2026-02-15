// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecInput {
    pub database: String,
    pub sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecOutput {
    pub changes: i64,
    pub duration: f64,
    pub last_row_id: i64,
    pub rows_read: i64,
    pub rows_written: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrepareInput {
    pub database: String,
    pub sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrepareOutput {
    pub statement: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunInput {
    pub bind_values: Vec<String>,
    pub database: String,
    pub sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunOutput {
    pub changes: i64,
    pub duration: f64,
    pub last_row_id: i64,
    pub results: Vec<String>,
    pub rows_read: i64,
    pub rows_written: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FirstInput {
    pub bind_values: Vec<String>,
    pub column: String,
    pub database: String,
    pub sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FirstOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllInput {
    pub bind_values: Vec<String>,
    pub database: String,
    pub sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllOutput {
    pub changes: i64,
    pub duration: f64,
    pub last_row_id: i64,
    pub results: Vec<String>,
    pub rows_read: i64,
    pub rows_written: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawInput {
    pub bind_values: Vec<String>,
    pub database: String,
    pub sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawOutput {
    pub columns: Vec<String>,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchInput {
    pub database: String,
    pub statements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchOutput {
    pub results: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DumpInput {
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DumpOutput {
    pub data: String,
    pub success: bool,
}

#[async_trait]
pub trait D1Action {
    async fn exec(&self, input: ExecInput) -> Result<ExecOutput, Box<dyn std::error::Error>>;
    async fn prepare(&self, input: PrepareInput) -> Result<PrepareOutput, Box<dyn std::error::Error>>;
    async fn run(&self, input: RunInput) -> Result<RunOutput, Box<dyn std::error::Error>>;
    async fn first(&self, input: FirstInput) -> Result<FirstOutput, Box<dyn std::error::Error>>;
    async fn all(&self, input: AllInput) -> Result<AllOutput, Box<dyn std::error::Error>>;
    async fn raw(&self, input: RawInput) -> Result<RawOutput, Box<dyn std::error::Error>>;
    async fn batch(&self, input: BatchInput) -> Result<BatchOutput, Box<dyn std::error::Error>>;
    async fn dump(&self, input: DumpInput) -> Result<DumpOutput, Box<dyn std::error::Error>>;
}

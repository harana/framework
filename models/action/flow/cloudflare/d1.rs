// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecOutput {
    pub changes: i64,
    pub duration: f64,
    pub last_row_id: i64,
    pub rows_read: i64,
    pub rows_written: i64,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawOutput {
    pub columns: Vec<String>,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfD1Database {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database_id: String,
    pub database_name: String,
    pub file_size: i64,
    pub num_tables: i64,
    pub region: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfD1QueryLog {
    pub changes: i64,
    pub database_id: String,
    pub duration_ms: f64,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub last_row_id: i64,
    pub rows_read: i64,
    pub rows_written: i64,
    pub sql: String,
    pub status: String,
}

#[async_trait]
pub trait D1Action {
    async fn exec(&self, database: String, sql: String) -> Result<ExecOutput, Box<dyn std::error::Error>>;
    async fn prepare(&self, database: String, sql: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn run(&self, bind_values: Vec<String>, database: String, sql: String) -> Result<RunOutput, Box<dyn std::error::Error>>;
    async fn first(&self, bind_values: Vec<String>, column: String, database: String, sql: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn all(&self, bind_values: Vec<String>, database: String, sql: String) -> Result<AllOutput, Box<dyn std::error::Error>>;
    async fn raw(&self, bind_values: Vec<String>, database: String, sql: String) -> Result<RawOutput, Box<dyn std::error::Error>>;
    async fn batch(&self, database: String, statements: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn dump(&self, database: String) -> Result<String, Box<dyn std::error::Error>>;
}

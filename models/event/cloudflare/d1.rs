// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareD1QueryExecuted {
    pub database: String,
    pub operation: String,
    pub rows_read: Option<i64>,
    pub rows_written: Option<i64>,
    pub changes: Option<i64>,
    pub duration_ms: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareD1QueryExecuted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareD1QueryFailed {
    pub database: String,
    pub error_message: String,
    pub operation: String,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareD1QueryFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareD1BatchExecuted {
    pub database: String,
    pub statement_count: i64,
    pub total_rows_read: Option<i64>,
    pub total_rows_written: Option<i64>,
    pub duration_ms: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareD1BatchExecuted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareD1DatabaseDumped {
    pub database: String,
    pub size_bytes: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub dumped_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareD1DatabaseDumped {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


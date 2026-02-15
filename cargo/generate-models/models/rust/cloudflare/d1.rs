// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cloudflare_d1_database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareD1Database {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database_id: Option<String>,
    pub database_name: String,
    pub file_size: i64,
    pub num_tables: i64,
    pub region: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: Option<String>,
}

impl CloudflareD1Database {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_d1_query_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareD1QueryLog {
    pub changes: i64,
    /// Reference: cf_d1_database.id
    pub database_id: String,
    pub duration_ms: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub last_row_id: Option<i64>,
    pub rows_read: i64,
    pub rows_written: i64,
    pub sql: String,
    pub status: String,
}

impl CloudflareD1QueryLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


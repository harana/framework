// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// sql_connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlConnection {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database: String,
    pub driver: String,
    pub host: String,
    #[serde(default)]
    pub is_active: bool,
    pub max_connections: i64,
    pub password: Option<String>,
    pub port: i64,
    pub ssl_mode: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub username: Option<String>,
}

impl SqlConnection {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sql_table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlTable {
    /// Reference: sql_connection.id
    pub connection_id: String,
    pub row_count: Option<i64>,
    pub schema_name: String,
    pub table_name: String,
}

impl SqlTable {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sql_column
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlColumn {
    pub column_name: String,
    pub data_type: String,
    pub default_value: Option<String>,
    #[serde(default)]
    pub is_nullable: bool,
    #[serde(default)]
    pub is_primary_key: bool,
    pub max_length: Option<i64>,
    pub sort_order: i64,
    /// Reference: sql_table.id
    pub table_id: String,
}

impl SqlColumn {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sql_index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlIndex {
    pub columns: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub index_name: String,
    #[serde(default)]
    pub is_unique: bool,
    /// Reference: sql_table.id
    pub table_id: String,
}

impl SqlIndex {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sql_query_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlQueryLog {
    pub affected_rows: Option<i64>,
    /// Reference: sql_connection.id
    pub connection_id: String,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub query_hash: Option<String>,
    pub query_text: Option<String>,
    pub query_type: String,
    pub status: String,
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl SqlQueryLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sql_transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlTransaction {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: sql_connection.id
    pub connection_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub isolation_level: String,
    pub status: String,
    pub transaction_id: String,
}

impl SqlTransaction {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


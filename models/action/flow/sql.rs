// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteOutput {
    pub affected_rows: i64,
    pub last_insert_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SelectOutput {
    pub column_names: Vec<String>,
    pub count: i64,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertOutput {
    pub affected_rows: i64,
    pub last_insert_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchOutput {
    pub failed_count: i64,
    pub results: Vec<String>,
    pub success_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CallProcedureOutput {
    pub out_parameters: String,
    pub result_sets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSchemaOutput {
    pub columns: Vec<String>,
    pub indexes: Vec<String>,
    pub primary_key: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PreparedStatementOutput {
    pub affected_rows: i64,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlQuery {
    pub query: String,
    pub parameters: Vec<String>,
    pub database: String,
    pub affected_rows: i64,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlRow {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlBatchQuery {
    pub query: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlBatchResult {
    pub affected_rows: i64,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlOutputParameters {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlResultSet {
    pub columns: Vec<String>,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default_value: String,
    pub max_length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlIndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlColumnDefinition {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default_value: String,
    pub primary_key: bool,
}

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
    pub password: String,
    pub port: i64,
    pub ssl_mode: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlTable {
    pub connection_id: String,
    pub row_count: i64,
    pub schema_name: String,
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlColumn {
    pub column_name: String,
    pub data_type: String,
    pub default_value: String,
    #[serde(default)]
    pub is_nullable: bool,
    #[serde(default)]
    pub is_primary_key: bool,
    pub max_length: i64,
    pub sort_order: i64,
    pub table_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlIndex {
    pub columns: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub index_name: String,
    #[serde(default)]
    pub is_unique: bool,
    pub table_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlQueryLog {
    pub affected_rows: i64,
    pub connection_id: String,
    pub duration_ms: i64,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub query_hash: String,
    pub query_text: String,
    pub query_type: String,
    pub status: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlTransaction {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub connection_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub isolation_level: String,
    pub status: String,
    pub transaction_id: String,
}

#[async_trait]
pub trait SqlAction {
    async fn execute(&self, database: String, parameters: Vec<String>, query: String) -> Result<ExecuteOutput, Box<dyn std::error::Error>>;
    async fn select(&self, database: String, limit: i64, offset: i64, parameters: Vec<String>, query: String) -> Result<SelectOutput, Box<dyn std::error::Error>>;
    async fn insert(&self, database: String, parameters: Vec<String>, query: String) -> Result<InsertOutput, Box<dyn std::error::Error>>;
    async fn update(&self, database: String, parameters: Vec<String>, query: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn delete(&self, database: String, parameters: Vec<String>, query: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn batch(&self, database: String, queries: Vec<String>) -> Result<BatchOutput, Box<dyn std::error::Error>>;
    async fn begin_transaction(&self, database: String, isolation_level: String, transaction_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn commit(&self, database: String, transaction_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn rollback(&self, database: String, transaction_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn call_procedure(&self, database: String, parameters: Vec<String>, procedure_name: String) -> Result<CallProcedureOutput, Box<dyn std::error::Error>>;
    async fn get_schema(&self, database: String, table_name: String) -> Result<GetSchemaOutput, Box<dyn std::error::Error>>;
    async fn list_tables(&self, database: String, schema: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_table(&self, columns: Vec<String>, database: String, if_not_exists: bool, table_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn drop_table(&self, database: String, if_exists: bool, table_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn truncate_table(&self, database: String, table_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_index(&self, columns: Vec<String>, database: String, index_name: String, table_name: String, unique: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn drop_index(&self, database: String, if_exists: bool, index_name: String, table_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn bulk_insert(&self, columns: Vec<String>, database: String, rows: Vec<String>, table_name: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn prepared_statement(&self, database: String, parameters: Vec<String>, statement: String) -> Result<PreparedStatementOutput, Box<dyn std::error::Error>>;
}

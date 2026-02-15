// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteInput {
    pub database: String,
    pub parameters: Vec<String>,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteOutput {
    pub affected_rows: i64,
    pub last_insert_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SelectInput {
    pub database: String,
    pub limit: i64,
    pub offset: i64,
    pub parameters: Vec<String>,
    pub query: String,
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
pub struct InsertInput {
    pub database: String,
    pub parameters: Vec<String>,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertOutput {
    pub affected_rows: i64,
    pub last_insert_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateInput {
    pub database: String,
    pub parameters: Vec<String>,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOutput {
    pub affected_rows: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub database: String,
    pub parameters: Vec<String>,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub affected_rows: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchInput {
    pub database: String,
    pub queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchOutput {
    pub failed_count: i64,
    pub results: Vec<String>,
    pub success: bool,
    pub success_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BeginTransactionInput {
    pub database: String,
    pub isolation_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BeginTransactionOutput {
    pub success: bool,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitInput {
    pub database: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RollbackInput {
    pub database: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RollbackOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CallProcedureInput {
    pub database: String,
    pub parameters: Vec<String>,
    pub procedure_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CallProcedureOutput {
    pub out_parameters: String,
    pub result_sets: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSchemaInput {
    pub database: String,
    pub table_name: String,
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
pub struct ListTablesInput {
    pub database: String,
    pub schema: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTablesOutput {
    pub tables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTableInput {
    pub columns: Vec<String>,
    pub database: String,
    #[serde(default)]
    pub if_not_exists: bool,
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTableOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropTableInput {
    pub database: String,
    #[serde(default)]
    pub if_exists: bool,
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropTableOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TruncateTableInput {
    pub database: String,
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TruncateTableOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexInput {
    pub columns: Vec<String>,
    pub database: String,
    pub index_name: String,
    pub table_name: String,
    #[serde(default)]
    pub unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropIndexInput {
    pub database: String,
    #[serde(default)]
    pub if_exists: bool,
    pub index_name: String,
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkInsertInput {
    pub columns: Vec<String>,
    pub database: String,
    pub rows: Vec<String>,
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkInsertOutput {
    pub affected_rows: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PreparedStatementInput {
    pub database: String,
    pub parameters: Vec<String>,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PreparedStatementOutput {
    pub affected_rows: i64,
    pub rows: Vec<String>,
    pub success: bool,
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
    pub success: bool,
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

#[async_trait]
pub trait SqlAction {
    async fn execute(&self, input: ExecuteInput) -> Result<ExecuteOutput, Box<dyn std::error::Error>>;
    async fn select(&self, input: SelectInput) -> Result<SelectOutput, Box<dyn std::error::Error>>;
    async fn insert(&self, input: InsertInput) -> Result<InsertOutput, Box<dyn std::error::Error>>;
    async fn update(&self, input: UpdateInput) -> Result<UpdateOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn batch(&self, input: BatchInput) -> Result<BatchOutput, Box<dyn std::error::Error>>;
    async fn begin_transaction(&self, input: BeginTransactionInput) -> Result<BeginTransactionOutput, Box<dyn std::error::Error>>;
    async fn commit(&self, input: CommitInput) -> Result<CommitOutput, Box<dyn std::error::Error>>;
    async fn rollback(&self, input: RollbackInput) -> Result<RollbackOutput, Box<dyn std::error::Error>>;
    async fn call_procedure(&self, input: CallProcedureInput) -> Result<CallProcedureOutput, Box<dyn std::error::Error>>;
    async fn get_schema(&self, input: GetSchemaInput) -> Result<GetSchemaOutput, Box<dyn std::error::Error>>;
    async fn list_tables(&self, input: ListTablesInput) -> Result<ListTablesOutput, Box<dyn std::error::Error>>;
    async fn create_table(&self, input: CreateTableInput) -> Result<CreateTableOutput, Box<dyn std::error::Error>>;
    async fn drop_table(&self, input: DropTableInput) -> Result<DropTableOutput, Box<dyn std::error::Error>>;
    async fn truncate_table(&self, input: TruncateTableInput) -> Result<TruncateTableOutput, Box<dyn std::error::Error>>;
    async fn create_index(&self, input: CreateIndexInput) -> Result<CreateIndexOutput, Box<dyn std::error::Error>>;
    async fn drop_index(&self, input: DropIndexInput) -> Result<DropIndexOutput, Box<dyn std::error::Error>>;
    async fn bulk_insert(&self, input: BulkInsertInput) -> Result<BulkInsertOutput, Box<dyn std::error::Error>>;
    async fn prepared_statement(&self, input: PreparedStatementInput) -> Result<PreparedStatementOutput, Box<dyn std::error::Error>>;
}

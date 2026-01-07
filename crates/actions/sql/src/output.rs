// Harana Actions - Sql Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOutput {
    pub success_count: i32,
    pub failed_count: i32,
    pub results: Vec<HashMap<String, Value>>,
    pub success: bool
}

// begin_transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeginTransactionOutput {
    pub success: bool,
    pub transaction_id: String
}

// bulk_insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkInsertOutput {
    pub success: bool,
    pub affected_rows: i32
}

// call_procedure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallProcedureOutput {
    pub out_parameters: HashMap<String, Value>,
    pub result_sets: Vec<HashMap<String, Value>>,
    pub success: bool
}

// commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitOutput {
    pub success: bool
}

// create_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndexOutput {
    pub success: bool
}

// create_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableOutput {
    pub success: bool
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub affected_rows: i32,
    pub success: bool
}

// drop_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropIndexOutput {
    pub success: bool
}

// drop_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropTableOutput {
    pub success: bool
}

// execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteOutput {
    pub last_insert_id: String,
    pub success: bool,
    pub affected_rows: i32
}

// get_schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSchemaOutput {
    pub columns: Vec<HashMap<String, Value>>,
    pub primary_key: Vec<String>,
    pub indexes: Vec<HashMap<String, Value>>
}

// insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertOutput {
    pub affected_rows: i32,
    pub success: bool,
    pub last_insert_id: String
}

// list_tables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTablesOutput {
    pub tables: Vec<String>
}

// prepared_statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedStatementOutput {
    pub success: bool,
    pub rows: Vec<HashMap<String, Value>>,
    pub affected_rows: i32
}

// rollback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackOutput {
    pub success: bool
}

// select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOutput {
    pub column_names: Vec<String>,
    pub rows: Vec<HashMap<String, Value>>,
    pub count: i32
}

// truncate_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruncateTableOutput {
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool,
    pub affected_rows: i32
}

// Harana Actions - SQL Module Output Types
// Auto-generated output structs for SQL action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteOutput {
    pub affected_rows: i32,
    pub last_insert_id: String,
    pub success: bool,
}

// select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOutput {
    pub column_names: Vec<String>,
    pub count: i32,
    pub rows: Vec<HashMap<String, Value>>,
}

// insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertOutput {
    pub affected_rows: i32,
    pub last_insert_id: String,
    pub success: bool,
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub affected_rows: i32,
    pub success: bool,
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub affected_rows: i32,
    pub success: bool,
}


// batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOutput {
    pub success: bool,
    pub results: Vec<HashMap<String, Value>>,
    pub failed_count: i32,
    pub success_count: i32
}

// begin_transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeginTransactionOutput {
    pub transaction_id: String,
    pub success: bool
}

// commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitOutput {
    pub success: bool
}

// rollback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackOutput {
    pub success: bool
}

// call_procedure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallProcedureOutput {
    pub out_parameters: HashMap<String, Value>,
    pub result_sets: Vec<HashMap<String, Value>>,
    pub success: bool
}

// get_schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSchemaOutput {
    pub primary_key: Vec<String>,
    pub columns: Vec<HashMap<String, Value>>,
    pub indexes: Vec<HashMap<String, Value>>
}

// list_tables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTablesOutput {
    pub tables: Vec<String>
}

// create_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTableOutput {
    pub success: bool
}

// drop_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropTableOutput {
    pub success: bool
}

// truncate_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruncateTableOutput {
    pub success: bool
}

// create_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndexOutput {
    pub success: bool
}

// drop_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropIndexOutput {
    pub success: bool
}

// bulk_insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkInsertOutput {
    pub success: bool,
    pub affected_rows: i32
}

// prepared_statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedStatementOutput {
    pub rows: Vec<HashMap<String, Value>>,
    pub success: bool,
    pub affected_rows: i32
}
// TODO: Add remaining output types - see core/schema/actions/sql.yml

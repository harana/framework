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

// TODO: Add remaining output types - see core/schema/actions/sql.yml

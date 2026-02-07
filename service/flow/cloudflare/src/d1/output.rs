// Harana Actions - Cloudflare D1 Module Output Types

use serde::{Deserialize, Serialize};

// exec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecOutput {
    pub changes: i32,
    pub duration: f64,
    pub last_row_id: i32,
    pub rows_read: i32,
    pub rows_written: i32,
    pub success: bool,
}

// prepare
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareOutput {
    pub statement: D1PreparedStatement,
    pub success: bool,
}

// run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOutput {
    pub changes: i32,
    pub duration: f64,
    pub last_row_id: i32,
    pub results: Vec<serde_json::Value>,
    pub rows_read: i32,
    pub rows_written: i32,
    pub success: bool,
}

// first
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstOutput {
    pub result: serde_json::Value,
}

// all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllOutput {
    pub changes: i32,
    pub duration: f64,
    pub last_row_id: i32,
    pub results: Vec<serde_json::Value>,
    pub rows_read: i32,
    pub rows_written: i32,
    pub success: bool,
}

// raw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawOutput {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

// batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOutput {
    pub results: Vec<D1BatchResult>,
    pub success: bool,
}

// dump
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DumpOutput {
    pub data: Vec<u8>,
    pub success: bool,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D1PreparedStatement {
    pub sql: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D1BatchStatement {
    pub sql: String,
    pub params: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D1BatchResult {
    pub changes: i32,
    pub duration: f64,
    pub last_row_id: i32,
    pub results: Vec<serde_json::Value>,
    pub rows_read: i32,
    pub rows_written: i32,
    pub success: bool,
}

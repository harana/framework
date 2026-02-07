// Harana Actions - Cloudflare Analytics Engine Module Output Types

use serde::{Deserialize, Serialize};

// write_data_point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteDataPointOutput {
    pub success: bool,
}

// query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOutput {
    pub data: Vec<serde_json::Value>,
    pub rows: i32,
}

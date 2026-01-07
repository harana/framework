// Harana Actions - Cache Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// clear
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearOutput {
    pub keys_deleted: i32
}

// decrement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecrementOutput {
    pub value: i32
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistsOutput {
    pub exists: bool
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub value: String,
    pub found: bool
}

// get_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetManyOutput {
    pub values: HashMap<String, Value>
}

// increment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementOutput {
    pub value: i32
}

// set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOutput {
    pub success: bool
}

// set_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetManyOutput {
    pub success: bool
}

// ttl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtlOutput {
    pub expires_at: String,
    pub ttl: i32
}

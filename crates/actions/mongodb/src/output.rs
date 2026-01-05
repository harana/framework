// Harana Actions - MongoDB Module Output Types
// Auto-generated output structs for MongoDB action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertOutput {
    pub inserted_id: String,
    pub success: bool,
}

// insert_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertManyOutput {
    pub inserted_count: i32,
    pub inserted_ids: Vec<String>,
    pub success: bool,
}

// find_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindOneOutput {
    pub document: HashMap<String, Value>,
    pub found: bool,
}

// find
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindOutput {
    pub count: i32,
    pub documents: Vec<HashMap<String, Value>>,
}

// TODO: Add remaining output types - see core/schema/actions/mongodb.yml

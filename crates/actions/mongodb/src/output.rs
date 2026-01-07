// Harana Actions - Mongodb Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateOutput {
    pub documents: Vec<HashMap<String, Value>>
}

// bulk_write
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkWriteOutput {
    pub success: bool,
    pub modified_count: i32,
    pub matched_count: i32,
    pub inserted_count: i32,
    pub deleted_count: i32,
    pub upserted_count: i32
}

// count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountOutput {
    pub count: i32
}

// create_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndexOutput {
    pub index_name: String,
    pub success: bool
}

// delete_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteManyOutput {
    pub deleted_count: i32,
    pub success: bool
}

// delete_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOneOutput {
    pub deleted_count: i32,
    pub success: bool
}

// drop_collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropCollectionOutput {
    pub success: bool
}

// drop_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropIndexOutput {
    pub success: bool
}

// find
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindOutput {
    pub documents: Vec<HashMap<String, Value>>,
    pub count: i32
}

// find_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindOneOutput {
    pub document: HashMap<String, Value>,
    pub found: bool
}

// insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertOutput {
    pub inserted_id: String,
    pub success: bool
}

// insert_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertManyOutput {
    pub inserted_count: i32,
    pub success: bool,
    pub inserted_ids: Vec<String>
}

// list_collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollectionsOutput {
    pub collections: Vec<String>
}

// replace_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceOneOutput {
    pub upserted_id: String,
    pub matched_count: i32,
    pub modified_count: i32,
    pub success: bool
}

// update_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManyOutput {
    pub modified_count: i32,
    pub upserted_id: String,
    pub success: bool,
    pub matched_count: i32
}

// update_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOneOutput {
    pub success: bool,
    pub matched_count: i32,
    pub upserted_id: String,
    pub modified_count: i32
}

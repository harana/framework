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


// update_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOneOutput {
    pub upserted_id: String,
    pub modified_count: i32,
    pub success: bool,
    pub matched_count: i32
}

// update_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManyOutput {
    pub modified_count: i32,
    pub matched_count: i32,
    pub success: bool,
    pub upserted_id: String
}

// replace_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceOneOutput {
    pub matched_count: i32,
    pub upserted_id: String,
    pub modified_count: i32,
    pub success: bool
}

// delete_one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOneOutput {
    pub deleted_count: i32,
    pub success: bool
}

// delete_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteManyOutput {
    pub success: bool,
    pub deleted_count: i32
}

// count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountOutput {
    pub count: i32
}

// aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateOutput {
    pub documents: Vec<HashMap<String, Value>>
}

// create_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndexOutput {
    pub success: bool,
    pub index_name: String
}

// drop_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropIndexOutput {
    pub success: bool
}

// list_collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollectionsOutput {
    pub collections: Vec<String>
}

// drop_collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropCollectionOutput {
    pub success: bool
}

// bulk_write
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkWriteOutput {
    pub modified_count: i32,
    pub deleted_count: i32,
    pub matched_count: i32,
    pub inserted_count: i32,
    pub success: bool,
    pub upserted_count: i32
}
// TODO: Add remaining output types - see core/schema/actions/mongodb.yml

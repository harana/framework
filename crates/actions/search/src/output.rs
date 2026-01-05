// Harana Actions - Search Module Output Types
// Auto-generated output structs for Search action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// search_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQueryOutput {
    pub hits: Vec<HashMap<String, Value>>,
    pub took_ms: i32,
    pub total: i32,
}

// index_document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDocumentOutput {
    pub success: bool,
    pub version: i32,
}

// bulk_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkIndexOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub failed: i32,
    pub indexed: i32,
}

// get_document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentOutput {
    pub document: HashMap<String, Value>,
    pub found: bool,
    pub version: i32,
}

// update_document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentOutput {
    pub success: bool,
    pub version: i32,
}

// delete_document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentOutput {
    pub success: bool,
}

// delete_by_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteByQueryOutput {
    pub deleted: i32,
    pub success: bool,
}

// create_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndexOutput {
    pub success: bool,
}

// delete_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIndexOutput {
    pub success: bool,
}

// list_indexes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIndexesOutput {
    pub indexes: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// get_index_stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIndexStatsOutput {
    pub document_count: i32,
    pub mappings: HashMap<String, Value>,
    pub size_bytes: i32,
}

// suggest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestOutput {
    pub suggestions: Vec<String>,
}

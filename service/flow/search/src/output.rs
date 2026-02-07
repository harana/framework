// Harana Actions - Search Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// bulk_index
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct BulkIndexOutput {
    pub indexed: i32,
    pub errors: Vec<HashMap<String, Value>>,
    pub failed: i32
}

pub struct CreateIndexOutput {
    pub success: bool
}

pub struct DeleteByQueryOutput {
    pub deleted: i32,
    pub success: bool
}

pub struct DeleteDocumentOutput {
    pub success: bool
}

pub struct DeleteIndexOutput {
    pub success: bool
}

pub struct GetDocumentOutput {
    pub found: bool,
    pub document: HashMap<String, Value>,
    pub version: i32
}

pub struct GetIndexStatsOutput {
    pub mappings: HashMap<String, Value>,
    pub size_bytes: i32,
    pub document_count: i32
}

pub struct IndexDocumentOutput {
    pub success: bool,
    pub version: i32
}

pub struct ListIndexesOutput {
    pub indexes: Vec<HashMap<String, Value>>,
    pub total: i32
}

pub struct SearchQueryOutput {
    pub total: i32,
    pub hits: Vec<HashMap<String, Value>>,
    pub took_ms: i32
}

pub struct SuggestOutput {
    pub suggestions: Vec<String>
}

pub struct UpdateDocumentOutput {
    pub success: bool,
    pub version: i32
}

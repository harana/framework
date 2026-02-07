// Harana Actions - Cloudflare KV Module Output Types

use serde::{Deserialize, Serialize};

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub found: bool,
    pub metadata: serde_json::Value,
    pub value: serde_json::Value,
}

// get_with_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithMetadataOutput {
    pub found: bool,
    pub metadata: serde_json::Value,
    pub value: serde_json::Value,
}

// put
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutOutput {
    pub success: bool,
}

// put_with_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutWithMetadataOutput {
    pub success: bool,
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool,
}

// list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOutput {
    pub cursor: String,
    pub keys: Vec<KvKey>,
    pub list_complete: bool,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvKey {
    pub name: String,
    pub expiration: Option<i64>,
    pub metadata: Option<serde_json::Value>,
}

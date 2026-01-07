// Harana Actions - Http Client Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub headers: HashMap<String, Value>,
    pub body: String,
    pub status_code: i32
}

// download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadOutput {
    pub content_type: String,
    pub content: Vec<u8>,
    pub size: i32
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub headers: HashMap<String, Value>,
    pub body: String,
    pub status_code: i32
}

// graphql_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlQueryOutput {
    pub data: String,
    pub errors: Vec<HashMap<String, Value>>
}

// patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOutput {
    pub body: String,
    pub headers: HashMap<String, Value>,
    pub status_code: i32
}

// post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostOutput {
    pub body: String,
    pub headers: HashMap<String, Value>,
    pub status_code: i32
}

// put
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutOutput {
    pub body: String,
    pub headers: HashMap<String, Value>,
    pub status_code: i32
}

// upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadOutput {
    pub status_code: i32,
    pub body: String
}

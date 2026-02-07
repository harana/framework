// Harana Actions - Http Client Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub body: String,
    pub headers: HashMap<String, String>,
    pub status_code: i32,
}

// post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostOutput {
    pub body: String,
    pub headers: HashMap<String, String>,
    pub status_code: i32,
}

// put
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutOutput {
    pub body: String,
    pub headers: HashMap<String, String>,
    pub status_code: i32,
}

// patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOutput {
    pub body: String,
    pub headers: HashMap<String, String>,
    pub status_code: i32,
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub body: String,
    pub headers: HashMap<String, String>,
    pub status_code: i32,
}

// download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadOutput {
    pub content: Vec<u8>,
    pub content_type: String,
    pub size: i32,
}

// upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadOutput {
    pub body: String,
    pub status_code: i32,
}

// graphql_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlQueryOutput {
    pub data: Option<Value>,
    pub errors: Vec<GraphqlError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlError {
    pub message: String,
    pub path: Vec<String>,
    pub locations: Vec<GraphqlErrorLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlErrorLocation {
    pub line: i32,
    pub column: i32,
}

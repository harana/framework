// Harana Actions - Client Module Output Types
// Auto-generated output structs for Client action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// http_get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpGetOutput {
    pub body: Value,
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
}

// http_post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpPostOutput {
    pub body: Value,
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
}

// http_put
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpPutOutput {
    pub body: Value,
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
}

// http_patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpPatchOutput {
    pub body: Value,
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
}

// http_delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpDeleteOutput {
    pub body: Value,
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
}

// http_download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpDownloadOutput {
    pub content: Vec<u8>,
    pub content_type: String,
    pub size: i32,
}

// http_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpUploadOutput {
    pub body: Value,
    pub status_code: i32,
}

// graphql_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlQueryOutput {
    pub data: Value,
    pub errors: Vec<HashMap<String, Value>>,
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
    pub body: String
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
    pub status_code: i32,
    pub body: String,
    pub headers: HashMap<String, Value>
}

// patch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOutput {
    pub body: String,
    pub status_code: i32,
    pub headers: HashMap<String, Value>
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub body: String,
    pub headers: HashMap<String, Value>,
    pub status_code: i32
}

// download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadOutput {
    pub size: i32,
    pub content: Vec<u8>,
    pub content_type: String
}

// upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadOutput {
    pub body: String,
    pub status_code: i32
}
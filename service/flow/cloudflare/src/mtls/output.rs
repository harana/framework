// Harana Actions - Cloudflare mTLS Module Output Types

use serde::{Deserialize, Serialize};

// fetch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchOutput {
    pub body: serde_json::Value,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i32,
}

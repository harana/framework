// Harana Actions - Cloudflare Worker Module Output Types

use serde::{Deserialize, Serialize};

// fetch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchOutput {
    pub body: serde_json::Value,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i32,
}

// get_var
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVarOutput {
    pub value: String,
}

// get_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretOutput {
    pub value: String,
}

// scheduled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledOutput {
    pub scheduled_time: String,
    pub success: bool,
}

// wait_until
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitUntilOutput {
    pub success: bool,
}

// pass_through
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassThroughOutput {
    pub response: serde_json::Value,
}

// service_binding_fetch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBindingFetchOutput {
    pub body: serde_json::Value,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i32,
}

// get_version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionOutput {
    pub id: String,
    pub message: String,
    pub tag: String,
    pub timestamp: String,
}

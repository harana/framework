// Harana Actions - Route Module Output Types
// Auto-generated output structs for Route action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// redirect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectOutput {
    pub success: bool,
}

// forward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardOutput {
    pub body: Value,
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
}

// rewrite_path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewritePathOutput {
    pub new_path: String,
    pub success: bool,
}

// add_headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddHeadersOutput {
    pub success: bool,
}

// remove_headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveHeadersOutput {
    pub success: bool,
}

// set_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStatusOutput {
    pub success: bool,
}

// json_response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonResponseOutput {
    pub success: bool,
}

// html_response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlResponseOutput {
    pub success: bool,
}

// rate_limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitOutput {
    pub allowed: bool,
    pub remaining: i32,
    pub reset_at: String, // datetime
}

// match_route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRouteOutput {
    pub matched: bool,
    pub params: HashMap<String, Value>,
}

// proxy_websocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyWebsocketOutput {
    pub success: bool,
}

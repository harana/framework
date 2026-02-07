// Harana Actions - Route Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add_headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddHeadersOutput {
    pub success: bool
}

// forward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardOutput {
    pub headers: HashMap<String, Value>,
    pub status_code: i32,
    pub body: String
}

// html_response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlResponseOutput {
    pub success: bool
}

// json_response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonResponseOutput {
    pub success: bool
}

// match_route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRouteOutput {
    pub params: HashMap<String, Value>,
    pub matched: bool
}

// proxy_websocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyWebsocketOutput {
    pub success: bool
}

// rate_limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitOutput {
    pub reset_at: String,
    pub allowed: bool,
    pub remaining: i32
}

// redirect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectOutput {
    pub success: bool
}

// remove_headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveHeadersOutput {
    pub success: bool
}

// rewrite_path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewritePathOutput {
    pub new_path: String,
    pub success: bool
}

// set_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStatusOutput {
    pub success: bool
}

// Harana Actions - HTTP Response Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// render_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTemplateOutput {
    pub rendered: String,
    pub success: bool,
}

// render_template_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTemplateFileOutput {
    pub rendered: String,
    pub success: bool,
}

// json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonOutput {
    pub success: bool,
}

// text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOutput {
    pub success: bool,
}

// html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlOutput {
    pub success: bool,
}

// xml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlOutput {
    pub success: bool,
}

// binary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOutput {
    pub success: bool,
}

// sse_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEventOutput {
    pub success: bool,
}

// sse_stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseStreamOutput {
    pub success: bool,
}

// not_found
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundOutput {
    pub success: bool,
}

// bad_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadRequestOutput {
    pub success: bool,
}

// unauthorized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnauthorizedOutput {
    pub success: bool,
}

// forbidden
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenOutput {
    pub success: bool,
}

// internal_error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalErrorOutput {
    pub success: bool,
}

// redirect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectOutput {
    pub success: bool,
}

// set_header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetHeaderOutput {
    pub success: bool,
}

// set_cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCookieOutput {
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEvent {
    pub data: Value,
    pub event: Option<String>,
    pub id: Option<String>,
    pub retry: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieOptions {
    pub domain: Option<String>,
    pub expires: Option<i64>,
    pub http_only: bool,
    pub max_age: Option<i32>,
    pub path: Option<String>,
    pub same_site: Option<String>,
    pub secure: bool,
}

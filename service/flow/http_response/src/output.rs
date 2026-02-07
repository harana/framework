// Harana Actions - HTTP Response Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// HTTP Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status_code: i32,
    pub headers: HashMap<String, String>,
    pub body: Option<Value>,
    pub content_type: String,
}

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
    pub response: HttpResponse,
    pub success: bool,
}

// text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// xml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// binary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// sse_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEventOutput {
    pub formatted: String,
    pub success: bool,
}

// sse_stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseStreamOutput {
    pub events: Vec<String>,
    pub success: bool,
}

// not_found
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// bad_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadRequestOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// unauthorized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnauthorizedOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// forbidden
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// internal_error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalErrorOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// bad_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadGatewayOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// service_unavailable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUnavailableOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// redirect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// set_header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetHeaderOutput {
    pub name: String,
    pub value: String,
    pub success: bool,
}

// set_cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCookieOutput {
    pub header_value: String,
    pub success: bool,
}

// delete_cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCookieOutput {
    pub header_value: String,
    pub success: bool,
}

// file_download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// empty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// cors_preflight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsPreflightOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// graphql
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// paginated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedOutput {
    pub response: HttpResponse,
    pub success: bool,
}

// csv
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvOutput {
    pub response: HttpResponse,
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

impl Default for CookieOptions {
    fn default() -> Self {
        Self {
            domain: None,
            expires: None,
            http_only: true,
            max_age: None,
            path: Some("/".to_string()),
            same_site: None,
            secure: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<GraphQLLocation>>,
    pub path: Option<Vec<Value>>,
    pub extensions: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLLocation {
    pub line: i32,
    pub column: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMetadata {
    pub page: i32,
    pub page_size: i32,
    pub total: i32,
    pub total_pages: i32,
    pub has_next: bool,
    pub has_previous: bool,
}

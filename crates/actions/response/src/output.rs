// Harana Actions - Response Module Output Types
// Auto-generated output structs for Response action methods.

use serde::{Deserialize, Serialize};

// template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateOutput {
    pub rendered: String,
    pub success: bool,
}

// template_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFileOutput {
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

// render_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTemplateOutput {
    pub rendered: String,
    pub success: bool
}

// render_template_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTemplateFileOutput {
    pub rendered: String,
    pub success: bool
}

// unauthorized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnauthorizedOutput {
    pub success: bool
}

// forbidden
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenOutput {
    pub success: bool
}

// internal_server_error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalServerErrorOutput {
    pub success: bool
}

// bad_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadGatewayOutput {
    pub success: bool
}

// service_unavailable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUnavailableOutput {
    pub success: bool
}

// error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub success: bool
}

// set_cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCookieOutput {
    pub success: bool
}

// delete_cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCookieOutput {
    pub success: bool
}

// file_download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadOutput {
    pub success: bool
}

// streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingOutput {
    pub success: bool
}

// empty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyOutput {
    pub success: bool
}

// redirect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectOutput {
    pub success: bool
}

// cors_preflight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsPreflightOutput {
    pub success: bool
}

// graphql
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlOutput {
    pub success: bool
}

// paginated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedOutput {
    pub success: bool
}

// csv
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvOutput {
    pub success: bool
}
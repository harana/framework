// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// response_template
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseTemplate {
    pub body_template: Option<String>,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub headers: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub status_code: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ResponseTemplate {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// response_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseLog {
    pub content_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub request_method: String,
    pub request_path: String,
    pub response_size: Option<i64>,
    pub status_code: i64,
    /// Reference: response_template.id
    pub template_id: Option<String>,
}

impl ResponseLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// bad_gateway_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BadGatewayOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl BadGatewayOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// bad_request_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BadRequestOutput {
    pub status_code: i64,
    pub message: String,
    pub errors: Vec<String>,
    pub body: String,
}

impl BadRequestOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// binary_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BinaryOutput {
    pub data: String,
    pub content_type: String,
    pub filename: String,
    pub status_code: i64,
}

impl BinaryOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cors_preflight_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CorsPreflightOutput {
    pub status_code: i64,
    pub headers: String,
}

impl CorsPreflightOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// csv_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvOutput {
    pub data: String,
    pub content_type: String,
    pub filename: String,
}

impl CsvOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// delete_cookie_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCookieOutput {
    pub set_cookie_header: String,
}

impl DeleteCookieOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// empty_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmptyOutput {
    pub status_code: i64,
}

impl EmptyOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// error_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorOutput {
    pub status_code: i64,
    pub message: String,
    pub error_code: String,
    pub details: String,
    pub body: String,
}

impl ErrorOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// file_download_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileDownloadOutput {
    pub data: String,
    pub content_type: String,
    pub filename: String,
    pub content_disposition: String,
}

impl FileDownloadOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// forbidden_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForbiddenOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl ForbiddenOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// graphql_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlOutput {
    pub data: String,
    pub errors: String,
    pub extensions: String,
    pub body: String,
}

impl GraphqlOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// html_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlOutput {
    pub html: String,
    pub status_code: i64,
}

impl HtmlOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// internal_server_error_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InternalServerErrorOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl InternalServerErrorOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// json_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonOutput {
    pub data: String,
    pub status_code: i64,
}

impl JsonOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// not_found_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotFoundOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl NotFoundOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// paginated_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaginatedOutput {
    pub data: Vec<String>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_previous: bool,
}

impl PaginatedOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// redirect_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RedirectOutput {
    pub url: String,
    pub status_code: i64,
}

impl RedirectOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// render_template_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateOutput {
    pub content: String,
    pub content_type: String,
    pub status_code: i64,
}

impl RenderTemplateOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// render_template_file_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateFileOutput {
    pub content: String,
    pub content_type: String,
    pub status_code: i64,
}

impl RenderTemplateFileOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// service_unavailable_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceUnavailableOutput {
    pub status_code: i64,
    pub message: String,
    pub retry_after: i64,
    pub body: String,
}

impl ServiceUnavailableOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// set_cookie_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetCookieOutput {
    pub set_cookie_header: String,
}

impl SetCookieOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sse_event_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SseEventOutput {
    pub event_string: String,
}

impl SseEventOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sse_stream_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SseStreamOutput {
    pub content_type: String,
    pub stream: String,
}

impl SseStreamOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// streaming_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamingOutput {
    pub content_type: String,
    pub chunk_size: i64,
}

impl StreamingOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// text_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextOutput {
    pub text: String,
    pub content_type: String,
    pub status_code: i64,
}

impl TextOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// unauthorized_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnauthorizedOutput {
    pub status_code: i64,
    pub message: String,
    pub www_authenticate: String,
    pub body: String,
}

impl UnauthorizedOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// xml_output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlOutput {
    pub xml: String,
    pub status_code: i64,
}

impl XmlOutput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


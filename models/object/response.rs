// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub template_id: Option<String>,
}

impl ResponseLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BadGatewayOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl BadGatewayOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BadRequestOutput {
    pub status_code: i64,
    pub message: String,
    pub errors: Vec<String>,
    pub body: String,
}

impl BadRequestOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BinaryOutput {
    pub data: String,
    pub content_type: String,
    pub filename: String,
    pub status_code: i64,
}

impl BinaryOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CorsPreflightOutput {
    pub status_code: i64,
    pub headers: std::collections::HashMap<String, String>,
}

impl CorsPreflightOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvOutput {
    pub data: String,
    pub content_type: String,
    pub filename: String,
}

impl CsvOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCookieOutput {
    pub set_cookie_header: String,
}

impl DeleteCookieOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmptyOutput {
    pub status_code: i64,
}

impl EmptyOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileDownloadOutput {
    pub data: String,
    pub content_type: String,
    pub filename: String,
    pub content_disposition: String,
}

impl FileDownloadOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForbiddenOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl ForbiddenOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlOutput {
    pub data: String,
    pub errors: String,
    pub extensions: std::collections::HashMap<String, String>,
    pub body: String,
}

impl GraphqlOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlOutput {
    pub html: String,
    pub status_code: i64,
}

impl HtmlOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InternalServerErrorOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl InternalServerErrorOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonOutput {
    pub data: String,
    pub status_code: i64,
}

impl JsonOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotFoundOutput {
    pub status_code: i64,
    pub message: String,
    pub body: String,
}

impl NotFoundOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RedirectOutput {
    pub url: String,
    pub status_code: i64,
}

impl RedirectOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateOutput {
    pub content: String,
    pub content_type: String,
    pub status_code: i64,
}

impl RenderTemplateOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateFileOutput {
    pub content: String,
    pub content_type: String,
    pub status_code: i64,
}

impl RenderTemplateFileOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceUnavailableOutput {
    pub status_code: i64,
    pub message: String,
    pub retry_after: i64,
    pub body: String,
}

impl ServiceUnavailableOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetCookieOutput {
    pub set_cookie_header: String,
}

impl SetCookieOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SseEventOutput {
    pub event_string: String,
}

impl SseEventOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SseStreamOutput {
    pub content_type: String,
    pub stream: String,
}

impl SseStreamOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamingOutput {
    pub content_type: String,
    pub chunk_size: i64,
}

impl StreamingOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextOutput {
    pub text: String,
    pub content_type: String,
    pub status_code: i64,
}

impl TextOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnauthorizedOutput {
    pub status_code: i64,
    pub message: String,
    pub www_authenticate: String,
    pub body: String,
}

impl UnauthorizedOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlOutput {
    pub xml: String,
    pub status_code: i64,
}

impl XmlOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


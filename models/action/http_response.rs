// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateInput {
    pub content_type: String,
    pub data: String,
    pub template: String,
    pub template_engine: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateOutput {
    pub rendered: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateFromFileInput {
    pub content_type: String,
    pub data: String,
    pub template_engine: String,
    pub template_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderTemplateFromFileOutput {
    pub rendered: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnJSONResponseInput {
    pub data: String,
    #[serde(default)]
    pub pretty: bool,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnJSONResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnPlainTextResponseInput {
    pub content_type: String,
    pub status_code: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnPlainTextResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnHTMLResponseInput {
    pub html: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnHTMLResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnXMLResponseInput {
    pub status_code: i64,
    pub xml: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnXMLResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnBinaryResponseInput {
    pub content_type: String,
    pub data: String,
    pub filename: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnBinaryResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamSSEEventInput {
    pub data: String,
    pub event: String,
    pub id: String,
    pub retry: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamSSEEventOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamSSEEventsInput {
    pub events: String,
    pub keep_alive_interval: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamSSEEventsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return404NotFoundInput {
    pub custom_data: String,
    #[serde(default)]
    pub include_body: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return404NotFoundOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return400BadRequestInput {
    pub custom_data: String,
    pub errors: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return400BadRequestOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return401UnauthorizedInput {
    pub custom_data: String,
    pub message: String,
    pub www_authenticate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return401UnauthorizedOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return403ForbiddenInput {
    pub custom_data: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return403ForbiddenOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return500InternalServerErrorInput {
    pub custom_data: String,
    pub error: String,
    #[serde(default)]
    pub include_details: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return500InternalServerErrorOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return502BadGatewayInput {
    pub custom_data: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return502BadGatewayOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return503ServiceUnavailableInput {
    pub custom_data: String,
    pub message: String,
    pub retry_after: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Return503ServiceUnavailableOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnCustomErrorResponseInput {
    pub details: String,
    pub error_code: String,
    pub message: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnCustomErrorResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetResponseCookieInput {
    pub domain: String,
    pub expires: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub http_only: bool,
    pub max_age: i64,
    pub name: String,
    pub path: String,
    pub same_site: String,
    #[serde(default)]
    pub secure: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetResponseCookieOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteResponseCookieInput {
    pub domain: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteResponseCookieOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnFileDownloadInput {
    pub content_type: String,
    pub file_path: String,
    pub filename: String,
    #[serde(default)]
    pub inline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnFileDownloadOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnStreamingResponseInput {
    pub chunk_size: i64,
    pub content_type: String,
    pub stream_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnStreamingResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnEmptyResponseInput {
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnEmptyResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnRedirectResponseInput {
    #[serde(default)]
    pub preserve_method: bool,
    pub status_code: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnRedirectResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnCORSPreflightResponseInput {
    pub allowed_headers: String,
    pub allowed_methods: String,
    pub allowed_origins: String,
    pub max_age: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnCORSPreflightResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnGraphQLResponseInput {
    pub data: String,
    pub errors: String,
    pub extensions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnGraphQLResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnPaginatedResponseInput {
    pub data: String,
    #[serde(default)]
    pub include_metadata: bool,
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnPaginatedResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnCSVResponseInput {
    pub data: String,
    pub delimiter: String,
    pub filename: String,
    #[serde(default)]
    pub include_headers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReturnCSVResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpResponse {
    pub status_code: i64,
    pub headers: String,
    pub body: String,
    pub content_type: String,
}

#[async_trait]
pub trait HttpResponseAction {
    async fn render _template(&self, input: RenderTemplateInput) -> Result<RenderTemplateOutput, Box<dyn std::error::Error>>;
    async fn render _template _from _file(&self, input: RenderTemplateFromFileInput) -> Result<RenderTemplateFromFileOutput, Box<dyn std::error::Error>>;
    async fn return _j_s_o_n _response(&self, input: ReturnJSONResponseInput) -> Result<ReturnJSONResponseOutput, Box<dyn std::error::Error>>;
    async fn return _plain _text _response(&self, input: ReturnPlainTextResponseInput) -> Result<ReturnPlainTextResponseOutput, Box<dyn std::error::Error>>;
    async fn return _h_t_m_l _response(&self, input: ReturnHTMLResponseInput) -> Result<ReturnHTMLResponseOutput, Box<dyn std::error::Error>>;
    async fn return _x_m_l _response(&self, input: ReturnXMLResponseInput) -> Result<ReturnXMLResponseOutput, Box<dyn std::error::Error>>;
    async fn return _binary _response(&self, input: ReturnBinaryResponseInput) -> Result<ReturnBinaryResponseOutput, Box<dyn std::error::Error>>;
    async fn stream _s_s_e _event(&self, input: StreamSSEEventInput) -> Result<StreamSSEEventOutput, Box<dyn std::error::Error>>;
    async fn stream _s_s_e _events(&self, input: StreamSSEEventsInput) -> Result<StreamSSEEventsOutput, Box<dyn std::error::Error>>;
    async fn return 404 _not _found(&self, input: Return404NotFoundInput) -> Result<Return404NotFoundOutput, Box<dyn std::error::Error>>;
    async fn return 400 _bad _request(&self, input: Return400BadRequestInput) -> Result<Return400BadRequestOutput, Box<dyn std::error::Error>>;
    async fn return 401 _unauthorized(&self, input: Return401UnauthorizedInput) -> Result<Return401UnauthorizedOutput, Box<dyn std::error::Error>>;
    async fn return 403 _forbidden(&self, input: Return403ForbiddenInput) -> Result<Return403ForbiddenOutput, Box<dyn std::error::Error>>;
    async fn return 500 _internal _server _error(&self, input: Return500InternalServerErrorInput) -> Result<Return500InternalServerErrorOutput, Box<dyn std::error::Error>>;
    async fn return 502 _bad _gateway(&self, input: Return502BadGatewayInput) -> Result<Return502BadGatewayOutput, Box<dyn std::error::Error>>;
    async fn return 503 _service _unavailable(&self, input: Return503ServiceUnavailableInput) -> Result<Return503ServiceUnavailableOutput, Box<dyn std::error::Error>>;
    async fn return _custom _error _response(&self, input: ReturnCustomErrorResponseInput) -> Result<ReturnCustomErrorResponseOutput, Box<dyn std::error::Error>>;
    async fn set _response _cookie(&self, input: SetResponseCookieInput) -> Result<SetResponseCookieOutput, Box<dyn std::error::Error>>;
    async fn delete _response _cookie(&self, input: DeleteResponseCookieInput) -> Result<DeleteResponseCookieOutput, Box<dyn std::error::Error>>;
    async fn return _file _download(&self, input: ReturnFileDownloadInput) -> Result<ReturnFileDownloadOutput, Box<dyn std::error::Error>>;
    async fn return _streaming _response(&self, input: ReturnStreamingResponseInput) -> Result<ReturnStreamingResponseOutput, Box<dyn std::error::Error>>;
    async fn return _empty _response(&self, input: ReturnEmptyResponseInput) -> Result<ReturnEmptyResponseOutput, Box<dyn std::error::Error>>;
    async fn return _redirect _response(&self, input: ReturnRedirectResponseInput) -> Result<ReturnRedirectResponseOutput, Box<dyn std::error::Error>>;
    async fn return _c_o_r_s _preflight _response(&self, input: ReturnCORSPreflightResponseInput) -> Result<ReturnCORSPreflightResponseOutput, Box<dyn std::error::Error>>;
    async fn return _graph_q_l _response(&self, input: ReturnGraphQLResponseInput) -> Result<ReturnGraphQLResponseOutput, Box<dyn std::error::Error>>;
    async fn return _paginated _response(&self, input: ReturnPaginatedResponseInput) -> Result<ReturnPaginatedResponseOutput, Box<dyn std::error::Error>>;
    async fn return _c_s_v _response(&self, input: ReturnCSVResponseInput) -> Result<ReturnCSVResponseOutput, Box<dyn std::error::Error>>;
}

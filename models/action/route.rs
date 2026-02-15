// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RedirectInput {
    #[serde(default)]
    pub preserve_query: bool,
    pub status_code: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RedirectOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForwardInput {
    pub headers: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub preserve_host: bool,
    pub target_url: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForwardOutput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RewritePathInput {
    pub pattern: String,
    pub replacement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RewritePathOutput {
    pub new_path: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddHeadersInput {
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddHeadersOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveHeadersInput {
    pub headers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveHeadersOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetStatusInput {
    pub message: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetStatusOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonResponseInput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlResponseInput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RateLimitInput {
    pub key: String,
    pub limit: i64,
    pub window_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RateLimitOutput {
    pub allowed: bool,
    pub remaining: i64,
    pub reset_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MatchRouteInput {
    pub method: String,
    pub path: String,
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MatchRouteOutput {
    pub matched: bool,
    pub params: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProxyWebsocketInput {
    pub headers: std::collections::HashMap<String, String>,
    pub target_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProxyWebsocketOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Route {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub middleware: Vec<String>,
    pub params: std::collections::HashMap<String, String>,
}

#[async_trait]
pub trait RouteAction {
    async fn redirect(&self, input: RedirectInput) -> Result<RedirectOutput, Box<dyn std::error::Error>>;
    async fn forward(&self, input: ForwardInput) -> Result<ForwardOutput, Box<dyn std::error::Error>>;
    async fn rewrite_path(&self, input: RewritePathInput) -> Result<RewritePathOutput, Box<dyn std::error::Error>>;
    async fn add_headers(&self, input: AddHeadersInput) -> Result<AddHeadersOutput, Box<dyn std::error::Error>>;
    async fn remove_headers(&self, input: RemoveHeadersInput) -> Result<RemoveHeadersOutput, Box<dyn std::error::Error>>;
    async fn set_status(&self, input: SetStatusInput) -> Result<SetStatusOutput, Box<dyn std::error::Error>>;
    async fn json_response(&self, input: JsonResponseInput) -> Result<JsonResponseOutput, Box<dyn std::error::Error>>;
    async fn html_response(&self, input: HtmlResponseInput) -> Result<HtmlResponseOutput, Box<dyn std::error::Error>>;
    async fn rate_limit(&self, input: RateLimitInput) -> Result<RateLimitOutput, Box<dyn std::error::Error>>;
    async fn match_route(&self, input: MatchRouteInput) -> Result<MatchRouteOutput, Box<dyn std::error::Error>>;
    async fn proxy_websocket(&self, input: ProxyWebsocketInput) -> Result<ProxyWebsocketOutput, Box<dyn std::error::Error>>;
}

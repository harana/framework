// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PatchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadOutput {
    pub content: String,
    pub content_type: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadOutput {
    pub body: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlQueryOutput {
    pub data: String,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: String,
    pub query_params: String,
    pub body: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Headers {
    pub content_type: String,
    pub authorization: String,
    pub accept: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryParams {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Variables {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphQlError {
    pub message: String,
    pub path: Vec<String>,
    pub locations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphQlErrorLocation {
    pub line: i64,
    pub column: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpClient {
    pub base_url: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_headers: String,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub max_retries: i64,
    pub timeout_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpRequestLog {
    pub client_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: i64,
    pub error_message: String,
    pub method: String,
    pub request_body: String,
    pub request_headers: String,
    pub response_body: String,
    pub response_headers: String,
    pub status_code: i64,
    pub url: String,
}

#[async_trait]
pub trait HttpClientAction {
    async fn get(&self, headers: String, query_params: String, timeout: i64, url: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn post(&self, body: String, content_type: String, headers: String, query_params: String, timeout: i64, url: String) -> Result<PostOutput, Box<dyn std::error::Error>>;
    async fn put(&self, body: String, content_type: String, headers: String, query_params: String, timeout: i64, url: String) -> Result<PutOutput, Box<dyn std::error::Error>>;
    async fn patch(&self, body: String, content_type: String, headers: String, query_params: String, timeout: i64, url: String) -> Result<PatchOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, headers: String, query_params: String, timeout: i64, url: String) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn download(&self, headers: String, timeout: i64, url: String) -> Result<DownloadOutput, Box<dyn std::error::Error>>;
    async fn upload(&self, file: String, filename: String, headers: String, timeout: i64, url: String) -> Result<UploadOutput, Box<dyn std::error::Error>>;
    async fn graphql_query(&self, headers: String, query: String, timeout: i64, url: String, variables: String) -> Result<GraphqlQueryOutput, Box<dyn std::error::Error>>;
}

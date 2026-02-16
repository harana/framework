// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseTemplate {
    pub body_template: String,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub headers: String,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub status_code: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseLog {
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: i64,
    pub request_method: String,
    pub request_path: String,
    pub response_size: i64,
    pub status_code: i64,
    pub template_id: String,
}

#[async_trait]
pub trait ResponseAction {
    async fn response/bad_gateway(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/bad_request(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/binary(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/cors_preflight(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/csv(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/delete_cookie(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/empty(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/error(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/file_download(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/forbidden(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/graphql(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/html(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/internal_server_error(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/json(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/not_found(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/paginated(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/redirect(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/render_template(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/render_template_file(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/service_unavailable(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/set_cookie(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/sse_event(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/sse_stream(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/streaming(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/text(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/unauthorized(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn response/xml(&self) -> Result<(), Box<dyn std::error::Error>>;
}

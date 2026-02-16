// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderOutput {
    pub content: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractContentOutput {
    pub content: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfBrowserRenderingSession {
    pub account_id: String,
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfBrowserRenderingResult {
    pub content: String,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub rendered_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
    pub status_code: i64,
    pub title: String,
    pub url: String,
}

#[async_trait]
pub trait BrowserRenderingAction {
    async fn render(&self, binding: String, url: String) -> Result<RenderOutput, Box<dyn std::error::Error>>;
    async fn screenshot(&self, binding: String, full_page: bool, height: i64, type: String, url: String, width: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn extract_content(&self, binding: String, selector: String, url: String) -> Result<ExtractContentOutput, Box<dyn std::error::Error>>;
    async fn pdf(&self, binding: String, format: String, landscape: bool, print_background: bool, url: String) -> Result<String, Box<dyn std::error::Error>>;
}

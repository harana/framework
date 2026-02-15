// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderInput {
    pub binding: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenderOutput {
    pub content: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScreenshotInput {
    pub binding: String,
    #[serde(default)]
    pub full_page: bool,
    pub height: i64,
    pub type: String,
    pub url: String,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScreenshotOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractContentInput {
    pub binding: String,
    pub selector: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractContentOutput {
    pub content: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfInput {
    pub binding: String,
    pub format: String,
    #[serde(default)]
    pub landscape: bool,
    #[serde(default)]
    pub print_background: bool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfOutput {
    pub pdf: String,
}

#[async_trait]
pub trait BrowserRenderingAction {
    async fn render(&self, input: RenderInput) -> Result<RenderOutput, Box<dyn std::error::Error>>;
    async fn screenshot(&self, input: ScreenshotInput) -> Result<ScreenshotOutput, Box<dyn std::error::Error>>;
    async fn extract_content(&self, input: ExtractContentInput) -> Result<ExtractContentOutput, Box<dyn std::error::Error>>;
    async fn pdf(&self, input: PdfInput) -> Result<PdfOutput, Box<dyn std::error::Error>>;
}

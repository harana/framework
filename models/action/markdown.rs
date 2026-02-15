// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToHtmlInput {
    pub data: String,
    #[serde(default)]
    pub gfm: bool,
    #[serde(default)]
    pub highlight_code: bool,
    #[serde(default)]
    pub sanitize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToHtmlOutput {
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FromHtmlInput {
    pub bullet_style: String,
    pub code_block_style: String,
    pub data: String,
    pub heading_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FromHtmlOutput {
    pub markdown: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractFrontmatterInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractFrontmatterOutput {
    pub content: String,
    pub frontmatter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownDocument {
    pub content: String,
    pub frontmatter: String,
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownFrontmatter {
    pub data: std::collections::HashMap<String, String>,
}

#[async_trait]
pub trait MarkdownAction {
    async fn to_html(&self, input: ToHtmlInput) -> Result<ToHtmlOutput, Box<dyn std::error::Error>>;
    async fn from_html(&self, input: FromHtmlInput) -> Result<FromHtmlOutput, Box<dyn std::error::Error>>;
    async fn extract_frontmatter(&self, input: ExtractFrontmatterInput) -> Result<ExtractFrontmatterOutput, Box<dyn std::error::Error>>;
}

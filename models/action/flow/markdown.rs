// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    async fn to_html(&self, data: String, gfm: bool, highlight_code: bool, sanitize: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn from_html(&self, bullet_style: String, code_block_style: String, data: String, heading_style: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn extract_frontmatter(&self, data: String) -> Result<ExtractFrontmatterOutput, Box<dyn std::error::Error>>;
}

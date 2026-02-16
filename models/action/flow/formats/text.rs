// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextString {
    pub content: String,
    pub length: i64,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateData {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextTemplate {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub engine: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub variables: String,
}

#[async_trait]
pub trait TextAction {
    async fn template(&self, data: String, engine: String, template: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn regex_match(&self, flags: String, pattern: String, text: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn regex_replace(&self, flags: String, pattern: String, replacement: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn split(&self, delimiter: String, limit: i64, text: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn join(&self, items: Vec<String>, separator: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn trim(&self, characters: String, mode: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn case_convert(&self, format: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn truncate(&self, length: i64, suffix: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn slugify(&self, lowercase: bool, separator: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
}

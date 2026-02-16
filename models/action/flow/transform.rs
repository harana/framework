// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformResult {
    pub input_format: String,
    pub output_format: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformJsonObject {
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownOptions {
    pub gfm: bool,
    pub breaks: bool,
    pub sanitize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformJob {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error_message: String,
    pub input_format: String,
    pub input_size: i64,
    pub output_format: String,
    pub output_size: i64,
    pub status: String,
    pub transform_type: String,
}

#[async_trait]
pub trait TransformAction {
    async fn json_to_xml(&self, data: String, root_element: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn xml_to_json(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn csv_to_json(&self, data: String, delimiter: String, headers: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn json_to_csv(&self, data: Vec<String>, delimiter: String, headers: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn yaml_to_json(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn json_to_yaml(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn base64_encode(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn base64_decode(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn url_encode(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn url_decode(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn html_encode(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn html_decode(&self, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn markdown_to_html(&self, data: String, options: String) -> Result<String, Box<dyn std::error::Error>>;
}

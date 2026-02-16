// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlDocument {
    pub content: String,
    pub root_element: String,
    pub namespaces: String,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlObject {
    pub elements: std::collections::HashMap<String, String>,
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlValidationError {
    pub line: i64,
    pub column: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlNamespaces {
    pub namespaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlNamespace {
    pub prefix: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlSchema {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub schema_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlValidationResult {
    pub document_id: String,
    pub error_count: i64,
    pub errors: String,
    #[serde(default)]
    pub is_valid: bool,
    pub schema_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait XmlAction {
    async fn parse(&self, data: String, preserve_attributes: bool, preserve_namespaces: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn generate(&self, data: String, declaration: bool, indent: bool, root_element: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn validate(&self, data: String, schema: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn xpath_query(&self, data: String, namespaces: String, query: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

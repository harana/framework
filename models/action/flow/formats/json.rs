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
pub struct DiffOutput {
    pub added: String,
    pub changed: String,
    pub removed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonDocument {
    pub data: String,
    pub schema: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonSchema {
    pub type: String,
    pub properties: std::collections::HashMap<String, String>,
    pub required: Vec<String>,
    pub additional_properties: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonValidationError {
    pub path: String,
    pub message: String,
    pub keyword: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonObject {
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonValidationResult {
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
pub trait JsonAction {
    async fn parse(&self, data: String, strict: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn stringify(&self, data: String, indent: i64, sort_keys: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn validate(&self, data: String, schema: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn jmespath_query(&self, data: String, query: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn merge(&self, objects: Vec<String>, strategy: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn diff(&self, include_unchanged: bool, source: String, target: String) -> Result<DiffOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub data: String,
    #[serde(default)]
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StringifyInput {
    pub data: String,
    pub indent: i64,
    #[serde(default)]
    pub sort_keys: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StringifyOutput {
    pub json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateInput {
    pub data: String,
    pub schema: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JmespathQueryInput {
    pub data: String,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JmespathQueryOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeInput {
    pub objects: Vec<String>,
    pub strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DiffInput {
    #[serde(default)]
    pub include_unchanged: bool,
    pub source: String,
    pub target: String,
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

#[async_trait]
pub trait JsonAction {
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn stringify(&self, input: StringifyInput) -> Result<StringifyOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn jmespath_query(&self, input: JmespathQueryInput) -> Result<JmespathQueryOutput, Box<dyn std::error::Error>>;
    async fn merge(&self, input: MergeInput) -> Result<MergeOutput, Box<dyn std::error::Error>>;
    async fn diff(&self, input: DiffInput) -> Result<DiffOutput, Box<dyn std::error::Error>>;
}

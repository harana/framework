// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub data: String,
    #[serde(default)]
    pub preserve_attributes: bool,
    #[serde(default)]
    pub preserve_namespaces: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateInput {
    pub data: String,
    #[serde(default)]
    pub declaration: bool,
    #[serde(default)]
    pub indent: bool,
    pub root_element: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateOutput {
    pub xml: String,
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
pub struct XpathQueryInput {
    pub data: String,
    pub namespaces: String,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XpathQueryOutput {
    pub results: Vec<String>,
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

#[async_trait]
pub trait XmlAction {
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn generate(&self, input: GenerateInput) -> Result<GenerateOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn xpath_query(&self, input: XpathQueryInput) -> Result<XpathQueryOutput, Box<dyn std::error::Error>>;
}

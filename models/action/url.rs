// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub fragment: String,
    pub host: String,
    pub path: String,
    pub port: i64,
    pub protocol: String,
    pub query: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildInput {
    pub fragment: String,
    pub host: String,
    pub path: String,
    pub port: i64,
    pub protocol: String,
    pub query: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildOutput {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncodeInput {
    #[serde(default)]
    pub component: bool,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncodeOutput {
    pub encoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecodeInput {
    #[serde(default)]
    pub component: bool,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecodeOutput {
    pub decoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortenInput {
    pub custom_alias: String,
    pub expiration: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortenOutput {
    pub short_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExpandInput {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExpandOutput {
    pub original_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateInput {
    pub allowed_protocols: Vec<String>,
    #[serde(default)]
    pub require_tld: bool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Url {
    pub url: String,
    pub protocol: String,
    pub host: String,
    pub port: i64,
    pub path: String,
    pub query: std::collections::HashMap<String, String>,
    pub fragment: String,
}

#[async_trait]
pub trait UrlAction {
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn build(&self, input: BuildInput) -> Result<BuildOutput, Box<dyn std::error::Error>>;
    async fn encode(&self, input: EncodeInput) -> Result<EncodeOutput, Box<dyn std::error::Error>>;
    async fn decode(&self, input: DecodeInput) -> Result<DecodeOutput, Box<dyn std::error::Error>>;
    async fn shorten(&self, input: ShortenInput) -> Result<ShortenOutput, Box<dyn std::error::Error>>;
    async fn expand(&self, input: ExpandInput) -> Result<ExpandOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}

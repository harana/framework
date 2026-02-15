// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateV4Input {
    pub count: i64,
    #[serde(default)]
    pub uppercase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateV4Output {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateV7Input {
    pub count: i64,
    #[serde(default)]
    pub uppercase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateV7Output {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateInput {
    pub uuid: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub valid: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub timestamp: i64,
    pub variant: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Uuid {
    pub value: String,
    pub version: i64,
    pub variant: String,
    pub timestamp: i64,
}

#[async_trait]
pub trait UuidAction {
    async fn generate_v4(&self, input: GenerateV4Input) -> Result<GenerateV4Output, Box<dyn std::error::Error>>;
    async fn generate_v7(&self, input: GenerateV7Input) -> Result<GenerateV7Output, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
}

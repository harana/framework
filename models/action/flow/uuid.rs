// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub valid: bool,
    pub version: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UuidRegistry {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub entity_id: String,
    pub entity_type: String,
    pub value: String,
    pub variant: String,
    pub version: i64,
}

#[async_trait]
pub trait UuidAction {
    async fn generate_v4(&self, count: i64, uppercase: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn generate_v7(&self, count: i64, uppercase: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn validate(&self, uuid: String, version: i64) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn parse(&self, uuid: String) -> Result<ParseOutput, Box<dyn std::error::Error>>;
}

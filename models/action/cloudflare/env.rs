// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVarInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVarOutput {
    pub found: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretOutput {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStoreSecretInput {
    pub name: String,
    pub store_binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStoreSecretOutput {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVarsOutput {
    pub variables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVersionInput {
    pub binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVersionOutput {
    pub id: String,
    pub message: String,
    pub tag: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait EnvAction {
    async fn get_var(&self, input: GetVarInput) -> Result<GetVarOutput, Box<dyn std::error::Error>>;
    async fn get_secret(&self, input: GetSecretInput) -> Result<GetSecretOutput, Box<dyn std::error::Error>>;
    async fn get_store_secret(&self, input: GetStoreSecretInput) -> Result<GetStoreSecretOutput, Box<dyn std::error::Error>>;
    async fn list_vars(&self) -> Result<ListVarsOutput, Box<dyn std::error::Error>>;
    async fn get_version(&self, input: GetVersionInput) -> Result<GetVersionOutput, Box<dyn std::error::Error>>;
}

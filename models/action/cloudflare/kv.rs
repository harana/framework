// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub cache_ttl: i64,
    pub key: String,
    pub namespace: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub found: bool,
    pub metadata: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWithMetadataInput {
    pub cache_ttl: i64,
    pub key: String,
    pub namespace: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWithMetadataOutput {
    pub found: bool,
    pub metadata: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutInput {
    pub expiration: i64,
    pub expiration_ttl: i64,
    pub key: String,
    pub metadata: String,
    pub namespace: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutWithMetadataInput {
    pub expiration: i64,
    pub expiration_ttl: i64,
    pub key: String,
    pub metadata: String,
    pub namespace: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutWithMetadataOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub key: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListInput {
    pub cursor: String,
    pub limit: i64,
    pub namespace: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListOutput {
    pub cursor: String,
    pub keys: Vec<String>,
    pub list_complete: bool,
}

#[async_trait]
pub trait KvAction {
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn get_with_metadata(&self, input: GetWithMetadataInput) -> Result<GetWithMetadataOutput, Box<dyn std::error::Error>>;
    async fn put(&self, input: PutInput) -> Result<PutOutput, Box<dyn std::error::Error>>;
    async fn put_with_metadata(&self, input: PutWithMetadataInput) -> Result<PutWithMetadataOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn list(&self, input: ListInput) -> Result<ListOutput, Box<dyn std::error::Error>>;
}

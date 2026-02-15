// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub key: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub found: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetInput {
    pub key: String,
    pub namespace: String,
    pub ttl: i64,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetOutput {
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
pub struct ExistsInput {
    pub key: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExistsOutput {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClearInput {
    pub namespace: String,
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClearOutput {
    pub keys_deleted: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetManyInput {
    pub keys: Vec<String>,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetManyOutput {
    pub values: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetManyInput {
    pub entries: String,
    pub namespace: String,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetManyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IncrementInput {
    pub amount: i64,
    pub key: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IncrementOutput {
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecrementInput {
    pub amount: i64,
    pub key: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecrementOutput {
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TtlInput {
    pub key: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TtlOutput {
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheEntry {
    pub key: String,
    pub value: String,
    pub namespace: String,
    pub ttl: i64,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheValues {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheEntries {
    pub entries: std::collections::HashMap<String, String>,
}

#[async_trait]
pub trait CacheAction {
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn set(&self, input: SetInput) -> Result<SetOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn exists(&self, input: ExistsInput) -> Result<ExistsOutput, Box<dyn std::error::Error>>;
    async fn clear(&self, input: ClearInput) -> Result<ClearOutput, Box<dyn std::error::Error>>;
    async fn get_many(&self, input: GetManyInput) -> Result<GetManyOutput, Box<dyn std::error::Error>>;
    async fn set_many(&self, input: SetManyInput) -> Result<SetManyOutput, Box<dyn std::error::Error>>;
    async fn increment(&self, input: IncrementInput) -> Result<IncrementOutput, Box<dyn std::error::Error>>;
    async fn decrement(&self, input: DecrementInput) -> Result<DecrementOutput, Box<dyn std::error::Error>>;
    async fn ttl(&self, input: TtlInput) -> Result<TtlOutput, Box<dyn std::error::Error>>;
}

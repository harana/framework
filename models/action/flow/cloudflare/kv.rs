// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub found: bool,
    pub metadata: String,
    pub value: String,
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
pub struct ListOutput {
    pub cursor: String,
    pub keys: Vec<String>,
    pub list_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfKvNamespace {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub namespace_id: String,
    pub namespace_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfKvEntry {
    pub cache_ttl: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expiration: i64,
    pub expiration_ttl: i64,
    pub key: String,
    pub metadata: String,
    pub namespace_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub value: String,
    pub value_type: String,
}

#[async_trait]
pub trait KvAction {
    async fn get(&self, cache_ttl: i64, key: String, namespace: String, type: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn get_with_metadata(&self, cache_ttl: i64, key: String, namespace: String, type: String) -> Result<GetWithMetadataOutput, Box<dyn std::error::Error>>;
    async fn put(&self, expiration: i64, expiration_ttl: i64, key: String, metadata: String, namespace: String, value: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn put_with_metadata(&self, expiration: i64, expiration_ttl: i64, key: String, metadata: String, namespace: String, value: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, key: String, namespace: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list(&self, cursor: String, limit: i64, namespace: String, prefix: String) -> Result<ListOutput, Box<dyn std::error::Error>>;
}

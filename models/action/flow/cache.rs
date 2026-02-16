// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub found: bool,
    pub value: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheConfig {
    pub default_ttl_seconds: i64,
    #[serde(default)]
    pub is_enabled: bool,
    pub max_entries: i64,
    pub strategy: String,
}

#[async_trait]
pub trait CacheAction {
    async fn get(&self, key: String, namespace: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn set(&self, key: String, namespace: String, ttl: i64, value: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, key: String, namespace: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn exists(&self, key: String, namespace: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn clear(&self, namespace: String, pattern: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn get_many(&self, keys: Vec<String>, namespace: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn set_many(&self, entries: String, namespace: String, ttl: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn increment(&self, amount: i64, key: String, namespace: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn decrement(&self, amount: i64, key: String, namespace: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn ttl(&self, key: String, namespace: String) -> Result<TtlOutput, Box<dyn std::error::Error>>;
}

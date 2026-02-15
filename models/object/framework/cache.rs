// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheEntry {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub key: String,
    pub namespace: String,
    pub ttl_seconds: Option<i64>,
    pub value: String,
}

impl CacheEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl CacheConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheValues {
    pub values: std::collections::HashMap<String, String>,
}

impl CacheValues {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheEntries {
    pub entries: std::collections::HashMap<String, String>,
}

impl CacheEntries {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CachePutOptions {
    pub expiration: Option<i64>,
    pub expiration_ttl: Option<i64>,
    pub metadata: Option<String>,
}

impl CachePutOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheGetOptions {
    pub cache_ttl: Option<i64>,
}

impl CacheGetOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheListOptions {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub prefix: Option<String>,
}

impl CacheListOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheKeyEntry {
    pub expiration: Option<i64>,
    pub metadata: Option<String>,
    pub name: String,
}

impl CacheKeyEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheListResponse {
    pub cursor: Option<String>,
    pub keys: Vec<String>,
    pub list_complete: bool,
}

impl CacheListResponse {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


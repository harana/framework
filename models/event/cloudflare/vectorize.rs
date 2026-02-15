// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorsInserted {
    pub index: String,
    pub count: i64,
    pub mutation_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub inserted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareVectorsInserted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorsUpserted {
    pub index: String,
    pub count: i64,
    pub mutation_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub upserted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareVectorsUpserted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorsQueried {
    pub index: String,
    pub top_k: i64,
    pub match_count: Option<i64>,
    pub namespace: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub queried_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareVectorsQueried {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorsDeleted {
    pub index: String,
    pub count: Option<i64>,
    pub mutation_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareVectorsDeleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorizeMetadataIndexCreated {
    pub index: String,
    pub property_name: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareVectorizeMetadataIndexCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorizeMetadataIndexDeleted {
    pub index: String,
    pub property_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareVectorizeMetadataIndexDeleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


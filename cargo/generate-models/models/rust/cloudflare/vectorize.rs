// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cloudflare_vectorize_index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorizeIndex {
    pub account_id: String,
    pub binding: String,
    pub configured_dimensions: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub index_name: String,
    pub metric: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vectors_count: i64,
}

impl CloudflareVectorizeIndex {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_vectorize_vector
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorizeVector {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: cf_vectorize_index.id
    pub index_id: String,
    pub metadata: Option<String>,
    pub namespace: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub values: Option<String>,
    pub vector_id: String,
}

impl CloudflareVectorizeVector {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_vectorize_metadata_index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareVectorizeMetadataIndex {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: cf_vectorize_index.id
    pub index_id: String,
    pub property_name: String,
    pub type: String,
}

impl CloudflareVectorizeMetadataIndex {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


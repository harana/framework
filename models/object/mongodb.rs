// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongodbConnection {
    pub connection_string: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database: String,
    pub host: String,
    #[serde(default)]
    pub is_active: bool,
    pub port: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub use_ssl: bool,
}

impl MongodbConnection {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongodbCollection {
    pub connection_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub document_count: i64,
    pub name: String,
    pub size: i64,
}

impl MongodbCollection {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongodbIndex {
    pub collection_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_unique: bool,
    pub keys: String,
    pub name: String,
}

impl MongodbIndex {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongodbQueryLog {
    pub collection_id: String,
    pub documents_affected: i64,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub operation: String,
    pub status: String,
    pub user_id: Option<String>,
}

impl MongodbQueryLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoDocument {
    pub _id: String,
    pub collection: String,
    pub database: String,
    pub document: String,
}

impl MongoDocument {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoDocumentData {
    pub data: std::collections::HashMap<String, String>,
}

impl MongoDocumentData {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoFilter {
    pub conditions: std::collections::HashMap<String, String>,
}

impl MongoFilter {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoProjection {
    pub fields: std::collections::HashMap<String, String>,
}

impl MongoProjection {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoSort {
    pub fields: std::collections::HashMap<String, String>,
}

impl MongoSort {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoUpdate {
    pub operators: std::collections::HashMap<String, String>,
}

impl MongoUpdate {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoPipelineStage {
    pub stage: std::collections::HashMap<String, String>,
}

impl MongoPipelineStage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoIndexKeys {
    pub fields: std::collections::HashMap<String, String>,
}

impl MongoIndexKeys {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoBulkOperation {
    pub operation_type: String,
    pub filter: String,
    pub document: String,
    pub update: String,
}

impl MongoBulkOperation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


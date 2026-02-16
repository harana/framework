// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertManyOutput {
    pub inserted_count: i64,
    pub inserted_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindOneOutput {
    pub document: String,
    pub found: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindOutput {
    pub count: i64,
    pub documents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOneOutput {
    pub matched_count: i64,
    pub modified_count: i64,
    pub upserted_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateManyOutput {
    pub matched_count: i64,
    pub modified_count: i64,
    pub upserted_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplaceOneOutput {
    pub matched_count: i64,
    pub modified_count: i64,
    pub upserted_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkWriteOutput {
    pub deleted_count: i64,
    pub inserted_count: i64,
    pub matched_count: i64,
    pub modified_count: i64,
    pub upserted_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoDocument {
    pub _id: String,
    pub collection: String,
    pub database: String,
    pub document: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoDocumentData {
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoFilter {
    pub conditions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoProjection {
    pub fields: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoSort {
    pub fields: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoUpdate {
    pub operators: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoPipelineStage {
    pub stage: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoIndexKeys {
    pub fields: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongoBulkOperation {
    pub operation_type: String,
    pub filter: String,
    pub document: String,
    pub update: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongodbConnection {
    pub connection_string: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MongodbQueryLog {
    pub collection_id: String,
    pub documents_affected: i64,
    pub duration_ms: i64,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub operation: String,
    pub status: String,
    pub user_id: String,
}

#[async_trait]
pub trait MongodbAction {
    async fn insert(&self, collection: String, database: String, document: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn insert_many(&self, collection: String, database: String, documents: Vec<String>, ordered: bool) -> Result<InsertManyOutput, Box<dyn std::error::Error>>;
    async fn find_one(&self, collection: String, database: String, filter: String, projection: String) -> Result<FindOneOutput, Box<dyn std::error::Error>>;
    async fn find(&self, collection: String, database: String, filter: String, limit: i64, projection: String, skip: i64, sort: String) -> Result<FindOutput, Box<dyn std::error::Error>>;
    async fn update_one(&self, collection: String, database: String, filter: String, update: String, upsert: bool) -> Result<UpdateOneOutput, Box<dyn std::error::Error>>;
    async fn update_many(&self, collection: String, database: String, filter: String, update: String, upsert: bool) -> Result<UpdateManyOutput, Box<dyn std::error::Error>>;
    async fn replace_one(&self, collection: String, database: String, filter: String, replacement: String, upsert: bool) -> Result<ReplaceOneOutput, Box<dyn std::error::Error>>;
    async fn delete_one(&self, collection: String, database: String, filter: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn delete_many(&self, collection: String, database: String, filter: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn count(&self, collection: String, database: String, filter: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn aggregate(&self, collection: String, database: String, pipeline: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_index(&self, collection: String, database: String, keys: String, name: String, unique: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn drop_index(&self, collection: String, database: String, index_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_collections(&self, database: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn drop_collection(&self, collection: String, database: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn bulk_write(&self, collection: String, database: String, operations: Vec<String>, ordered: bool) -> Result<BulkWriteOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertOutput {
    pub count: i64,
    pub mutation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpsertOutput {
    pub count: i64,
    pub mutation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryOutput {
    pub count: i64,
    pub matches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryByIdOutput {
    pub count: i64,
    pub matches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteByIdsOutput {
    pub count: i64,
    pub mutation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOutput {
    pub configured_dimensions: i64,
    pub description: String,
    pub metric: String,
    pub name: String,
    pub vectors_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoOutput {
    pub dimensions: i64,
    pub metric: String,
    pub name: String,
    pub vectors_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfVectorizeIndex {
    pub account_id: String,
    pub binding: String,
    pub configured_dimensions: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub index_name: String,
    pub metric: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vectors_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfVectorizeVector {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub index_id: String,
    pub metadata: String,
    pub namespace: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub values: String,
    pub vector_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfVectorizeMetadataIndex {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub index_id: String,
    pub property_name: String,
    pub type: String,
}

#[async_trait]
pub trait VectorizeAction {
    async fn insert(&self, index: String, vectors: Vec<String>) -> Result<InsertOutput, Box<dyn std::error::Error>>;
    async fn upsert(&self, index: String, vectors: Vec<String>) -> Result<UpsertOutput, Box<dyn std::error::Error>>;
    async fn query(&self, filter: String, index: String, namespace: String, return_metadata: String, return_values: bool, top_k: i64, vector: Vec<f64>) -> Result<QueryOutput, Box<dyn std::error::Error>>;
    async fn query_by_id(&self, filter: String, index: String, namespace: String, return_metadata: String, return_values: bool, top_k: i64, vector_id: String) -> Result<QueryByIdOutput, Box<dyn std::error::Error>>;
    async fn get_by_ids(&self, ids: Vec<String>, index: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn delete_by_ids(&self, ids: Vec<String>, index: String) -> Result<DeleteByIdsOutput, Box<dyn std::error::Error>>;
    async fn describe(&self, index: String) -> Result<DescribeOutput, Box<dyn std::error::Error>>;
    async fn info(&self, index: String) -> Result<InfoOutput, Box<dyn std::error::Error>>;
    async fn create_metadata_index(&self, index: String, property_name: String, type: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_metadata_index(&self, index: String, property_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_metadata_indexes(&self, index: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

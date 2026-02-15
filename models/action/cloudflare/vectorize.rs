// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertInput {
    pub index: String,
    pub vectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertOutput {
    pub count: i64,
    pub mutation_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpsertInput {
    pub index: String,
    pub vectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpsertOutput {
    pub count: i64,
    pub mutation_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryInput {
    pub filter: String,
    pub index: String,
    pub namespace: String,
    pub return_metadata: String,
    #[serde(default)]
    pub return_values: bool,
    pub top_k: i64,
    pub vector: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryOutput {
    pub count: i64,
    pub matches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryByIdInput {
    pub filter: String,
    pub index: String,
    pub namespace: String,
    pub return_metadata: String,
    #[serde(default)]
    pub return_values: bool,
    pub top_k: i64,
    pub vector_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryByIdOutput {
    pub count: i64,
    pub matches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetByIdsInput {
    pub ids: Vec<String>,
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetByIdsOutput {
    pub vectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteByIdsInput {
    pub ids: Vec<String>,
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteByIdsOutput {
    pub count: i64,
    pub mutation_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInput {
    pub index: String,
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
pub struct InfoInput {
    pub index: String,
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
pub struct CreateMetadataIndexInput {
    pub index: String,
    pub property_name: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMetadataIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMetadataIndexInput {
    pub index: String,
    pub property_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMetadataIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMetadataIndexesInput {
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMetadataIndexesOutput {
    pub metadata_indexes: Vec<String>,
}

#[async_trait]
pub trait VectorizeAction {
    async fn insert(&self, input: InsertInput) -> Result<InsertOutput, Box<dyn std::error::Error>>;
    async fn upsert(&self, input: UpsertInput) -> Result<UpsertOutput, Box<dyn std::error::Error>>;
    async fn query(&self, input: QueryInput) -> Result<QueryOutput, Box<dyn std::error::Error>>;
    async fn query_by_id(&self, input: QueryByIdInput) -> Result<QueryByIdOutput, Box<dyn std::error::Error>>;
    async fn get_by_ids(&self, input: GetByIdsInput) -> Result<GetByIdsOutput, Box<dyn std::error::Error>>;
    async fn delete_by_ids(&self, input: DeleteByIdsInput) -> Result<DeleteByIdsOutput, Box<dyn std::error::Error>>;
    async fn describe(&self, input: DescribeInput) -> Result<DescribeOutput, Box<dyn std::error::Error>>;
    async fn info(&self, input: InfoInput) -> Result<InfoOutput, Box<dyn std::error::Error>>;
    async fn create_metadata_index(&self, input: CreateMetadataIndexInput) -> Result<CreateMetadataIndexOutput, Box<dyn std::error::Error>>;
    async fn delete_metadata_index(&self, input: DeleteMetadataIndexInput) -> Result<DeleteMetadataIndexOutput, Box<dyn std::error::Error>>;
    async fn list_metadata_indexes(&self, input: ListMetadataIndexesInput) -> Result<ListMetadataIndexesOutput, Box<dyn std::error::Error>>;
}

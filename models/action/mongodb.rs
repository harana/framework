// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertInput {
    pub collection: String,
    pub database: String,
    pub document: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertOutput {
    pub inserted_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertManyInput {
    pub collection: String,
    pub database: String,
    pub documents: Vec<String>,
    #[serde(default)]
    pub ordered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InsertManyOutput {
    pub inserted_count: i64,
    pub inserted_ids: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindOneInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
    pub projection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindOneOutput {
    pub document: String,
    pub found: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
    pub limit: i64,
    pub projection: String,
    pub skip: i64,
    pub sort: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindOutput {
    pub count: i64,
    pub documents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOneInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
    pub update: String,
    #[serde(default)]
    pub upsert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOneOutput {
    pub matched_count: i64,
    pub modified_count: i64,
    pub success: bool,
    pub upserted_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateManyInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
    pub update: String,
    #[serde(default)]
    pub upsert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateManyOutput {
    pub matched_count: i64,
    pub modified_count: i64,
    pub success: bool,
    pub upserted_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplaceOneInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
    pub replacement: String,
    #[serde(default)]
    pub upsert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplaceOneOutput {
    pub matched_count: i64,
    pub modified_count: i64,
    pub success: bool,
    pub upserted_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOneInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOneOutput {
    pub deleted_count: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteManyInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteManyOutput {
    pub deleted_count: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CountInput {
    pub collection: String,
    pub database: String,
    pub filter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CountOutput {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AggregateInput {
    pub collection: String,
    pub database: String,
    pub pipeline: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AggregateOutput {
    pub documents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexInput {
    pub collection: String,
    pub database: String,
    pub keys: String,
    pub name: String,
    #[serde(default)]
    pub unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexOutput {
    pub index_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropIndexInput {
    pub collection: String,
    pub database: String,
    pub index_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCollectionsInput {
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCollectionsOutput {
    pub collections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropCollectionInput {
    pub collection: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropCollectionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkWriteInput {
    pub collection: String,
    pub database: String,
    pub operations: Vec<String>,
    #[serde(default)]
    pub ordered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkWriteOutput {
    pub deleted_count: i64,
    pub inserted_count: i64,
    pub matched_count: i64,
    pub modified_count: i64,
    pub success: bool,
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

#[async_trait]
pub trait MongodbAction {
    async fn insert(&self, input: InsertInput) -> Result<InsertOutput, Box<dyn std::error::Error>>;
    async fn insert_many(&self, input: InsertManyInput) -> Result<InsertManyOutput, Box<dyn std::error::Error>>;
    async fn find_one(&self, input: FindOneInput) -> Result<FindOneOutput, Box<dyn std::error::Error>>;
    async fn find(&self, input: FindInput) -> Result<FindOutput, Box<dyn std::error::Error>>;
    async fn update_one(&self, input: UpdateOneInput) -> Result<UpdateOneOutput, Box<dyn std::error::Error>>;
    async fn update_many(&self, input: UpdateManyInput) -> Result<UpdateManyOutput, Box<dyn std::error::Error>>;
    async fn replace_one(&self, input: ReplaceOneInput) -> Result<ReplaceOneOutput, Box<dyn std::error::Error>>;
    async fn delete_one(&self, input: DeleteOneInput) -> Result<DeleteOneOutput, Box<dyn std::error::Error>>;
    async fn delete_many(&self, input: DeleteManyInput) -> Result<DeleteManyOutput, Box<dyn std::error::Error>>;
    async fn count(&self, input: CountInput) -> Result<CountOutput, Box<dyn std::error::Error>>;
    async fn aggregate(&self, input: AggregateInput) -> Result<AggregateOutput, Box<dyn std::error::Error>>;
    async fn create_index(&self, input: CreateIndexInput) -> Result<CreateIndexOutput, Box<dyn std::error::Error>>;
    async fn drop_index(&self, input: DropIndexInput) -> Result<DropIndexOutput, Box<dyn std::error::Error>>;
    async fn list_collections(&self, input: ListCollectionsInput) -> Result<ListCollectionsOutput, Box<dyn std::error::Error>>;
    async fn drop_collection(&self, input: DropCollectionInput) -> Result<DropCollectionOutput, Box<dyn std::error::Error>>;
    async fn bulk_write(&self, input: BulkWriteInput) -> Result<BulkWriteOutput, Box<dyn std::error::Error>>;
}

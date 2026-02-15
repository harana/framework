// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQueryInput {
    pub fields: Vec<String>,
    pub filters: String,
    #[serde(default)]
    pub highlight: bool,
    pub index: String,
    pub limit: i64,
    pub offset: i64,
    pub query: String,
    pub sort: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQueryOutput {
    pub hits: Vec<String>,
    pub took_ms: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IndexDocumentInput {
    pub document: String,
    pub document_id: String,
    pub index: String,
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IndexDocumentOutput {
    pub success: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkIndexInput {
    pub documents: Vec<String>,
    pub index: String,
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkIndexOutput {
    pub errors: Vec<String>,
    pub failed: i64,
    pub indexed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDocumentInput {
    pub document_id: String,
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDocumentOutput {
    pub document: String,
    pub found: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDocumentInput {
    pub document: String,
    pub document_id: String,
    pub index: String,
    #[serde(default)]
    pub upsert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDocumentOutput {
    pub success: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDocumentInput {
    pub document_id: String,
    pub index: String,
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDocumentOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteByQueryInput {
    pub index: String,
    pub query: String,
    #[serde(default)]
    pub refresh: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteByQueryOutput {
    pub deleted: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexInput {
    pub index: String,
    pub mappings: String,
    pub settings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteIndexInput {
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIndexesInput {
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIndexesOutput {
    pub indexes: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIndexStatsInput {
    pub index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIndexStatsOutput {
    pub document_count: i64,
    pub mappings: String,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SuggestInput {
    pub field: String,
    pub index: String,
    pub size: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SuggestOutput {
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchDocument {
    pub document_id: String,
    pub index: String,
    pub document: String,
    pub score: f64,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchDocumentData {
    pub fields: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFilters {
    pub conditions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchSortField {
    pub field: String,
    pub order: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchHit {
    pub document_id: String,
    pub score: f64,
    pub source: String,
    pub highlight: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexError {
    pub document_id: String,
    pub error: String,
    pub status: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQuery {
    pub query: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchMappings {
    pub properties: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFieldMapping {
    pub type: String,
    pub analyzer: String,
    pub index: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexSettings {
    pub number_of_shards: i64,
    pub number_of_replicas: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexInfo {
    pub name: String,
    pub document_count: i64,
    pub size_bytes: i64,
    pub status: String,
}

#[async_trait]
pub trait SearchAction {
    async fn search_query(&self, input: SearchQueryInput) -> Result<SearchQueryOutput, Box<dyn std::error::Error>>;
    async fn index_document(&self, input: IndexDocumentInput) -> Result<IndexDocumentOutput, Box<dyn std::error::Error>>;
    async fn bulk_index(&self, input: BulkIndexInput) -> Result<BulkIndexOutput, Box<dyn std::error::Error>>;
    async fn get_document(&self, input: GetDocumentInput) -> Result<GetDocumentOutput, Box<dyn std::error::Error>>;
    async fn update_document(&self, input: UpdateDocumentInput) -> Result<UpdateDocumentOutput, Box<dyn std::error::Error>>;
    async fn delete_document(&self, input: DeleteDocumentInput) -> Result<DeleteDocumentOutput, Box<dyn std::error::Error>>;
    async fn delete_by_query(&self, input: DeleteByQueryInput) -> Result<DeleteByQueryOutput, Box<dyn std::error::Error>>;
    async fn create_index(&self, input: CreateIndexInput) -> Result<CreateIndexOutput, Box<dyn std::error::Error>>;
    async fn delete_index(&self, input: DeleteIndexInput) -> Result<DeleteIndexOutput, Box<dyn std::error::Error>>;
    async fn list_indexes(&self, input: ListIndexesInput) -> Result<ListIndexesOutput, Box<dyn std::error::Error>>;
    async fn get_index_stats(&self, input: GetIndexStatsInput) -> Result<GetIndexStatsOutput, Box<dyn std::error::Error>>;
    async fn suggest(&self, input: SuggestInput) -> Result<SuggestOutput, Box<dyn std::error::Error>>;
}

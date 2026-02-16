// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQueryOutput {
    pub hits: Vec<String>,
    pub took_ms: i64,
    pub total: i64,
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
pub struct GetDocumentOutput {
    pub document: String,
    pub found: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIndexesOutput {
    pub indexes: Vec<String>,
    pub total: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndex {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub settings: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchField {
    pub boost: f64,
    pub index_id: String,
    #[serde(default)]
    pub is_facet: bool,
    #[serde(default)]
    pub is_filterable: bool,
    #[serde(default)]
    pub is_searchable: bool,
    #[serde(default)]
    pub is_sortable: bool,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchSynonym {
    pub index_id: String,
    pub synonyms: String,
    pub term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQueryLog {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub index_id: String,
    pub query: String,
    pub response_time_ms: i64,
    pub results_count: i64,
    pub user_id: String,
}

#[async_trait]
pub trait SearchAction {
    async fn search_query(&self, fields: Vec<String>, filters: String, highlight: bool, index: String, limit: i64, offset: i64, query: String, sort: Vec<String>) -> Result<SearchQueryOutput, Box<dyn std::error::Error>>;
    async fn index_document(&self, document: String, document_id: String, index: String, refresh: bool, version: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn bulk_index(&self, documents: Vec<String>, index: String, refresh: bool) -> Result<BulkIndexOutput, Box<dyn std::error::Error>>;
    async fn get_document(&self, document_id: String, index: String) -> Result<GetDocumentOutput, Box<dyn std::error::Error>>;
    async fn update_document(&self, document: String, document_id: String, index: String, upsert: bool, version: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_document(&self, document_id: String, index: String, refresh: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_by_query(&self, index: String, query: String, refresh: bool) -> Result<i64, Box<dyn std::error::Error>>;
    async fn create_index(&self, index: String, mappings: String, settings: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_index(&self, index: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_indexes(&self, pattern: String) -> Result<ListIndexesOutput, Box<dyn std::error::Error>>;
    async fn get_index_stats(&self, index: String) -> Result<GetIndexStatsOutput, Box<dyn std::error::Error>>;
    async fn suggest(&self, field: String, index: String, size: i64, text: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

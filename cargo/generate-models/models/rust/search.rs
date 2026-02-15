// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// search_index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndex {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub settings: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl SearchIndex {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchField {
    pub boost: f64,
    /// Reference: search_index.id
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

impl SearchField {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_synonym
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchSynonym {
    /// Reference: search_index.id
    pub index_id: String,
    pub synonyms: String,
    pub term: String,
}

impl SearchSynonym {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_query_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQueryLog {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: search_index.id
    pub index_id: String,
    pub query: String,
    pub response_time_ms: Option<i64>,
    pub results_count: i64,
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl SearchQueryLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchDocument {
    pub document_id: String,
    pub index: String,
    pub document: String,
    pub score: f64,
    pub version: i64,
}

impl SearchDocument {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_document_data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchDocumentData {
    pub fields: String,
}

impl SearchDocumentData {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFilters {
    pub conditions: String,
}

impl SearchFilters {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_sort_field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchSortField {
    pub field: String,
    pub order: String,
}

impl SearchSortField {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_hit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchHit {
    pub document_id: String,
    pub score: f64,
    pub source: String,
    pub highlight: String,
}

impl SearchHit {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_index_error
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexError {
    pub document_id: String,
    pub error: String,
    pub status: i64,
}

impl SearchIndexError {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQuery {
    pub query: String,
}

impl SearchQuery {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchMappings {
    pub properties: String,
}

impl SearchMappings {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_field_mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFieldMapping {
    pub type: String,
    pub analyzer: String,
    pub index: bool,
}

impl SearchFieldMapping {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_index_settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexSettings {
    pub number_of_shards: i64,
    pub number_of_replicas: i64,
}

impl SearchIndexSettings {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// search_index_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexInfo {
    pub name: String,
    pub document_count: i64,
    pub size_bytes: i64,
    pub status: String,
}

impl SearchIndexInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


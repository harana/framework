// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl SearchField {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchSynonym {
    pub index_id: String,
    pub synonyms: String,
    pub term: String,
}

impl SearchSynonym {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQueryLog {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub index_id: String,
    pub query: String,
    pub response_time_ms: Option<i64>,
    pub results_count: i64,
    pub user_id: Option<String>,
}

impl SearchQueryLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl SearchDocument {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchDocumentData {
    pub fields: std::collections::HashMap<String, String>,
}

impl SearchDocumentData {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFilters {
    pub conditions: std::collections::HashMap<String, String>,
}

impl SearchFilters {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchSortField {
    pub field: String,
    pub order: String,
}

impl SearchSortField {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchHit {
    pub document_id: String,
    pub score: f64,
    pub source: String,
    pub highlight: std::collections::HashMap<String, String>,
}

impl SearchHit {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexError {
    pub document_id: String,
    pub error: String,
    pub status: i64,
}

impl SearchIndexError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchQuery {
    pub query: std::collections::HashMap<String, String>,
}

impl SearchQuery {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchMappings {
    pub properties: std::collections::HashMap<String, String>,
}

impl SearchMappings {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFieldMapping {
    pub type: String,
    pub analyzer: String,
    pub index: bool,
}

impl SearchFieldMapping {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexSettings {
    pub number_of_shards: i64,
    pub number_of_replicas: i64,
}

impl SearchIndexSettings {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchIndexInfo {
    pub name: String,
    pub document_count: i64,
    pub size_bytes: i64,
    pub status: String,
}

impl SearchIndexInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


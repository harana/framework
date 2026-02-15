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


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchInitiated {
    pub search_id: String,
    pub search_query: String,
    pub search_type: String,
    pub search_context: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub initiated_at: chrono::DateTime<chrono::Utc>,
}

impl SearchInitiated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchCleared {
    pub search_id: String,
    pub previous_query: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub cleared_at: chrono::DateTime<chrono::Utc>,
}

impl SearchCleared {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchResultsShown {
    pub search_id: String,
    pub search_query: String,
    pub results_count: i64,
    pub search_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub shown_at: chrono::DateTime<chrono::Utc>,
}

impl SearchResultsShown {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchResultSelected {
    pub search_id: String,
    pub result_id: String,
    pub result_index: Option<i64>,
    pub result_type: Option<String>,
    pub search_query: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl SearchResultSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchEmptyState {
    pub search_id: String,
    pub search_query: String,
    #[serde(default)]
    pub suggestions_shown: bool,
    #[serde(default = "chrono::Utc::now")]
    pub shown_at: chrono::DateTime<chrono::Utc>,
}

impl SearchEmptyState {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


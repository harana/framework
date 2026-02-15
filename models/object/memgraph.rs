// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphNode {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub labels: Option<String>,
    pub node_id: String,
    pub properties: Option<String>,
}

impl MemgraphNode {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphRelationship {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub from_node_id: String,
    pub properties: Option<String>,
    pub relationship_id: String,
    pub to_node_id: String,
    pub type: String,
}

impl MemgraphRelationship {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphIndex {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    pub property: String,
}

impl MemgraphIndex {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphQueryLog {
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub nodes_affected: i64,
    pub query: String,
    pub relationships_affected: i64,
    pub status: String,
    pub user_id: Option<String>,
}

impl MemgraphQueryLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphNode {
    pub node_id: String,
    pub labels: Vec<String>,
    pub properties: std::collections::HashMap<String, String>,
}

impl GraphNode {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuerySummary {
    pub nodes_created: i64,
    pub nodes_deleted: i64,
    pub relationships_created: i64,
    pub relationships_deleted: i64,
    pub properties_set: i64,
}

impl QuerySummary {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphPathElement {
    pub node_id: String,
    pub relationship_id: String,
    pub relationship_type: String,
}

impl GraphPathElement {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphPath {
    pub elements: Vec<String>,
    pub length: i64,
}

impl GraphPath {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphStats {
    pub labels: std::collections::HashMap<String, String>,
    pub relationship_types: std::collections::HashMap<String, String>,
    pub indexes: Vec<String>,
}

impl GraphStats {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlgorithmResult {
    pub node_id: String,
    pub score: f64,
}

impl AlgorithmResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


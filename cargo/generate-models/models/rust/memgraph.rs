// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// memgraph_node
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// memgraph_relationship
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// memgraph_index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphIndex {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    pub property: String,
}

impl MemgraphIndex {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// memgraph_query_log
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
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl MemgraphQueryLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// graph_node
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphNode {
    pub node_id: String,
    pub labels: Vec<String>,
    pub properties: String,
}

impl GraphNode {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// query_summary
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// graph_path_element
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphPathElement {
    pub node_id: String,
    pub relationship_id: String,
    pub relationship_type: String,
}

impl GraphPathElement {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// graph_path
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphPath {
    pub elements: Vec<String>,
    pub length: i64,
}

impl GraphPath {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// graph_stats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphStats {
    pub labels: String,
    pub relationship_types: String,
    pub indexes: Vec<String>,
}

impl GraphStats {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// algorithm_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlgorithmResult {
    pub node_id: String,
    pub score: f64,
}

impl AlgorithmResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindNodesOutput {
    pub count: i64,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindNodeOutput {
    pub found: bool,
    pub node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteQueryOutput {
    pub records: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortestPathOutput {
    pub found: bool,
    pub length: i64,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindPathsOutput {
    pub count: i64,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatsOutput {
    pub node_count: i64,
    pub relationship_count: i64,
    pub stats: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MatchPatternOutput {
    pub count: i64,
    pub results: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphNode {
    pub node_id: String,
    pub labels: Vec<String>,
    pub properties: std::collections::HashMap<String, String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphPathElement {
    pub node_id: String,
    pub relationship_id: String,
    pub relationship_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphPath {
    pub elements: Vec<String>,
    pub length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphStats {
    pub labels: std::collections::HashMap<String, String>,
    pub relationship_types: std::collections::HashMap<String, String>,
    pub indexes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AlgorithmResult {
    pub node_id: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphNode {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub labels: String,
    pub node_id: String,
    pub properties: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphRelationship {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub from_node_id: String,
    pub properties: String,
    pub relationship_id: String,
    pub to_node_id: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphIndex {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MemgraphQueryLog {
    pub duration_ms: i64,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub nodes_affected: i64,
    pub query: String,
    pub relationships_affected: i64,
    pub status: String,
    pub user_id: String,
}

#[async_trait]
pub trait MemgraphAction {
    async fn create_node(&self, labels: Vec<String>, properties: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_relationship(&self, from_node_id: String, properties: String, to_node_id: String, type: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn find_nodes(&self, labels: Vec<String>, limit: i64, properties: String, skip: i64) -> Result<FindNodesOutput, Box<dyn std::error::Error>>;
    async fn find_node(&self, node_id: String) -> Result<FindNodeOutput, Box<dyn std::error::Error>>;
    async fn update_node(&self, node_id: String, properties: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_node(&self, detach: bool, node_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_relationship(&self, relationship_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn execute_query(&self, parameters: std::collections::HashMap<String, String>, query: String) -> Result<ExecuteQueryOutput, Box<dyn std::error::Error>>;
    async fn shortest_path(&self, from_node_id: String, max_depth: i64, relationship_types: Vec<String>, to_node_id: String) -> Result<ShortestPathOutput, Box<dyn std::error::Error>>;
    async fn find_paths(&self, from_node_id: String, limit: i64, max_depth: i64, relationship_types: Vec<String>, to_node_id: String) -> Result<FindPathsOutput, Box<dyn std::error::Error>>;
    async fn get_neighbors(&self, depth: i64, direction: String, node_id: String, relationship_types: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn get_degree(&self, direction: String, node_id: String, relationship_types: Vec<String>) -> Result<i64, Box<dyn std::error::Error>>;
    async fn create_index(&self, label: String, property: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn drop_index(&self, label: String, property: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_stats(&self) -> Result<GetStatsOutput, Box<dyn std::error::Error>>;
    async fn run_algorithm(&self, algorithm: String, parameters: std::collections::HashMap<String, String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn match_pattern(&self, limit: i64, parameters: std::collections::HashMap<String, String>, pattern: String, where_clause: String) -> Result<MatchPatternOutput, Box<dyn std::error::Error>>;
}

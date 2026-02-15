// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNodeInput {
    pub labels: Vec<String>,
    pub properties: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNodeOutput {
    pub node_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRelationshipInput {
    pub from_node_id: String,
    pub properties: String,
    pub to_node_id: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRelationshipOutput {
    pub relationship_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindNodesInput {
    pub labels: Vec<String>,
    pub limit: i64,
    pub properties: String,
    pub skip: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindNodesOutput {
    pub count: i64,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindNodeInput {
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindNodeOutput {
    pub found: bool,
    pub node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateNodeInput {
    pub node_id: String,
    pub properties: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateNodeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNodeInput {
    #[serde(default)]
    pub detach: bool,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNodeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRelationshipInput {
    pub relationship_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRelationshipOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteQueryInput {
    pub parameters: std::collections::HashMap<String, String>,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteQueryOutput {
    pub records: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortestPathInput {
    pub from_node_id: String,
    pub max_depth: i64,
    pub relationship_types: Vec<String>,
    pub to_node_id: String,
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
pub struct FindPathsInput {
    pub from_node_id: String,
    pub limit: i64,
    pub max_depth: i64,
    pub relationship_types: Vec<String>,
    pub to_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindPathsOutput {
    pub count: i64,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetNeighborsInput {
    pub depth: i64,
    pub direction: String,
    pub node_id: String,
    pub relationship_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetNeighborsOutput {
    pub neighbors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDegreeInput {
    pub direction: String,
    pub node_id: String,
    pub relationship_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDegreeOutput {
    pub degree: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexInput {
    pub label: String,
    pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIndexOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropIndexInput {
    pub label: String,
    pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropIndexOutput {
    pub success: bool,
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
pub struct RunAlgorithmInput {
    pub algorithm: String,
    pub parameters: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunAlgorithmOutput {
    pub results: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MatchPatternInput {
    pub limit: i64,
    pub parameters: std::collections::HashMap<String, String>,
    pub pattern: String,
    pub where_clause: String,
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

#[async_trait]
pub trait MemgraphAction {
    async fn create_node(&self, input: CreateNodeInput) -> Result<CreateNodeOutput, Box<dyn std::error::Error>>;
    async fn create_relationship(&self, input: CreateRelationshipInput) -> Result<CreateRelationshipOutput, Box<dyn std::error::Error>>;
    async fn find_nodes(&self, input: FindNodesInput) -> Result<FindNodesOutput, Box<dyn std::error::Error>>;
    async fn find_node(&self, input: FindNodeInput) -> Result<FindNodeOutput, Box<dyn std::error::Error>>;
    async fn update_node(&self, input: UpdateNodeInput) -> Result<UpdateNodeOutput, Box<dyn std::error::Error>>;
    async fn delete_node(&self, input: DeleteNodeInput) -> Result<DeleteNodeOutput, Box<dyn std::error::Error>>;
    async fn delete_relationship(&self, input: DeleteRelationshipInput) -> Result<DeleteRelationshipOutput, Box<dyn std::error::Error>>;
    async fn execute_query(&self, input: ExecuteQueryInput) -> Result<ExecuteQueryOutput, Box<dyn std::error::Error>>;
    async fn shortest_path(&self, input: ShortestPathInput) -> Result<ShortestPathOutput, Box<dyn std::error::Error>>;
    async fn find_paths(&self, input: FindPathsInput) -> Result<FindPathsOutput, Box<dyn std::error::Error>>;
    async fn get_neighbors(&self, input: GetNeighborsInput) -> Result<GetNeighborsOutput, Box<dyn std::error::Error>>;
    async fn get_degree(&self, input: GetDegreeInput) -> Result<GetDegreeOutput, Box<dyn std::error::Error>>;
    async fn create_index(&self, input: CreateIndexInput) -> Result<CreateIndexOutput, Box<dyn std::error::Error>>;
    async fn drop_index(&self, input: DropIndexInput) -> Result<DropIndexOutput, Box<dyn std::error::Error>>;
    async fn get_stats(&self) -> Result<GetStatsOutput, Box<dyn std::error::Error>>;
    async fn run_algorithm(&self, input: RunAlgorithmInput) -> Result<RunAlgorithmOutput, Box<dyn std::error::Error>>;
    async fn match_pattern(&self, input: MatchPatternInput) -> Result<MatchPatternOutput, Box<dyn std::error::Error>>;
}

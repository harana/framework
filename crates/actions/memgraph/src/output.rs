// Harana Actions - Memgraph Module Output Types
// Auto-generated output structs for Memgraph action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNodeOutput {
    pub node_id: String,
    pub success: bool,
}

// create_relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRelationshipOutput {
    pub relationship_id: String,
    pub success: bool,
}

// execute_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteQueryOutput {
    pub records: Vec<HashMap<String, Value>>,
    pub summary: HashMap<String, Value>,
}


// find_nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindNodesOutput {
    pub count: i32,
    pub nodes: Vec<HashMap<String, Value>>
}

// find_node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindNodeOutput {
    pub found: bool,
    pub node: HashMap<String, Value>
}

// update_node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNodeOutput {
    pub success: bool
}

// delete_node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNodeOutput {
    pub success: bool
}

// delete_relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRelationshipOutput {
    pub success: bool
}

// shortest_path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortestPathOutput {
    pub length: i32,
    pub path: Vec<HashMap<String, Value>>,
    pub found: bool
}

// find_paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindPathsOutput {
    pub count: i32,
    pub paths: Vec<HashMap<String, Value>>
}

// get_neighbors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNeighborsOutput {
    pub neighbors: Vec<HashMap<String, Value>>
}

// get_degree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDegreeOutput {
    pub degree: i32
}

// create_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndexOutput {
    pub success: bool
}

// drop_index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropIndexOutput {
    pub success: bool
}

// get_stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatsOutput {
    pub node_count: i32,
    pub relationship_count: i32,
    pub stats: HashMap<String, Value>
}

// run_algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAlgorithmOutput {
    pub results: Vec<HashMap<String, Value>>,
    pub success: bool
}

// match_pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPatternOutput {
    pub results: Vec<HashMap<String, Value>>,
    pub count: i32
}
// TODO: Add remaining output types - see core/schema/actions/memgraph.yml

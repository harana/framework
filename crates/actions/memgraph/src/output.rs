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

// TODO: Add remaining output types - see core/schema/actions/memgraph.yml

// Harana Actions - Json Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffOutput {
    pub removed: HashMap<String, Value>,
    pub changed: HashMap<String, Value>,
    pub added: HashMap<String, Value>
}

// jmespath_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JmespathQueryOutput {
    pub result: String
}

// merge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeOutput {
    pub result: HashMap<String, Value>
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub result: String
}

// stringify
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringifyOutput {
    pub json: String
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub valid: bool,
    pub errors: Vec<HashMap<String, Value>>
}

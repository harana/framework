// Harana Actions - Xml Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOutput {
    pub xml: String
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub result: HashMap<String, Value>
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub valid: bool
}

// xpath_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XpathQueryOutput {
    pub results: Vec<Value>
}

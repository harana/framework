// Harana Actions - Address Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// autocomplete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteOutput {
    pub suggestions: Vec<HashMap<String, Value>>
}

// normalize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizeOutput {
    pub address: HashMap<String, Value>
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub state: String
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub valid: bool
}

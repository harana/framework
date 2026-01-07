// Harana Actions - Csv Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOutput {
    pub csv: String
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub rows: Vec<HashMap<String, Value>>
}

// transform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformOutput {
    pub csv: String
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub valid: bool
}

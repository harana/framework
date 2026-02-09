use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteOutput {
    pub suggestions: Vec<HashMap<String, Value>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizeOutput {
    pub address: HashMap<String, Value>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub city: String,
    pub street: String,
    pub postal_code: String,
    pub country: String,
    pub state: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub valid: bool,
    pub errors: Vec<HashMap<String, Value>>
}

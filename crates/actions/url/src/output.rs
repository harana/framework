// Harana Actions - Url Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildOutput {
    pub url: String
}

// decode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeOutput {
    pub decoded: String
}

// encode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodeOutput {
    pub encoded: String
}

// expand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandOutput {
    pub original_url: String
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub protocol: String,
    pub query: HashMap<String, Value>,
    pub fragment: String,
    pub path: String,
    pub host: String,
    pub port: i32
}

// shorten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenOutput {
    pub short_url: String
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool
}

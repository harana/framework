// Harana Actions - Html Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// css_select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssSelectOutput {
    pub results: Vec<Value>
}

// extract_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractTextOutput {
    pub text: String
}

// minify
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinifyOutput {
    pub html: String
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub result: HashMap<String, Value>
}

// sanitize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeOutput {
    pub html: String
}

// Harana Actions - Markdown Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// to_html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToHtmlOutput {
    pub html: String,
}

// from_html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromHtmlOutput {
    pub markdown: String,
}

// extract_frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractFrontmatterOutput {
    pub content: String,
    pub frontmatter: HashMap<String, Value>,
}

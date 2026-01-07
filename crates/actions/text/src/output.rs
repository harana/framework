// Harana Actions - Text Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// case_convert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseConvertOutput {
    pub result: String
}

// join
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinOutput {
    pub result: String
}

// regex_match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexMatchOutput {
    pub matches: Vec<String>
}

// regex_replace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexReplaceOutput {
    pub result: String
}

// slugify
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlugifyOutput {
    pub slug: String
}

// split
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitOutput {
    pub parts: Vec<String>
}

// template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateOutput {
    pub result: String
}

// trim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrimOutput {
    pub result: String
}

// truncate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruncateOutput {
    pub result: String
}

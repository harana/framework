// Harana Actions - Xml Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOutput {
    pub xml: String,
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub result: Value,
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub errors: Vec<XmlValidationError>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlValidationError {
    pub line: i32,
    pub column: i32,
    pub message: String,
}

// xpath_query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XpathQueryOutput {
    pub results: Vec<Value>,
}

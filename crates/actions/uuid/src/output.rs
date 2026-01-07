// Harana Actions - Uuid Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// generate_v4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateV4Output {
    pub uuid: String
}

// generate_v7
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateV7Output {
    pub uuid: String
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub variant: String,
    pub timestamp: i32,
    pub version: i32
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub version: i32,
    pub valid: bool
}

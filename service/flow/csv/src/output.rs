// Harana Actions - Csv Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOutput {
    pub csv: String,
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub rows: Vec<HashMap<String, String>>,
}

// transform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformOutput {
    pub csv: String,
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub errors: Vec<CsvValidationError>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvValidationError {
    pub row: i32,
    pub column: String,
    pub message: String,
}


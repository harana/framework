// Harana Actions - Csv Module
// This module provides csv actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Generate CSV From Data
pub async fn generate(
    data: Vec<HashMap<String, Value>>,
    delimiter: Option<&str>,
    headers: Option<Vec<String>>,
    include_headers: Option<bool>,
) -> Result<GenerateOutput, String> {
    unimplemented!("generate")
}

/// Parse CSV To Data
pub async fn parse(
    data: &str,
    skip_empty_lines: Option<bool>,
    delimiter: Option<&str>,
    headers: Option<Vec<String>>,
    has_headers: Option<bool>,
) -> Result<ParseOutput, String> {
    unimplemented!("parse")
}

/// Transform CSV Columns And Rows
pub async fn transform(
    data: &str,
    operations: Vec<HashMap<String, Value>>,
    delimiter: Option<&str>,
) -> Result<TransformOutput, String> {
    unimplemented!("transform")
}

/// Validate CSV Against Schema
pub async fn validate(
    data: &str,
    schema: HashMap<String, Value>,
    delimiter: Option<&str>,
) -> Result<ValidateOutput, String> {
    unimplemented!("validate")
}

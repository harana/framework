pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Get Address Autocomplete Suggestions
pub async fn autocomplete(
    query: &str,
    country: Option<&str>,
    types: Option<Vec<String>>,
    limit: Option<i32>,
) -> Result<AutocompleteOutput, String> {
    unimplemented!("autocomplete")
}

/// Normalize Address Format
pub async fn normalize(
    address: HashMap<String, Value>,
    format: Option<&str>,
    country: Option<&str>,
) -> Result<NormalizeOutput, String> {
    unimplemented!("normalize")
}

/// Parse Address String To Components
pub async fn parse(
    address: &str,
    country: Option<&str>,
) -> Result<ParseOutput, String> {
    unimplemented!("parse")
}

/// Validate Postal Address
pub async fn validate(
    address: HashMap<String, Value>,
    country: Option<&str>,
) -> Result<ValidateOutput, String> {
    unimplemented!("validate")
}

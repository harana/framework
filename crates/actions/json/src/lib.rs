// Harana Actions - Json Module
// This module provides json actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Compare Two JSON Objects
pub async fn diff(
    target: &str,
    source: &str,
    include_unchanged: Option<bool>,
) -> Result<DiffOutput, String> {
    unimplemented!("diff")
}

/// Query JSON With JMESPath
pub async fn jmespath_query(
    query: &str,
    data: &str,
) -> Result<JmespathQueryOutput, String> {
    unimplemented!("jmespath_query")
}

/// Deep Merge JSON Objects
pub async fn merge(
    objects: Vec<HashMap<String, Value>>,
    strategy: Option<&str>,
) -> Result<MergeOutput, String> {
    unimplemented!("merge")
}

/// Parse JSON String To Object
pub async fn parse(
    data: &str,
    strict: Option<bool>,
) -> Result<ParseOutput, String> {
    unimplemented!("parse")
}

/// Convert Object To JSON String
pub async fn stringify(
    data: &str,
    indent: Option<i32>,
    sort_keys: Option<bool>,
) -> Result<StringifyOutput, String> {
    unimplemented!("stringify")
}

/// Validate JSON Against Schema
pub async fn validate(
    data: &str,
    schema: HashMap<String, Value>,
) -> Result<ValidateOutput, String> {
    unimplemented!("validate")
}

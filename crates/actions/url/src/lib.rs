// Harana Actions - Url Module
// This module provides url actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Build URL From Components
pub async fn build(
    host: &str,
    query: Option<HashMap<String, Value>>,
    path: Option<&str>,
    protocol: Option<&str>,
    fragment: Option<&str>,
    port: Option<i32>,
) -> Result<BuildOutput, String> {
    unimplemented!("build")
}

/// URL Decode String
pub async fn decode(
    data: &str,
    component: Option<bool>,
) -> Result<DecodeOutput, String> {
    unimplemented!("decode")
}

/// URL Encode String
pub async fn encode(
    data: &str,
    component: Option<bool>,
) -> Result<EncodeOutput, String> {
    unimplemented!("encode")
}

/// Expand Short URL To Original
pub async fn expand(
    url: &str,
) -> Result<ExpandOutput, String> {
    unimplemented!("expand")
}

/// Parse URL To Components
pub async fn parse(
    url: &str,
) -> Result<ParseOutput, String> {
    unimplemented!("parse")
}

/// Create Short URL
pub async fn shorten(
    url: &str,
    custom_alias: Option<&str>,
    expiration: Option<&str>,
) -> Result<ShortenOutput, String> {
    unimplemented!("shorten")
}

/// Validate URL Format
pub async fn validate(
    url: &str,
    allowed_protocols: Option<Vec<String>>,
    require_tld: Option<bool>,
) -> Result<ValidateOutput, String> {
    unimplemented!("validate")
}

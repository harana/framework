// Harana Actions - Html Module
// This module provides html actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Query HTML With CSS Selectors
pub async fn css_select(
    data: &str,
    selector: &str,
    attribute: Option<&str>,
) -> Result<CssSelectOutput, String> {
    unimplemented!("css_select")
}

/// Extract Text From HTML
pub async fn extract_text(
    data: &str,
    preserve_whitespace: Option<bool>,
    separator: Option<&str>,
) -> Result<ExtractTextOutput, String> {
    unimplemented!("extract_text")
}

/// Minify HTML Content
pub async fn minify(
    data: &str,
    minify_css: Option<bool>,
    minify_js: Option<bool>,
    remove_comments: Option<bool>,
    collapse_whitespace: Option<bool>,
) -> Result<MinifyOutput, String> {
    unimplemented!("minify")
}

/// Parse HTML To DOM
pub async fn parse(
    data: &str,
    fragment: Option<bool>,
) -> Result<ParseOutput, String> {
    unimplemented!("parse")
}

/// Sanitize HTML Content
pub async fn sanitize(
    data: &str,
    allowed_tags: Option<Vec<String>>,
    strip_comments: Option<bool>,
    allowed_attributes: Option<HashMap<String, Value>>,
) -> Result<SanitizeOutput, String> {
    unimplemented!("sanitize")
}

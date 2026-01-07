// Harana Actions - Markdown Module
// This module provides markdown actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Extract YAML Frontmatter
pub async fn extract_frontmatter(
    data: &str,
) -> Result<ExtractFrontmatterOutput, String> {
    unimplemented!("extract_frontmatter")
}

/// Convert HTML To Markdown
pub async fn from_html(
    data: &str,
    bullet_style: Option<&str>,
    heading_style: Option<&str>,
    code_block_style: Option<&str>,
) -> Result<FromHtmlOutput, String> {
    unimplemented!("from_html")
}

/// Convert Markdown To HTML
pub async fn to_html(
    data: &str,
    sanitize: Option<bool>,
    highlight_code: Option<bool>,
    gfm: Option<bool>,
) -> Result<ToHtmlOutput, String> {
    unimplemented!("to_html")
}

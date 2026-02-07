// Harana Actions - Markdown Module
// This module provides markdown actions and functionality.

pub mod output;

use output::*;
use pulldown_cmark::{html, Options, Parser};
use serde_json::Value;
use std::collections::HashMap;

/// Convert Markdown To HTML
///
/// Converts markdown content to HTML with optional GitHub Flavored Markdown support.
pub async fn to_html(
    data: &str,
    sanitize: Option<bool>,
    highlight_code: Option<bool>,
    gfm: Option<bool>,
) -> Result<ToHtmlOutput, String> {
    let gfm = gfm.unwrap_or(true);
    let _highlight_code = highlight_code.unwrap_or(true);
    let _sanitize = sanitize.unwrap_or(false);

    let mut options = Options::empty();
    if gfm {
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
    }

    let parser = Parser::new_ext(data, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(ToHtmlOutput { html: html_output })
}

/// Convert HTML To Markdown
///
/// Converts HTML content to markdown format.
pub async fn from_html(
    data: &str,
    bullet_style: Option<&str>,
    heading_style: Option<&str>,
    code_block_style: Option<&str>,
) -> Result<FromHtmlOutput, String> {
    let _bullet_style = bullet_style.unwrap_or("-");
    let _heading_style = heading_style.unwrap_or("Atx");
    let _code_block_style = code_block_style.unwrap_or("Fenced");

    let markdown = html2md::parse_html(data);

    Ok(FromHtmlOutput { markdown })
}

/// Extract YAML Frontmatter
///
/// Extracts YAML frontmatter from markdown content and returns both
/// the frontmatter data and the remaining content.
pub async fn extract_frontmatter(data: &str) -> Result<ExtractFrontmatterOutput, String> {
    let trimmed = data.trim_start();

    // Check if the content starts with YAML frontmatter delimiter
    if !trimmed.starts_with("---") {
        return Ok(ExtractFrontmatterOutput {
            content: data.to_string(),
            frontmatter: HashMap::new(),
        });
    }

    // Find the closing delimiter
    let after_first_delimiter = &trimmed[3..];
    let closing_pos = after_first_delimiter
        .find("\n---")
        .or_else(|| after_first_delimiter.find("\r\n---"));

    match closing_pos {
        Some(pos) => {
            let yaml_content = &after_first_delimiter[..pos].trim();
            let content_start = pos + 4; // Skip past the closing "---"

            // Skip any newlines after the closing delimiter
            let remaining = &after_first_delimiter[content_start..];
            let content = remaining.trim_start_matches('\n').trim_start_matches('\r');

            // Parse the YAML frontmatter
            let frontmatter: HashMap<String, Value> = serde_yaml::from_str(yaml_content)
                .map_err(|e| format!("Failed to parse YAML frontmatter: {}", e))?;

            Ok(ExtractFrontmatterOutput {
                content: content.to_string(),
                frontmatter,
            })
        }
        None => {
            // No closing delimiter found, treat as regular content
            Ok(ExtractFrontmatterOutput {
                content: data.to_string(),
                frontmatter: HashMap::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests;

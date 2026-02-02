// Harana Actions - Html Module
// This module provides html actions and functionality.

pub mod output;

use std::collections::{HashMap, HashSet};
use scraper::{Html, Selector, ElementRef};
use serde_json::{Value, json};
use output::*;

/// Query HTML With CSS Selectors
pub async fn css_select(
    data: &str,
    selector: &str,
    attribute: Option<&str>,
) -> Result<CssSelectOutput, String> {
    let document = Html::parse_document(data);
    let css_selector = Selector::parse(selector)
        .map_err(|e| format!("Invalid CSS selector: {:?}", e))?;
    
    let results: Vec<Value> = document.select(&css_selector)
        .map(|element| {
            if let Some(attr) = attribute {
                // Return the attribute value if specified
                json!(element.value().attr(attr).unwrap_or(""))
            } else {
                // Return the element's inner HTML
                json!({
                    "html": element.inner_html(),
                    "text": element.text().collect::<Vec<_>>().join(""),
                    "tag": element.value().name(),
                    "attributes": element.value().attrs()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<HashMap<String, String>>()
                })
            }
        })
        .collect();
    
    Ok(CssSelectOutput { results })
}

/// Extract Text From HTML
pub async fn extract_text(
    data: &str,
    preserve_whitespace: Option<bool>,
    separator: Option<&str>,
) -> Result<ExtractTextOutput, String> {
    let document = Html::parse_document(data);
    let preserve = preserve_whitespace.unwrap_or(false);
    let sep = separator.unwrap_or(" ");
    
    let text_parts: Vec<String> = document.root_element()
        .text()
        .map(|s| {
            if preserve {
                s.to_string()
            } else {
                s.split_whitespace().collect::<Vec<_>>().join(" ")
            }
        })
        .filter(|s| !s.is_empty())
        .collect();
    
    let text = text_parts.join(sep);
    
    Ok(ExtractTextOutput { text })
}

/// Minify HTML Content
pub async fn minify(
    data: &str,
    _minify_css: Option<bool>,
    _minify_js: Option<bool>,
    remove_comments: Option<bool>,
    collapse_whitespace: Option<bool>,
) -> Result<MinifyOutput, String> {
    let mut html = data.to_string();
    
    // Remove comments if requested (default: true)
    if remove_comments.unwrap_or(true) {
        // Simple comment removal using string operations
        while let Some(start) = html.find("<!--") {
            if let Some(end) = html[start..].find("-->") {
                html = format!("{}{}", &html[..start], &html[start + end + 3..]);
            } else {
                break;
            }
        }
    }
    
    // Collapse whitespace if requested (default: true)
    if collapse_whitespace.unwrap_or(true) {
        // Replace multiple whitespace with single space
        let mut result = String::with_capacity(html.len());
        let mut last_was_whitespace = false;
        let mut in_pre_tag = false;
        
        let chars: Vec<char> = html.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            // Check for <pre> tag start
            if i + 4 < chars.len() {
                let slice: String = chars[i..i+4].iter().collect();
                if slice.to_lowercase() == "<pre" {
                    in_pre_tag = true;
                }
            }
            
            // Check for </pre> tag end
            if i + 6 <= chars.len() {
                let slice: String = chars[i..i+6].iter().collect();
                if slice.to_lowercase() == "</pre>" {
                    in_pre_tag = false;
                }
            }
            
            let c = chars[i];
            
            if in_pre_tag {
                result.push(c);
                last_was_whitespace = false;
            } else if c.is_whitespace() {
                if !last_was_whitespace {
                    result.push(' ');
                    last_was_whitespace = true;
                }
            } else {
                result.push(c);
                last_was_whitespace = false;
            }
            
            i += 1;
        }
        
        html = result;
        
        // Remove whitespace between tags
        html = html.replace("> <", "><");
    }
    
    Ok(MinifyOutput { html: html.trim().to_string() })
}

/// Parse HTML To DOM
pub async fn parse(
    data: &str,
    fragment: Option<bool>,
) -> Result<ParseOutput, String> {
    let document = if fragment.unwrap_or(false) {
        Html::parse_fragment(data)
    } else {
        Html::parse_document(data)
    };
    
    fn element_to_value(element: ElementRef) -> Value {
        let tag = element.value().name().to_string();
        let attributes: HashMap<String, String> = element.value().attrs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        
        let children: Vec<Value> = element.children()
            .filter_map(|child| {
                if let Some(elem) = ElementRef::wrap(child) {
                    Some(element_to_value(elem))
                } else if let Some(text) = child.value().as_text() {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        Some(json!({ "text": trimmed }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        json!({
            "tag": tag,
            "attributes": attributes,
            "children": children
        })
    }
    
    let root = document.root_element();
    let result_value = element_to_value(root);
    
    let result: HashMap<String, Value> = match result_value {
        Value::Object(map) => map.into_iter().collect(),
        _ => {
            let mut map = HashMap::new();
            map.insert("root".to_string(), result_value);
            map
        }
    };
    
    Ok(ParseOutput { result })
}

/// Sanitize HTML Content
pub async fn sanitize(
    data: &str,
    allowed_tags: Option<Vec<String>>,
    strip_comments: Option<bool>,
    _allowed_attributes: Option<HashMap<String, Value>>,
) -> Result<SanitizeOutput, String> {
    // For custom allowed tags, we need to build a new sanitizer
    // Otherwise use defaults
    let html = if let Some(ref tags) = allowed_tags {
        // Create a custom sanitizer with specific tags
        let tag_refs: Vec<&str> = tags.iter().map(|s| s.as_str()).collect();
        let tag_set: HashSet<&str> = tag_refs.into_iter().collect();
        
        ammonia::Builder::new()
            .tags(tag_set)
            .strip_comments(strip_comments.unwrap_or(true))
            .clean(data)
            .to_string()
    } else {
        // Use default safe tags
        ammonia::Builder::default()
            .strip_comments(strip_comments.unwrap_or(true))
            .clean(data)
            .to_string()
    };
    
    Ok(SanitizeOutput { html })
}

#[cfg(test)]
mod tests;

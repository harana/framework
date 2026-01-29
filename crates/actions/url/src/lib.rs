// Harana Actions - Url Module
// This module provides url actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use output::*;
use percent_encoding::{percent_decode_str, percent_encode, AsciiSet, CONTROLS, NON_ALPHANUMERIC};
use serde_json::Value;
use url::Url;

/// Characters that need to be encoded in URL components
const COMPONENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b']')
    .add(b'\\')
    .add(b'^')
    .add(b'|');

/// Build URL From Components
/// 
/// Constructs a URL from its individual components.
pub async fn build(
    host: &str,
    query: Option<HashMap<String, Value>>,
    path: Option<&str>,
    protocol: Option<&str>,
    fragment: Option<&str>,
    port: Option<i32>,
) -> Result<BuildOutput, String> {
    let protocol = protocol.unwrap_or("https");
    let path = path.unwrap_or("");
    
    let mut url_str = format!("{}://{}", protocol, host);
    
    if let Some(p) = port {
        url_str.push_str(&format!(":{}", p));
    }
    
    if !path.is_empty() {
        if !path.starts_with('/') {
            url_str.push('/');
        }
        url_str.push_str(path);
    }
    
    if let Some(q) = query {
        if !q.is_empty() {
            let query_parts: Vec<String> = q
                .iter()
                .map(|(k, v)| {
                    let value_str = match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => v.to_string(),
                    };
                    format!("{}={}", 
                        percent_encode(k.as_bytes(), NON_ALPHANUMERIC),
                        percent_encode(value_str.as_bytes(), NON_ALPHANUMERIC))
                })
                .collect();
            url_str.push('?');
            url_str.push_str(&query_parts.join("&"));
        }
    }
    
    if let Some(f) = fragment {
        url_str.push('#');
        url_str.push_str(f);
    }
    
    Ok(BuildOutput { url: url_str })
}

/// URL Decode String
/// 
/// Decodes a URL-encoded string.
pub async fn decode(
    data: &str,
    _component: Option<bool>,
) -> Result<DecodeOutput, String> {
    let decoded = percent_decode_str(data)
        .decode_utf8()
        .map_err(|e| format!("Invalid UTF-8 in URL: {}", e))?
        .to_string();
    
    Ok(DecodeOutput { decoded })
}

/// URL Encode String
/// 
/// Encodes a string for use in a URL.
pub async fn encode(
    data: &str,
    component: Option<bool>,
) -> Result<EncodeOutput, String> {
    let encoded = if component.unwrap_or(false) {
        percent_encode(data.as_bytes(), COMPONENT_ENCODE_SET).to_string()
    } else {
        percent_encode(data.as_bytes(), NON_ALPHANUMERIC).to_string()
    };
    
    Ok(EncodeOutput { encoded })
}

/// Expand Short URL To Original
/// 
/// Note: This is a placeholder - actual implementation would require HTTP client
/// to follow redirects.
pub async fn expand(
    url: &str,
) -> Result<ExpandOutput, String> {
    // In a real implementation, this would follow redirects
    // For now, just return the input URL
    Ok(ExpandOutput {
        original_url: url.to_string(),
    })
}

/// Parse URL To Components
/// 
/// Parses a URL string into its component parts.
pub async fn parse(
    url: &str,
) -> Result<ParseOutput, String> {
    let parsed = Url::parse(url)
        .map_err(|e| format!("Invalid URL: {}", e))?;
    
    let query: HashMap<String, Value> = parsed
        .query_pairs()
        .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
        .collect();
    
    Ok(ParseOutput {
        protocol: parsed.scheme().to_string(),
        host: parsed.host_str().unwrap_or("").to_string(),
        port: parsed.port().map(|p| p as i32),
        path: parsed.path().to_string(),
        query,
        fragment: parsed.fragment().map(|f| f.to_string()),
    })
}

/// Create Short URL
/// 
/// Note: This is a placeholder - actual implementation would require
/// integration with a URL shortening service.
pub async fn shorten(
    url: &str,
    _custom_alias: Option<&str>,
    _expiration: Option<&str>,
) -> Result<ShortenOutput, String> {
    // In a real implementation, this would call a URL shortening API
    // For now, just validate the URL and return a placeholder
    let _ = Url::parse(url)
        .map_err(|e| format!("Invalid URL: {}", e))?;
    
    Ok(ShortenOutput {
        short_url: format!("https://short.url/{}", &url[url.len().saturating_sub(8)..]),
    })
}

/// Validate URL Format
/// 
/// Validates that a string is a properly formatted URL.
pub async fn validate(
    url: &str,
    allowed_protocols: Option<Vec<String>>,
    require_tld: Option<bool>,
) -> Result<ValidateOutput, String> {
    let mut errors = Vec::new();
    
    match Url::parse(url) {
        Ok(parsed) => {
            // Check protocol
            if let Some(protocols) = allowed_protocols {
                if !protocols.iter().any(|p| p == parsed.scheme()) {
                    errors.push(format!(
                        "Protocol '{}' not in allowed list: {:?}",
                        parsed.scheme(),
                        protocols
                    ));
                }
            }
            
            // Check TLD requirement
            if require_tld.unwrap_or(false) {
                if let Some(host) = parsed.host_str() {
                    if !host.contains('.') || host == "localhost" {
                        errors.push("URL must have a TLD".to_string());
                    }
                }
            }
        }
        Err(e) => {
            errors.push(format!("Invalid URL: {}", e));
        }
    }
    
    Ok(ValidateOutput {
        valid: errors.is_empty(),
        errors,
    })
}

#[cfg(test)]
mod tests;

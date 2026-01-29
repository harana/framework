// Harana Actions - Text Module
// This module provides text actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase};
use output::*;
use regex::{Regex, RegexBuilder};
use serde_json::Value;

/// Convert String Case
/// 
/// Converts a string to the specified case format.
/// Supported formats: snake_case, camelCase, PascalCase, kebab-case, SCREAMING_SNAKE_CASE, Title Case
pub async fn case_convert(
    text: &str,
    format: &str,
) -> Result<CaseConvertOutput, String> {
    let result = match format.to_lowercase().as_str() {
        "snake" | "snake_case" => text.to_snake_case(),
        "camel" | "camelcase" => text.to_lower_camel_case(),
        "pascal" | "pascalcase" => text.to_pascal_case(),
        "kebab" | "kebab-case" => text.to_kebab_case(),
        "screaming" | "screaming_snake_case" | "constant" => text.to_shouty_snake_case(),
        "title" | "title_case" => text.to_title_case(),
        "lower" | "lowercase" => text.to_lowercase(),
        "upper" | "uppercase" => text.to_uppercase(),
        _ => return Err(format!("Unsupported case format: {}", format)),
    };
    
    Ok(CaseConvertOutput { result })
}

/// Join Strings Together
/// 
/// Joins a list of strings with an optional separator.
pub async fn join(
    items: Vec<String>,
    separator: Option<&str>,
) -> Result<JoinOutput, String> {
    let sep = separator.unwrap_or("");
    let result = items.join(sep);
    
    Ok(JoinOutput { result })
}

/// Match Regex Pattern
/// 
/// Finds all matches of a regex pattern in the text.
pub async fn regex_match(
    pattern: &str,
    text: &str,
    flags: Option<&str>,
) -> Result<RegexMatchOutput, String> {
    let flags_str = flags.unwrap_or("");
    let case_insensitive = flags_str.contains('i');
    let multiline = flags_str.contains('m');
    
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .multi_line(multiline)
        .build()
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
    
    let matches: Vec<String> = regex
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect();
    
    let matched = !matches.is_empty();
    
    Ok(RegexMatchOutput { matches, matched })
}

/// Replace With Regex Pattern
/// 
/// Replaces matches of a regex pattern in the text with a replacement string.
pub async fn regex_replace(
    replacement: &str,
    text: &str,
    pattern: &str,
    flags: Option<&str>,
) -> Result<RegexReplaceOutput, String> {
    let flags_str = flags.unwrap_or("");
    let case_insensitive = flags_str.contains('i');
    let multiline = flags_str.contains('m');
    let global = flags_str.contains('g');
    
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .multi_line(multiline)
        .build()
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
    
    let result = if global {
        regex.replace_all(text, replacement).to_string()
    } else {
        regex.replace(text, replacement).to_string()
    };
    
    Ok(RegexReplaceOutput { result })
}

/// Convert Text To URL Slug
/// 
/// Converts text to a URL-friendly slug format.
pub async fn slugify(
    text: &str,
    lowercase: Option<bool>,
    separator: Option<&str>,
) -> Result<SlugifyOutput, String> {
    let sep = separator.unwrap_or("-");
    let lowercase = lowercase.unwrap_or(true);
    
    // Replace non-alphanumeric characters with separator
    let slug: String = text
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '_' || c == '-' {
                sep.chars().next().unwrap_or('-')
            } else {
                sep.chars().next().unwrap_or('-')
            }
        })
        .collect();
    
    // Remove consecutive separators
    let sep_char = sep.chars().next().unwrap_or('-');
    let mut prev_was_sep = false;
    let slug: String = slug
        .chars()
        .filter(|&c| {
            if c == sep_char {
                if prev_was_sep {
                    return false;
                }
                prev_was_sep = true;
            } else {
                prev_was_sep = false;
            }
            true
        })
        .collect();
    
    // Trim separators from start and end
    let slug = slug.trim_matches(sep_char).to_string();
    
    let slug = if lowercase {
        slug.to_lowercase()
    } else {
        slug
    };
    
    Ok(SlugifyOutput { slug })
}

/// Split String By Delimiter
/// 
/// Splits a string by the specified delimiter.
pub async fn split(
    delimiter: &str,
    text: &str,
    limit: Option<i32>,
) -> Result<SplitOutput, String> {
    let parts: Vec<String> = if let Some(limit) = limit {
        text.splitn(limit as usize, delimiter)
            .map(|s| s.to_string())
            .collect()
    } else {
        text.split(delimiter).map(|s| s.to_string()).collect()
    };
    
    Ok(SplitOutput { parts })
}

/// Render Template String
/// 
/// Renders a template string with the provided data using Jinja2 syntax.
pub async fn template(
    data: HashMap<String, Value>,
    template: &str,
    _engine: Option<&str>,
) -> Result<TemplateOutput, String> {
    let mut env = minijinja::Environment::new();
    env.add_template("template", template)
        .map_err(|e| format!("Template parse error: {}", e))?;
    
    let tmpl = env.get_template("template")
        .map_err(|e| format!("Template error: {}", e))?;
    
    let result = tmpl.render(&data)
        .map_err(|e| format!("Template render error: {}", e))?;
    
    Ok(TemplateOutput { result })
}

/// Trim Whitespace From String
/// 
/// Trims characters from the string. Default trims whitespace.
pub async fn trim(
    text: &str,
    characters: Option<&str>,
    mode: Option<&str>,
) -> Result<TrimOutput, String> {
    let result = match (characters, mode.unwrap_or("both")) {
        (None, "start" | "left") => text.trim_start().to_string(),
        (None, "end" | "right") => text.trim_end().to_string(),
        (None, _) => text.trim().to_string(),
        (Some(chars), mode) => {
            let chars: Vec<char> = chars.chars().collect();
            let char_slice: &[char] = &chars;
            match mode {
                "start" | "left" => text.trim_start_matches(char_slice).to_string(),
                "end" | "right" => text.trim_end_matches(char_slice).to_string(),
                _ => text.trim_matches(char_slice).to_string(),
            }
        }
    };
    
    Ok(TrimOutput { result })
}

/// Truncate String Length
/// 
/// Truncates a string to the specified length, optionally adding a suffix.
pub async fn truncate(
    text: &str,
    length: i32,
    suffix: Option<&str>,
) -> Result<TruncateOutput, String> {
    if length <= 0 {
        return Err("Length must be positive".to_string());
    }
    
    let suffix = suffix.unwrap_or("");
    let length = length as usize;
    
    let result = if text.len() <= length {
        text.to_string()
    } else {
        let truncate_at = length.saturating_sub(suffix.len());
        let mut truncated: String = text.chars().take(truncate_at).collect();
        truncated.push_str(suffix);
        truncated
    };
    
    Ok(TruncateOutput { result, truncated: text.len() > length })
}

#[cfg(test)]
mod tests;

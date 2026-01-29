pub mod output;

use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use serde_json::Value;
use regex::Regex;
use output::*;

pub async fn credit_card(
    number: &str,
    luhn: Option<bool>,
) -> Result<CreditCardOutput, String> {
    let cleaned: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
    
    if cleaned.is_empty() {
        return Ok(CreditCardOutput {
            valid: false,
            card_type: "Unknown".to_string(),
            last_four: String::new(),
        });
    }
    
    let card_type = detect_card_type(&cleaned);
    let last_four = if cleaned.len() >= 4 {
        cleaned[cleaned.len() - 4..].to_string()
    } else {
        cleaned.clone()
    };
    
    let use_luhn = luhn.unwrap_or(true);
    let valid = if use_luhn {
        luhn_check(&cleaned)
    } else {
        cleaned.len() >= 13 && cleaned.len() <= 19
    };
    
    Ok(CreditCardOutput { valid, card_type, last_four })
}

fn detect_card_type(number: &str) -> String {
    if number.starts_with('4') {
        "Visa".to_string()
    } else if number.starts_with("51") || number.starts_with("52") || 
              number.starts_with("53") || number.starts_with("54") || 
              number.starts_with("55") {
        "MasterCard".to_string()
    } else if number.starts_with("34") || number.starts_with("37") {
        "American Express".to_string()
    } else if number.starts_with("6011") || number.starts_with("65") {
        "Discover".to_string()
    } else if number.starts_with("35") {
        "JCB".to_string()
    } else if number.starts_with("30") || number.starts_with("36") || number.starts_with("38") {
        "Diners Club".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn luhn_check(number: &str) -> bool {
    let digits: Vec<u32> = number.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.is_empty() { return false; }
    
    let sum: u32 = digits.iter().rev().enumerate().map(|(i, &d)| {
        if i % 2 == 1 {
            let doubled = d * 2;
            if doubled > 9 { doubled - 9 } else { doubled }
        } else { d }
    }).sum();
    
    sum % 10 == 0
}

pub async fn date(
    date_str: &str,
    format: Option<&str>,
) -> Result<DateOutput, String> {
    let fmt = format.unwrap_or("%Y-%m-%d");
    
    match chrono::NaiveDate::parse_from_str(date_str, fmt) {
        Ok(parsed) => Ok(DateOutput {
            valid: true,
            parsed: parsed.format(fmt).to_string(),
            reason: String::new(),
        }),
        Err(_) => match chrono::NaiveDateTime::parse_from_str(date_str, fmt) {
            Ok(parsed) => Ok(DateOutput {
                valid: true,
                parsed: parsed.format(fmt).to_string(),
                reason: String::new(),
            }),
            Err(e) => Ok(DateOutput {
                valid: false,
                parsed: String::new(),
                reason: format!("Invalid date format: {}", e),
            }),
        }
    }
}

pub async fn email_format(email: &str) -> Result<EmailFormatOutput, String> {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).map_err(|e| format!("Regex error: {}", e))?;
    
    if email.is_empty() {
        return Ok(EmailFormatOutput { valid: false, reason: "Email cannot be empty".to_string() });
    }
    
    if email.len() > 254 {
        return Ok(EmailFormatOutput { valid: false, reason: "Email exceeds maximum length".to_string() });
    }
    
    if email_regex.is_match(email) {
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Ok(EmailFormatOutput { valid: false, reason: "Invalid email format".to_string() });
        }
        if parts[0].len() > 64 {
            return Ok(EmailFormatOutput { valid: false, reason: "Local part exceeds 64 characters".to_string() });
        }
        if !parts[1].contains('.') {
            return Ok(EmailFormatOutput { valid: false, reason: "Domain must contain a dot".to_string() });
        }
        Ok(EmailFormatOutput { valid: true, reason: String::new() })
    } else {
        Ok(EmailFormatOutput { valid: false, reason: "Invalid email format".to_string() })
    }
}

pub async fn field(
    rules: Vec<String>,
    value: &str,
    field_name: Option<&str>,
) -> Result<FieldOutput, String> {
    let name = field_name.unwrap_or("field");
    let mut errors = Vec::new();
    
    for rule in rules {
        let parts: Vec<&str> = rule.splitn(2, ':').collect();
        let rule_name = parts[0];
        let rule_value = parts.get(1).copied();
        
        match rule_name {
            "required" => {
                if value.is_empty() { errors.push(format!("{} is required", name)); }
            }
            "min" => {
                if let Some(min_str) = rule_value {
                    if let Ok(min) = min_str.parse::<usize>() {
                        if value.len() < min { errors.push(format!("{} must be at least {} characters", name, min)); }
                    }
                }
            }
            "max" => {
                if let Some(max_str) = rule_value {
                    if let Ok(max) = max_str.parse::<usize>() {
                        if value.len() > max { errors.push(format!("{} must be at most {} characters", name, max)); }
                    }
                }
            }
            "email" => {
                let result = email_format(value).await?;
                if !result.valid { errors.push(format!("{} must be a valid email", name)); }
            }
            "numeric" => {
                if !value.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
                    errors.push(format!("{} must be numeric", name));
                }
            }
            "alpha" => {
                if !value.chars().all(|c| c.is_alphabetic()) {
                    errors.push(format!("{} must contain only letters", name));
                }
            }
            "alphanumeric" => {
                if !value.chars().all(|c| c.is_alphanumeric()) {
                    errors.push(format!("{} must be alphanumeric", name));
                }
            }
            "regex" => {
                if let Some(pattern) = rule_value {
                    if let Ok(re) = Regex::new(pattern) {
                        if !re.is_match(value) { errors.push(format!("{} does not match required pattern", name)); }
                    }
                }
            }
            _ => {}
        }
    }
    
    Ok(FieldOutput { valid: errors.is_empty(), errors })
}

pub async fn ip(ip_str: &str, version: Option<&str>) -> Result<IpOutput, String> {
    match IpAddr::from_str(ip_str) {
        Ok(addr) => {
            let detected_version = if addr.is_ipv4() { 4 } else { 6 };
            let version_match = match version {
                Some("v4") | Some("4") => detected_version == 4,
                Some("v6") | Some("6") => detected_version == 6,
                _ => true,
            };
            let is_private = match addr {
                IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
                IpAddr::V6(v6) => v6.is_loopback(),
            };
            Ok(IpOutput { valid: version_match, version: detected_version, is_private })
        }
        Err(_) => Ok(IpOutput { valid: false, version: 0, is_private: false }),
    }
}

pub async fn json(json_string: &str) -> Result<JsonOutput, String> {
    match serde_json::from_str::<Value>(json_string) {
        Ok(parsed) => {
            let formatted = serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| json_string.to_string());
            Ok(JsonOutput { valid: true, parsed: formatted, error: String::new() })
        }
        Err(e) => Ok(JsonOutput { valid: false, parsed: String::new(), error: e.to_string() }),
    }
}

pub async fn password(
    pwd: &str,
    require_lowercase: Option<bool>,
    require_numbers: Option<bool>,
    min_length: Option<i32>,
    require_symbols: Option<bool>,
    require_uppercase: Option<bool>,
) -> Result<PasswordOutput, String> {
    let mut suggestions = Vec::new();
    let mut score = 0;
    
    let min_len = min_length.unwrap_or(8) as usize;
    let need_lowercase = require_lowercase.unwrap_or(true);
    let need_uppercase = require_uppercase.unwrap_or(true);
    let need_numbers = require_numbers.unwrap_or(true);
    let need_symbols = require_symbols.unwrap_or(true);
    
    if pwd.len() < min_len {
        suggestions.push(format!("Password should be at least {} characters", min_len));
    } else {
        score += 1;
        if pwd.len() >= 12 { score += 1; }
        if pwd.len() >= 16 { score += 1; }
    }
    
    let has_lowercase = pwd.chars().any(|c| c.is_lowercase());
    if need_lowercase && !has_lowercase { suggestions.push("Add lowercase letters".to_string()); }
    else if has_lowercase { score += 1; }
    
    let has_uppercase = pwd.chars().any(|c| c.is_uppercase());
    if need_uppercase && !has_uppercase { suggestions.push("Add uppercase letters".to_string()); }
    else if has_uppercase { score += 1; }
    
    let has_numbers = pwd.chars().any(|c| c.is_ascii_digit());
    if need_numbers && !has_numbers { suggestions.push("Add numbers".to_string()); }
    else if has_numbers { score += 1; }
    
    let has_symbols = pwd.chars().any(|c| !c.is_alphanumeric());
    if need_symbols && !has_symbols { suggestions.push("Add special characters".to_string()); }
    else if has_symbols { score += 1; }
    
    let strength = match score {
        0..=2 => "weak",
        3..=4 => "fair",
        5..=6 => "strong",
        _ => "very strong",
    }.to_string();
    
    Ok(PasswordOutput { valid: suggestions.is_empty(), strength, suggestions })
}

pub async fn phone(phone_str: &str, country_code: Option<&str>) -> Result<PhoneOutput, String> {
    let cleaned: String = phone_str.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
    
    if cleaned.is_empty() {
        return Ok(PhoneOutput { valid: false, formatted: String::new(), country: String::new() });
    }
    
    let digits_only: String = cleaned.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits_only.len() < 7 || digits_only.len() > 15 {
        return Ok(PhoneOutput { valid: false, formatted: phone_str.to_string(), country: String::new() });
    }
    
    let country = if let Some(code) = country_code {
        code.to_uppercase()
    } else if cleaned.starts_with("+1") || (digits_only.len() == 10 && !cleaned.starts_with('+')) {
        "US".to_string()
    } else if cleaned.starts_with("+44") { "GB".to_string() }
    else if cleaned.starts_with("+49") { "DE".to_string() }
    else if cleaned.starts_with("+33") { "FR".to_string() }
    else if cleaned.starts_with("+81") { "JP".to_string() }
    else if cleaned.starts_with("+86") { "CN".to_string() }
    else if cleaned.starts_with("+61") { "AU".to_string() }
    else { "Unknown".to_string() };
    
    let formatted = if digits_only.len() == 10 && country == "US" {
        format!("({}) {}-{}", &digits_only[0..3], &digits_only[3..6], &digits_only[6..10])
    } else if cleaned.starts_with('+') { cleaned.clone() }
    else { format!("+{}", digits_only) };
    
    Ok(PhoneOutput { valid: true, formatted, country })
}

pub async fn sanitize_html(
    html: &str,
    _allowed_attributes: Option<HashMap<String, Value>>,
    _allowed_tags: Option<Vec<String>>,
) -> Result<SanitizeHtmlOutput, String> {
    let sanitized = ammonia::clean(html);
    let original_len = html.len();
    let sanitized_len = sanitized.len();
    
    let removed_count = if original_len > sanitized_len {
        (original_len - sanitized_len) as i32
    } else { 0 };
    
    Ok(SanitizeHtmlOutput { sanitized, removed_count })
}

pub async fn schema(
    data: &str,
    schema_def: HashMap<String, Value>,
    _strict: Option<bool>,
) -> Result<SchemaOutput, String> {
    let data_value: Value = serde_json::from_str(data)
        .map_err(|e| format!("Invalid JSON data: {}", e))?;
    
    let schema_value = serde_json::to_value(schema_def)
        .map_err(|e| format!("Invalid schema: {}", e))?;
    
    let validator = jsonschema::validator_for(&schema_value)
        .map_err(|e| format!("Schema compilation error: {}", e))?;
    
    let validation_result = validator.validate(&data_value);
    
    match validation_result {
        Ok(_) => Ok(SchemaOutput { valid: true, errors: Vec::new() }),
        Err(error) => {
            let mut error_map = HashMap::new();
            error_map.insert("message".to_string(), Value::String(error.to_string()));
            Ok(SchemaOutput { valid: false, errors: vec![error_map] })
        }
    }
}

pub async fn url(url_str: &str, allowed_schemes: Option<Vec<String>>) -> Result<UrlOutput, String> {
    match url::Url::parse(url_str) {
        Ok(parsed) => {
            if let Some(schemes) = allowed_schemes {
                let scheme = parsed.scheme().to_lowercase();
                if !schemes.iter().any(|s| s.to_lowercase() == scheme) {
                    return Ok(UrlOutput {
                        valid: false,
                        reason: format!("URL scheme '{}' not in allowed list", scheme),
                    });
                }
            }
            Ok(UrlOutput { valid: true, reason: String::new() })
        }
        Err(e) => Ok(UrlOutput { valid: false, reason: format!("Invalid URL: {}", e) }),
    }
}

pub async fn uuid(uuid_str: &str, version: Option<i32>) -> Result<UuidOutput, String> {
    match uuid::Uuid::parse_str(uuid_str) {
        Ok(parsed) => {
            let detected_version = parsed.get_version_num();
            let version_match = match version {
                Some(v) => detected_version == v as usize,
                None => true,
            };
            Ok(UuidOutput { valid: version_match, version: detected_version as i32 })
        }
        Err(_) => Ok(UuidOutput { valid: false, version: 0 }),
    }
}

#[cfg(test)]
mod tests;

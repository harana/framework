// Harana Actions - Validation Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// credit_card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardOutput {
    pub last_four: String,
    pub valid: bool,
    pub card_type: String
}

// date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateOutput {
    pub parsed: String,
    pub reason: String,
    pub valid: bool
}

// email_format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailFormatOutput {
    pub reason: String,
    pub valid: bool
}

// field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOutput {
    pub valid: bool,
    pub errors: Vec<String>
}

// ip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpOutput {
    pub version: i32,
    pub is_private: bool,
    pub valid: bool
}

// json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonOutput {
    pub parsed: String,
    pub valid: bool,
    pub error: String
}

// password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordOutput {
    pub valid: bool,
    pub strength: String,
    pub suggestions: Vec<String>
}

// phone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneOutput {
    pub formatted: String,
    pub valid: bool,
    pub country: String
}

// sanitize_html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeHtmlOutput {
    pub removed_count: i32,
    pub sanitized: String
}

// schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub valid: bool
}

// url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlOutput {
    pub reason: String,
    pub valid: bool
}

// uuid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidOutput {
    pub valid: bool,
    pub version: i32
}

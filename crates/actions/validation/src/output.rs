// Harana Actions - Validation Module Output Types
// Auto-generated output structs for Validation action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// validate_schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateSchemaOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub valid: bool,
}

// validate_field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateFieldOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

// validate_email_format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateEmailFormatOutput {
    pub reason: String,
    pub valid: bool,
}

// validate_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateUrlOutput {
    pub reason: String,
    pub valid: bool,
}

// validate_phone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePhoneOutput {
    pub country: String,
    pub formatted: String,
    pub valid: bool,
}

// validate_date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateDateOutput {
    pub parsed: Option<String>, // datetime
    pub reason: String,
    pub valid: bool,
}

// validate_json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateJsonOutput {
    pub error: Option<String>,
    pub parsed: Option<Value>,
    pub valid: bool,
}

// validate_credit_card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateCreditCardOutput {
    pub card_type: String,
    pub last_four: String,
    pub valid: bool,
}

// validate_uuid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateUuidOutput {
    pub valid: bool,
    pub version: i32,
}

// validate_ip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateIpOutput {
    pub is_private: bool,
    pub valid: bool,
    pub version: i32,
}

// validate_password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePasswordOutput {
    pub strength: String, // weak | fair | strong | very_strong
    pub suggestions: Vec<String>,
    pub valid: bool,
}

// sanitize_html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeHtmlOutput {
    pub removed_count: i32,
    pub sanitized: String,
}

// schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOutput {
    pub valid: bool,
    pub errors: Vec<HashMap<String, Value>>
}

// field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOutput {
    pub valid: bool,
    pub errors: Vec<String>
}

// email_format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailFormatOutput {
    pub valid: bool,
    pub reason: String
}

// url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlOutput {
    pub valid: bool,
    pub reason: String
}

// phone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneOutput {
    pub formatted: String,
    pub country: String,
    pub valid: bool
}

// date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateOutput {
    pub valid: bool,
    pub reason: String,
    pub parsed: String
}

// json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonOutput {
    pub valid: bool,
    pub parsed: String,
    pub error: String
}

// credit_card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardOutput {
    pub card_type: String,
    pub last_four: String,
    pub valid: bool
}

// uuid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidOutput {
    pub version: i32,
    pub valid: bool
}

// ip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpOutput {
    pub is_private: bool,
    pub valid: bool,
    pub version: i32
}

// password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordOutput {
    pub strength: String,
    pub suggestions: Vec<String>,
    pub valid: bool
}
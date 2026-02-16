// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SchemaOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailFormatOutput {
    pub reason: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlOutput {
    pub reason: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PhoneOutput {
    pub country: String,
    pub formatted: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DateOutput {
    pub parsed: chrono::DateTime<chrono::Utc>,
    pub reason: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonOutput {
    pub error: String,
    pub parsed: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreditCardOutput {
    pub card_type: String,
    pub last_four: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UuidOutput {
    pub valid: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IpOutput {
    pub is_private: bool,
    pub valid: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasswordOutput {
    pub strength: String,
    pub suggestions: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SanitizeHtmlOutput {
    pub removed_count: i64,
    pub sanitized: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub field_name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationSchema {
    pub type: String,
    pub properties: std::collections::HashMap<String, String>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlAllowedAttributes {
    pub global: Vec<String>,
    pub per_tag: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRule {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub pattern: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRuleSet {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRuleSetAssignment {
    pub rule_id: String,
    pub rule_set_id: String,
    pub sort_order: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationErrorLog {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub field_name: String,
    pub input_value: String,
    pub rule_id: String,
    pub rule_set_id: String,
    pub violation_message: String,
}

#[async_trait]
pub trait ValidationAction {
    async fn schema(&self, data: String, schema: String, strict: bool) -> Result<SchemaOutput, Box<dyn std::error::Error>>;
    async fn field(&self, field_name: String, rules: Vec<String>, value: String) -> Result<FieldOutput, Box<dyn std::error::Error>>;
    async fn email_format(&self, email: String) -> Result<EmailFormatOutput, Box<dyn std::error::Error>>;
    async fn url(&self, allowed_schemes: Vec<String>, url: String) -> Result<UrlOutput, Box<dyn std::error::Error>>;
    async fn phone(&self, country_code: String, phone: String) -> Result<PhoneOutput, Box<dyn std::error::Error>>;
    async fn date(&self, date: String, format: String) -> Result<DateOutput, Box<dyn std::error::Error>>;
    async fn json(&self, json_string: String) -> Result<JsonOutput, Box<dyn std::error::Error>>;
    async fn credit_card(&self, number: String, luhn: bool) -> Result<CreditCardOutput, Box<dyn std::error::Error>>;
    async fn uuid(&self, uuid: String, version: i64) -> Result<UuidOutput, Box<dyn std::error::Error>>;
    async fn ip(&self, ip: String, version: String) -> Result<IpOutput, Box<dyn std::error::Error>>;
    async fn password(&self, min_length: i64, password: String, require_lowercase: bool, require_numbers: bool, require_symbols: bool, require_uppercase: bool) -> Result<PasswordOutput, Box<dyn std::error::Error>>;
    async fn sanitize_html(&self, allowed_attributes: String, allowed_tags: Vec<String>, html: String) -> Result<SanitizeHtmlOutput, Box<dyn std::error::Error>>;
}

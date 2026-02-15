// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SchemaInput {
    pub data: String,
    pub schema: String,
    #[serde(default)]
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SchemaOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldInput {
    pub field_name: String,
    pub rules: Vec<String>,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FieldOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailFormatInput {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmailFormatOutput {
    pub reason: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlInput {
    pub allowed_schemes: Vec<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlOutput {
    pub reason: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PhoneInput {
    pub country_code: String,
    pub phone: String,
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
pub struct DateInput {
    pub date: String,
    pub format: String,
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
pub struct JsonInput {
    pub json_string: String,
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
pub struct CreditCardInput {
    pub number: String,
    #[serde(default)]
    pub luhn: bool,
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
pub struct UuidInput {
    pub uuid: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UuidOutput {
    pub valid: bool,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IpInput {
    pub ip: String,
    pub version: String,
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
pub struct PasswordInput {
    pub min_length: i64,
    pub password: String,
    #[serde(default)]
    pub require_lowercase: bool,
    #[serde(default)]
    pub require_numbers: bool,
    #[serde(default)]
    pub require_symbols: bool,
    #[serde(default)]
    pub require_uppercase: bool,
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
pub struct SanitizeHtmlInput {
    pub allowed_attributes: String,
    pub allowed_tags: Vec<String>,
    pub html: String,
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

#[async_trait]
pub trait ValidationAction {
    async fn schema(&self, input: SchemaInput) -> Result<SchemaOutput, Box<dyn std::error::Error>>;
    async fn field(&self, input: FieldInput) -> Result<FieldOutput, Box<dyn std::error::Error>>;
    async fn email_format(&self, input: EmailFormatInput) -> Result<EmailFormatOutput, Box<dyn std::error::Error>>;
    async fn url(&self, input: UrlInput) -> Result<UrlOutput, Box<dyn std::error::Error>>;
    async fn phone(&self, input: PhoneInput) -> Result<PhoneOutput, Box<dyn std::error::Error>>;
    async fn date(&self, input: DateInput) -> Result<DateOutput, Box<dyn std::error::Error>>;
    async fn json(&self, input: JsonInput) -> Result<JsonOutput, Box<dyn std::error::Error>>;
    async fn credit_card(&self, input: CreditCardInput) -> Result<CreditCardOutput, Box<dyn std::error::Error>>;
    async fn uuid(&self, input: UuidInput) -> Result<UuidOutput, Box<dyn std::error::Error>>;
    async fn ip(&self, input: IpInput) -> Result<IpOutput, Box<dyn std::error::Error>>;
    async fn password(&self, input: PasswordInput) -> Result<PasswordOutput, Box<dyn std::error::Error>>;
    async fn sanitize_html(&self, input: SanitizeHtmlInput) -> Result<SanitizeHtmlOutput, Box<dyn std::error::Error>>;
}

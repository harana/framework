// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub state: String,
    pub street: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddressValidationError {
    pub field: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddressSuggestion {
    pub description: String,
    pub place_id: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddressValidationLog {
    pub address_id: String,
    pub errors: String,
    #[serde(default)]
    pub is_valid: bool,
    pub provider: String,
    #[serde(default = "chrono::Utc::now")]
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait AddressAction {
    async fn validate(&self, address: String, country: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn normalize(&self, address: String, country: String, format: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn parse(&self, address: String, country: String) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn autocomplete(&self, country: String, limit: i64, query: String, types: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

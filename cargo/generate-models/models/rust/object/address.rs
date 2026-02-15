// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub entity_id: String,
    pub entity_type: String,
    #[serde(default)]
    pub is_primary: bool,
    #[serde(default)]
    pub is_verified: bool,
    pub label: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
    pub street: Option<String>,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Address {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// address_validation_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddressValidationLog {
    /// Reference: address.id
    pub address_id: String,
    pub errors: Option<String>,
    #[serde(default)]
    pub is_valid: bool,
    pub provider: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

impl AddressValidationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// geocode_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodeResult {
    pub city: Option<String>,
    pub confidence: Option<f64>,
    pub country: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub formatted_address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub postal_code: Option<String>,
    pub provider: String,
    pub query: String,
    pub region: Option<String>,
    pub street: Option<String>,
    pub type: String,
}

impl GeocodeResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// geocode_cache
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodeCache {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub latitude: f64,
    pub longitude: f64,
    pub query_hash: String,
    pub response: String,
}

impl GeocodeCache {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


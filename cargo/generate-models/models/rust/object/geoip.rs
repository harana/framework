// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// geoip_lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoipLookup {
    pub city: Option<String>,
    pub country: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub region: Option<String>,
    pub timezone: Option<String>,
}

impl GeoipLookup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// geoip_cache
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoipCache {
    pub city: Option<String>,
    pub country: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ip_address: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub looked_up_at: chrono::DateTime<chrono::Utc>,
    pub region: Option<String>,
    pub timezone: Option<String>,
}

impl GeoipCache {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


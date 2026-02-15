// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoCoordinate {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodingResult {
    pub latitude: f64,
    pub longitude: f64,
    pub formatted_address: String,
    pub confidence: f64,
}

impl GeocodingResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoAddress {
    pub street: String,
    pub city: String,
    pub county: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoAddress {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteStep {
    pub instruction: String,
    pub distance: f64,
    pub duration: i64,
    pub latitude: f64,
    pub longitude: f64,
}

impl RouteStep {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


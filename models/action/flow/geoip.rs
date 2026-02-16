// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LookupOutput {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodeOutput {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReverseGeocodeOutput {
    pub address: String,
    pub formatted_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoLocation {
    pub ip: String,
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoAddress {
    pub street: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoipLookup {
    pub city: String,
    pub country: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoipCache {
    pub city: String,
    pub country: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default = "chrono::Utc::now")]
    pub looked_up_at: chrono::DateTime<chrono::Utc>,
    pub region: String,
    pub timezone: String,
}

#[async_trait]
pub trait GeoipAction {
    async fn lookup(&self, ip: String) -> Result<LookupOutput, Box<dyn std::error::Error>>;
    async fn distance(&self, from_lat: f64, from_lon: f64, to_lat: f64, to_lon: f64, unit: String) -> Result<f64, Box<dyn std::error::Error>>;
    async fn geocode(&self, address: String) -> Result<GeocodeOutput, Box<dyn std::error::Error>>;
    async fn reverse_geocode(&self, latitude: f64, longitude: f64) -> Result<ReverseGeocodeOutput, Box<dyn std::error::Error>>;
}

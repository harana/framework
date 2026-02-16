// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DistanceOutput {
    pub distance: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForwardOutput {
    pub latitude: f64,
    pub longitude: f64,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReverseOutput {
    pub address: String,
    pub formatted_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteOutput {
    pub distance: f64,
    pub duration: i64,
    pub polyline: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimezoneOutput {
    pub id: String,
    pub name: String,
    pub offset: i64,
    pub dst_offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodingResult {
    pub latitude: f64,
    pub longitude: f64,
    pub formatted_address: String,
    pub confidence: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteStep {
    pub instruction: String,
    pub distance: f64,
    pub duration: i64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodeResult {
    pub city: String,
    pub confidence: f64,
    pub country: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub formatted_address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub postal_code: String,
    pub provider: String,
    pub query: String,
    pub region: String,
    pub street: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodeCache {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub query_hash: String,
    pub response: String,
}

#[async_trait]
pub trait GeocodeAction {
    async fn distance(&self, from: String, to: String, unit: String) -> Result<DistanceOutput, Box<dyn std::error::Error>>;
    async fn forward(&self, address: String, country: String, limit: i64) -> Result<ForwardOutput, Box<dyn std::error::Error>>;
    async fn reverse(&self, latitude: f64, longitude: f64, language: String) -> Result<ReverseOutput, Box<dyn std::error::Error>>;
    async fn route(&self, from: String, to: String, mode: String, waypoints: Vec<String>, avoid: Vec<String>) -> Result<RouteOutput, Box<dyn std::error::Error>>;
    async fn timezone(&self, latitude: f64, longitude: f64, timestamp: i64) -> Result<TimezoneOutput, Box<dyn std::error::Error>>;
}

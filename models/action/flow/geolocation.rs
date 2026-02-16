// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
pub struct DistanceOutput {
    pub distance: f64,
    pub unit: String,
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
    pub dst_offset: i64,
    pub id: String,
    pub name: String,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoResult {
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
    pub region: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteStep {
    pub instruction: String,
    pub distance: f64,
    pub duration: i64,
    pub start_location: String,
    pub end_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeolocationPoint {
    pub accuracy: f64,
    pub altitude: f64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub label: String,
    pub latitude: f64,
    pub longitude: f64,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeolocationRoute {
    #[serde(default = "chrono::Utc::now")]
    pub calculated_at: chrono::DateTime<chrono::Utc>,
    pub distance: f64,
    pub duration_seconds: i64,
    pub from_point_id: String,
    pub mode: String,
    pub polyline: String,
    pub to_point_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeolocationFence {
    pub center_latitude: f64,
    pub center_longitude: f64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub radius_meters: f64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait GeolocationAction {
    async fn forward(&self, address: String, country: String, limit: i64) -> Result<ForwardOutput, Box<dyn std::error::Error>>;
    async fn reverse(&self, language: String, latitude: f64, longitude: f64) -> Result<ReverseOutput, Box<dyn std::error::Error>>;
    async fn distance(&self, from: String, to: String, unit: String) -> Result<DistanceOutput, Box<dyn std::error::Error>>;
    async fn route(&self, avoid: Vec<String>, from: String, mode: String, to: String, waypoints: Vec<String>) -> Result<RouteOutput, Box<dyn std::error::Error>>;
    async fn timezone(&self, latitude: f64, longitude: f64, timestamp: i64) -> Result<TimezoneOutput, Box<dyn std::error::Error>>;
}

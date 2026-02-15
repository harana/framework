// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForwardInput {
    pub address: String,
    pub country: String,
    pub limit: i64,
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
pub struct ReverseInput {
    pub language: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReverseOutput {
    pub address: String,
    pub formatted_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DistanceInput {
    pub from: String,
    pub to: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DistanceOutput {
    pub distance: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteInput {
    pub avoid: Vec<String>,
    pub from: String,
    pub mode: String,
    pub to: String,
    pub waypoints: Vec<String>,
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
pub struct TimezoneInput {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
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

#[async_trait]
pub trait GeolocationAction {
    async fn forward(&self, input: ForwardInput) -> Result<ForwardOutput, Box<dyn std::error::Error>>;
    async fn reverse(&self, input: ReverseInput) -> Result<ReverseOutput, Box<dyn std::error::Error>>;
    async fn distance(&self, input: DistanceInput) -> Result<DistanceOutput, Box<dyn std::error::Error>>;
    async fn route(&self, input: RouteInput) -> Result<RouteOutput, Box<dyn std::error::Error>>;
    async fn timezone(&self, input: TimezoneInput) -> Result<TimezoneOutput, Box<dyn std::error::Error>>;
}

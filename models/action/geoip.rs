// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LookupInput {
    pub ip: String,
}

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
pub struct DistanceInput {
    pub from_lat: f64,
    pub from_lon: f64,
    pub to_lat: f64,
    pub to_lon: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DistanceOutput {
    pub distance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeocodeInput {
    pub address: String,
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
pub struct ReverseGeocodeInput {
    pub latitude: f64,
    pub longitude: f64,
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

#[async_trait]
pub trait GeoipAction {
    async fn lookup(&self, input: LookupInput) -> Result<LookupOutput, Box<dyn std::error::Error>>;
    async fn distance(&self, input: DistanceInput) -> Result<DistanceOutput, Box<dyn std::error::Error>>;
    async fn geocode(&self, input: GeocodeInput) -> Result<GeocodeOutput, Box<dyn std::error::Error>>;
    async fn reverse_geocode(&self, input: ReverseGeocodeInput) -> Result<ReverseGeocodeOutput, Box<dyn std::error::Error>>;
}

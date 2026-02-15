// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeolocationPoint {
    pub accuracy: Option<f64>,
    pub altitude: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub label: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub metadata: Option<String>,
}

impl GeolocationPoint {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeolocationRoute {
    #[serde(default = "chrono::Utc::now")]
    pub calculated_at: chrono::DateTime<chrono::Utc>,
    pub distance: Option<f64>,
    pub duration_seconds: Option<i64>,
    pub from_point_id: String,
    pub mode: String,
    pub polyline: Option<String>,
    pub to_point_id: String,
}

impl GeolocationRoute {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl GeolocationFence {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub accuracy: f64,
}

impl Coordinates {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeoResult {
    pub latitude: f64,
    pub longitude: f64,
    pub formatted_address: String,
    pub confidence: f64,
}

impl GeoResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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
    pub start_location: String,
    pub end_location: String,
}

impl RouteStep {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


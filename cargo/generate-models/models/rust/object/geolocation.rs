// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// geolocation_point
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// geolocation_route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeolocationRoute {
    #[serde(default = "chrono::Utc::now")]
    pub calculated_at: chrono::DateTime<chrono::Utc>,
    pub distance: Option<f64>,
    pub duration_seconds: Option<i64>,
    /// Reference: geolocation_point.id
    pub from_point_id: String,
    pub mode: String,
    pub polyline: Option<String>,
    /// Reference: geolocation_point.id
    pub to_point_id: String,
}

impl GeolocationRoute {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// geolocation_fence
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


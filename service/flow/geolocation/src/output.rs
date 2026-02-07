// Harana Actions - Geolocation Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// forward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardOutput {
    pub latitude: f64,
    pub longitude: f64,
    pub results: Vec<GeoResult>,
}

// reverse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseOutput {
    pub address: GeoAddress,
    pub formatted_address: String,
}

// distance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceOutput {
    pub distance: f64,
    pub unit: String,
}

// route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOutput {
    pub distance: f64,
    pub duration: i32,
    pub polyline: String,
    pub steps: Vec<RouteStep>,
}

// timezone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneOutput {
    pub dst_offset: i32,
    pub id: String,
    pub name: String,
    pub offset: i32,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoResult {
    pub latitude: f64,
    pub longitude: f64,
    pub formatted_address: String,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoAddress {
    pub street: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStep {
    pub instruction: String,
    pub distance: f64,
    pub duration: i32,
    pub start_location: Coordinates,
    pub end_location: Coordinates,
}

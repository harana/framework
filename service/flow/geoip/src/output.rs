// Harana Actions - Geoip Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// distance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceOutput {
    pub distance: f64
}

// geocode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeocodeOutput {
    pub latitude: f64,
    pub country: String,
    pub city: String,
    pub region: String,
    pub longitude: f64
}

// lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupOutput {
    pub region: String,
    pub timezone: String,
    pub city: String,
    pub longitude: f64,
    pub country: String,
    pub latitude: f64
}

// reverse_geocode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseGeocodeOutput {
    pub address: HashMap<String, Value>,
    pub formatted_address: String
}

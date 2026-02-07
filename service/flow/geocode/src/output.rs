// Harana Actions - Geocode Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// distance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceOutput {
    pub distance: f64,
    pub unit: String
}

// forward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardOutput {
    pub latitude: f64,
    pub longitude: f64,
    pub results: Vec<HashMap<String, Value>>
}

// reverse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseOutput {
    pub address: HashMap<String, Value>,
    pub formatted_address: String
}

// route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOutput {
    pub duration: i32,
    pub steps: Vec<HashMap<String, Value>>,
    pub polyline: String,
    pub distance: f64
}

// timezone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneOutput {
    pub id: String,
    pub offset: i32,
    pub dst_offset: i32,
    pub name: String
}

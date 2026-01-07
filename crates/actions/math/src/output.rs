// Harana Actions - Math Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// abs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsOutput {
    pub result: f64
}

// average
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AverageOutput {
    pub result: f64
}

// ceil
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeilOutput {
    pub result: f64
}

// floor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorOutput {
    pub result: f64
}

// max
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxOutput {
    pub result: f64
}

// min
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinOutput {
    pub result: f64
}

// percentage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentageOutput {
    pub result: f64
}

// round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundOutput {
    pub result: f64
}

// sum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SumOutput {
    pub result: f64
}

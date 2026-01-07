// Harana Actions - Math Module
// This module provides math actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Absolute Number Value
pub async fn abs(
    value: f64,
) -> Result<AbsOutput, String> {
    unimplemented!("abs")
}

/// Average Of Values
pub async fn average(
    values: Vec<f64>,
) -> Result<AverageOutput, String> {
    unimplemented!("average")
}

/// Ceiling Number Value
pub async fn ceil(
    value: f64,
) -> Result<CeilOutput, String> {
    unimplemented!("ceil")
}

/// Floor Number Value
pub async fn floor(
    value: f64,
) -> Result<FloorOutput, String> {
    unimplemented!("floor")
}

/// Maximum Of Values
pub async fn max(
    values: Vec<f64>,
) -> Result<MaxOutput, String> {
    unimplemented!("max")
}

/// Minimum Of Values
pub async fn min(
    values: Vec<f64>,
) -> Result<MinOutput, String> {
    unimplemented!("min")
}

/// Calculate Percentage
pub async fn percentage(
    value: f64,
    total: f64,
    precision: Option<i32>,
) -> Result<PercentageOutput, String> {
    unimplemented!("percentage")
}

/// Round Number Value
pub async fn round(
    value: f64,
    precision: Option<i32>,
) -> Result<RoundOutput, String> {
    unimplemented!("round")
}

/// Sum Of Values
pub async fn sum(
    values: Vec<f64>,
) -> Result<SumOutput, String> {
    unimplemented!("sum")
}

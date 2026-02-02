// Harana Actions - Math Module
// This module provides math actions and functionality.

pub mod output;

use output::*;

/// Absolute Number Value
/// 
/// Returns the absolute value of a number.
pub async fn abs(
    value: f64,
) -> Result<AbsOutput, String> {
    Ok(AbsOutput {
        result: value.abs(),
    })
}

/// Average Of Values
/// 
/// Calculates the arithmetic mean of a list of numbers.
pub async fn average(
    values: Vec<f64>,
) -> Result<AverageOutput, String> {
    if values.is_empty() {
        return Err("Values list cannot be empty".to_string());
    }
    
    let sum: f64 = values.iter().sum();
    let result = sum / values.len() as f64;
    
    Ok(AverageOutput { result })
}

/// Ceiling Number Value
/// 
/// Returns the smallest integer greater than or equal to the number.
pub async fn ceil(
    value: f64,
) -> Result<CeilOutput, String> {
    Ok(CeilOutput {
        result: value.ceil(),
    })
}

/// Floor Number Value
/// 
/// Returns the largest integer less than or equal to the number.
pub async fn floor(
    value: f64,
) -> Result<FloorOutput, String> {
    Ok(FloorOutput {
        result: value.floor(),
    })
}

/// Maximum Of Values
/// 
/// Returns the maximum value from a list of numbers.
pub async fn max(
    values: Vec<f64>,
) -> Result<MaxOutput, String> {
    if values.is_empty() {
        return Err("Values list cannot be empty".to_string());
    }
    
    let result = values
        .into_iter()
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));
    
    Ok(MaxOutput { result })
}

/// Minimum Of Values
/// 
/// Returns the minimum value from a list of numbers.
pub async fn min(
    values: Vec<f64>,
) -> Result<MinOutput, String> {
    if values.is_empty() {
        return Err("Values list cannot be empty".to_string());
    }
    
    let result = values
        .into_iter()
        .fold(f64::INFINITY, |a, b| a.min(b));
    
    Ok(MinOutput { result })
}

/// Calculate Percentage
/// 
/// Calculates what percentage `value` is of `total`.
pub async fn percentage(
    value: f64,
    total: f64,
    precision: Option<i32>,
) -> Result<PercentageOutput, String> {
    if total == 0.0 {
        return Err("Total cannot be zero".to_string());
    }
    
    let pct = (value / total) * 100.0;
    let precision = precision.unwrap_or(2).max(0) as u32;
    let multiplier = 10f64.powi(precision as i32);
    let result = (pct * multiplier).round() / multiplier;
    
    Ok(PercentageOutput { result })
}

/// Round Number Value
/// 
/// Rounds a number to the specified precision.
pub async fn round(
    value: f64,
    precision: Option<i32>,
) -> Result<RoundOutput, String> {
    let precision = precision.unwrap_or(0);
    let multiplier = 10f64.powi(precision);
    let result = (value * multiplier).round() / multiplier;
    
    Ok(RoundOutput { result })
}

/// Sum Of Values
/// 
/// Calculates the sum of a list of numbers.
pub async fn sum(
    values: Vec<f64>,
) -> Result<SumOutput, String> {
    let result: f64 = values.iter().sum();
    
    Ok(SumOutput { result })
}

#[cfg(test)]
mod tests;

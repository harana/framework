// Harana Actions - Geoip Module
// This module provides geoip actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Calculate Distance Between Coordinates
pub async fn distance(
    from_lon: f64,
    to_lat: f64,
    to_lon: f64,
    from_lat: f64,
    unit: Option<&str>,
) -> Result<DistanceOutput, String> {
    unimplemented!("distance")
}

/// Geocode Address To Coordinates
pub async fn geocode(
    address: &str,
) -> Result<GeocodeOutput, String> {
    unimplemented!("geocode")
}

/// Lookup IP Address Location
pub async fn lookup(
    ip: &str,
) -> Result<LookupOutput, String> {
    unimplemented!("lookup")
}

/// Reverse Geocode Coordinates To Address
pub async fn reverse_geocode(
    latitude: f64,
    longitude: f64,
) -> Result<ReverseGeocodeOutput, String> {
    unimplemented!("reverse_geocode")
}

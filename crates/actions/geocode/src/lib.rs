// Harana Actions - Geocode Module
// This module provides geocode actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

const EARTH_RADIUS_KM: f64 = 6371.0;
const EARTH_RADIUS_MI: f64 = 3958.8;

/// Calculate Distance Between Points using Haversine formula
pub async fn distance(
    from: HashMap<String, Value>,
    to: HashMap<String, Value>,
    unit: Option<&str>,
) -> Result<DistanceOutput, String> {
    // Extract coordinates from input
    let from_lat = from.get("latitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'latitude' in from")?;
    let from_lon = from.get("longitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'longitude' in from")?;
    let to_lat = to.get("latitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'latitude' in to")?;
    let to_lon = to.get("longitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'longitude' in to")?;

    // Validate coordinate ranges
    if from_lat < -90.0 || from_lat > 90.0 || to_lat < -90.0 || to_lat > 90.0 {
        return Err("Latitude must be between -90 and 90".to_string());
    }
    if from_lon < -180.0 || from_lon > 180.0 || to_lon < -180.0 || to_lon > 180.0 {
        return Err("Longitude must be between -180 and 180".to_string());
    }

    // Haversine formula
    let lat1_rad = from_lat.to_radians();
    let lat2_rad = to_lat.to_radians();
    let delta_lat = (to_lat - from_lat).to_radians();
    let delta_lon = (to_lon - from_lon).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    let unit_str = unit.unwrap_or("km");
    let (dist, unit_out) = match unit_str.to_lowercase().as_str() {
        "mi" | "miles" => (EARTH_RADIUS_MI * c, "mi".to_string()),
        "m" | "meters" => (EARTH_RADIUS_KM * c * 1000.0, "m".to_string()),
        _ => (EARTH_RADIUS_KM * c, "km".to_string()),
    };

    Ok(DistanceOutput {
        distance: (dist * 1000.0).round() / 1000.0, // round to 3 decimals
        unit: unit_out,
    })
}

/// Convert Address To Coordinates (simulated geocoding)
pub async fn forward(
    address: &str,
    _country: Option<&str>,
    limit: Option<i32>,
) -> Result<ForwardOutput, String> {
    if address.trim().is_empty() {
        return Err("Address cannot be empty".to_string());
    }

    // Simulated geocoding - return mock results based on known addresses
    let mut results: Vec<HashMap<String, Value>> = Vec::new();
    let limit = limit.unwrap_or(5).min(10).max(1) as usize;

    let addr_lower = address.to_lowercase();

    // Mock database of known locations
    let known_locations: Vec<(&str, f64, f64, &str)> = vec![
        ("new york", 40.7128, -74.0060, "New York, NY, USA"),
        ("los angeles", 34.0522, -118.2437, "Los Angeles, CA, USA"),
        ("london", 51.5074, -0.1278, "London, England, UK"),
        ("paris", 48.8566, 2.3522, "Paris, France"),
        ("tokyo", 35.6762, 139.6503, "Tokyo, Japan"),
        ("sydney", -33.8688, 151.2093, "Sydney, NSW, Australia"),
        ("san francisco", 37.7749, -122.4194, "San Francisco, CA, USA"),
        ("chicago", 41.8781, -87.6298, "Chicago, IL, USA"),
        ("berlin", 52.5200, 13.4050, "Berlin, Germany"),
        ("toronto", 43.6532, -79.3832, "Toronto, ON, Canada"),
    ];

    for (name, lat, lon, formatted) in known_locations {
        if addr_lower.contains(name) || name.contains(&addr_lower) {
            let mut result = HashMap::new();
            result.insert("latitude".to_string(), Value::from(lat));
            result.insert("longitude".to_string(), Value::from(lon));
            result.insert("formatted_address".to_string(), Value::String(formatted.to_string()));
            result.insert("confidence".to_string(), Value::from(0.95));
            results.push(result);
            if results.len() >= limit {
                break;
            }
        }
    }

    // If no match found, return a generic result with coordinates at (0, 0)
    let (latitude, longitude) = if results.is_empty() {
        // Generate pseudo-random coords based on address hash for demonstration
        let hash: u64 = address.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));
        let lat = ((hash % 18000) as f64 / 100.0) - 90.0;
        let lon = ((hash % 36000) as f64 / 100.0) - 180.0;

        let mut result = HashMap::new();
        result.insert("latitude".to_string(), Value::from(lat));
        result.insert("longitude".to_string(), Value::from(lon));
        result.insert("formatted_address".to_string(), Value::String(address.to_string()));
        result.insert("confidence".to_string(), Value::from(0.5));
        results.push(result);
        (lat, lon)
    } else {
        let first = &results[0];
        (
            first.get("latitude").and_then(|v| v.as_f64()).unwrap_or(0.0),
            first.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0),
        )
    };

    Ok(ForwardOutput {
        latitude,
        longitude,
        results,
    })
}

/// Convert Coordinates To Address (simulated reverse geocoding)
pub async fn reverse(
    longitude: f64,
    latitude: f64,
    _language: Option<&str>,
) -> Result<ReverseOutput, String> {
    // Validate coordinates
    if latitude < -90.0 || latitude > 90.0 {
        return Err("Latitude must be between -90 and 90".to_string());
    }
    if longitude < -180.0 || longitude > 180.0 {
        return Err("Longitude must be between -180 and 180".to_string());
    }

    // Mock database of locations for reverse geocoding
    let known_locations: Vec<(f64, f64, &str, &str, &str, &str, &str)> = vec![
        (40.7128, -74.0060, "New York", "New York", "NY", "USA", "10001"),
        (34.0522, -118.2437, "Los Angeles", "Los Angeles", "CA", "USA", "90001"),
        (51.5074, -0.1278, "London", "London", "England", "UK", "SW1A 1AA"),
        (48.8566, 2.3522, "Paris", "Paris", "Île-de-France", "France", "75001"),
        (35.6762, 139.6503, "Tokyo", "Tokyo", "Tokyo", "Japan", "100-0001"),
        (-33.8688, 151.2093, "Sydney", "Sydney", "NSW", "Australia", "2000"),
        (37.7749, -122.4194, "San Francisco", "San Francisco", "CA", "USA", "94102"),
    ];

    // Find the closest known location
    let mut closest: Option<(f64, &str, &str, &str, &str, &str)> = None;
    let mut min_dist = f64::MAX;

    for (lat, lon, city, county, state, country, postal) in &known_locations {
        let dlat = latitude - lat;
        let dlon = longitude - lon;
        let dist = (dlat * dlat + dlon * dlon).sqrt();
        if dist < min_dist {
            min_dist = dist;
            closest = Some((dist, *city, *county, *state, *country, *postal));
        }
    }

    let mut address = HashMap::new();

    if let Some((dist, city, county, state, country, postal)) = closest {
        if dist < 5.0 {
            // Close enough to a known location
            address.insert("city".to_string(), Value::String(city.to_string()));
            address.insert("county".to_string(), Value::String(county.to_string()));
            address.insert("state".to_string(), Value::String(state.to_string()));
            address.insert("country".to_string(), Value::String(country.to_string()));
            address.insert("postal_code".to_string(), Value::String(postal.to_string()));
            address.insert("latitude".to_string(), Value::from(latitude));
            address.insert("longitude".to_string(), Value::from(longitude));

            let formatted = format!("{}, {}, {}", city, state, country);
            return Ok(ReverseOutput {
                address,
                formatted_address: formatted,
            });
        }
    }

    // Unknown location - return generic data
    address.insert("latitude".to_string(), Value::from(latitude));
    address.insert("longitude".to_string(), Value::from(longitude));
    address.insert("country".to_string(), Value::String("Unknown".to_string()));

    Ok(ReverseOutput {
        address,
        formatted_address: format!("{:.4}, {:.4}", latitude, longitude),
    })
}

/// Get Route Between Points (simulated routing)
pub async fn route(
    from: HashMap<String, Value>,
    to: HashMap<String, Value>,
    _avoid: Option<Vec<String>>,
    mode: Option<&str>,
    _waypoints: Option<Vec<HashMap<String, Value>>>,
) -> Result<RouteOutput, String> {
    // Extract coordinates
    let from_lat = from.get("latitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'latitude' in from")?;
    let from_lon = from.get("longitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'longitude' in from")?;
    let to_lat = to.get("latitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'latitude' in to")?;
    let to_lon = to.get("longitude")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid 'longitude' in to")?;

    // Calculate straight-line distance
    let dist_result = distance(from.clone(), to.clone(), Some("km")).await?;
    let straight_distance = dist_result.distance;

    // Estimate route distance (typically 1.3-1.5x straight-line for roads)
    let route_factor = 1.4;
    let route_distance = straight_distance * route_factor;

    // Estimate duration based on mode
    let mode_str = mode.unwrap_or("driving");
    let speed_kmh = match mode_str.to_lowercase().as_str() {
        "walking" => 5.0,
        "cycling" | "bicycling" => 15.0,
        "transit" => 30.0,
        _ => 50.0, // driving
    };

    let duration_hours = route_distance / speed_kmh;
    let duration_seconds = (duration_hours * 3600.0) as i32;

    // Generate a simple polyline (mock encoded polyline)
    let polyline = format!(
        "mock_polyline_{:.2}_{:.2}_{:.2}_{:.2}",
        from_lat, from_lon, to_lat, to_lon
    );

    // Generate route steps
    let mut steps: Vec<HashMap<String, Value>> = Vec::new();

    // Start step
    let mut step1 = HashMap::new();
    step1.insert("instruction".to_string(), Value::String("Start at origin".to_string()));
    step1.insert("distance".to_string(), Value::from(0.0));
    step1.insert("duration".to_string(), Value::from(0));
    step1.insert("latitude".to_string(), Value::from(from_lat));
    step1.insert("longitude".to_string(), Value::from(from_lon));
    steps.push(step1);

    // Mid step (midpoint)
    let mid_lat = (from_lat + to_lat) / 2.0;
    let mid_lon = (from_lon + to_lon) / 2.0;
    let mut step2 = HashMap::new();
    step2.insert("instruction".to_string(), Value::String("Continue on route".to_string()));
    step2.insert("distance".to_string(), Value::from(route_distance / 2.0));
    step2.insert("duration".to_string(), Value::from(duration_seconds / 2));
    step2.insert("latitude".to_string(), Value::from(mid_lat));
    step2.insert("longitude".to_string(), Value::from(mid_lon));
    steps.push(step2);

    // End step
    let mut step3 = HashMap::new();
    step3.insert("instruction".to_string(), Value::String("Arrive at destination".to_string()));
    step3.insert("distance".to_string(), Value::from(route_distance));
    step3.insert("duration".to_string(), Value::from(duration_seconds));
    step3.insert("latitude".to_string(), Value::from(to_lat));
    step3.insert("longitude".to_string(), Value::from(to_lon));
    steps.push(step3);

    Ok(RouteOutput {
        duration: duration_seconds,
        steps,
        polyline,
        distance: (route_distance * 1000.0).round() / 1000.0,
    })
}

/// Get Timezone For Coordinates
pub async fn timezone(
    latitude: f64,
    longitude: f64,
    _timestamp: Option<i32>,
) -> Result<TimezoneOutput, String> {
    // Validate coordinates
    if latitude < -90.0 || latitude > 90.0 {
        return Err("Latitude must be between -90 and 90".to_string());
    }
    if longitude < -180.0 || longitude > 180.0 {
        return Err("Longitude must be between -180 and 180".to_string());
    }

    // Calculate timezone offset based on longitude (rough approximation)
    // Each 15 degrees of longitude = 1 hour offset
    let base_offset = (longitude / 15.0).round() as i32;
    let offset_seconds = base_offset * 3600;

    // Determine timezone name based on offset and region
    let (tz_id, tz_name, dst_offset) = determine_timezone(latitude, longitude, base_offset);

    Ok(TimezoneOutput {
        id: tz_id,
        offset: offset_seconds,
        dst_offset,
        name: tz_name,
    })
}

fn determine_timezone(latitude: f64, longitude: f64, base_offset: i32) -> (String, String, i32) {
    // Simplified timezone database based on coordinates
    // In production, use a proper timezone database

    // North America
    if latitude > 24.0 && latitude < 72.0 && longitude > -170.0 && longitude < -50.0 {
        if longitude > -75.0 {
            return ("America/New_York".to_string(), "Eastern Time".to_string(), 3600);
        } else if longitude > -95.0 {
            return ("America/Chicago".to_string(), "Central Time".to_string(), 3600);
        } else if longitude > -115.0 {
            return ("America/Denver".to_string(), "Mountain Time".to_string(), 3600);
        } else {
            return ("America/Los_Angeles".to_string(), "Pacific Time".to_string(), 3600);
        }
    }

    // Europe
    if latitude > 35.0 && latitude < 72.0 && longitude > -10.0 && longitude < 40.0 {
        if longitude < 7.5 {
            return ("Europe/London".to_string(), "Greenwich Mean Time".to_string(), 3600);
        } else if longitude < 15.0 {
            return ("Europe/Paris".to_string(), "Central European Time".to_string(), 3600);
        } else {
            return ("Europe/Berlin".to_string(), "Central European Time".to_string(), 3600);
        }
    }

    // Asia Pacific
    if latitude > -50.0 && latitude < 60.0 && longitude > 100.0 {
        if longitude > 135.0 && latitude > 30.0 {
            return ("Asia/Tokyo".to_string(), "Japan Standard Time".to_string(), 0);
        } else if longitude > 145.0 && latitude < 0.0 {
            return ("Australia/Sydney".to_string(), "Australian Eastern Time".to_string(), 3600);
        }
    }

    // Default: UTC offset based on longitude
    let offset_str = if base_offset >= 0 {
        format!("UTC+{}", base_offset)
    } else {
        format!("UTC{}", base_offset)
    };
    (format!("Etc/GMT{:+}", -base_offset), offset_str.clone(), 0)
}

#[cfg(test)]
mod tests;

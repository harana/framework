// Harana Actions - Geoip Module
// This module provides geoip actions and functionality.

pub mod output;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use output::*;
use serde_json::json;
use std::collections::HashMap;

// In-memory mock GeoIP database
static GEOIP_DB: Lazy<DashMap<String, GeoLocation>> = Lazy::new(|| {
    let db = DashMap::new();
    
    // Add some mock data for testing
    db.insert("8.8.8.8".to_string(), GeoLocation {
        latitude: 37.386051,
        longitude: -122.083855,
        city: "Mountain View".to_string(),
        region: "California".to_string(),
        country: "United States".to_string(),
        timezone: "America/Los_Angeles".to_string(),
    });
    
    db.insert("1.1.1.1".to_string(), GeoLocation {
        latitude: -33.8688,
        longitude: 151.2093,
        city: "Sydney".to_string(),
        region: "New South Wales".to_string(),
        country: "Australia".to_string(),
        timezone: "Australia/Sydney".to_string(),
    });
    
    db.insert("208.67.222.222".to_string(), GeoLocation {
        latitude: 37.7749,
        longitude: -122.4194,
        city: "San Francisco".to_string(),
        region: "California".to_string(),
        country: "United States".to_string(),
        timezone: "America/Los_Angeles".to_string(),
    });
    
    db
});

// In-memory mock geocoding database
static GEOCODE_DB: Lazy<DashMap<String, GeoCodeLocation>> = Lazy::new(|| {
    let db = DashMap::new();
    
    db.insert("1600 Amphitheatre Parkway, Mountain View, CA".to_lowercase(), GeoCodeLocation {
        latitude: 37.423021,
        longitude: -122.083739,
        city: "Mountain View".to_string(),
        region: "California".to_string(),
        country: "United States".to_string(),
    });
    
    db.insert("sydney opera house".to_lowercase(), GeoCodeLocation {
        latitude: -33.8568,
        longitude: 151.2153,
        city: "Sydney".to_string(),
        region: "New South Wales".to_string(),
        country: "Australia".to_string(),
    });
    
    db.insert("eiffel tower".to_lowercase(), GeoCodeLocation {
        latitude: 48.8584,
        longitude: 2.2945,
        city: "Paris".to_string(),
        region: "Île-de-France".to_string(),
        country: "France".to_string(),
    });
    
    db
});

#[derive(Debug, Clone)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
    city: String,
    region: String,
    country: String,
    timezone: String,
}

#[derive(Debug, Clone)]
struct GeoCodeLocation {
    latitude: f64,
    longitude: f64,
    city: String,
    region: String,
    country: String,
}

/// Calculate Distance Between Coordinates using the Haversine formula
pub async fn distance(
    from_lat: f64,
    from_lon: f64,
    to_lat: f64,
    to_lon: f64,
    unit: Option<&str>,
) -> Result<DistanceOutput, String> {
    // Validate coordinates
    if from_lat < -90.0 || from_lat > 90.0 {
        return Err("from_lat must be between -90 and 90".to_string());
    }
    if to_lat < -90.0 || to_lat > 90.0 {
        return Err("to_lat must be between -90 and 90".to_string());
    }
    if from_lon < -180.0 || from_lon > 180.0 {
        return Err("from_lon must be between -180 and 180".to_string());
    }
    if to_lon < -180.0 || to_lon > 180.0 {
        return Err("to_lon must be between -180 and 180".to_string());
    }
    
    let unit = unit.unwrap_or("Km");
    
    // Earth's radius in kilometers
    let r = 6371.0;
    
    // Convert to radians
    let lat1 = from_lat.to_radians();
    let lat2 = to_lat.to_radians();
    let delta_lat = (to_lat - from_lat).to_radians();
    let delta_lon = (to_lon - from_lon).to_radians();
    
    // Haversine formula
    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    let distance_km = r * c;
    
    // Convert to requested unit
    let distance = match unit {
        "Mi" => distance_km * 0.621371, // km to miles
        "M" => distance_km * 1000.0,    // km to meters
        _ => distance_km,                // default to km
    };
    
    Ok(DistanceOutput { distance })
}

/// Geocode Address To Coordinates
pub async fn geocode(address: &str) -> Result<GeocodeOutput, String> {
    if address.is_empty() {
        return Err("Address cannot be empty".to_string());
    }
    
    // Try to find exact match first
    let normalized_address = address.to_lowercase();
    
    if let Some(location) = GEOCODE_DB.get(&normalized_address) {
        return Ok(GeocodeOutput {
            latitude: location.latitude,
            longitude: location.longitude,
            city: location.city.clone(),
            region: location.region.clone(),
            country: location.country.clone(),
        });
    }
    
    // Try partial match
    for entry in GEOCODE_DB.iter() {
        if entry.key().contains(&normalized_address) || normalized_address.contains(entry.key()) {
            return Ok(GeocodeOutput {
                latitude: entry.value().latitude,
                longitude: entry.value().longitude,
                city: entry.value().city.clone(),
                region: entry.value().region.clone(),
                country: entry.value().country.clone(),
            });
        }
    }
    
    // Return a default location if not found
    Err(format!("Address not found: {}", address))
}

/// Lookup IP Address Location
pub async fn lookup(ip: &str) -> Result<LookupOutput, String> {
    if ip.is_empty() {
        return Err("IP address cannot be empty".to_string());
    }
    
    // Validate IP format (simple check)
    if !ip.contains('.') && !ip.contains(':') {
        return Err("Invalid IP address format".to_string());
    }
    
    if let Some(location) = GEOIP_DB.get(ip) {
        return Ok(LookupOutput {
            city: location.city.clone(),
            country: location.country.clone(),
            latitude: location.latitude,
            longitude: location.longitude,
            region: location.region.clone(),
            timezone: location.timezone.clone(),
        });
    }
    
    // Return a default location for unknown IPs (based on first octet heuristic)
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() == 4 {
        if let Ok(first_octet) = parts[0].parse::<u8>() {
            // Simple heuristic based on IP ranges
            let (city, country, region, timezone, lat, lon) = match first_octet {
                0..=50 => ("New York", "United States", "New York", "America/New_York", 40.7128, -74.0060),
                51..=100 => ("London", "United Kingdom", "England", "Europe/London", 51.5074, -0.1278),
                101..=150 => ("Tokyo", "Japan", "Tokyo", "Asia/Tokyo", 35.6762, 139.6503),
                151..=200 => ("Sydney", "Australia", "New South Wales", "Australia/Sydney", -33.8688, 151.2093),
                _ => ("San Francisco", "United States", "California", "America/Los_Angeles", 37.7749, -122.4194),
            };
            
            return Ok(LookupOutput {
                city: city.to_string(),
                country: country.to_string(),
                latitude: lat,
                longitude: lon,
                region: region.to_string(),
                timezone: timezone.to_string(),
            });
        }
    }
    
    Err(format!("Unable to lookup IP: {}", ip))
}

/// Reverse Geocode Coordinates To Address
pub async fn reverse_geocode(
    latitude: f64,
    longitude: f64,
) -> Result<ReverseGeocodeOutput, String> {
    // Validate coordinates
    if latitude < -90.0 || latitude > 90.0 {
        return Err("Latitude must be between -90 and 90".to_string());
    }
    if longitude < -180.0 || longitude > 180.0 {
        return Err("Longitude must be between -180 and 180".to_string());
    }
    
    // Find the closest known location
    let mut closest_distance = f64::MAX;
    let mut closest_location: Option<GeoCodeLocation> = None;
    
    for entry in GEOCODE_DB.iter() {
        let loc = entry.value();
        let dist = ((loc.latitude - latitude).powi(2) + (loc.longitude - longitude).powi(2)).sqrt();
        if dist < closest_distance {
            closest_distance = dist;
            closest_location = Some(loc.clone());
        }
    }
    
    if let Some(location) = closest_location {
        let mut address = HashMap::new();
        address.insert("street".to_string(), json!(""));
        address.insert("city".to_string(), json!(location.city.clone()));
        address.insert("region".to_string(), json!(location.region.clone()));
        address.insert("postal_code".to_string(), json!(""));
        address.insert("country".to_string(), json!(location.country.clone()));
        
        let formatted_address = format!("{}, {}, {}", 
            location.city, location.region, location.country);
        
        return Ok(ReverseGeocodeOutput {
            address,
            formatted_address,
        });
    }
    
    // Default fallback
    let mut address = HashMap::new();
    address.insert("street".to_string(), json!(""));
    address.insert("city".to_string(), json!("Unknown"));
    address.insert("region".to_string(), json!("Unknown"));
    address.insert("postal_code".to_string(), json!(""));
    address.insert("country".to_string(), json!("Unknown"));
    
    Ok(ReverseGeocodeOutput {
        address,
        formatted_address: "Unknown Location".to_string(),
    })
}

#[cfg(test)]
mod tests;

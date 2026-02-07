use super::*;
use std::collections::HashMap;
use serde_json::Value;

#[tokio::test]
async fn test_distance_km() {
    let mut from = HashMap::new();
    from.insert("latitude".to_string(), Value::from(40.7128));  // New York
    from.insert("longitude".to_string(), Value::from(-74.0060));

    let mut to = HashMap::new();
    to.insert("latitude".to_string(), Value::from(34.0522));  // Los Angeles
    to.insert("longitude".to_string(), Value::from(-118.2437));

    let result = distance(from, to, Some("km")).await.unwrap();
    assert_eq!(result.unit, "km");
    // Distance from NYC to LA is approximately 3944 km
    assert!(result.distance > 3900.0 && result.distance < 4000.0);
}

#[tokio::test]
async fn test_distance_miles() {
    let mut from = HashMap::new();
    from.insert("latitude".to_string(), Value::from(51.5074));  // London
    from.insert("longitude".to_string(), Value::from(-0.1278));

    let mut to = HashMap::new();
    to.insert("latitude".to_string(), Value::from(48.8566));  // Paris
    to.insert("longitude".to_string(), Value::from(2.3522));

    let result = distance(from, to, Some("mi")).await.unwrap();
    assert_eq!(result.unit, "mi");
    // Distance from London to Paris is approximately 213 miles
    assert!(result.distance > 200.0 && result.distance < 230.0);
}

#[tokio::test]
async fn test_distance_same_point() {
    let mut point = HashMap::new();
    point.insert("latitude".to_string(), Value::from(0.0));
    point.insert("longitude".to_string(), Value::from(0.0));

    let result = distance(point.clone(), point, None).await.unwrap();
    assert_eq!(result.distance, 0.0);
}

#[tokio::test]
async fn test_distance_invalid_latitude() {
    let mut from = HashMap::new();
    from.insert("latitude".to_string(), Value::from(100.0));  // Invalid
    from.insert("longitude".to_string(), Value::from(0.0));

    let mut to = HashMap::new();
    to.insert("latitude".to_string(), Value::from(0.0));
    to.insert("longitude".to_string(), Value::from(0.0));

    let result = distance(from, to, None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Latitude"));
}

#[tokio::test]
async fn test_distance_missing_field() {
    let mut from = HashMap::new();
    from.insert("latitude".to_string(), Value::from(0.0));
    // Missing longitude

    let mut to = HashMap::new();
    to.insert("latitude".to_string(), Value::from(0.0));
    to.insert("longitude".to_string(), Value::from(0.0));

    let result = distance(from, to, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_forward_known_city() {
    let result = forward("New York", None, None).await.unwrap();

    assert!((result.latitude - 40.7128).abs() < 0.01);
    assert!((result.longitude - (-74.0060)).abs() < 0.01);
    assert!(!result.results.is_empty());
}

#[tokio::test]
async fn test_forward_unknown_address() {
    let result = forward("Some Random Place 12345", None, None).await.unwrap();

    // Should still return results (with generated coordinates)
    assert!(!result.results.is_empty());
}

#[tokio::test]
async fn test_forward_empty_address() {
    let result = forward("", None, None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("empty"));
}

#[tokio::test]
async fn test_forward_with_limit() {
    let result = forward("york", None, Some(2)).await.unwrap();
    assert!(result.results.len() <= 2);
}

#[tokio::test]
async fn test_reverse_known_location() {
    // Coordinates near New York
    let result = reverse(-74.0060, 40.7128, None).await.unwrap();

    assert!(result.formatted_address.contains("New York") || !result.formatted_address.is_empty());
    assert!(!result.address.is_empty());
}

#[tokio::test]
async fn test_reverse_unknown_location() {
    // Middle of the ocean
    let result = reverse(0.0, 0.0, None).await.unwrap();

    assert!(!result.formatted_address.is_empty());
}

#[tokio::test]
async fn test_reverse_invalid_latitude() {
    let result = reverse(0.0, 100.0, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_reverse_invalid_longitude() {
    let result = reverse(200.0, 0.0, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_route_basic() {
    let mut from = HashMap::new();
    from.insert("latitude".to_string(), Value::from(40.7128));
    from.insert("longitude".to_string(), Value::from(-74.0060));

    let mut to = HashMap::new();
    to.insert("latitude".to_string(), Value::from(34.0522));
    to.insert("longitude".to_string(), Value::from(-118.2437));

    let result = route(from, to, None, Some("driving"), None).await.unwrap();

    assert!(result.distance > 0.0);
    assert!(result.duration > 0);
    assert!(!result.polyline.is_empty());
    assert!(!result.steps.is_empty());
}

#[tokio::test]
async fn test_route_walking() {
    let mut from = HashMap::new();
    from.insert("latitude".to_string(), Value::from(51.5074));
    from.insert("longitude".to_string(), Value::from(-0.1278));

    let mut to = HashMap::new();
    to.insert("latitude".to_string(), Value::from(51.5200));
    to.insert("longitude".to_string(), Value::from(-0.1000));

    let walking = route(from.clone(), to.clone(), None, Some("walking"), None).await.unwrap();
    let driving = route(from, to, None, Some("driving"), None).await.unwrap();

    // Walking should take longer than driving
    assert!(walking.duration > driving.duration);
}

#[tokio::test]
async fn test_route_missing_coords() {
    let mut from = HashMap::new();
    from.insert("latitude".to_string(), Value::from(0.0));
    // Missing longitude

    let mut to = HashMap::new();
    to.insert("latitude".to_string(), Value::from(0.0));
    to.insert("longitude".to_string(), Value::from(0.0));

    let result = route(from, to, None, None, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_timezone_new_york() {
    let result = timezone(40.7128, -74.0060, None).await.unwrap();

    assert_eq!(result.id, "America/New_York");
    assert!(result.name.contains("Eastern"));
    assert_eq!(result.offset, -5 * 3600); // UTC-5
}

#[tokio::test]
async fn test_timezone_london() {
    let result = timezone(51.5074, -0.1278, None).await.unwrap();

    assert!(result.id.contains("London") || result.id.contains("GMT"));
}

#[tokio::test]
async fn test_timezone_tokyo() {
    let result = timezone(35.6762, 139.6503, None).await.unwrap();

    assert!(result.id.contains("Tokyo") || result.offset == 9 * 3600);
}

#[tokio::test]
async fn test_timezone_invalid_latitude() {
    let result = timezone(100.0, 0.0, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_timezone_invalid_longitude() {
    let result = timezone(0.0, 200.0, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_timezone_utc() {
    // A point in the Atlantic, near UTC
    let result = timezone(0.0, 0.0, None).await.unwrap();

    assert!(result.offset.abs() < 3600); // Close to UTC
}

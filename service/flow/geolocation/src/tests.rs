#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_forward_geocoding() {
        let result = forward("1600 Amphitheatre Parkway", None, None).await.unwrap();
        assert!(result.latitude > 0.0);
        assert!(result.longitude < 0.0);
        assert_eq!(result.results.len(), 1);
        assert!(result.results[0].formatted_address.contains("1600 Amphitheatre Parkway"));
    }

    #[tokio::test]
    async fn test_forward_geocoding_multiple_results() {
        let result = forward("Main Street", None, Some(5)).await.unwrap();
        assert_eq!(result.results.len(), 5);
        // Each result should have decreasing confidence
        for i in 0..4 {
            assert!(result.results[i].confidence.unwrap() > result.results[i + 1].confidence.unwrap());
        }
    }

    #[tokio::test]
    async fn test_forward_geocoding_empty_address() {
        let result = forward("", None, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_reverse_geocoding() {
        let result = reverse(37.7749, -122.4194, None).await.unwrap();
        assert!(result.address.street.is_some());
        assert!(result.address.city.is_some());
        assert_eq!(result.address.city.as_ref().unwrap(), "San Francisco");
        assert!(result.formatted_address.contains("San Francisco"));
    }

    #[tokio::test]
    async fn test_reverse_geocoding_invalid_latitude() {
        let result = reverse(91.0, -122.4194, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Latitude must be between"));
    }

    #[tokio::test]
    async fn test_reverse_geocoding_invalid_longitude() {
        let result = reverse(37.7749, 181.0, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Longitude must be between"));
    }

    #[tokio::test]
    async fn test_distance_kilometers() {
        let from = Coordinates {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };
        let to = Coordinates {
            latitude: 34.0522,
            longitude: -118.2437,
            altitude: None,
            accuracy: None,
        };
        
        let result = distance(from, to, Some("Km")).await.unwrap();
        assert!(result.distance > 0.0);
        assert_eq!(result.unit, "Km");
        // Distance between SF and LA should be roughly 550-600 km
        assert!(result.distance > 500.0 && result.distance < 650.0);
    }

    #[tokio::test]
    async fn test_distance_miles() {
        let from = Coordinates {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };
        let to = Coordinates {
            latitude: 34.0522,
            longitude: -118.2437,
            altitude: None,
            accuracy: None,
        };
        
        let result = distance(from, to, Some("Miles")).await.unwrap();
        assert!(result.distance > 0.0);
        assert_eq!(result.unit, "Miles");
        // Distance between SF and LA should be roughly 340-380 miles
        assert!(result.distance > 300.0 && result.distance < 420.0);
    }

    #[tokio::test]
    async fn test_distance_meters() {
        let from = Coordinates {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };
        let to = Coordinates {
            latitude: 37.7750,
            longitude: -122.4195,
            altitude: None,
            accuracy: None,
        };
        
        let result = distance(from, to, Some("Meters")).await.unwrap();
        assert!(result.distance > 0.0);
        assert_eq!(result.unit, "Meters");
        // Very short distance should be less than 200 meters
        assert!(result.distance < 200.0);
    }

    #[tokio::test]
    async fn test_route_basic() {
        let from = Coordinates {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };
        let to = Coordinates {
            latitude: 34.0522,
            longitude: -118.2437,
            altitude: None,
            accuracy: None,
        };
        
        let result = route(from.clone(), to.clone(), None, Some("Driving"), None).await.unwrap();
        assert!(result.distance > 0.0);
        assert!(result.duration > 0);
        assert!(!result.polyline.is_empty());
        assert!(!result.steps.is_empty());
        
        // First step should start at the origin
        assert_eq!(result.steps[0].start_location.latitude, from.latitude);
        // Last step should end at the destination
        let last_step = result.steps.last().unwrap();
        assert_eq!(last_step.end_location.latitude, to.latitude);
    }

    #[tokio::test]
    async fn test_route_with_mode() {
        let from = Coordinates {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };
        let to = Coordinates {
            latitude: 37.7850,
            longitude: -122.4094,
            altitude: None,
            accuracy: None,
        };
        
        // Walking should take longer than driving for the same distance
        let walking = route(from.clone(), to.clone(), None, Some("Walking"), None).await.unwrap();
        let driving = route(from.clone(), to.clone(), None, Some("Driving"), None).await.unwrap();
        
        assert!(walking.duration > driving.duration);
        // Distance should be the same
        assert!((walking.distance - driving.distance).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_route_with_waypoints() {
        let from = Coordinates {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };
        let to = Coordinates {
            latitude: 37.7950,
            longitude: -122.3994,
            altitude: None,
            accuracy: None,
        };
        let waypoint = Coordinates {
            latitude: 37.7850,
            longitude: -122.4094,
            altitude: None,
            accuracy: None,
        };
        
        let result = route(from, to, None, Some("Driving"), Some(vec![waypoint])).await.unwrap();
        assert!(!result.steps.is_empty());
    }

    #[tokio::test]
    async fn test_timezone_los_angeles() {
        let result = timezone(34.0522, -118.2437, None).await.unwrap();
        assert_eq!(result.id, "America/Los_Angeles");
        assert!(result.name.contains("Los_Angeles") || result.name.contains("America"));
        // Pacific time is UTC-8 or UTC-7 (DST)
        assert!(result.offset == -28800 || result.offset == -25200);
    }

    #[tokio::test]
    async fn test_timezone_new_york() {
        let result = timezone(40.7128, -74.0060, None).await.unwrap();
        assert_eq!(result.id, "America/New_York");
        // Eastern time is UTC-5 or UTC-4 (DST)
        assert!(result.offset == -18000 || result.offset == -14400);
    }

    #[tokio::test]
    async fn test_timezone_london() {
        let result = timezone(51.5074, -0.1278, None).await.unwrap();
        assert_eq!(result.id, "Europe/London");
        // London is UTC+0 or UTC+1 (BST)
        assert!(result.offset == 0 || result.offset == 3600);
    }

    #[tokio::test]
    async fn test_timezone_tokyo() {
        let result = timezone(35.6762, 139.6503, None).await.unwrap();
        assert_eq!(result.id, "Asia/Tokyo");
        // Tokyo is UTC+9
        assert_eq!(result.offset, 32400);
    }

    #[tokio::test]
    async fn test_timezone_with_timestamp() {
        // Test with a specific timestamp (Jan 1, 2024)
        let timestamp = 1704067200; // 2024-01-01 00:00:00 UTC
        let result = timezone(34.0522, -118.2437, Some(timestamp)).await.unwrap();
        assert_eq!(result.id, "America/Los_Angeles");
        assert!(result.offset < 0);
    }

    #[tokio::test]
    async fn test_timezone_invalid_latitude() {
        let result = timezone(91.0, -118.2437, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Latitude must be between"));
    }

    #[tokio::test]
    async fn test_timezone_invalid_longitude() {
        let result = timezone(34.0522, 181.0, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Longitude must be between"));
    }
}

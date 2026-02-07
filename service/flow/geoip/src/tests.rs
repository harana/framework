#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_distance_km() {
        // Distance from New York to London (approximately 5570 km)
        let result = distance(40.7128, -74.0060, 51.5074, -0.1278, None)
            .await
            .unwrap();
        assert!((result.distance - 5570.0).abs() < 50.0);
    }

    #[tokio::test]
    async fn test_distance_miles() {
        // Distance from New York to London in miles (approximately 3461 miles)
        let result = distance(40.7128, -74.0060, 51.5074, -0.1278, Some("Mi"))
            .await
            .unwrap();
        assert!((result.distance - 3461.0).abs() < 50.0);
    }

    #[tokio::test]
    async fn test_distance_meters() {
        // Distance from New York to London in meters (approximately 5.57M meters)
        let result = distance(40.7128, -74.0060, 51.5074, -0.1278, Some("M"))
            .await
            .unwrap();
        assert!((result.distance - 5570000.0).abs() < 50000.0);
    }

    #[tokio::test]
    async fn test_distance_zero() {
        // Distance from same point to same point should be 0
        let result = distance(40.7128, -74.0060, 40.7128, -74.0060, None)
            .await
            .unwrap();
        assert!(result.distance.abs() < 0.01);
    }

    #[tokio::test]
    async fn test_distance_invalid_lat() {
        let result = distance(95.0, -74.0060, 51.5074, -0.1278, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("from_lat"));
    }

    #[tokio::test]
    async fn test_distance_invalid_lon() {
        let result = distance(40.7128, -200.0, 51.5074, -0.1278, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("from_lon"));
    }

    #[tokio::test]
    async fn test_lookup_known_ip() {
        let result = lookup("8.8.8.8").await.unwrap();
        assert_eq!(result.city, "Mountain View");
        assert_eq!(result.country, "United States");
        assert_eq!(result.region, "California");
        assert_eq!(result.timezone, "America/Los_Angeles");
        assert!((result.latitude - 37.386051).abs() < 0.001);
        assert!((result.longitude - (-122.083855)).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_lookup_another_ip() {
        let result = lookup("1.1.1.1").await.unwrap();
        assert_eq!(result.city, "Sydney");
        assert_eq!(result.country, "Australia");
    }

    #[tokio::test]
    async fn test_lookup_unknown_ip() {
        let result = lookup("192.168.1.1").await.unwrap();
        // Should return a heuristic-based location
        assert!(!result.city.is_empty());
        assert!(!result.country.is_empty());
    }

    #[tokio::test]
    async fn test_lookup_empty_ip() {
        let result = lookup("").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_lookup_invalid_ip() {
        let result = lookup("invalid").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid IP"));
    }

    #[tokio::test]
    async fn test_geocode_known_address() {
        let result = geocode("1600 Amphitheatre Parkway, Mountain View, CA")
            .await
            .unwrap();
        assert_eq!(result.city, "Mountain View");
        assert_eq!(result.country, "United States");
        assert!((result.latitude - 37.423021).abs() < 0.001);
        assert!((result.longitude - (-122.083739)).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_geocode_landmark() {
        let result = geocode("Eiffel Tower").await.unwrap();
        assert_eq!(result.city, "Paris");
        assert_eq!(result.country, "France");
    }

    #[tokio::test]
    async fn test_geocode_partial_match() {
        let result = geocode("Opera House").await.unwrap();
        assert_eq!(result.city, "Sydney");
    }

    #[tokio::test]
    async fn test_geocode_empty_address() {
        let result = geocode("").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_geocode_unknown_address() {
        let result = geocode("Unknown Place That Does Not Exist 12345").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_reverse_geocode_near_paris() {
        // Coordinates near Paris
        let result = reverse_geocode(48.8584, 2.2945).await.unwrap();
        assert!(result.formatted_address.contains("Paris"));
        assert!(result.address.contains_key("city"));
        assert!(result.address.contains_key("country"));
    }

    #[tokio::test]
    async fn test_reverse_geocode_near_sydney() {
        // Coordinates near Sydney
        let result = reverse_geocode(-33.8568, 151.2153).await.unwrap();
        assert!(result.formatted_address.contains("Sydney"));
    }

    #[tokio::test]
    async fn test_reverse_geocode_invalid_lat() {
        let result = reverse_geocode(95.0, 2.2945).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Latitude"));
    }

    #[tokio::test]
    async fn test_reverse_geocode_invalid_lon() {
        let result = reverse_geocode(48.8584, 200.0).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Longitude"));
    }

    #[tokio::test]
    async fn test_reverse_geocode_middle_of_ocean() {
        // Coordinates in the middle of the ocean
        let result = reverse_geocode(0.0, 0.0).await.unwrap();
        // Should still return a result (closest known location)
        assert!(!result.formatted_address.is_empty());
    }

    #[tokio::test]
    async fn test_full_workflow() {
        // Geocode an address
        let geocode_result = geocode("Sydney Opera House").await.unwrap();
        
        // Reverse geocode the coordinates
        let reverse_result = reverse_geocode(geocode_result.latitude, geocode_result.longitude)
            .await
            .unwrap();
        
        assert!(reverse_result.formatted_address.contains("Sydney"));
        
        // Calculate distance to another known location
        let paris = geocode("Eiffel Tower").await.unwrap();
        let distance_result = distance(
            geocode_result.latitude,
            geocode_result.longitude,
            paris.latitude,
            paris.longitude,
            Some("Km")
        ).await.unwrap();
        
        // Distance from Sydney to Paris is approximately 16,960 km
        assert!((distance_result.distance - 16960.0).abs() < 500.0);
    }
}

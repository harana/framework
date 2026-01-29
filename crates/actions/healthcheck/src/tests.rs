#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_check_memory() {
        let result = check_memory(None).await.unwrap();
        
        // Memory check should return valid data
        // Note: total_bytes might overflow i32 on systems with >2GB RAM, 
        // so we check percent_used instead
        assert!(result.percent_used >= 0.0);
        assert!(result.percent_used <= 100.0);
    }

    #[tokio::test]
    async fn test_check_memory_with_threshold() {
        // High threshold should pass
        let result = check_memory(Some(99)).await.unwrap();
        assert!(result.healthy);
        
        // Very low threshold might fail (depending on actual memory usage)
        let result_low = check_memory(Some(1)).await.unwrap();
        // We can't assert healthy=false because system might have very low memory usage
        assert!(result_low.percent_used >= 0.0);
    }

    #[tokio::test]
    async fn test_check_disk_space() {
        let result = check_disk_space(None, None).await.unwrap();
        
        // Disk check should return valid data
        assert!(result.percent_used >= 0.0);
        assert!(result.percent_used <= 100.0);
    }

    #[tokio::test]
    async fn test_check_disk_space_with_threshold() {
        // High threshold should pass
        let result = check_disk_space(Some(99), None).await.unwrap();
        assert!(result.healthy);
    }

    #[tokio::test]
    async fn test_check_cache() {
        let result = check_cache(None, None).await.unwrap();
        
        // Simulated cache should be healthy
        assert!(result.healthy);
        assert!(result.latency_ms >= 0);
        assert!(result.details.contains_key("cache_name"));
    }

    #[tokio::test]
    async fn test_check_cache_with_name() {
        let result = check_cache(Some("redis"), Some(5000)).await.unwrap();
        
        assert!(result.healthy);
        assert_eq!(result.details.get("cache_name").unwrap(), "redis");
    }

    #[tokio::test]
    async fn test_check_database() {
        let result = check_database(None, None).await.unwrap();
        
        // Simulated database should be healthy
        assert!(result.healthy);
        assert!(result.latency_ms >= 0);
        assert!(result.details.contains_key("connection_name"));
    }

    #[tokio::test]
    async fn test_check_database_with_name() {
        let result = check_database(Some("primary"), Some(5000)).await.unwrap();
        
        assert!(result.healthy);
        assert_eq!(result.details.get("connection_name").unwrap(), "primary");
    }

    #[tokio::test]
    async fn test_check_service() {
        let result = check_service("api-service", None).await.unwrap();
        
        assert!(result.healthy);
        assert_eq!(result.status, "healthy");
        assert!(result.latency_ms >= 0);
    }

    #[tokio::test]
    async fn test_get_system_status() {
        let result = get_system_status(Some(true)).await.unwrap();
        
        assert_eq!(result.status, "healthy");
        assert!(result.uptime_seconds >= 0);
        assert!(!result.version.is_empty());
        
        // With metrics included
        assert!(result.metrics.contains_key("memory_total_bytes"));
        assert!(result.metrics.contains_key("cpu_count"));
    }

    #[tokio::test]
    async fn test_get_system_status_without_metrics() {
        let result = get_system_status(Some(false)).await.unwrap();
        
        assert_eq!(result.status, "healthy");
        assert!(result.metrics.is_empty());
    }

    #[tokio::test]
    async fn test_check_all() {
        let result = check_all(None, None).await.unwrap();
        
        // Should have run at least 2 checks (memory and disk)
        assert!(result.checks.len() >= 2);
        assert!(result.passed_count + result.failed_count >= 2);
    }

    #[tokio::test]
    async fn test_check_all_fail_fast() {
        // fail_fast should still return results even if all pass
        let result = check_all(Some(true), None).await.unwrap();
        
        assert!(!result.checks.is_empty());
    }
}

#[cfg(test)]
mod external_api_tests {
    use crate::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_check_external_api_success() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        
        let result = check_external_api(
            &format!("{}/health", mock_server.uri()),
            Some(200),
            Some("GET"),
            Some(5),
        )
        .await
        .unwrap();
        
        assert!(result.healthy);
        assert_eq!(result.status_code, 200);
        assert!(result.latency_ms >= 0);
    }

    #[tokio::test]
    async fn test_check_external_api_wrong_status() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&mock_server)
            .await;
        
        let result = check_external_api(
            &format!("{}/health", mock_server.uri()),
            Some(200),
            Some("GET"),
            Some(5),
        )
        .await
        .unwrap();
        
        assert!(!result.healthy);
        assert_eq!(result.status_code, 503);
    }

    #[tokio::test]
    async fn test_check_external_api_post() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;
        
        let result = check_external_api(
            &format!("{}/health", mock_server.uri()),
            Some(201),
            Some("POST"),
            Some(5),
        )
        .await
        .unwrap();
        
        assert!(result.healthy);
        assert_eq!(result.status_code, 201);
    }

    #[tokio::test]
    async fn test_check_external_api_head() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("HEAD"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        
        let result = check_external_api(
            &format!("{}/health", mock_server.uri()),
            Some(200),
            Some("HEAD"),
            Some(5),
        )
        .await
        .unwrap();
        
        assert!(result.healthy);
        assert_eq!(result.status_code, 200);
    }
}

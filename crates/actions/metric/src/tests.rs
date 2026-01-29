#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_record_metric() {
        let result = record_metric("prom.test.metric", 42.0, None, None)
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_record_metric_with_tags() {
        let mut tags = HashMap::new();
        tags.insert("env".to_string(), "test".to_string());
        tags.insert("service".to_string(), "api".to_string());
        
        let result = record_metric("prom.test.tagged_metric", 100.0, None, Some(tags))
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_increment_counter() {
        let result = increment_counter("prom.test.counter1", None, None)
            .await
            .unwrap();
        assert!(result.value >= 1.0);
        
        let result = increment_counter("prom.test.counter1", Some(5.0), None)
            .await
            .unwrap();
        assert!(result.value >= 6.0);
    }

    #[tokio::test]
    async fn test_increment_counter_with_tags() {
        let mut tags = HashMap::new();
        tags.insert("method".to_string(), "GET".to_string());
        
        let result = increment_counter("prom.test.requests", Some(1.0), Some(tags.clone()))
            .await
            .unwrap();
        assert!(result.value >= 1.0);
        
        let result = increment_counter("prom.test.requests", Some(2.0), Some(tags))
            .await
            .unwrap();
        assert!(result.value >= 3.0);
    }

    #[tokio::test]
    async fn test_set_gauge() {
        let result = set_gauge("prom.test.gauge1", 75.5, None)
            .await
            .unwrap();
        assert!(result.success);
        
        // Setting gauge again should overwrite
        let result = set_gauge("prom.test.gauge1", 80.0, None)
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_record_histogram() {
        let result = record_histogram(0.5, "prom.test.latency", None, None)
            .await
            .unwrap();
        assert!(result.success);
        
        // Record more values
        for value in [0.1, 0.2, 0.3, 0.4, 0.6, 0.7, 0.8, 0.9, 1.0] {
            let result = record_histogram(value, "prom.test.latency", None, None)
                .await
                .unwrap();
            assert!(result.success);
        }
    }

    #[tokio::test]
    async fn test_record_histogram_with_custom_buckets() {
        let buckets = vec![0.1, 0.5, 1.0, 5.0, 10.0];
        let result = record_histogram(0.75, "prom.test.custom_latency", Some(buckets), None)
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_start_and_stop_timer() {
        let start_result = start_timer("prom.test.timer1", None)
            .await
            .unwrap();
        assert!(!start_result.timer_id.is_empty());
        
        // Small delay to ensure measurable duration
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let stop_result = stop_timer(&start_result.timer_id)
            .await
            .unwrap();
        assert!(stop_result.success);
        assert!(stop_result.duration_ms >= 10.0);
    }

    #[tokio::test]
    async fn test_stop_timer_not_found() {
        let result = stop_timer("non-existent-timer").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_metrics() {
        // Record some metrics first
        record_metric("prom.list.metric1", 1.0, None, None).await.unwrap();
        increment_counter("prom.list.counter1", None, None).await.unwrap();
        set_gauge("prom.list.gauge1", 50.0, None).await.unwrap();
        
        let result = list_metrics(Some(100), Some("prom.list."))
            .await
            .unwrap();
        
        assert!(result.total >= 3);
        assert!(result.metrics.iter().any(|m| m.name == "prom.list.metric1"));
        assert!(result.metrics.iter().any(|m| m.name == "prom.list.counter1"));
        assert!(result.metrics.iter().any(|m| m.name == "prom.list.gauge1"));
    }

    #[tokio::test]
    async fn test_list_metrics_with_limit() {
        // Record several metrics
        for i in 0..5 {
            record_metric(&format!("prom.limited.metric{}", i), i as f64, None, None)
                .await
                .unwrap();
        }
        
        let result = list_metrics(Some(2), Some("prom.limited."))
            .await
            .unwrap();
        
        assert!(result.total <= 2);
    }

    #[tokio::test]
    async fn test_get_metric_summary() {
        // Record several values
        for value in [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0] {
            record_metric("prom.summary.test", value, None, None)
                .await
                .unwrap();
        }
        
        let result = get_metric_summary("prom.summary.test", None, Some("1h"))
            .await
            .unwrap();
        
        assert_eq!(result.count, 10);
        assert_eq!(result.min, 10.0);
        assert_eq!(result.max, 100.0);
        assert_eq!(result.sum, 550.0);
        assert_eq!(result.avg, 55.0);
    }

    #[tokio::test]
    async fn test_get_metric_summary_empty() {
        let result = get_metric_summary("prom.non.existent.metric", None, None)
            .await
            .unwrap();
        
        assert_eq!(result.count, 0);
        assert_eq!(result.min, 0.0);
        assert_eq!(result.max, 0.0);
    }

    #[tokio::test]
    async fn test_query_metrics() {
        let now = chrono::Utc::now();
        let start_time = (now - chrono::Duration::hours(1)).to_rfc3339();
        
        // Record a metric
        record_metric("prom.query.test", 42.0, None, None)
            .await
            .unwrap();
        
        let result = query_metrics(
            &start_time,
            "prom.query.test",
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        assert!(result.total >= 1);
        assert!(result.datapoints.iter().any(|d| d.value == 42.0));
    }

    #[tokio::test]
    async fn test_query_metrics_with_time_range() {
        let now = chrono::Utc::now();
        let start_time = (now - chrono::Duration::hours(1)).to_rfc3339();
        let end_time = now.to_rfc3339();
        
        record_metric("prom.query.range", 50.0, None, None)
            .await
            .unwrap();
        
        let result = query_metrics(
            &start_time,
            "prom.query.range",
            Some("avg"),
            Some(&end_time),
            None,
            Some("5m"),
        )
        .await
        .unwrap();
        
        assert!(result.total >= 1);
    }

    #[tokio::test]
    async fn test_delete_metric() {
        // Record metrics to delete
        record_metric("prom.delete.test", 1.0, None, None).await.unwrap();
        record_metric("prom.delete.test", 2.0, None, None).await.unwrap();
        record_metric("prom.delete.test", 3.0, None, None).await.unwrap();
        
        let result = delete_metric("prom.delete.test", None, None)
            .await
            .unwrap();
        
        assert!(result.deleted_count >= 3);
    }

    #[tokio::test]
    async fn test_delete_metric_with_tags() {
        let mut tags = HashMap::new();
        tags.insert("region".to_string(), "us-east".to_string());
        
        record_metric("prom.delete.tagged", 100.0, None, Some(tags.clone()))
            .await
            .unwrap();
        
        let result = delete_metric("prom.delete.tagged", None, Some(tags))
            .await
            .unwrap();
        
        assert!(result.deleted_count >= 1);
    }

    #[tokio::test]
    async fn test_metric_with_timestamp() {
        let timestamp = "2024-01-15T10:30:00Z";
        let result = record_metric("prom.timestamp.test", 99.0, Some(timestamp), None)
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_timer_with_tags() {
        let mut tags = HashMap::new();
        tags.insert("operation".to_string(), "query".to_string());
        
        let start_result = start_timer("prom.timer.tagged", Some(tags))
            .await
            .unwrap();
        assert!(!start_result.timer_id.is_empty());
        
        let stop_result = stop_timer(&start_result.timer_id)
            .await
            .unwrap();
        assert!(stop_result.success);
    }

    #[tokio::test]
    async fn test_parse_duration() {
        // Test internal duration parsing
        assert_eq!(parse_duration_ms("100ms"), 100);
        assert_eq!(parse_duration_ms("5s"), 5000);
        assert_eq!(parse_duration_ms("1m"), 60000);
        assert_eq!(parse_duration_ms("1h"), 3600000);
        assert_eq!(parse_duration_ms("1d"), 86400000);
    }

    #[tokio::test]
    async fn test_export_prometheus_metrics() {
        // Record some metrics
        set_gauge("prom.export.gauge", 123.0, None).await.unwrap();
        increment_counter("prom.export.counter", Some(5.0), None).await.unwrap();
        
        // Export metrics
        let output = export_prometheus_metrics().unwrap();
        
        // Verify the output contains Prometheus format
        assert!(output.contains("prom_export_gauge"));
        assert!(output.contains("prom_export_counter"));
    }

    #[tokio::test]
    async fn test_get_registry() {
        let registry = get_registry();
        
        // The registry should be accessible and gatherable
        let _families = registry.gather();
    }
}

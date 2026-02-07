#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    use serde_json::json;

    // Initialize tracing for tests
    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .try_init();
    }

    // Debug tests
    #[tokio::test]
    async fn test_debug_simple() {
        init_tracing();
        let result = debug("Debug message", None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_debug_with_context() {
        init_tracing();
        let mut context = HashMap::new();
        context.insert("user_id".to_string(), json!("123"));
        context.insert("action".to_string(), json!("test"));
        
        let result = debug("Debug message with context", Some(context)).await.unwrap();
        assert!(result.success);
    }

    // Info tests
    #[tokio::test]
    async fn test_info_simple() {
        init_tracing();
        let result = info("Info message", None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_info_with_context() {
        init_tracing();
        let mut context = HashMap::new();
        context.insert("request_id".to_string(), json!("abc-123"));
        
        let result = info("Processing request", Some(context)).await.unwrap();
        assert!(result.success);
    }

    // Warn tests
    #[tokio::test]
    async fn test_warn_simple() {
        init_tracing();
        let result = warn("Warning message", None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_warn_with_context() {
        init_tracing();
        let mut context = HashMap::new();
        context.insert("threshold".to_string(), json!(80));
        context.insert("current".to_string(), json!(85));
        
        let result = warn("Resource usage high", Some(context)).await.unwrap();
        assert!(result.success);
    }

    // Error tests
    #[tokio::test]
    async fn test_error_simple() {
        init_tracing();
        let result = error("Error message", None, None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_error_with_context() {
        init_tracing();
        let mut context = HashMap::new();
        context.insert("operation".to_string(), json!("database_query"));
        
        let result = error("Operation failed", Some(context), None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_error_with_error_info() {
        init_tracing();
        let mut error_info = HashMap::new();
        error_info.insert("code".to_string(), json!("DB_CONN_FAILED"));
        error_info.insert("message".to_string(), json!("Connection refused"));
        
        let result = error("Database error", None, Some(error_info)).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_error_full() {
        init_tracing();
        let mut context = HashMap::new();
        context.insert("service".to_string(), json!("user-service"));
        
        let mut error_info = HashMap::new();
        error_info.insert("type".to_string(), json!("ValidationError"));
        error_info.insert("field".to_string(), json!("email"));
        
        let result = error("Validation failed", Some(context), Some(error_info)).await.unwrap();
        assert!(result.success);
    }

    // Structured tests
    #[tokio::test]
    async fn test_structured_debug() {
        init_tracing();
        let mut data = HashMap::new();
        data.insert("event".to_string(), json!("user_login"));
        data.insert("user_id".to_string(), json!(12345));
        
        let result = structured("debug", data).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_structured_info() {
        init_tracing();
        let mut data = HashMap::new();
        data.insert("metric".to_string(), json!("request_count"));
        data.insert("value".to_string(), json!(100));
        
        let result = structured("info", data).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_structured_warn() {
        init_tracing();
        let mut data = HashMap::new();
        data.insert("warning_type".to_string(), json!("rate_limit"));
        data.insert("remaining".to_string(), json!(10));
        
        let result = structured("warn", data).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_structured_error() {
        init_tracing();
        let mut data = HashMap::new();
        data.insert("error_code".to_string(), json!("E001"));
        data.insert("stack_trace".to_string(), json!("..."));
        
        let result = structured("error", data).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_structured_unknown_level() {
        init_tracing();
        let mut data = HashMap::new();
        data.insert("data".to_string(), json!("test"));
        
        // Unknown level should default to info
        let result = structured("unknown", data).await.unwrap();
        assert!(result.success);
    }

    // Edge cases
    #[tokio::test]
    async fn test_empty_message() {
        init_tracing();
        let result = info("", None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_special_characters() {
        init_tracing();
        let result = info("Message with special chars: <>&\"'", None).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_unicode_message() {
        init_tracing();
        let result = info("Unicode: こんにちは 🌍 émoji", None).await.unwrap();
        assert!(result.success);
    }
}

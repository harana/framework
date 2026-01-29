#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    use serde_json::Value;

    #[tokio::test]
    async fn test_build_simple() {
        let result = build("example.com", None, None, None, None, None).await.unwrap();
        assert_eq!(result.url, "https://example.com");
    }

    #[tokio::test]
    async fn test_build_full() {
        let mut query = HashMap::new();
        query.insert("key".to_string(), Value::String("value".to_string()));
        
        let result = build(
            "example.com",
            Some(query),
            Some("/path"),
            Some("http"),
            Some("section"),
            Some(8080),
        ).await.unwrap();
        
        assert!(result.url.starts_with("http://example.com:8080/path"));
        assert!(result.url.contains("key=value"));
        assert!(result.url.ends_with("#section"));
    }

    #[tokio::test]
    async fn test_decode() {
        let result = decode("hello%20world", None).await.unwrap();
        assert_eq!(result.decoded, "hello world");
    }

    #[tokio::test]
    async fn test_encode() {
        let result = encode("hello world", None).await.unwrap();
        assert!(result.encoded.contains("%20") || result.encoded.contains("+"));
    }

    #[tokio::test]
    async fn test_parse() {
        let result = parse("https://example.com:8080/path?key=value#section").await.unwrap();
        assert_eq!(result.protocol, "https");
        assert_eq!(result.host, "example.com");
        assert_eq!(result.port, Some(8080));
        assert_eq!(result.path, "/path");
        assert_eq!(result.query.get("key"), Some(&Value::String("value".to_string())));
        assert_eq!(result.fragment, Some("section".to_string()));
    }

    #[tokio::test]
    async fn test_parse_simple() {
        let result = parse("https://example.com").await.unwrap();
        assert_eq!(result.protocol, "https");
        assert_eq!(result.host, "example.com");
        assert_eq!(result.port, None);
    }

    #[tokio::test]
    async fn test_validate_valid() {
        let result = validate("https://example.com", None, None).await.unwrap();
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_invalid() {
        let result = validate("not a url", None, None).await.unwrap();
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_protocol_restriction() {
        let result = validate(
            "http://example.com",
            Some(vec!["https".to_string()]),
            None,
        ).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_validate_require_tld() {
        let result = validate("http://localhost", None, Some(true)).await.unwrap();
        assert!(!result.valid);
    }
}

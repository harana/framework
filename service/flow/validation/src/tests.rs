#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    use serde_json::json;

    #[tokio::test]
    async fn test_credit_card_valid_visa() {
        let result = credit_card("4111111111111111", Some(true)).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.card_type, "Visa");
        assert_eq!(result.last_four, "1111");
    }

    #[tokio::test]
    async fn test_credit_card_valid_mastercard() {
        let result = credit_card("5500000000000004", Some(true)).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.card_type, "MasterCard");
    }

    #[tokio::test]
    async fn test_credit_card_invalid() {
        let result = credit_card("1234567890123456", Some(true)).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_credit_card_with_spaces() {
        let result = credit_card("4111 1111 1111 1111", Some(true)).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.card_type, "Visa");
    }

    #[tokio::test]
    async fn test_date_valid_default_format() {
        let result = date("2024-01-15", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.parsed, "2024-01-15");
    }

    #[tokio::test]
    async fn test_date_valid_custom_format() {
        let result = date("15/01/2024", Some("%d/%m/%Y")).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_date_invalid() {
        let result = date("not-a-date", None).await.unwrap();
        assert!(!result.valid);
        assert!(!result.reason.is_empty());
    }

    #[tokio::test]
    async fn test_email_valid() {
        let result = email_format("user@example.com").await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_email_valid_with_plus() {
        let result = email_format("user+tag@example.com").await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_email_invalid_no_at() {
        let result = email_format("invalid-email").await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_email_invalid_no_domain() {
        let result = email_format("user@").await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_email_empty() {
        let result = email_format("").await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_field_required_valid() {
        let result = field(vec!["required".to_string()], "value", Some("name")).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_field_required_invalid() {
        let result = field(vec!["required".to_string()], "", Some("name")).await.unwrap();
        assert!(!result.valid);
        assert!(result.errors[0].contains("required"));
    }

    #[tokio::test]
    async fn test_field_min_length() {
        let result = field(vec!["min:5".to_string()], "abc", Some("field")).await.unwrap();
        assert!(!result.valid);
        assert!(result.errors[0].contains("at least 5"));
    }

    #[tokio::test]
    async fn test_field_max_length() {
        let result = field(vec!["max:3".to_string()], "abcdef", Some("field")).await.unwrap();
        assert!(!result.valid);
        assert!(result.errors[0].contains("at most 3"));
    }

    #[tokio::test]
    async fn test_field_multiple_rules() {
        let rules = vec!["required".to_string(), "min:5".to_string(), "max:10".to_string()];
        let result = field(rules, "hello", Some("field")).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_field_numeric() {
        let result = field(vec!["numeric".to_string()], "123.45", None).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_field_alpha() {
        let result = field(vec!["alpha".to_string()], "abc", None).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_ip_valid_v4() {
        let result = ip("192.168.1.1", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.version, 4);
        assert!(result.is_private);
    }

    #[tokio::test]
    async fn test_ip_valid_v6() {
        let result = ip("::1", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.version, 6);
    }

    #[tokio::test]
    async fn test_ip_invalid() {
        let result = ip("not-an-ip", None).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_ip_version_mismatch() {
        let result = ip("192.168.1.1", Some("v6")).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_json_valid() {
        let result = json(r#"{"key": "value"}"#).await.unwrap();
        assert!(result.valid);
        assert!(result.parsed.contains("key"));
    }

    #[tokio::test]
    async fn test_json_invalid() {
        let result = json("not json").await.unwrap();
        assert!(!result.valid);
        assert!(!result.error.is_empty());
    }

    #[tokio::test]
    async fn test_json_array() {
        let result = json(r#"[1, 2, 3]"#).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_password_strong() {
        let result = password("MyP@ssw0rd123", None, None, None, None, None).await.unwrap();
        assert!(result.valid);
        assert!(result.strength == "strong" || result.strength == "very strong");
    }

    #[tokio::test]
    async fn test_password_weak() {
        let result = password("pass", None, None, None, None, None).await.unwrap();
        assert!(!result.valid);
        assert!(!result.suggestions.is_empty());
    }

    #[tokio::test]
    async fn test_password_custom_requirements() {
        let result = password("abcdefgh", Some(true), Some(false), Some(8), Some(false), Some(false)).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_phone_valid_us() {
        let result = phone("5551234567", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.country, "US");
    }

    #[tokio::test]
    async fn test_phone_with_country_code() {
        let result = phone("+44 20 7946 0958", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.country, "GB");
    }

    #[tokio::test]
    async fn test_phone_invalid_too_short() {
        let result = phone("123", None).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_sanitize_html_xss() {
        let html = "<script>alert('xss')</script><p>Safe content</p>";
        let result = sanitize_html(html, None, None).await.unwrap();
        assert!(!result.sanitized.contains("<script>"));
        assert!(result.sanitized.contains("Safe content"));
    }

    #[tokio::test]
    async fn test_sanitize_html_safe() {
        let html = "<p>Hello <strong>World</strong></p>";
        let result = sanitize_html(html, None, None).await.unwrap();
        assert!(result.sanitized.contains("<p>"));
        assert!(result.sanitized.contains("<strong>"));
    }

    #[tokio::test]
    async fn test_schema_valid() {
        let mut schema_def = HashMap::new();
        schema_def.insert("type".to_string(), json!("object"));
        schema_def.insert("properties".to_string(), json!({"name": {"type": "string"}, "age": {"type": "integer"}}));
        schema_def.insert("required".to_string(), json!(["name"]));
        let data = r#"{"name": "John", "age": 30}"#;
        let result = schema(data, schema_def, None).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_schema_invalid_missing_required() {
        let mut schema_def = HashMap::new();
        schema_def.insert("type".to_string(), json!("object"));
        schema_def.insert("required".to_string(), json!(["name"]));
        let data = r#"{"age": 30}"#;
        let result = schema(data, schema_def, None).await.unwrap();
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_url_valid() {
        let result = url("https://example.com/path?query=1", None).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_url_invalid() {
        let result = url("not a url", None).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_url_allowed_schemes() {
        let schemes = vec!["http".to_string(), "https".to_string()];
        let result = url("ftp://example.com", Some(schemes)).await.unwrap();
        assert!(!result.valid);
        assert!(result.reason.contains("not in allowed"));
    }

    #[tokio::test]
    async fn test_uuid_valid_v4() {
        let result = uuid("550e8400-e29b-41d4-a716-446655440000", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.version, 4);
    }

    #[tokio::test]
    async fn test_uuid_invalid() {
        let result = uuid("not-a-uuid", None).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_uuid_version_check() {
        let result = uuid("550e8400-e29b-41d4-a716-446655440000", Some(4)).await.unwrap();
        assert!(result.valid);
        let result = uuid("550e8400-e29b-41d4-a716-446655440000", Some(7)).await.unwrap();
        assert!(!result.valid);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    use serde_json::Value;

    #[tokio::test]
    async fn test_diff_added() {
        let source = r#"{"a": 1}"#;
        let target = r#"{"a": 1, "b": 2}"#;
        let result = diff(target, source, None).await.unwrap();
        assert!(result.added.contains_key("b"));
        assert!(result.removed.is_empty());
    }

    #[tokio::test]
    async fn test_diff_removed() {
        let source = r#"{"a": 1, "b": 2}"#;
        let target = r#"{"a": 1}"#;
        let result = diff(target, source, None).await.unwrap();
        assert!(result.removed.contains_key("b"));
        assert!(result.added.is_empty());
    }

    #[tokio::test]
    async fn test_diff_changed() {
        let source = r#"{"a": 1}"#;
        let target = r#"{"a": 2}"#;
        let result = diff(target, source, None).await.unwrap();
        assert!(result.changed.contains_key("a"));
    }

    #[tokio::test]
    async fn test_jmespath_query() {
        let data = r#"{"items": [1, 2, 3]}"#;
        let result = jmespath_query("$.items[*]", data).await.unwrap();
        assert!(result.result.is_array());
    }

    #[tokio::test]
    async fn test_merge_shallow() {
        let obj1: HashMap<String, Value> = [("a".to_string(), Value::Number(1.into()))].into();
        let obj2: HashMap<String, Value> = [("b".to_string(), Value::Number(2.into()))].into();
        let result = merge(vec![obj1, obj2], Some("shallow")).await.unwrap();
        assert!(result.result.contains_key("a"));
        assert!(result.result.contains_key("b"));
    }

    #[tokio::test]
    async fn test_parse() {
        let result = parse(r#"{"key": "value"}"#, None).await.unwrap();
        assert_eq!(result.result["key"], "value");
    }

    #[tokio::test]
    async fn test_parse_invalid() {
        let result = parse("not json", None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stringify() {
        let result = stringify(r#"{"a":1}"#, None, None).await.unwrap();
        assert!(result.json.contains("\"a\""));
    }

    #[tokio::test]
    async fn test_stringify_pretty() {
        let result = stringify(r#"{"a":1}"#, Some(2), None).await.unwrap();
        assert!(result.json.contains('\n'));
    }

    #[tokio::test]
    async fn test_validate_valid() {
        let schema: HashMap<String, Value> = [
            ("type".to_string(), Value::String("object".to_string())),
            ("required".to_string(), Value::Array(vec![Value::String("name".to_string())])),
        ].into();
        let result = validate(r#"{"name": "test"}"#, schema).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_validate_missing_required() {
        let schema: HashMap<String, Value> = [
            ("required".to_string(), Value::Array(vec![Value::String("name".to_string())])),
        ].into();
        let result = validate(r#"{"other": "test"}"#, schema).await.unwrap();
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }
}

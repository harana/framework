#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_bytes() {
        let result = bytes(16).await.unwrap();
        assert_eq!(result.bytes.len(), 16);
        assert!(!result.base64.is_empty());
    }

    #[tokio::test]
    async fn test_bytes_invalid_length() {
        let result = bytes(0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_choice() {
        let items = vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
        ];
        let result = choice(items.clone()).await.unwrap();
        assert!(items.contains(&result.item));
        assert!(result.index < 3);
    }

    #[tokio::test]
    async fn test_choice_empty() {
        let result = choice(vec![]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_number_float() {
        let result = number(None, Some(10.0), Some(0.0)).await.unwrap();
        assert!(result.number >= 0.0 && result.number < 10.0);
    }

    #[tokio::test]
    async fn test_number_integer() {
        let result = number(Some(true), Some(10.0), Some(1.0)).await.unwrap();
        assert!(result.number >= 1.0 && result.number <= 10.0);
        assert_eq!(result.number, result.number.floor());
    }

    #[tokio::test]
    async fn test_shuffle() {
        let items: Vec<Value> = (1..=10).map(|n| Value::Number(n.into())).collect();
        let result = shuffle(items.clone()).await.unwrap();
        assert_eq!(result.items.len(), 10);
    }

    #[tokio::test]
    async fn test_string() {
        let result = string(10, None).await.unwrap();
        assert_eq!(result.string.len(), 10);
    }

    #[tokio::test]
    async fn test_string_custom_charset() {
        let result = string(10, Some("abc")).await.unwrap();
        assert_eq!(result.string.len(), 10);
        assert!(result.string.chars().all(|c| "abc".contains(c)));
    }

    #[tokio::test]
    async fn test_uuid_v4() {
        let result = uuid(Some("v4")).await.unwrap();
        assert_eq!(result.uuid.len(), 36);
    }

    #[tokio::test]
    async fn test_uuid_v7() {
        let result = uuid(Some("v7")).await.unwrap();
        assert_eq!(result.uuid.len(), 36);
    }
}

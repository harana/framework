#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_generate_v4() {
        let result = generate_v4(None, None).await.unwrap();
        assert!(!result.uuid.is_empty());
        assert_eq!(result.uuids.len(), 1);
        
        // Validate the generated UUID
        let validate_result = validate(&result.uuid, Some(4)).await.unwrap();
        assert!(validate_result.valid);
        assert_eq!(validate_result.version, 4);
    }

    #[tokio::test]
    async fn test_generate_v4_multiple() {
        let result = generate_v4(None, Some(5)).await.unwrap();
        assert_eq!(result.uuids.len(), 5);
    }

    #[tokio::test]
    async fn test_generate_v4_uppercase() {
        let result = generate_v4(Some(true), None).await.unwrap();
        assert_eq!(result.uuid, result.uuid.to_uppercase());
    }

    #[tokio::test]
    async fn test_generate_v7() {
        let result = generate_v7(None, None).await.unwrap();
        assert!(!result.uuid.is_empty());
        
        let validate_result = validate(&result.uuid, Some(7)).await.unwrap();
        assert!(validate_result.valid);
        assert_eq!(validate_result.version, 7);
    }

    #[tokio::test]
    async fn test_parse() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let result = parse(uuid).await.unwrap();
        assert_eq!(result.version, 4);
        assert_eq!(result.variant, "RFC4122");
    }

    #[tokio::test]
    async fn test_validate_valid() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let result = validate(uuid, None).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_validate_invalid() {
        let result = validate("not-a-uuid", None).await.unwrap();
        assert!(!result.valid);
    }
}

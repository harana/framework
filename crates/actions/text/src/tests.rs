#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    use serde_json::Value;

    #[tokio::test]
    async fn test_case_convert_snake() {
        let result = case_convert("Hello World", "snake").await.unwrap();
        assert_eq!(result.result, "hello_world");
    }

    #[tokio::test]
    async fn test_case_convert_camel() {
        let result = case_convert("hello_world", "camel").await.unwrap();
        assert_eq!(result.result, "helloWorld");
    }

    #[tokio::test]
    async fn test_case_convert_pascal() {
        let result = case_convert("hello_world", "pascal").await.unwrap();
        assert_eq!(result.result, "HelloWorld");
    }

    #[tokio::test]
    async fn test_case_convert_kebab() {
        let result = case_convert("Hello World", "kebab").await.unwrap();
        assert_eq!(result.result, "hello-world");
    }

    #[tokio::test]
    async fn test_join() {
        let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = join(items, Some(", ")).await.unwrap();
        assert_eq!(result.result, "a, b, c");
    }

    #[tokio::test]
    async fn test_regex_match() {
        let result = regex_match(r"\d+", "abc123def456", None).await.unwrap();
        assert_eq!(result.matches, vec!["123", "456"]);
        assert!(result.matched);
    }

    #[tokio::test]
    async fn test_regex_match_no_match() {
        let result = regex_match(r"\d+", "abcdef", None).await.unwrap();
        assert!(result.matches.is_empty());
        assert!(!result.matched);
    }

    #[tokio::test]
    async fn test_regex_replace() {
        let result = regex_replace("X", "abc123def", r"\d+", Some("g")).await.unwrap();
        assert_eq!(result.result, "abcXdef");
    }

    #[tokio::test]
    async fn test_slugify() {
        let result = slugify("Hello World!", None, None).await.unwrap();
        assert_eq!(result.slug, "hello-world");
    }

    #[tokio::test]
    async fn test_slugify_custom_separator() {
        let result = slugify("Hello World", None, Some("_")).await.unwrap();
        assert_eq!(result.slug, "hello_world");
    }

    #[tokio::test]
    async fn test_split() {
        let result = split(",", "a,b,c", None).await.unwrap();
        assert_eq!(result.parts, vec!["a", "b", "c"]);
    }

    #[tokio::test]
    async fn test_split_limit() {
        let result = split(",", "a,b,c,d", Some(2)).await.unwrap();
        assert_eq!(result.parts, vec!["a", "b,c,d"]);
    }

    #[tokio::test]
    async fn test_template() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::String("World".to_string()));
        let result = template(data, "Hello, {{ name }}!", None).await.unwrap();
        assert_eq!(result.result, "Hello, World!");
    }

    #[tokio::test]
    async fn test_trim() {
        let result = trim("  hello  ", None, None).await.unwrap();
        assert_eq!(result.result, "hello");
    }

    #[tokio::test]
    async fn test_trim_start() {
        let result = trim("  hello  ", None, Some("start")).await.unwrap();
        assert_eq!(result.result, "hello  ");
    }

    #[tokio::test]
    async fn test_truncate() {
        let result = truncate("Hello World", 8, Some("...")).await.unwrap();
        assert_eq!(result.result, "Hello...");
        assert!(result.truncated);
    }

    #[tokio::test]
    async fn test_truncate_no_change() {
        let result = truncate("Hi", 10, None).await.unwrap();
        assert_eq!(result.result, "Hi");
        assert!(!result.truncated);
    }
}

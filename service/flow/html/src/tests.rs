#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;
    use serde_json::json;

    const TEST_HTML: &str = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test Page</title></head>
        <body>
            <div id="main" class="container">
                <h1>Hello World</h1>
                <p class="content">This is a paragraph.</p>
                <p class="content">Another paragraph.</p>
                <a href="https://example.com">Link</a>
                <!-- This is a comment -->
            </div>
        </body>
        </html>
    "#;

    // css_select tests
    #[tokio::test]
    async fn test_css_select_tag() {
        let result = css_select(TEST_HTML, "h1", None).await.unwrap();
        assert_eq!(result.results.len(), 1);
        assert!(result.results[0]["text"].as_str().unwrap().contains("Hello World"));
    }

    #[tokio::test]
    async fn test_css_select_class() {
        let result = css_select(TEST_HTML, ".content", None).await.unwrap();
        assert_eq!(result.results.len(), 2);
    }

    #[tokio::test]
    async fn test_css_select_id() {
        let result = css_select(TEST_HTML, "#main", None).await.unwrap();
        assert_eq!(result.results.len(), 1);
    }

    #[tokio::test]
    async fn test_css_select_attribute() {
        let result = css_select(TEST_HTML, "a", Some("href")).await.unwrap();
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0].as_str().unwrap(), "https://example.com");
    }

    #[tokio::test]
    async fn test_css_select_no_match() {
        let result = css_select(TEST_HTML, ".nonexistent", None).await.unwrap();
        assert_eq!(result.results.len(), 0);
    }

    #[tokio::test]
    async fn test_css_select_invalid_selector() {
        // scraper may be lenient with selectors, so just verify it handles edge cases
        let result = css_select(TEST_HTML, ":::", None).await;
        // Either returns an error or empty results for invalid selectors
        match result {
            Ok(output) => assert!(output.results.is_empty()),
            Err(_) => {} // Error is also acceptable
        }
    }

    // extract_text tests
    #[tokio::test]
    async fn test_extract_text() {
        let result = extract_text(TEST_HTML, None, None).await.unwrap();
        assert!(result.text.contains("Hello World"));
        assert!(result.text.contains("This is a paragraph"));
    }

    #[tokio::test]
    async fn test_extract_text_with_separator() {
        let result = extract_text("<p>One</p><p>Two</p><p>Three</p>", None, Some("\n")).await.unwrap();
        assert!(result.text.contains("One"));
        assert!(result.text.contains("Two"));
        assert!(result.text.contains("Three"));
    }

    #[tokio::test]
    async fn test_extract_text_empty() {
        let result = extract_text("<div></div>", None, None).await.unwrap();
        assert!(result.text.is_empty() || result.text.trim().is_empty());
    }

    // minify tests
    #[tokio::test]
    async fn test_minify_removes_comments() {
        let html = "<div><!-- comment -->content</div>";
        let result = minify(html, None, None, Some(true), None).await.unwrap();
        assert!(!result.html.contains("comment"));
        assert!(result.html.contains("content"));
    }

    #[tokio::test]
    async fn test_minify_preserves_content() {
        let html = "<div>Hello World</div>";
        let result = minify(html, None, None, None, None).await.unwrap();
        assert!(result.html.contains("Hello World"));
    }

    #[tokio::test]
    async fn test_minify_collapses_whitespace() {
        let html = "<div>   Multiple    Spaces   </div>";
        let result = minify(html, None, None, None, Some(true)).await.unwrap();
        assert!(!result.html.contains("   "));
    }

    #[tokio::test]
    async fn test_minify_keeps_comments_when_disabled() {
        let html = "<div><!-- keep me -->content</div>";
        let result = minify(html, None, None, Some(false), None).await.unwrap();
        assert!(result.html.contains("keep me"));
    }

    // parse tests
    #[tokio::test]
    async fn test_parse_document() {
        let result = parse("<html><body><p>Test</p></body></html>", None).await.unwrap();
        assert!(!result.result.is_empty());
        assert!(result.result.contains_key("tag") || result.result.contains_key("children"));
    }

    #[tokio::test]
    async fn test_parse_fragment() {
        let result = parse("<p>Test paragraph</p>", Some(true)).await.unwrap();
        assert!(!result.result.is_empty());
    }

    #[tokio::test]
    async fn test_parse_nested() {
        let result = parse("<div><span>Nested</span></div>", Some(true)).await.unwrap();
        assert!(!result.result.is_empty());
    }

    // sanitize tests
    #[tokio::test]
    async fn test_sanitize_removes_script() {
        let html = "<div>Safe<script>alert('xss')</script></div>";
        let result = sanitize(html, None, None, None).await.unwrap();
        assert!(!result.html.contains("script"));
        assert!(!result.html.contains("alert"));
        assert!(result.html.contains("Safe"));
    }

    #[tokio::test]
    async fn test_sanitize_removes_onclick() {
        let html = r#"<button onclick="alert('xss')">Click</button>"#;
        let result = sanitize(html, None, None, None).await.unwrap();
        assert!(!result.html.contains("onclick"));
    }

    #[tokio::test]
    async fn test_sanitize_allowed_tags() {
        let html = "<div><p>Keep</p><script>Remove</script></div>";
        let result = sanitize(html, Some(vec!["p".to_string()]), None, None).await.unwrap();
        assert!(result.html.contains("<p>"));
        assert!(!result.html.contains("script"));
    }

    #[tokio::test]
    async fn test_sanitize_strips_comments() {
        let html = "<div><!-- comment -->Content</div>";
        let result = sanitize(html, None, Some(true), None).await.unwrap();
        assert!(!result.html.contains("comment"));
        assert!(result.html.contains("Content"));
    }

    #[tokio::test]
    async fn test_sanitize_preserves_safe_content() {
        let html = "<p>Hello <strong>World</strong></p>";
        let result = sanitize(html, None, None, None).await.unwrap();
        assert!(result.html.contains("Hello"));
        assert!(result.html.contains("World"));
    }

    #[tokio::test]
    async fn test_sanitize_with_allowed_attributes() {
        let html = r#"<a href="https://example.com" onclick="bad()">Link</a>"#;
        let mut attrs = HashMap::new();
        attrs.insert("a".to_string(), json!(["href"]));
        let result = sanitize(html, None, None, Some(attrs)).await.unwrap();
        assert!(result.html.contains("href"));
        assert!(!result.html.contains("onclick"));
    }

    // Edge cases
    #[tokio::test]
    async fn test_empty_html() {
        let result = css_select("", "div", None).await.unwrap();
        assert!(result.results.is_empty());
    }

    #[tokio::test]
    async fn test_malformed_html() {
        // HTML5 parser should handle malformed HTML gracefully
        let result = css_select("<div><p>Unclosed", "p", None).await.unwrap();
        assert_eq!(result.results.len(), 1);
    }
}

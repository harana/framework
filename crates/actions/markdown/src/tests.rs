#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_to_html_basic() {
        let result = to_html("# Hello World", None, None, None).await.unwrap();
        assert!(result.html.contains("<h1>"));
        assert!(result.html.contains("Hello World"));
    }

    #[tokio::test]
    async fn test_to_html_paragraph() {
        let result = to_html("This is a paragraph.", None, None, None)
            .await
            .unwrap();
        assert!(result.html.contains("<p>"));
        assert!(result.html.contains("This is a paragraph."));
    }

    #[tokio::test]
    async fn test_to_html_gfm_tables() {
        let markdown = r#"
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
"#;
        let result = to_html(markdown, None, None, Some(true)).await.unwrap();
        assert!(result.html.contains("<table>"));
        assert!(result.html.contains("<th>"));
    }

    #[tokio::test]
    async fn test_to_html_gfm_strikethrough() {
        let result = to_html("~~strikethrough~~", None, None, Some(true))
            .await
            .unwrap();
        assert!(result.html.contains("<del>"));
    }

    #[tokio::test]
    async fn test_to_html_gfm_tasklist() {
        let markdown = r#"
- [x] Task 1
- [ ] Task 2
"#;
        let result = to_html(markdown, None, None, Some(true)).await.unwrap();
        assert!(result.html.contains("checked"));
    }

    #[tokio::test]
    async fn test_to_html_code_block() {
        let markdown = r#"
```rust
fn main() {
    println!("Hello");
}
```
"#;
        let result = to_html(markdown, None, None, None).await.unwrap();
        assert!(result.html.contains("<code"));
        assert!(result.html.contains("fn main"));
    }

    #[tokio::test]
    async fn test_to_html_links() {
        let result = to_html("[Link](https://example.com)", None, None, None)
            .await
            .unwrap();
        assert!(result.html.contains("<a"));
        assert!(result.html.contains("href=\"https://example.com\""));
    }

    #[tokio::test]
    async fn test_from_html_basic() {
        let result = from_html("<h1>Hello World</h1>", None, None, None)
            .await
            .unwrap();
        assert!(result.markdown.contains("Hello World"));
        // html2md may use different heading styles (# or underline)
        assert!(
            result.markdown.contains("#") || result.markdown.contains("="),
            "Expected heading marker, got: {}",
            result.markdown
        );
    }

    #[tokio::test]
    async fn test_from_html_paragraph() {
        let result = from_html("<p>This is a paragraph.</p>", None, None, None)
            .await
            .unwrap();
        assert!(result.markdown.contains("This is a paragraph."));
    }

    #[tokio::test]
    async fn test_from_html_list() {
        let html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        let result = from_html(html, None, None, None).await.unwrap();
        assert!(result.markdown.contains("Item 1"));
        assert!(result.markdown.contains("Item 2"));
    }

    #[tokio::test]
    async fn test_from_html_link() {
        let html = "<a href=\"https://example.com\">Link</a>";
        let result = from_html(html, None, None, None).await.unwrap();
        assert!(result.markdown.contains("[Link]"));
        assert!(result.markdown.contains("https://example.com"));
    }

    #[tokio::test]
    async fn test_extract_frontmatter_basic() {
        let markdown = r#"---
title: Hello World
author: John Doe
---

# Content

This is the content."#;

        let result = extract_frontmatter(markdown).await.unwrap();
        assert!(result.frontmatter.contains_key("title"));
        assert!(result.frontmatter.contains_key("author"));
        assert_eq!(result.frontmatter["title"], "Hello World");
        assert_eq!(result.frontmatter["author"], "John Doe");
        assert!(result.content.contains("# Content"));
    }

    #[tokio::test]
    async fn test_extract_frontmatter_no_frontmatter() {
        let markdown = "# Hello World\n\nThis is content.";
        let result = extract_frontmatter(markdown).await.unwrap();
        assert!(result.frontmatter.is_empty());
        assert_eq!(result.content, markdown);
    }

    #[tokio::test]
    async fn test_extract_frontmatter_complex() {
        let markdown = r#"---
title: Complex Example
tags:
  - rust
  - markdown
date: 2024-01-15
draft: false
---

# Main Content

More text here."#;

        let result = extract_frontmatter(markdown).await.unwrap();
        assert_eq!(result.frontmatter["title"], "Complex Example");
        assert!(result.frontmatter.contains_key("tags"));
        assert!(result.frontmatter.contains_key("date"));
        assert_eq!(result.frontmatter["draft"], false);
        assert!(result.content.contains("# Main Content"));
    }

    #[tokio::test]
    async fn test_extract_frontmatter_empty_content() {
        let markdown = r#"---
title: Empty
---
"#;

        let result = extract_frontmatter(markdown).await.unwrap();
        assert_eq!(result.frontmatter["title"], "Empty");
        assert!(result.content.is_empty() || result.content.trim().is_empty());
    }

    #[tokio::test]
    async fn test_roundtrip_simple() {
        let markdown = "# Hello\n\nWorld";
        let html_result = to_html(markdown, None, None, None).await.unwrap();
        let md_result = from_html(&html_result.html, None, None, None)
            .await
            .unwrap();

        // The markdown should contain the essential content
        assert!(md_result.markdown.contains("Hello"));
        assert!(md_result.markdown.contains("World"));
    }
}

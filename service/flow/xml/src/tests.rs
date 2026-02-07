#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_parse_simple() {
        let xml = r#"<root><name>Alice</name><age>30</age></root>"#;
        let result = parse(xml, None, None).await.unwrap();
        
        assert_eq!(result.result["root"]["name"], "Alice");
        assert_eq!(result.result["root"]["age"], "30");
    }

    #[tokio::test]
    async fn test_parse_with_attributes() {
        let xml = r#"<person id="123" active="true"><name>Bob</name></person>"#;
        let result = parse(xml, Some(true), None).await.unwrap();
        
        assert_eq!(result.result["person"]["@id"], "123");
        assert_eq!(result.result["person"]["@active"], "true");
        assert_eq!(result.result["person"]["name"], "Bob");
    }

    #[tokio::test]
    async fn test_parse_without_attributes() {
        let xml = r#"<person id="123"><name>Bob</name></person>"#;
        let result = parse(xml, Some(false), None).await.unwrap();
        
        assert!(result.result["person"].get("@id").is_none());
        assert_eq!(result.result["person"]["name"], "Bob");
    }

    #[tokio::test]
    async fn test_parse_nested() {
        let xml = r#"<root><parent><child>value</child></parent></root>"#;
        let result = parse(xml, None, None).await.unwrap();
        
        assert_eq!(result.result["root"]["parent"]["child"], "value");
    }

    #[tokio::test]
    async fn test_parse_multiple_siblings() {
        let xml = r#"<root><item>one</item><item>two</item><item>three</item></root>"#;
        let result = parse(xml, None, None).await.unwrap();
        
        let items = result.result["root"]["item"].as_array().unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "one");
        assert_eq!(items[1], "two");
        assert_eq!(items[2], "three");
    }

    #[tokio::test]
    async fn test_parse_empty_element() {
        let xml = r#"<root><empty/></root>"#;
        let result = parse(xml, None, None).await.unwrap();
        
        assert!(result.result["root"]["empty"].is_null());
    }

    #[tokio::test]
    async fn test_generate_simple() {
        let data = json!({
            "root": {
                "name": "Alice",
                "age": "30"
            }
        });
        
        let result = generate(data, Some(false), None, Some(false)).await.unwrap();
        
        assert!(result.xml.contains("<root>"));
        assert!(result.xml.contains("<name>Alice</name>"));
        assert!(result.xml.contains("<age>30</age>"));
        assert!(result.xml.contains("</root>"));
    }

    #[tokio::test]
    async fn test_generate_with_declaration() {
        let data = json!({
            "root": "content"
        });
        
        let result = generate(data, Some(false), None, Some(true)).await.unwrap();
        
        assert!(result.xml.contains("<?xml"));
        assert!(result.xml.contains("UTF-8"));
    }

    #[tokio::test]
    async fn test_generate_without_declaration() {
        let data = json!({
            "root": "content"
        });
        
        let result = generate(data, Some(false), None, Some(false)).await.unwrap();
        
        assert!(!result.xml.contains("<?xml"));
    }

    #[tokio::test]
    async fn test_generate_with_attributes() {
        let data = json!({
            "person": {
                "@id": "123",
                "name": "Bob"
            }
        });
        
        let result = generate(data, Some(false), None, Some(false)).await.unwrap();
        
        assert!(result.xml.contains("id=\"123\""));
        assert!(result.xml.contains("<name>Bob</name>"));
    }

    #[tokio::test]
    async fn test_generate_array() {
        let data = json!({
            "root": {
                "item": ["one", "two", "three"]
            }
        });
        
        let result = generate(data, Some(false), None, Some(false)).await.unwrap();
        
        assert!(result.xml.contains("<item>one</item>"));
        assert!(result.xml.contains("<item>two</item>"));
        assert!(result.xml.contains("<item>three</item>"));
    }

    #[tokio::test]
    async fn test_generate_empty_element() {
        let data = json!({
            "root": {
                "empty": null
            }
        });
        
        let result = generate(data, Some(false), None, Some(false)).await.unwrap();
        
        assert!(result.xml.contains("<empty/>"));
    }

    #[tokio::test]
    async fn test_validate_valid_xml() {
        let xml = r#"<root><child>content</child></root>"#;
        let result = validate(xml, "").await.unwrap();
        
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_invalid_xml() {
        let xml = r#"<root><child>content</root>"#; // Missing closing tag
        let result = validate(xml, "").await.unwrap();
        
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_xpath_query_simple() {
        let xml = r#"<root><name>Alice</name></root>"#;
        let result = xpath_query(xml, "/root/name", None).await.unwrap();
        
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0], "Alice");
    }

    #[tokio::test]
    async fn test_xpath_query_recursive() {
        let xml = r#"<root><a><name>one</name></a><b><name>two</name></b></root>"#;
        let result = xpath_query(xml, "//name", None).await.unwrap();
        
        assert_eq!(result.results.len(), 2);
    }

    #[tokio::test]
    async fn test_roundtrip_parse_generate() {
        let original = json!({
            "person": {
                "@id": "123",
                "name": "Alice",
                "age": "30"
            }
        });
        
        // Generate XML
        let generated = generate(original.clone(), Some(false), None, Some(false)).await.unwrap();
        
        // Parse it back
        let parsed = parse(&generated.xml, Some(true), None).await.unwrap();
        
        assert_eq!(parsed.result["person"]["@id"], "123");
        assert_eq!(parsed.result["person"]["name"], "Alice");
        assert_eq!(parsed.result["person"]["age"], "30");
    }

    #[tokio::test]
    async fn test_parse_cdata() {
        let xml = r#"<root><content><![CDATA[<special>&characters</special>]]></content></root>"#;
        let result = parse(xml, None, None).await.unwrap();
        
        assert_eq!(result.result["root"]["content"], "<special>&characters</special>");
    }

    #[tokio::test]
    async fn test_generate_indented() {
        let data = json!({
            "root": {
                "child": "value"
            }
        });
        
        let result = generate(data, Some(true), None, Some(false)).await.unwrap();
        
        // Indented XML should have newlines
        assert!(result.xml.contains('\n'));
    }
}

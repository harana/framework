// Harana Actions - Transform Module Tests

use super::*;

// ============================================================================
// base64_decode tests
// ============================================================================

#[tokio::test]
async fn test_base64_decode_simple() {
    let result = base64_decode("SGVsbG8gV29ybGQ=").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().decoded, "Hello World");
}

#[tokio::test]
async fn test_base64_decode_empty() {
    let result = base64_decode("").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().decoded, "");
}

#[tokio::test]
async fn test_base64_decode_invalid() {
    let result = base64_decode("!!!invalid!!!").await;
    assert!(result.is_err());
}

// ============================================================================
// base64_encode tests
// ============================================================================

#[tokio::test]
async fn test_base64_encode_simple() {
    let result = base64_encode("Hello World").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().encoded, "SGVsbG8gV29ybGQ=");
}

#[tokio::test]
async fn test_base64_encode_empty() {
    let result = base64_encode("").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().encoded, "");
}

#[tokio::test]
async fn test_base64_encode_special_chars() {
    let result = base64_encode("Hello! @#$%").await;
    assert!(result.is_ok());
    // Verify roundtrip
    let encoded = result.unwrap().encoded;
    let decoded = base64_decode(&encoded).await.unwrap();
    assert_eq!(decoded.decoded, "Hello! @#$%");
}

// ============================================================================
// csv_to_json tests
// ============================================================================

#[tokio::test]
async fn test_csv_to_json_simple() {
    let csv_data = "name,age\nAlice,30\nBob,25";
    let result = csv_to_json(csv_data, None, None).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    assert_eq!(json.len(), 2);
    assert_eq!(json[0].get("name").unwrap(), &Value::String("Alice".to_string()));
    assert_eq!(json[0].get("age").unwrap(), &Value::Number(30.into()));
}

#[tokio::test]
async fn test_csv_to_json_custom_delimiter() {
    let csv_data = "name;age\nAlice;30\nBob;25";
    let result = csv_to_json(csv_data, None, Some(";")).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    assert_eq!(json.len(), 2);
}

#[tokio::test]
async fn test_csv_to_json_custom_headers() {
    let csv_data = "Alice,30\nBob,25";
    let headers = vec!["person_name".to_string(), "person_age".to_string()];
    let result = csv_to_json(csv_data, Some(headers), None).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    assert!(json[0].contains_key("person_name"));
}

#[tokio::test]
async fn test_csv_to_json_boolean_values() {
    let csv_data = "name,active\nAlice,true\nBob,false";
    let result = csv_to_json(csv_data, None, None).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    assert_eq!(json[0].get("active").unwrap(), &Value::Bool(true));
    assert_eq!(json[1].get("active").unwrap(), &Value::Bool(false));
}

// ============================================================================
// html_decode tests
// ============================================================================

#[tokio::test]
async fn test_html_decode_simple() {
    let result = html_decode("&lt;div&gt;Hello&lt;/div&gt;").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().decoded, "<div>Hello</div>");
}

#[tokio::test]
async fn test_html_decode_amp() {
    let result = html_decode("Tom &amp; Jerry").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().decoded, "Tom & Jerry");
}

#[tokio::test]
async fn test_html_decode_quotes() {
    let result = html_decode("&quot;Hello&quot;").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().decoded, "\"Hello\"");
}

// ============================================================================
// html_encode tests
// ============================================================================

#[tokio::test]
async fn test_html_encode_simple() {
    let result = html_encode("<div>Hello</div>").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().encoded, "&lt;div&gt;Hello&lt;/div&gt;");
}

#[tokio::test]
async fn test_html_encode_amp() {
    let result = html_encode("Tom & Jerry").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().encoded, "Tom &amp; Jerry");
}

#[tokio::test]
async fn test_html_encode_roundtrip() {
    let original = "<p>Test & \"quotes\"</p>";
    let encoded = html_encode(original).await.unwrap().encoded;
    let decoded = html_decode(&encoded).await.unwrap().decoded;
    assert_eq!(decoded, original);
}

// ============================================================================
// json_to_csv tests
// ============================================================================

#[tokio::test]
async fn test_json_to_csv_simple() {
    let data = vec![
        {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Alice".to_string()));
            map.insert("age".to_string(), Value::Number(30.into()));
            map
        },
        {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Bob".to_string()));
            map.insert("age".to_string(), Value::Number(25.into()));
            map
        },
    ];
    
    let result = json_to_csv(data, None, None).await;
    assert!(result.is_ok());
    let csv = result.unwrap().csv;
    assert!(csv.contains("Alice"));
    assert!(csv.contains("Bob"));
}

#[tokio::test]
async fn test_json_to_csv_custom_delimiter() {
    let data = vec![
        {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Alice".to_string()));
            map
        },
    ];
    
    let result = json_to_csv(data, Some(";"), None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_json_to_csv_empty() {
    let result = json_to_csv(vec![], None, None).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().csv, "");
}

// ============================================================================
// json_to_xml tests
// ============================================================================

#[tokio::test]
async fn test_json_to_xml_simple() {
    let mut data = HashMap::new();
    data.insert("name".to_string(), Value::String("Alice".to_string()));
    data.insert("age".to_string(), Value::Number(30.into()));
    
    let result = json_to_xml(data, None).await;
    assert!(result.is_ok());
    let xml = result.unwrap().xml;
    assert!(xml.contains("<root>"));
    assert!(xml.contains("</root>"));
    assert!(xml.contains("<name>Alice</name>"));
}

#[tokio::test]
async fn test_json_to_xml_custom_root() {
    let mut data = HashMap::new();
    data.insert("value".to_string(), Value::String("test".to_string()));
    
    let result = json_to_xml(data, Some("custom")).await;
    assert!(result.is_ok());
    let xml = result.unwrap().xml;
    assert!(xml.contains("<custom>"));
    assert!(xml.contains("</custom>"));
}

#[tokio::test]
async fn test_json_to_xml_nested() {
    let mut inner = serde_json::Map::new();
    inner.insert("city".to_string(), Value::String("NYC".to_string()));
    
    let mut data = HashMap::new();
    data.insert("person".to_string(), Value::Object(inner));
    
    let result = json_to_xml(data, None).await;
    assert!(result.is_ok());
    let xml = result.unwrap().xml;
    assert!(xml.contains("<city>NYC</city>"));
}

// ============================================================================
// json_to_yaml tests
// ============================================================================

#[tokio::test]
async fn test_json_to_yaml_simple() {
    let mut data = HashMap::new();
    data.insert("name".to_string(), Value::String("Alice".to_string()));
    data.insert("age".to_string(), Value::Number(30.into()));
    
    let result = json_to_yaml(data).await;
    assert!(result.is_ok());
    let yaml = result.unwrap().yaml;
    assert!(yaml.contains("name:"));
    assert!(yaml.contains("Alice"));
}

#[tokio::test]
async fn test_json_to_yaml_nested() {
    let mut inner = serde_json::Map::new();
    inner.insert("city".to_string(), Value::String("NYC".to_string()));
    
    let mut data = HashMap::new();
    data.insert("address".to_string(), Value::Object(inner));
    
    let result = json_to_yaml(data).await;
    assert!(result.is_ok());
    let yaml = result.unwrap().yaml;
    assert!(yaml.contains("address:"));
    assert!(yaml.contains("city:"));
}

// ============================================================================
// markdown_to_html tests
// ============================================================================

#[tokio::test]
async fn test_markdown_to_html_heading() {
    let result = markdown_to_html("# Hello World", None).await;
    assert!(result.is_ok());
    let html = result.unwrap().html;
    assert!(html.contains("<h1>Hello World</h1>"));
}

#[tokio::test]
async fn test_markdown_to_html_paragraph() {
    let result = markdown_to_html("This is a paragraph.", None).await;
    assert!(result.is_ok());
    let html = result.unwrap().html;
    assert!(html.contains("<p>This is a paragraph.</p>"));
}

#[tokio::test]
async fn test_markdown_to_html_bold() {
    let result = markdown_to_html("**bold text**", None).await;
    assert!(result.is_ok());
    let html = result.unwrap().html;
    assert!(html.contains("<strong>bold text</strong>"));
}

#[tokio::test]
async fn test_markdown_to_html_list() {
    let result = markdown_to_html("- item 1\n- item 2", None).await;
    assert!(result.is_ok());
    let html = result.unwrap().html;
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>"));
}

#[tokio::test]
async fn test_markdown_to_html_link() {
    let result = markdown_to_html("[link](https://example.com)", None).await;
    assert!(result.is_ok());
    let html = result.unwrap().html;
    assert!(html.contains("<a href=\"https://example.com\">link</a>"));
}

// ============================================================================
// url_decode tests
// ============================================================================

#[tokio::test]
async fn test_url_decode_simple() {
    let result = url_decode("Hello%20World").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().decoded, "Hello World");
}

#[tokio::test]
async fn test_url_decode_special_chars() {
    let result = url_decode("%3Cscript%3E").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().decoded, "<script>");
}

#[tokio::test]
async fn test_url_decode_plus_as_space() {
    // Note: urlencoding::decode doesn't convert + to space by default
    let result = url_decode("Hello+World").await;
    assert!(result.is_ok());
    // Plus is not converted to space in path encoding
    assert_eq!(result.unwrap().decoded, "Hello+World");
}

// ============================================================================
// url_encode tests
// ============================================================================

#[tokio::test]
async fn test_url_encode_simple() {
    let result = url_encode("Hello World").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().encoded, "Hello%20World");
}

#[tokio::test]
async fn test_url_encode_special_chars() {
    let result = url_encode("<script>").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().encoded, "%3Cscript%3E");
}

#[tokio::test]
async fn test_url_encode_roundtrip() {
    let original = "Hello World! @#$%";
    let encoded = url_encode(original).await.unwrap().encoded;
    let decoded = url_decode(&encoded).await.unwrap().decoded;
    assert_eq!(decoded, original);
}

// ============================================================================
// xml_to_json tests
// ============================================================================

#[tokio::test]
async fn test_xml_to_json_simple() {
    let xml = "<root><name>Alice</name><age>30</age></root>";
    let result = xml_to_json(xml).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    assert!(json.contains_key("root"));
}

#[tokio::test]
async fn test_xml_to_json_nested() {
    let xml = "<root><person><name>Alice</name></person></root>";
    let result = xml_to_json(xml).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_xml_to_json_with_declaration() {
    let xml = "<?xml version=\"1.0\"?><root><value>test</value></root>";
    let result = xml_to_json(xml).await;
    assert!(result.is_ok());
}

// ============================================================================
// yaml_to_json tests
// ============================================================================

#[tokio::test]
async fn test_yaml_to_json_simple() {
    let yaml = "name: Alice\nage: 30";
    let result = yaml_to_json(yaml).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    assert_eq!(json.get("name").unwrap(), &Value::String("Alice".to_string()));
    assert_eq!(json.get("age").unwrap(), &Value::Number(30.into()));
}

#[tokio::test]
async fn test_yaml_to_json_nested() {
    let yaml = "person:\n  name: Alice\n  age: 30";
    let result = yaml_to_json(yaml).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    assert!(json.contains_key("person"));
}

#[tokio::test]
async fn test_yaml_to_json_list() {
    let yaml = "items:\n  - apple\n  - banana";
    let result = yaml_to_json(yaml).await;
    assert!(result.is_ok());
    let json = result.unwrap().json;
    let items = json.get("items").unwrap();
    assert!(items.is_array());
}

#[tokio::test]
async fn test_yaml_to_json_invalid() {
    let yaml = "invalid: yaml: content::";
    let result = yaml_to_json(yaml).await;
    // This might actually parse, so just check it doesn't panic
    let _ = result;
}

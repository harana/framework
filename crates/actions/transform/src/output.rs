// Harana Actions - Transform Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// base64_decode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64DecodeOutput {
    pub decoded: String
}

// base64_encode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64EncodeOutput {
    pub encoded: String
}

// csv_to_json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvToJsonOutput {
    pub json: Vec<HashMap<String, Value>>
}

// html_decode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlDecodeOutput {
    pub decoded: String
}

// html_encode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlEncodeOutput {
    pub encoded: String
}

// json_to_csv
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonToCsvOutput {
    pub csv: String
}

// json_to_xml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonToXmlOutput {
    pub xml: String
}

// json_to_yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonToYamlOutput {
    pub yaml: String
}

// markdown_to_html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownToHtmlOutput {
    pub html: String
}

// url_decode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDecodeOutput {
    pub decoded: String
}

// url_encode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlEncodeOutput {
    pub encoded: String
}

// xml_to_json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlToJsonOutput {
    pub json: HashMap<String, Value>
}

// yaml_to_json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlToJsonOutput {
    pub json: HashMap<String, Value>
}

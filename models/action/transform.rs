// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonToXmlInput {
    pub data: String,
    pub root_element: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonToXmlOutput {
    pub xml: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlToJsonInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XmlToJsonOutput {
    pub json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvToJsonInput {
    pub data: String,
    pub delimiter: String,
    pub headers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvToJsonOutput {
    pub json: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonToCsvInput {
    pub data: Vec<String>,
    pub delimiter: String,
    pub headers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonToCsvOutput {
    pub csv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct YamlToJsonInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct YamlToJsonOutput {
    pub json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonToYamlInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JsonToYamlOutput {
    pub yaml: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Base64EncodeInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Base64EncodeOutput {
    pub encoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Base64DecodeInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Base64DecodeOutput {
    pub decoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlEncodeInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlEncodeOutput {
    pub encoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlDecodeInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UrlDecodeOutput {
    pub decoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlEncodeInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlEncodeOutput {
    pub encoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlDecodeInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlDecodeOutput {
    pub decoded: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownToHtmlInput {
    pub data: String,
    pub options: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownToHtmlOutput {
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformResult {
    pub input_format: String,
    pub output_format: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformJsonObject {
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownOptions {
    pub gfm: bool,
    pub breaks: bool,
    pub sanitize: bool,
}

#[async_trait]
pub trait TransformAction {
    async fn json_to_xml(&self, input: JsonToXmlInput) -> Result<JsonToXmlOutput, Box<dyn std::error::Error>>;
    async fn xml_to_json(&self, input: XmlToJsonInput) -> Result<XmlToJsonOutput, Box<dyn std::error::Error>>;
    async fn csv_to_json(&self, input: CsvToJsonInput) -> Result<CsvToJsonOutput, Box<dyn std::error::Error>>;
    async fn json_to_csv(&self, input: JsonToCsvInput) -> Result<JsonToCsvOutput, Box<dyn std::error::Error>>;
    async fn yaml_to_json(&self, input: YamlToJsonInput) -> Result<YamlToJsonOutput, Box<dyn std::error::Error>>;
    async fn json_to_yaml(&self, input: JsonToYamlInput) -> Result<JsonToYamlOutput, Box<dyn std::error::Error>>;
    async fn base64_encode(&self, input: Base64EncodeInput) -> Result<Base64EncodeOutput, Box<dyn std::error::Error>>;
    async fn base64_decode(&self, input: Base64DecodeInput) -> Result<Base64DecodeOutput, Box<dyn std::error::Error>>;
    async fn url_encode(&self, input: UrlEncodeInput) -> Result<UrlEncodeOutput, Box<dyn std::error::Error>>;
    async fn url_decode(&self, input: UrlDecodeInput) -> Result<UrlDecodeOutput, Box<dyn std::error::Error>>;
    async fn html_encode(&self, input: HtmlEncodeInput) -> Result<HtmlEncodeOutput, Box<dyn std::error::Error>>;
    async fn html_decode(&self, input: HtmlDecodeInput) -> Result<HtmlDecodeOutput, Box<dyn std::error::Error>>;
    async fn markdown_to_html(&self, input: MarkdownToHtmlInput) -> Result<MarkdownToHtmlOutput, Box<dyn std::error::Error>>;
}

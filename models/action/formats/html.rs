// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub data: String,
    #[serde(default)]
    pub fragment: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SanitizeInput {
    pub allowed_attributes: String,
    pub allowed_tags: Vec<String>,
    pub data: String,
    #[serde(default)]
    pub strip_comments: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SanitizeOutput {
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractTextInput {
    pub data: String,
    #[serde(default)]
    pub preserve_whitespace: bool,
    pub separator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractTextOutput {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CssSelectInput {
    pub attribute: String,
    pub data: String,
    pub selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CssSelectOutput {
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MinifyInput {
    #[serde(default)]
    pub collapse_whitespace: bool,
    pub data: String,
    #[serde(default)]
    pub minify_css: bool,
    #[serde(default)]
    pub minify_js: bool,
    #[serde(default)]
    pub remove_comments: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MinifyOutput {
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlDocument {
    pub content: String,
    pub title: String,
    pub charset: String,
    pub head: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlDom {
    pub document_type: String,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlElement {
    pub tag_name: String,
    pub attributes: std::collections::HashMap<String, String>,
    pub children: Vec<String>,
    pub text_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlNode {
    pub node_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlAllowedAttributes {
    pub global: Vec<String>,
    pub per_tag: std::collections::HashMap<String, String>,
}

#[async_trait]
pub trait HtmlAction {
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn sanitize(&self, input: SanitizeInput) -> Result<SanitizeOutput, Box<dyn std::error::Error>>;
    async fn extract_text(&self, input: ExtractTextInput) -> Result<ExtractTextOutput, Box<dyn std::error::Error>>;
    async fn css_select(&self, input: CssSelectInput) -> Result<CssSelectOutput, Box<dyn std::error::Error>>;
    async fn minify(&self, input: MinifyInput) -> Result<MinifyOutput, Box<dyn std::error::Error>>;
}

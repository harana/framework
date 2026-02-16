// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    async fn parse(&self, data: String, fragment: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn sanitize(&self, allowed_attributes: String, allowed_tags: Vec<String>, data: String, strip_comments: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn extract_text(&self, data: String, preserve_whitespace: bool, separator: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn css_select(&self, attribute: String, data: String, selector: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn minify(&self, collapse_whitespace: bool, data: String, minify_css: bool, minify_js: bool, remove_comments: bool) -> Result<String, Box<dyn std::error::Error>>;
}

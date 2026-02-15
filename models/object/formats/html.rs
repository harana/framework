// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlDocument {
    pub charset: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<String>,
    #[serde(default)]
    pub is_sanitized: bool,
    pub size: i64,
    pub source_path: Option<String>,
    pub title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl HtmlDocument {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlDom {
    pub document_type: String,
    pub root: String,
}

impl HtmlDom {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlElement {
    pub tag_name: String,
    pub attributes: std::collections::HashMap<String, String>,
    pub children: Vec<String>,
    pub text_content: String,
}

impl HtmlElement {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlNode {
    pub node_type: String,
    pub value: String,
}

impl HtmlNode {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HtmlAllowedAttributes {
    pub global: Vec<String>,
    pub per_tag: std::collections::HashMap<String, String>,
}

impl HtmlAllowedAttributes {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebDriverCapabilities {
    pub accept_insecure_certs: bool,
    pub browser_name: String,
    pub browser_version: String,
    pub extra: std::collections::HashMap<String, String>,
    pub headless: bool,
    pub page_load_strategy: String,
    pub platform_name: String,
    pub proxy: String,
    pub window_height: i64,
    pub window_width: i64,
}

impl WebDriverCapabilities {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebDriverProxy {
    pub auto_detect: bool,
    pub ftp_proxy: String,
    pub http_proxy: String,
    pub no_proxy: Vec<String>,
    pub proxy_type: String,
    pub ssl_proxy: String,
}

impl WebDriverProxy {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebDriverElement {
    pub class_name: String,
    pub element_id: String,
    pub id: String,
    pub tag_name: String,
    pub text: String,
    pub value: String,
}

impl WebDriverElement {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebDriverCookie {
    pub domain: String,
    pub expiry: i64,
    pub http_only: bool,
    pub name: String,
    pub path: String,
    pub same_site: String,
    pub secure: bool,
    pub value: String,
}

impl WebDriverCookie {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebDriverAction {
    pub action_type: String,
    pub duration_ms: i64,
    pub element_id: String,
    pub key: String,
    pub x: i64,
    pub y: i64,
}

impl WebDriverAction {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathQueryResult {
    pub elements: Vec<String>,
    pub error: String,
    pub found: bool,
    pub total: i64,
    pub xpath: String,
}

impl XPathQueryResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ElementRect {
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}

impl ElementRect {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WindowRect {
    pub height: i64,
    pub width: i64,
    pub x: i64,
    pub y: i64,
}

impl WindowRect {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


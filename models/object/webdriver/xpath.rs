// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathEntry {
    pub description: String,
    pub label: String,
    pub xpath: String,
}

impl XPathEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathCollection {
    pub entry_count: i64,
    pub name: String,
}

impl XPathCollection {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathTextResult {
    pub error: String,
    pub found: bool,
    pub texts: Vec<String>,
    pub total: i64,
    pub xpath: String,
}

impl XPathTextResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathAttributeResult {
    pub attribute: String,
    pub error: String,
    pub found: bool,
    pub total: i64,
    pub values: Vec<String>,
    pub xpath: String,
}

impl XPathAttributeResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathTestResult {
    pub count: i64,
    pub found: bool,
    pub xpath: String,
}

impl XPathTestResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathCountResult {
    pub count: i64,
    pub xpath: String,
}

impl XPathCountResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathActionResult {
    pub affected: i64,
    pub error: String,
    pub success: bool,
    pub xpath: String,
}

impl XPathActionResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathKeysInput {
    pub keys: String,
    pub xpath: String,
}

impl XPathKeysInput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


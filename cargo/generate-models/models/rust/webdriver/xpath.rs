// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// x_path_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathEntry {
    pub description: String,
    pub label: String,
    pub xpath: String,
}

impl XPathEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// x_path_collection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathCollection {
    pub entry_count: i64,
    pub name: String,
}

impl XPathCollection {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// x_path_text_result
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// x_path_attribute_result
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// x_path_test_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathTestResult {
    pub count: i64,
    pub found: bool,
    pub xpath: String,
}

impl XPathTestResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// x_path_count_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathCountResult {
    pub count: i64,
    pub xpath: String,
}

impl XPathCountResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// x_path_action_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathActionResult {
    pub affected: i64,
    pub error: String,
    pub success: bool,
    pub xpath: String,
}

impl XPathActionResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// x_path_keys_input
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XPathKeysInput {
    pub keys: String,
    pub xpath: String,
}

impl XPathKeysInput {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


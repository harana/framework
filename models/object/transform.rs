// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformJob {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error_message: Option<String>,
    pub input_format: String,
    pub input_size: Option<i64>,
    pub output_format: String,
    pub output_size: Option<i64>,
    pub status: String,
    pub transform_type: String,
}

impl TransformJob {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformResult {
    pub input_format: String,
    pub output_format: String,
    pub data: String,
}

impl TransformResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransformJsonObject {
    pub data: std::collections::HashMap<String, String>,
}

impl TransformJsonObject {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownOptions {
    pub gfm: bool,
    pub breaks: bool,
    pub sanitize: bool,
}

impl MarkdownOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkersAiModel {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_name: String,
    pub task_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkersAiModel {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkersAiInference {
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub input_tokens: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub output_tokens: Option<i64>,
    pub status: String,
    #[serde(default)]
    pub stream: bool,
}

impl CloudflareWorkersAiInference {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cf_workers_ai_model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkersAiModel {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_name: String,
    pub task_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CfWorkersAiModel {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cf_workers_ai_inference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkersAiInference {
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub input_tokens: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
    /// Reference: cf_workers_ai_model.id
    pub model_id: String,
    pub output_tokens: Option<i64>,
    pub status: String,
    #[serde(default)]
    pub stream: bool,
}

impl CfWorkersAiInference {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


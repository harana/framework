// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// run_process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunProcess {
    pub args: Option<String>,
    pub command: String,
    pub cpu_usage: Option<f64>,
    pub environment: Option<String>,
    pub exit_code: Option<i64>,
    #[serde(default)]
    pub is_detached: bool,
    pub memory_usage: Option<i64>,
    pub process_id: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub stopped_at: Option<chrono::DateTime<chrono::Utc>>,
    pub working_directory: Option<String>,
}

impl RunProcess {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// run_process_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunProcessLog {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: run_process.id
    pub process_id: String,
    pub stream: String,
}

impl RunProcessLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


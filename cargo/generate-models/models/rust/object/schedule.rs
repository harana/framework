// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cron_expression: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub interval_seconds: Option<i64>,
    pub last_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: Option<String>,
    pub name: String,
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub schedule_type: String,
    pub timezone: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Schedule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// schedule_execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleExecution {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub output: Option<String>,
    /// Reference: schedule.id
    pub schedule_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub triggered_by: String,
}

impl ScheduleExecution {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


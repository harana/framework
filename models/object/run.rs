// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunProcessLog {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub process_id: String,
    pub stream: String,
}

impl RunProcessLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProcessStatus {
    pub process_id: i64,
    pub running: bool,
    pub exit_code: i64,
    pub cpu_usage: f64,
    pub memory_usage: i64,
    pub uptime: i64,
}

impl ProcessStatus {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProcessInfo {
    pub process_id: i64,
    pub name: String,
    pub command: String,
    pub user: String,
    pub cpu_usage: f64,
    pub memory_usage: i64,
    pub status: String,
    pub uptime: i64,
}

impl ProcessInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


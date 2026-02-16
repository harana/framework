// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartOutput {
    pub process_id: i64,
    pub started: bool,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopOutput {
    pub stopped: bool,
    pub exit_code: i64,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KillOutput {
    pub killed: bool,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusOutput {
    pub status: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitOutput {
    pub completed: bool,
    pub exit_code: i64,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OutputOutput {
    pub output: String,
    pub error: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunProcess {
    pub args: String,
    pub command: String,
    pub cpu_usage: f64,
    pub environment: String,
    pub exit_code: i64,
    #[serde(default)]
    pub is_detached: bool,
    pub memory_usage: i64,
    pub process_id: i64,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub stopped_at: chrono::DateTime<chrono::Utc>,
    pub working_directory: String,
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

#[async_trait]
pub trait RunAction {
    async fn start(&self, command: String, args: Vec<String>, working_directory: String, environment: std::collections::HashMap<String, String>, detach: bool) -> Result<StartOutput, Box<dyn std::error::Error>>;
    async fn stop(&self, process_id: i64, force: bool, timeout: i64) -> Result<StopOutput, Box<dyn std::error::Error>>;
    async fn kill(&self, process_id: i64, signal: String) -> Result<KillOutput, Box<dyn std::error::Error>>;
    async fn status(&self, process_id: i64) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn list(&self, filter: String, user: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn wait(&self, process_id: i64, timeout: i64) -> Result<WaitOutput, Box<dyn std::error::Error>>;
    async fn output(&self, process_id: i64, stream: String) -> Result<OutputOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartInput {
    pub command: String,
    pub args: Vec<String>,
    pub working_directory: String,
    pub environment: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub detach: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartOutput {
    pub process_id: i64,
    pub started: bool,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopInput {
    pub process_id: i64,
    #[serde(default)]
    pub force: bool,
    pub timeout: i64,
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
pub struct KillInput {
    pub process_id: i64,
    pub signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KillOutput {
    pub killed: bool,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusInput {
    pub process_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusOutput {
    pub status: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListInput {
    pub filter: String,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListOutput {
    pub processes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitInput {
    pub process_id: i64,
    pub timeout: i64,
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
pub struct OutputInput {
    pub process_id: i64,
    pub stream: String,
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

#[async_trait]
pub trait RunAction {
    async fn start(&self, input: StartInput) -> Result<StartOutput, Box<dyn std::error::Error>>;
    async fn stop(&self, input: StopInput) -> Result<StopOutput, Box<dyn std::error::Error>>;
    async fn kill(&self, input: KillInput) -> Result<KillOutput, Box<dyn std::error::Error>>;
    async fn status(&self, input: StatusInput) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn list(&self, input: ListInput) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn wait(&self, input: WaitInput) -> Result<WaitOutput, Box<dyn std::error::Error>>;
    async fn output(&self, input: OutputInput) -> Result<OutputOutput, Box<dyn std::error::Error>>;
}

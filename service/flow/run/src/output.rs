// Harana Actions - Run Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOutput {
    pub process_id: u32,
    pub started: bool,
    pub error: Option<String>,
}

// stop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopOutput {
    pub stopped: bool,
    pub exit_code: Option<i32>,
    pub error: Option<String>,
}

// kill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillOutput {
    pub killed: bool,
    pub error: Option<String>,
}

// status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOutput {
    pub status: ProcessStatus,
    pub error: Option<String>,
}

// list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOutput {
    pub processes: Vec<ProcessInfo>,
}

// wait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitOutput {
    pub completed: bool,
    pub exit_code: Option<i32>,
    pub error: Option<String>,
}

// output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputOutput {
    pub output: String,
    pub error: Option<String>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessStatus {
    pub process_id: u32,
    pub running: bool,
    pub exit_code: Option<i32>,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<i64>,
    pub uptime_secs: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub process_id: u32,
    pub name: String,
    pub command: String,
    pub user: Option<String>,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<i64>,
    pub status: String,
    pub uptime_secs: Option<i64>,
}

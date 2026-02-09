use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOutput {
    pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResultOutput {
    pub completed: bool,
    pub output: String,
    pub error: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusOutput {
    pub completed_at: String,
    pub started_at: String,
    pub error: String,
    pub current_step: String,
    pub status: String,
    pub progress: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryOutput {
    pub total: i32,
    pub events: Vec<HashMap<String, Value>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExecutionsOutput {
    pub executions: Vec<HashMap<String, Value>>,
    pub total: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseOutput {
    pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeOutput {
    pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStepOutput {
    pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalOutput {
    pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipStepOutput {
    pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOutput {
    pub success: bool,
    pub execution_id: String,
    pub status: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateAllOutput {
    pub terminated_count: i32,
    pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForEventOutput {
    pub timed_out: bool,
    pub payload: String,
    pub received: bool
}

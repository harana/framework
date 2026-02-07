// Harana Actions - Workflow Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// cancel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOutput {
    pub success: bool
}

// get_result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResultOutput {
    pub completed: bool,
    pub output: String,
    pub error: String
}

// get_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusOutput {
    pub completed_at: String,
    pub started_at: String,
    pub error: String,
    pub current_step: String,
    pub status: String,
    pub progress: f64
}

// history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryOutput {
    pub total: i32,
    pub events: Vec<HashMap<String, Value>>
}

// list_executions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExecutionsOutput {
    pub executions: Vec<HashMap<String, Value>>,
    pub total: i32
}

// pause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseOutput {
    pub success: bool
}

// resume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeOutput {
    pub success: bool
}

// retry_step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStepOutput {
    pub success: bool
}

// signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalOutput {
    pub success: bool
}

// skip_step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipStepOutput {
    pub success: bool
}

// start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOutput {
    pub success: bool,
    pub execution_id: String,
    pub status: String
}

// terminate_all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateAllOutput {
    pub terminated_count: i32,
    pub success: bool
}

// wait_for_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForEventOutput {
    pub timed_out: bool,
    pub payload: String,
    pub received: bool
}

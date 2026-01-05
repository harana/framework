// Harana Actions - Workflow Module Output Types
// Auto-generated output structs for Workflow action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// start_workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartWorkflowOutput {
    pub execution_id: String,
    pub status: String,
    pub success: bool,
}

// pause_workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseWorkflowOutput {
    pub success: bool,
}

// resume_workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeWorkflowOutput {
    pub success: bool,
}

// cancel_workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelWorkflowOutput {
    pub success: bool,
}

// get_workflow_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorkflowStatusOutput {
    pub completed_at: Option<String>, // datetime
    pub current_step: String,
    pub error: Option<String>,
    pub progress: f32,
    pub started_at: Option<String>, // datetime
    pub status: String, // pending | running | paused | completed | failed | cancelled
}

// get_workflow_result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorkflowResultOutput {
    pub completed: bool,
    pub error: Option<String>,
    pub output: Option<Value>,
}

// list_executions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExecutionsOutput {
    pub executions: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// signal_workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalWorkflowOutput {
    pub success: bool,
}

// wait_for_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForEventOutput {
    pub payload: Option<Value>,
    pub received: bool,
    pub timed_out: bool,
}

// get_workflow_history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorkflowHistoryOutput {
    pub events: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// retry_step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStepOutput {
    pub success: bool,
}

// skip_step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipStepOutput {
    pub success: bool,
}

// terminate_all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateAllOutput {
    pub success: bool,
    pub terminated_count: i32,
}

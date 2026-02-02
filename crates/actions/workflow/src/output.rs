// Harana Actions - Workflow Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output for start action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOutput {
    pub success: bool,
    pub execution_id: String,
    pub status: String,
}

/// Output for pause action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseOutput {
    pub success: bool,
}

/// Output for resume action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeOutput {
    pub success: bool,
}

/// Output for cancel action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOutput {
    pub success: bool,
}

/// Output for get_status action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusOutput {
    pub status: String,
    pub current_step: Option<String>,
    pub progress: Option<f64>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub error: Option<String>,
}

/// Output for get_result action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResultOutput {
    pub completed: bool,
    pub output: Option<Value>,
    pub error: Option<String>,
}

/// Output for list_executions action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExecutionsOutput {
    pub executions: Vec<WorkflowExecutionInfo>,
    pub total: i32,
}

/// Output for signal action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalOutput {
    pub success: bool,
}

/// Output for wait_for_event action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitForEventOutput {
    pub received: bool,
    pub timed_out: bool,
    pub payload: Option<Value>,
}

/// Output for history action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryOutput {
    pub events: Vec<WorkflowHistoryEvent>,
    pub total: i32,
}

/// Output for retry_step action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStepOutput {
    pub success: bool,
}

/// Output for skip_step action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipStepOutput {
    pub success: bool,
}

/// Output for terminate_all action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateAllOutput {
    pub success: bool,
    pub terminated_count: i32,
}

/// Workflow execution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionInfo {
    pub execution_id: String,
    pub workflow_id: String,
    pub status: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Workflow history event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowHistoryEvent {
    pub event_id: String,
    pub event_type: String,
    pub step_id: Option<String>,
    pub timestamp: String,
    pub data: Option<Value>,
}

/// Workflow context
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowContext {
    pub values: HashMap<String, Value>,
}

/// Full workflow execution data (internal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub execution_id: String,
    pub workflow_id: String,
    pub status: String,
    pub current_step: Option<String>,
    pub input: Option<Value>,
    pub output: Option<Value>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub error: Option<String>,
    pub context: WorkflowContext,
    pub progress: f64,
}

/// Workflow status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

impl WorkflowStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkflowStatus::Pending => "Pending",
            WorkflowStatus::Running => "Running",
            WorkflowStatus::Paused => "Paused",
            WorkflowStatus::Completed => "Completed",
            WorkflowStatus::Failed => "Failed",
            WorkflowStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(WorkflowStatus::Pending),
            "running" => Some(WorkflowStatus::Running),
            "paused" => Some(WorkflowStatus::Paused),
            "completed" => Some(WorkflowStatus::Completed),
            "failed" => Some(WorkflowStatus::Failed),
            "cancelled" => Some(WorkflowStatus::Cancelled),
            _ => None,
        }
    }
}

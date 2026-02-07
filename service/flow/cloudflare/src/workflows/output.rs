// Harana Actions - Cloudflare Workflows Module Output Types

use serde::{Deserialize, Serialize};

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub id: String,
    pub success: bool,
}

// create_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchOutput {
    pub instances: Vec<WorkflowInstanceInfo>,
    pub success: bool,
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub id: String,
    pub status: WorkflowInstanceStatus,
}

// status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOutput {
    pub error: Option<WorkflowError>,
    pub output: serde_json::Value,
    pub status: String,
}

// pause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseOutput {
    pub success: bool,
}

// resume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeOutput {
    pub success: bool,
}

// restart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartOutput {
    pub success: bool,
}

// terminate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateOutput {
    pub success: bool,
}

// send_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEventOutput {
    pub success: bool,
}

// step_do
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepDoOutput {
    pub result: serde_json::Value,
}

// step_sleep
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepSleepOutput {
    pub success: bool,
}

// step_sleep_until
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepSleepUntilOutput {
    pub success: bool,
}

// step_wait_for_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepWaitForEventOutput {
    pub payload: serde_json::Value,
    pub r#type: String,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCreateOptions {
    pub id: Option<String>,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstanceInfo {
    pub id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstanceStatus {
    pub status: String,
    pub error: Option<WorkflowError>,
    pub output: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowError {
    pub message: String,
    pub name: Option<String>,
}

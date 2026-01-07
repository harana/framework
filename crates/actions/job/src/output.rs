// Harana Actions - Job Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// cancel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOutput {
    pub success: bool
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// get_result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResultOutput {
    pub completed: bool,
    pub error: String,
    pub result: String
}

// get_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusOutput {
    pub result: String,
    pub error: String,
    pub status: String,
    pub progress: f64,
    pub started_at: String,
    pub completed_at: String
}

// lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
    pub jobs: Vec<HashMap<String, Value>>,
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

// retry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryOutput {
    pub success: bool,
    pub new_id: String
}

// schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleOutput {
    pub job_id: String,
    pub scheduled_at: String
}

// update_progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProgressOutput {
    pub success: bool
}

// Harana Actions - Job Module Output Types
// Auto-generated output structs for Job action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// schedule_job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleJobOutput {
    pub job_id: String,
    pub scheduled_at: String, // datetime
}

// cancel_job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelJobOutput {
    pub success: bool,
}

// get_job_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetJobStatusOutput {
    pub completed_at: Option<String>, // datetime
    pub error: Option<String>,
    pub progress: f32,
    pub result: Option<Value>,
    pub started_at: Option<String>, // datetime
    pub status: String,             // pending | running | completed | failed | cancelled
}

// retry_job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryJobOutput {
    pub new_job_id: String,
    pub success: bool,
}

// list_jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobsOutput {
    pub jobs: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// pause_job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseJobOutput {
    pub success: bool,
}

// resume_job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeJobOutput {
    pub success: bool,
}

// get_job_result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetJobResultOutput {
    pub completed: bool,
    pub error: Option<String>,
    pub result: Option<Value>,
}

// update_job_progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJobProgressOutput {
    pub success: bool,
}

// delete_job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteJobOutput {
    pub success: bool,
}

// schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleOutput {
    pub job_id: String,
    pub scheduled_at: String
}

// cancel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOutput {
    pub success: bool
}

// get_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatusOutput {
    pub started_at: String,
    pub status: String,
    pub completed_at: String,
    pub error: String,
    pub result: String,
    pub progress: f64
}

// retry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryOutput {
    pub success: bool,
    pub new_id: String
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

// get_result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResultOutput {
    pub completed: bool,
    pub error: String,
    pub result: String
}

// update_progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProgressOutput {
    pub success: bool
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}
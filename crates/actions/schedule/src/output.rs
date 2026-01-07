// Harana Actions - Schedule Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// bulk_disable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDisableOutput {
    pub failed: i32,
    pub results: Vec<HashMap<String, Value>>,
    pub successful: i32
}

// bulk_enable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEnableOutput {
    pub results: Vec<HashMap<String, Value>>,
    pub successful: i32,
    pub failed: i32
}

// create_interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIntervalOutput {
    pub success: bool,
    pub next_run: i32,
    pub schedule_id: String
}

// create_one_time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOneTimeOutput {
    pub schedule_id: String,
    pub success: bool,
    pub run_at: i32
}

// create_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduleOutput {
    pub next_run: i32,
    pub success: bool,
    pub schedule_id: String,
    pub name: String
}

// delete_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScheduleOutput {
    pub schedule_id: String,
    pub success: bool
}

// disable_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableScheduleOutput {
    pub success: bool,
    pub schedule_id: String,
    pub enabled: bool
}

// enable_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnableScheduleOutput {
    pub schedule_id: String,
    pub enabled: bool,
    pub next_run: i32,
    pub success: bool
}

// get_next_run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNextRunOutput {
    pub next_runs: Vec<i32>,
    pub schedule_id: String
}

// get_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleOutput {
    pub created_at: i32,
    pub schedule_id: String,
    pub enabled: bool,
    pub last_run: i32,
    pub cron_expression: String,
    pub name: String,
    pub timezone: String,
    pub metadata: HashMap<String, Value>,
    pub next_run: i32
}

// get_schedule_history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleHistoryOutput {
    pub schedule_id: String,
    pub executions: Vec<HashMap<String, Value>>,
    pub total: i32
}

// get_schedule_stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleStatsOutput {
    pub total_executions: i32,
    pub average_duration: f64,
    pub successful_executions: i32,
    pub schedule_id: String,
    pub failed_executions: i32,
    pub last_execution: HashMap<String, Value>
}

// list_schedules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSchedulesOutput {
    pub schedules: Vec<HashMap<String, Value>>,
    pub total: i32
}

// pause_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseScheduleOutput {
    pub resume_at: i32,
    pub paused: bool,
    pub schedule_id: String,
    pub success: bool
}

// resume_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeScheduleOutput {
    pub schedule_id: String,
    pub paused: bool,
    pub success: bool,
    pub next_run: i32
}

// trigger_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerScheduleOutput {
    pub execution_id: String,
    pub schedule_id: String,
    pub success: bool,
    pub triggered_at: i32
}

// update_action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateActionOutput {
    pub schedule_id: String,
    pub success: bool
}

// update_schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScheduleOutput {
    pub schedule_id: String,
    pub success: bool,
    pub next_run: i32
}

// validate_cron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateCronOutput {
    pub valid: bool,
    pub error: String,
    pub next_run: i32
}

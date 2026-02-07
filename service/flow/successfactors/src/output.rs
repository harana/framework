// Harana Actions - SuccessFactors Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// get_employee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEmployeeOutput {
    pub custom_fields: Option<HashMap<String, Value>>,
    pub department: Option<String>,
    pub email: String,
    pub first_name: String,
    pub hire_date: DateTime<Utc>,
    pub job_title: Option<String>,
    pub last_name: String,
    pub manager_id: Option<String>,
    pub status: String,
    pub user_id: String,
}

// list_employees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEmployeesOutput {
    pub count: i32,
    pub employees: Vec<SfEmployee>,
    pub next_link: Option<String>,
}

// create_employee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeOutput {
    pub success: bool,
    pub user_id: String,
}

// update_employee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployeeOutput {
    pub success: bool,
}

// terminate_employee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateEmployeeOutput {
    pub success: bool,
}

// get_job_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetJobInfoOutput {
    pub cost_center: Option<String>,
    pub department: Option<String>,
    pub division: Option<String>,
    pub effective_start_date: Option<DateTime<Utc>>,
    pub employment_type: Option<String>,
    pub job_code: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub manager_id: Option<String>,
}

// list_job_history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobHistoryOutput {
    pub count: i32,
    pub job_history: Vec<SfJobHistoryEntry>,
}

// get_compensation_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompensationInfoOutput {
    pub base_salary: Option<f64>,
    pub currency: Option<String>,
    pub effective_date: Option<DateTime<Utc>>,
    pub pay_grade: Option<String>,
    pub pay_type: Option<String>,
}

// list_time_off
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTimeOffOutput {
    pub count: i32,
    pub time_off_requests: Vec<SfTimeOffRequest>,
}

// request_time_off
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTimeOffOutput {
    pub request_id: String,
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SfEmployee {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: Option<String>,
    pub job_title: Option<String>,
    pub status: String,
    pub hire_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SfJobHistoryEntry {
    pub job_title: String,
    pub department: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub event_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SfTimeOffRequest {
    pub request_id: String,
    pub time_off_type: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,
    pub quantity: f64,
}

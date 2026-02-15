// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// sf_employee
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfEmployee {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub department: Option<String>,
    pub email: String,
    pub first_name: String,
    pub hire_date: Option<chrono::DateTime<chrono::Utc>>,
    pub job_title: Option<String>,
    pub last_name: String,
    pub manager_id: Option<String>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

impl SfEmployee {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sf_job_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfJobInfo {
    pub cost_center: Option<String>,
    pub department: Option<String>,
    pub division: Option<String>,
    pub effective_start_date: chrono::DateTime<chrono::Utc>,
    /// Reference: sf_employee.id
    pub employee_id: String,
    pub employment_type: Option<String>,
    pub job_code: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub manager_id: Option<String>,
}

impl SfJobInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sf_compensation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfCompensation {
    pub bonus_target: Option<f64>,
    pub currency: Option<String>,
    pub effective_start_date: chrono::DateTime<chrono::Utc>,
    /// Reference: sf_employee.id
    pub employee_id: String,
    pub frequency: Option<String>,
    pub pay_grade: Option<String>,
    pub pay_type: Option<String>,
    pub salary: Option<f64>,
}

impl SfCompensation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sf_time_off_request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfTimeOffRequest {
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_hours: Option<f64>,
    pub duration_type: String,
    /// Reference: sf_employee.id
    pub employee_id: String,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub time_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl SfTimeOffRequest {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sf_time_off_balance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfTimeOffBalance {
    #[serde(default = "chrono::Utc::now")]
    pub as_of_date: chrono::DateTime<chrono::Utc>,
    pub balance: f64,
    /// Reference: sf_employee.id
    pub employee_id: String,
    pub time_type: String,
    pub unit: String,
}

impl SfTimeOffBalance {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sf_department
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfDepartment {
    pub cost_center: Option<String>,
    pub department_id: String,
    pub description: Option<String>,
    pub head_id: Option<String>,
    pub name: String,
    pub parent_id: Option<String>,
}

impl SfDepartment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


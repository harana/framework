// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfJobInfo {
    pub cost_center: Option<String>,
    pub department: Option<String>,
    pub division: Option<String>,
    pub effective_start_date: chrono::DateTime<chrono::Utc>,
    pub employee_id: String,
    pub employment_type: Option<String>,
    pub job_code: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub manager_id: Option<String>,
}

impl SfJobInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfCompensation {
    pub bonus_target: Option<f64>,
    pub currency: Option<String>,
    pub effective_start_date: chrono::DateTime<chrono::Utc>,
    pub employee_id: String,
    pub frequency: Option<String>,
    pub pay_grade: Option<String>,
    pub pay_type: Option<String>,
    pub salary: Option<f64>,
}

impl SfCompensation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfTimeOffRequest {
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_hours: Option<f64>,
    pub duration_type: String,
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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfTimeOffBalance {
    #[serde(default = "chrono::Utc::now")]
    pub as_of_date: chrono::DateTime<chrono::Utc>,
    pub balance: f64,
    pub employee_id: String,
    pub time_type: String,
    pub unit: String,
}

impl SfTimeOffBalance {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Employee {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub job_title: String,
    pub manager_id: String,
    pub hire_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub custom_fields: std::collections::HashMap<String, String>,
}

impl Employee {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfJobHistoryEntry {
    pub effective_date: chrono::DateTime<chrono::Utc>,
    pub job_title: String,
    pub department: String,
    pub job_code: String,
    pub employment_type: String,
}

impl SfJobHistoryEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfCompensationEntry {
    pub effective_date: chrono::DateTime<chrono::Utc>,
    pub salary: f64,
    pub currency: String,
    pub pay_grade: String,
    pub frequency: String,
}

impl SfCompensationEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfOrgChartNode {
    pub user_id: String,
    pub name: String,
    pub job_title: String,
    pub reports: Vec<String>,
}

impl SfOrgChartNode {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfPosition {
    pub position_id: String,
    pub title: String,
    pub department_id: String,
    pub job_code: String,
    pub status: String,
    pub incumbent_user_id: String,
}

impl SfPosition {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


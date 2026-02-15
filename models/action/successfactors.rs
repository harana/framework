// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEmployeeInput {
    pub expand: Vec<String>,
    pub select: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEmployeeOutput {
    pub custom_fields: std::collections::HashMap<String, String>,
    pub department: String,
    pub email: String,
    pub first_name: String,
    pub hire_date: chrono::DateTime<chrono::Utc>,
    pub job_title: String,
    pub last_name: String,
    pub manager_id: String,
    pub status: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEmployeesInput {
    pub expand: Vec<String>,
    pub filter: String,
    pub order_by: String,
    pub select: Vec<String>,
    pub skip: i64,
    pub top: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEmployeesOutput {
    pub count: i64,
    pub employees: Vec<String>,
    pub next_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEmployeeInput {
    pub custom_fields: std::collections::HashMap<String, String>,
    pub department: String,
    pub email: String,
    pub first_name: String,
    pub hire_date: chrono::DateTime<chrono::Utc>,
    pub job_title: String,
    pub last_name: String,
    pub manager_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEmployeeOutput {
    pub success: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEmployeeInput {
    pub custom_fields: std::collections::HashMap<String, String>,
    pub department: String,
    pub email: String,
    pub first_name: String,
    pub job_title: String,
    pub last_name: String,
    pub manager_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEmployeeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateEmployeeInput {
    pub last_day_worked: chrono::DateTime<chrono::Utc>,
    pub notes: String,
    pub termination_date: chrono::DateTime<chrono::Utc>,
    pub termination_reason: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateEmployeeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetJobInfoInput {
    pub effective_date: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetJobInfoOutput {
    pub cost_center: String,
    pub department: String,
    pub division: String,
    pub effective_start_date: chrono::DateTime<chrono::Utc>,
    pub employment_type: String,
    pub job_code: String,
    pub job_title: String,
    pub location: String,
    pub manager_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListJobHistoryInput {
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListJobHistoryOutput {
    pub count: i64,
    pub job_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCompensationInput {
    pub effective_date: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCompensationOutput {
    pub bonus_target: f64,
    pub currency: String,
    pub effective_start_date: chrono::DateTime<chrono::Utc>,
    pub frequency: String,
    pub pay_grade: String,
    pub pay_type: String,
    pub salary: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCompensationHistoryInput {
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCompensationHistoryOutput {
    pub compensation_history: Vec<String>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTimeOffBalanceInput {
    pub as_of_date: chrono::DateTime<chrono::Utc>,
    pub time_type: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTimeOffBalanceOutput {
    pub balances: Vec<String>,
    pub total_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestTimeOffInput {
    pub comment: String,
    pub duration_hours: f64,
    pub duration_type: String,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub time_type: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestTimeOffOutput {
    pub request_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTimeOffRequestsInput {
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub top: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTimeOffRequestsOutput {
    pub count: i64,
    pub requests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApproveTimeOffInput {
    pub comment: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApproveTimeOffOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RejectTimeOffInput {
    pub reason: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RejectTimeOffOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrgChartInput {
    pub depth: i64,
    pub direction: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOrgChartOutput {
    pub levels: i64,
    pub org_chart: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDirectReportsInput {
    #[serde(default)]
    pub include_indirect: bool,
    pub manager_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDirectReportsOutput {
    pub count: i64,
    pub direct_reports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDepartmentsInput {
    pub filter: String,
    pub top: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDepartmentsOutput {
    pub count: i64,
    pub departments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepartmentInput {
    pub department_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDepartmentOutput {
    pub cost_center: String,
    pub department_id: String,
    pub description: String,
    pub head_of_department: String,
    pub name: String,
    pub parent_department_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPositionsInput {
    pub department_id: String,
    pub status: String,
    pub top: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPositionsOutput {
    pub count: i64,
    pub positions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionInput {
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPositionOutput {
    pub department_id: String,
    pub incumbent_user_id: String,
    pub job_code: String,
    pub pay_grade: String,
    pub position_id: String,
    pub status: String,
    pub title: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfEmployee {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub job_title: String,
    pub manager_id: String,
    pub hire_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfCompensationEntry {
    pub effective_date: chrono::DateTime<chrono::Utc>,
    pub salary: f64,
    pub currency: String,
    pub pay_grade: String,
    pub frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfTimeOffBalance {
    pub time_type: String,
    pub balance: f64,
    pub used: f64,
    pub pending: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfTimeOffRequest {
    pub request_id: String,
    pub user_id: String,
    pub time_type: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub duration_hours: f64,
    pub status: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfOrgChartNode {
    pub user_id: String,
    pub name: String,
    pub job_title: String,
    pub reports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SfDepartment {
    pub department_id: String,
    pub name: String,
    pub description: String,
    pub parent_department_id: String,
    pub head_of_department: String,
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

#[async_trait]
pub trait SuccessfactorsAction {
    async fn get_employee(&self, input: GetEmployeeInput) -> Result<GetEmployeeOutput, Box<dyn std::error::Error>>;
    async fn list_employees(&self, input: ListEmployeesInput) -> Result<ListEmployeesOutput, Box<dyn std::error::Error>>;
    async fn create_employee(&self, input: CreateEmployeeInput) -> Result<CreateEmployeeOutput, Box<dyn std::error::Error>>;
    async fn update_employee(&self, input: UpdateEmployeeInput) -> Result<UpdateEmployeeOutput, Box<dyn std::error::Error>>;
    async fn terminate_employee(&self, input: TerminateEmployeeInput) -> Result<TerminateEmployeeOutput, Box<dyn std::error::Error>>;
    async fn get_job_info(&self, input: GetJobInfoInput) -> Result<GetJobInfoOutput, Box<dyn std::error::Error>>;
    async fn list_job_history(&self, input: ListJobHistoryInput) -> Result<ListJobHistoryOutput, Box<dyn std::error::Error>>;
    async fn get_compensation(&self, input: GetCompensationInput) -> Result<GetCompensationOutput, Box<dyn std::error::Error>>;
    async fn list_compensation_history(&self, input: ListCompensationHistoryInput) -> Result<ListCompensationHistoryOutput, Box<dyn std::error::Error>>;
    async fn get_time_off_balance(&self, input: GetTimeOffBalanceInput) -> Result<GetTimeOffBalanceOutput, Box<dyn std::error::Error>>;
    async fn request_time_off(&self, input: RequestTimeOffInput) -> Result<RequestTimeOffOutput, Box<dyn std::error::Error>>;
    async fn list_time_off_requests(&self, input: ListTimeOffRequestsInput) -> Result<ListTimeOffRequestsOutput, Box<dyn std::error::Error>>;
    async fn approve_time_off(&self, input: ApproveTimeOffInput) -> Result<ApproveTimeOffOutput, Box<dyn std::error::Error>>;
    async fn reject_time_off(&self, input: RejectTimeOffInput) -> Result<RejectTimeOffOutput, Box<dyn std::error::Error>>;
    async fn get_org_chart(&self, input: GetOrgChartInput) -> Result<GetOrgChartOutput, Box<dyn std::error::Error>>;
    async fn get_direct_reports(&self, input: GetDirectReportsInput) -> Result<GetDirectReportsOutput, Box<dyn std::error::Error>>;
    async fn list_departments(&self, input: ListDepartmentsInput) -> Result<ListDepartmentsOutput, Box<dyn std::error::Error>>;
    async fn get_department(&self, input: GetDepartmentInput) -> Result<GetDepartmentOutput, Box<dyn std::error::Error>>;
    async fn list_positions(&self, input: ListPositionsInput) -> Result<ListPositionsOutput, Box<dyn std::error::Error>>;
    async fn get_position(&self, input: GetPositionInput) -> Result<GetPositionOutput, Box<dyn std::error::Error>>;
}

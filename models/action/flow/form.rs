// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmitOutput {
    pub errors: Vec<String>,
    pub submission_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub fields: Vec<String>,
    pub metadata: String,
    pub validation_rules: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSubmissionOutput {
    pub data: String,
    pub form_id: String,
    pub status: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSubmissionsOutput {
    pub submissions: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExportSubmissionsOutput {
    pub content: String,
    pub count: i64,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormSubmission {
    pub submission_id: String,
    pub form_id: String,
    pub data: String,
    pub status: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormData {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormField {
    pub name: String,
    pub type: String,
    pub label: String,
    pub required: bool,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormMetadata {
    pub title: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormValidationRules {
    pub rules: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormSubmissionInfo {
    pub submission_id: String,
    pub form_id: String,
    pub status: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Form {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub slug: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait FormAction {
    async fn validate(&self, data: String, form_id: String, strict: bool) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn submit(&self, data: String, form_id: String, validate: bool) -> Result<SubmitOutput, Box<dyn std::error::Error>>;
    async fn get(&self, form_id: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn get_submission(&self, submission_id: String) -> Result<GetSubmissionOutput, Box<dyn std::error::Error>>;
    async fn list_submissions(&self, end_date: chrono::DateTime<chrono::Utc>, form_id: String, limit: i64, offset: i64, start_date: chrono::DateTime<chrono::Utc>, status: String) -> Result<ListSubmissionsOutput, Box<dyn std::error::Error>>;
    async fn update_submission(&self, data: String, submission_id: String, validate: bool) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn delete_submission(&self, submission_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn export_submissions(&self, end_date: chrono::DateTime<chrono::Utc>, form_id: String, format: String, start_date: chrono::DateTime<chrono::Utc>) -> Result<ExportSubmissionsOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateInput {
    pub data: String,
    pub form_id: String,
    #[serde(default)]
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmitInput {
    pub data: String,
    pub form_id: String,
    #[serde(default)]
    pub validate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmitOutput {
    pub errors: Vec<String>,
    pub submission_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub form_id: String,
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
pub struct GetSubmissionInput {
    pub submission_id: String,
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
pub struct ListSubmissionsInput {
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub form_id: String,
    pub limit: i64,
    pub offset: i64,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSubmissionsOutput {
    pub submissions: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateSubmissionInput {
    pub data: String,
    pub submission_id: String,
    #[serde(default)]
    pub validate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateSubmissionOutput {
    pub errors: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSubmissionInput {
    pub submission_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSubmissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExportSubmissionsInput {
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub form_id: String,
    pub format: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
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

#[async_trait]
pub trait FormAction {
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn submit(&self, input: SubmitInput) -> Result<SubmitOutput, Box<dyn std::error::Error>>;
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn get_submission(&self, input: GetSubmissionInput) -> Result<GetSubmissionOutput, Box<dyn std::error::Error>>;
    async fn list_submissions(&self, input: ListSubmissionsInput) -> Result<ListSubmissionsOutput, Box<dyn std::error::Error>>;
    async fn update_submission(&self, input: UpdateSubmissionInput) -> Result<UpdateSubmissionOutput, Box<dyn std::error::Error>>;
    async fn delete_submission(&self, input: DeleteSubmissionInput) -> Result<DeleteSubmissionOutput, Box<dyn std::error::Error>>;
    async fn export_submissions(&self, input: ExportSubmissionsInput) -> Result<ExportSubmissionsOutput, Box<dyn std::error::Error>>;
}

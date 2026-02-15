// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordConsentInput {
    pub consent_type: String,
    pub granted: bool,
    pub ip_address: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub user_agent: String,
    pub user_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordConsentOutput {
    pub consent_id: String,
    pub success: bool,
    pub timestamp: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeConsentInput {
    pub consent_type: String,
    pub reason: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeConsentOutput {
    pub consent_id: String,
    pub revoked_at: i64,
    pub success: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetConsentStatusInput {
    pub consent_type: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetConsentStatusOutput {
    pub consents: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateConsentInput {
    pub consent_id: String,
    pub granted: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateConsentOutput {
    pub consent_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExportDataInput {
    pub exclude_sections: Vec<String>,
    pub format: String,
    pub include_sections: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExportDataOutput {
    pub download_url: String,
    pub expires_at: i64,
    pub export_id: String,
    pub status: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetExportStatusInput {
    pub export_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetExportStatusOutput {
    pub completed_at: i64,
    pub created_at: i64,
    pub download_url: String,
    pub export_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDataInput {
    pub delete_type: String,
    pub reason: String,
    pub retain_sections: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDataOutput {
    pub deletion_id: String,
    pub scheduled_at: i64,
    pub status: String,
    pub success: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDeletionStatusInput {
    pub deletion_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDeletionStatusOutput {
    pub completed_at: i64,
    pub deletion_id: String,
    pub scheduled_at: i64,
    pub status: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelDeletionInput {
    pub deletion_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelDeletionOutput {
    pub cancelled: bool,
    pub deletion_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnonymizeDataInput {
    pub retain_sections: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnonymizeDataOutput {
    pub anonymization_id: String,
    pub success: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListConsentHistoryInput {
    pub consent_type: String,
    pub end_date: i64,
    pub limit: i64,
    pub start_date: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListConsentHistoryOutput {
    pub history: Vec<String>,
    pub total: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordDataAccessInput {
    pub access_type: String,
    pub accessor_id: String,
    pub purpose: String,
    pub resource: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordDataAccessOutput {
    pub access_id: String,
    pub success: bool,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccessLogInput {
    pub end_date: i64,
    pub limit: i64,
    pub start_date: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccessLogOutput {
    pub accesses: Vec<String>,
    pub total: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePolicyVersionInput {
    pub content: String,
    pub effective_date: i64,
    pub title: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePolicyVersionOutput {
    pub policy_id: String,
    pub success: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetActivePolicyInput {
    pub policy_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetActivePolicyOutput {
    pub content: String,
    pub effective_date: i64,
    pub policy_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordPolicyAcceptanceInput {
    pub ip_address: String,
    pub policy_id: String,
    pub user_agent: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordPolicyAcceptanceOutput {
    pub acceptance_id: String,
    pub success: bool,
    pub timestamp: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyRtbfEligibilityInput {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyRtbfEligibilityOutput {
    pub blocking_factors: Vec<String>,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Consent {
    pub consent_id: String,
    pub user_id: String,
    pub consent_type: String,
    pub granted: bool,
    pub version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyConsent {
    pub consent_id: String,
    pub consent_type: String,
    pub granted: bool,
    pub version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyConsentHistory {
    pub consent_id: String,
    pub consent_type: String,
    pub granted: bool,
    pub version: String,
    pub method: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataAccess {
    pub access_id: String,
    pub accessor_id: String,
    pub access_type: String,
    pub resource: String,
    pub purpose: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait PrivacyAction {
    async fn record_consent(&self, input: RecordConsentInput) -> Result<RecordConsentOutput, Box<dyn std::error::Error>>;
    async fn revoke_consent(&self, input: RevokeConsentInput) -> Result<RevokeConsentOutput, Box<dyn std::error::Error>>;
    async fn get_consent_status(&self, input: GetConsentStatusInput) -> Result<GetConsentStatusOutput, Box<dyn std::error::Error>>;
    async fn update_consent(&self, input: UpdateConsentInput) -> Result<UpdateConsentOutput, Box<dyn std::error::Error>>;
    async fn export_data(&self, input: ExportDataInput) -> Result<ExportDataOutput, Box<dyn std::error::Error>>;
    async fn get_export_status(&self, input: GetExportStatusInput) -> Result<GetExportStatusOutput, Box<dyn std::error::Error>>;
    async fn delete_data(&self, input: DeleteDataInput) -> Result<DeleteDataOutput, Box<dyn std::error::Error>>;
    async fn get_deletion_status(&self, input: GetDeletionStatusInput) -> Result<GetDeletionStatusOutput, Box<dyn std::error::Error>>;
    async fn cancel_deletion(&self, input: CancelDeletionInput) -> Result<CancelDeletionOutput, Box<dyn std::error::Error>>;
    async fn anonymize_data(&self, input: AnonymizeDataInput) -> Result<AnonymizeDataOutput, Box<dyn std::error::Error>>;
    async fn list_consent_history(&self, input: ListConsentHistoryInput) -> Result<ListConsentHistoryOutput, Box<dyn std::error::Error>>;
    async fn record_data_access(&self, input: RecordDataAccessInput) -> Result<RecordDataAccessOutput, Box<dyn std::error::Error>>;
    async fn get_access_log(&self, input: GetAccessLogInput) -> Result<GetAccessLogOutput, Box<dyn std::error::Error>>;
    async fn create_policy_version(&self, input: CreatePolicyVersionInput) -> Result<CreatePolicyVersionOutput, Box<dyn std::error::Error>>;
    async fn get_active_policy(&self, input: GetActivePolicyInput) -> Result<GetActivePolicyOutput, Box<dyn std::error::Error>>;
    async fn record_policy_acceptance(&self, input: RecordPolicyAcceptanceInput) -> Result<RecordPolicyAcceptanceOutput, Box<dyn std::error::Error>>;
    async fn verify_rtbf_eligibility(&self, input: VerifyRtbfEligibilityInput) -> Result<VerifyRtbfEligibilityOutput, Box<dyn std::error::Error>>;
}

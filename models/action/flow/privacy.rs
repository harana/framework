// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordConsentOutput {
    pub consent_id: String,
    pub timestamp: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeConsentOutput {
    pub consent_id: String,
    pub revoked_at: i64,
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
pub struct ExportDataOutput {
    pub download_url: String,
    pub expires_at: i64,
    pub export_id: String,
    pub status: String,
    pub user_id: String,
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
pub struct DeleteDataOutput {
    pub deletion_id: String,
    pub scheduled_at: i64,
    pub status: String,
    pub user_id: String,
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
pub struct CancelDeletionOutput {
    pub cancelled: bool,
    pub deletion_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnonymizeDataOutput {
    pub anonymization_id: String,
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
pub struct RecordDataAccessOutput {
    pub access_id: String,
    pub timestamp: i64,
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
pub struct CreatePolicyVersionOutput {
    pub policy_id: String,
    pub version: String,
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
pub struct RecordPolicyAcceptanceOutput {
    pub acceptance_id: String,
    pub timestamp: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataExport {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub download_url: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub format: String,
    pub status: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataDeletion {
    pub cancelled_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delete_type: String,
    pub reason: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataAccessLog {
    pub access_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub accessor_id: String,
    pub purpose: String,
    pub resource: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyPolicyVersion {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub effective_date: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub policy_type: String,
    pub title: String,
    pub version: String,
}

#[async_trait]
pub trait PrivacyAction {
    async fn record_consent(&self, consent_type: String, granted: bool, ip_address: String, metadata: std::collections::HashMap<String, String>, user_agent: String, user_id: String, version: String) -> Result<RecordConsentOutput, Box<dyn std::error::Error>>;
    async fn revoke_consent(&self, consent_type: String, reason: String, user_id: String) -> Result<RevokeConsentOutput, Box<dyn std::error::Error>>;
    async fn get_consent_status(&self, consent_type: String, user_id: String) -> Result<GetConsentStatusOutput, Box<dyn std::error::Error>>;
    async fn update_consent(&self, consent_id: String, granted: bool, version: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn export_data(&self, exclude_sections: Vec<String>, format: String, include_sections: Vec<String>, user_id: String) -> Result<ExportDataOutput, Box<dyn std::error::Error>>;
    async fn get_export_status(&self, export_id: String) -> Result<GetExportStatusOutput, Box<dyn std::error::Error>>;
    async fn delete_data(&self, delete_type: String, reason: String, retain_sections: Vec<String>, user_id: String) -> Result<DeleteDataOutput, Box<dyn std::error::Error>>;
    async fn get_deletion_status(&self, deletion_id: String) -> Result<GetDeletionStatusOutput, Box<dyn std::error::Error>>;
    async fn cancel_deletion(&self, deletion_id: String) -> Result<CancelDeletionOutput, Box<dyn std::error::Error>>;
    async fn anonymize_data(&self, retain_sections: Vec<String>, user_id: String) -> Result<AnonymizeDataOutput, Box<dyn std::error::Error>>;
    async fn list_consent_history(&self, consent_type: String, end_date: i64, limit: i64, start_date: i64, user_id: String) -> Result<ListConsentHistoryOutput, Box<dyn std::error::Error>>;
    async fn record_data_access(&self, access_type: String, accessor_id: String, purpose: String, resource: String, user_id: String) -> Result<RecordDataAccessOutput, Box<dyn std::error::Error>>;
    async fn get_access_log(&self, end_date: i64, limit: i64, start_date: i64, user_id: String) -> Result<GetAccessLogOutput, Box<dyn std::error::Error>>;
    async fn create_policy_version(&self, content: String, effective_date: i64, title: String, version: String) -> Result<CreatePolicyVersionOutput, Box<dyn std::error::Error>>;
    async fn get_active_policy(&self, policy_type: String) -> Result<GetActivePolicyOutput, Box<dyn std::error::Error>>;
    async fn record_policy_acceptance(&self, ip_address: String, policy_id: String, user_agent: String, user_id: String) -> Result<RecordPolicyAcceptanceOutput, Box<dyn std::error::Error>>;
    async fn verify_rtbf_eligibility(&self, user_id: String) -> Result<VerifyRtbfEligibilityOutput, Box<dyn std::error::Error>>;
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// === Action Outputs ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordConsentOutput {
    pub consent_id: String,
    pub user_id: String,
    pub timestamp: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeConsentOutput {
    pub consent_id: String,
    pub user_id: String,
    pub revoked_at: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConsentStatusOutput {
    pub user_id: String,
    pub consents: Vec<PrivacyConsent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConsentOutput {
    pub consent_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDataOutput {
    pub export_id: String,
    pub user_id: String,
    pub status: String,
    pub download_url: Option<String>,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportStatusOutput {
    pub export_id: String,
    pub status: ExportStatus,
    pub created_at: i64,
    pub completed_at: Option<i64>,
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDataOutput {
    pub deletion_id: String,
    pub user_id: String,
    pub status: String,
    pub scheduled_at: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeletionStatusOutput {
    pub deletion_id: String,
    pub user_id: String,
    pub status: DeletionStatus,
    pub scheduled_at: i64,
    pub completed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelDeletionOutput {
    pub deletion_id: String,
    pub cancelled: bool,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizeDataOutput {
    pub anonymization_id: String,
    pub user_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConsentHistoryOutput {
    pub user_id: String,
    pub history: Vec<PrivacyConsentHistory>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordDataAccessOutput {
    pub access_id: String,
    pub timestamp: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccessLogOutput {
    pub user_id: String,
    pub accesses: Vec<PrivacyDataAccess>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyVersionOutput {
    pub policy_id: String,
    pub version: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActivePolicyOutput {
    pub policy_id: String,
    pub version: String,
    pub effective_date: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPolicyAcceptanceOutput {
    pub acceptance_id: String,
    pub user_id: String,
    pub timestamp: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRtbfEligibilityOutput {
    pub user_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub blocking_factors: Vec<String>,
}

// === Class Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consent {
    pub consent_id: String,
    pub user_id: String,
    pub consent_type: ConsentType,
    pub granted: bool,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConsent {
    pub consent_id: String,
    pub consent_type: String,
    pub granted: bool,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConsentHistory {
    pub consent_id: String,
    pub consent_type: String,
    pub granted: bool,
    pub version: String,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyDataAccess {
    pub access_id: String,
    pub accessor_id: String,
    pub access_type: String,
    pub resource: String,
    pub purpose: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// === Enums ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentType {
    Marketing,
    Analytics,
    Functional,
    Necessary,
}

impl std::fmt::Display for ConsentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsentType::Marketing => write!(f, "Marketing"),
            ConsentType::Analytics => write!(f, "Analytics"),
            ConsentType::Functional => write!(f, "Functional"),
            ConsentType::Necessary => write!(f, "Necessary"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Xml,
}

impl Default for ExportFormat {
    fn default() -> Self {
        ExportFormat::Json
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeleteType {
    Soft,
    Hard,
}

impl Default for DeleteType {
    fn default() -> Self {
        DeleteType::Soft
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeletionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    Privacy,
    Terms,
    Cookie,
}

impl Default for PolicyType {
    fn default() -> Self {
        PolicyType::Privacy
    }
}

// === Internal Storage Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConsent {
    pub consent_id: String,
    pub user_id: String,
    pub consent_type: ConsentType,
    pub granted: bool,
    pub version: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredExport {
    pub export_id: String,
    pub user_id: String,
    pub format: ExportFormat,
    pub status: ExportStatus,
    pub include_sections: Vec<String>,
    pub exclude_sections: Vec<String>,
    pub download_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredDeletion {
    pub deletion_id: String,
    pub user_id: String,
    pub delete_type: DeleteType,
    pub reason: String,
    pub retain_sections: Vec<String>,
    pub status: DeletionStatus,
    pub scheduled_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredDataAccess {
    pub access_id: String,
    pub user_id: String,
    pub accessor_id: String,
    pub access_type: AccessType,
    pub resource: String,
    pub purpose: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPolicy {
    pub policy_id: String,
    pub policy_type: PolicyType,
    pub version: String,
    pub title: String,
    pub content: String,
    pub effective_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPolicyAcceptance {
    pub acceptance_id: String,
    pub user_id: String,
    pub policy_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConsentHistory {
    pub consent_id: String,
    pub user_id: String,
    pub consent_type: ConsentType,
    pub granted: bool,
    pub version: String,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

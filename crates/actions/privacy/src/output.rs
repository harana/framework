// Harana Actions - Privacy Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// anonymize_data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizeDataOutput {
    pub anonymization_id: String,
    pub user_id: String,
    pub success: bool
}

// cancel_deletion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelDeletionOutput {
    pub success: bool,
    pub cancelled: bool,
    pub deletion_id: String
}

// create_policy_version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyVersionOutput {
    pub policy_id: String,
    pub success: bool,
    pub version: String
}

// delete_data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDataOutput {
    pub status: String,
    pub deletion_id: String,
    pub scheduled_at: i32,
    pub user_id: String,
    pub success: bool
}

// export_data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDataOutput {
    pub expires_at: i32,
    pub status: String,
    pub download_url: String,
    pub export_id: String,
    pub user_id: String
}

// get_access_log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccessLogOutput {
    pub user_id: String,
    pub accesses: Vec<HashMap<String, Value>>,
    pub total: i32
}

// get_active_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActivePolicyOutput {
    pub effective_date: i32,
    pub content: String,
    pub version: String,
    pub policy_id: String
}

// get_consent_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConsentStatusOutput {
    pub user_id: String,
    pub consents: Vec<HashMap<String, Value>>
}

// get_deletion_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeletionStatusOutput {
    pub user_id: String,
    pub status: String,
    pub deletion_id: String,
    pub completed_at: i32,
    pub scheduled_at: i32
}

// get_export_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportStatusOutput {
    pub created_at: i32,
    pub completed_at: i32,
    pub download_url: String,
    pub status: String,
    pub export_id: String
}

// list_consent_history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConsentHistoryOutput {
    pub history: Vec<HashMap<String, Value>>,
    pub total: i32,
    pub user_id: String
}

// record_consent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordConsentOutput {
    pub consent_id: String,
    pub success: bool,
    pub timestamp: i32,
    pub user_id: String
}

// record_data_access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordDataAccessOutput {
    pub timestamp: i32,
    pub success: bool,
    pub access_id: String
}

// record_policy_acceptance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPolicyAcceptanceOutput {
    pub success: bool,
    pub user_id: String,
    pub acceptance_id: String,
    pub timestamp: i32
}

// revoke_consent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeConsentOutput {
    pub user_id: String,
    pub consent_id: String,
    pub revoked_at: i32,
    pub success: bool
}

// update_consent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConsentOutput {
    pub success: bool,
    pub consent_id: String
}

// verify_rtbf_eligibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRtbfEligibilityOutput {
    pub reasons: Vec<String>,
    pub eligible: bool,
    pub user_id: String,
    pub blocking_factors: Vec<String>
}

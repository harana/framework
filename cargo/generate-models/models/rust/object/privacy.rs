// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// privacy_consent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyConsent {
    pub consent_type: String,
    #[serde(default)]
    pub granted: bool,
    pub granted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ip_address: Option<String>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_agent: Option<String>,
    /// Reference: user.id
    pub user_id: String,
    pub version: String,
}

impl PrivacyConsent {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// privacy_consent_history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyConsentHistory {
    pub action: String,
    pub consent_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
    /// Reference: user.id
    pub user_id: String,
    pub version: Option<String>,
}

impl PrivacyConsentHistory {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// privacy_data_export
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataExport {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub download_url: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub format: String,
    pub status: String,
    /// Reference: user.id
    pub user_id: String,
}

impl PrivacyDataExport {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// privacy_data_deletion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataDeletion {
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delete_type: String,
    pub reason: String,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    /// Reference: user.id
    pub user_id: String,
}

impl PrivacyDataDeletion {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// privacy_data_access_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataAccessLog {
    pub access_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub accessor_id: String,
    pub purpose: Option<String>,
    pub resource: String,
    /// Reference: user.id
    pub user_id: String,
}

impl PrivacyDataAccessLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// privacy_policy_version
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

impl PrivacyPolicyVersion {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


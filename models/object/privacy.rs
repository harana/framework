// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub user_id: String,
    pub version: String,
}

impl PrivacyConsent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyConsentHistory {
    pub action: String,
    pub consent_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
    pub user_id: String,
    pub version: Option<String>,
}

impl PrivacyConsentHistory {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub user_id: String,
}

impl PrivacyDataExport {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub user_id: String,
}

impl PrivacyDataDeletion {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivacyDataAccessLog {
    pub access_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub accessor_id: String,
    pub purpose: Option<String>,
    pub resource: String,
    pub user_id: String,
}

impl PrivacyDataAccessLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl PrivacyPolicyVersion {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl Consent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl PrivacyDataAccess {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


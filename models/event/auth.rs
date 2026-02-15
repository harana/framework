// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthRegistrationStarted {
    pub user_id: Option<String>,
    pub email: String,
    pub username: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl AuthRegistrationStarted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthRegistrationCompleted {
    pub user_id: String,
    pub email: String,
    pub username: Option<String>,
    pub passkey_credential_id: String,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl AuthRegistrationCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthRegistrationFailed {
    pub email: String,
    pub reason: String,
    pub error_message: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl AuthRegistrationFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthLoginStarted {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl AuthLoginStarted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthLoginCompleted {
    pub user_id: String,
    pub email: Option<String>,
    pub passkey_credential_id: String,
    pub session_id: String,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl AuthLoginCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthLoginFailed {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub reason: String,
    pub error_message: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl AuthLoginFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthLogout {
    pub user_id: String,
    pub session_id: String,
    pub reason: String,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub logged_out_at: chrono::DateTime<chrono::Utc>,
}

impl AuthLogout {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthPasskeyCreated {
    pub user_id: String,
    pub passkey_credential_id: String,
    pub passkey_name: Option<String>,
    pub authenticator_type: String,
    #[serde(default)]
    pub discoverable: bool,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AuthPasskeyCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthPasskeyRemoved {
    pub user_id: String,
    pub passkey_credential_id: String,
    pub passkey_name: Option<String>,
    pub removed_by: String,
    pub reason: String,
    pub client_ip: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub removed_at: chrono::DateTime<chrono::Utc>,
}

impl AuthPasskeyRemoved {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthPasskeyUsed {
    pub user_id: String,
    pub passkey_credential_id: String,
    pub passkey_name: Option<String>,
    pub purpose: String,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub used_at: chrono::DateTime<chrono::Utc>,
}

impl AuthPasskeyUsed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthSessionCreated {
    pub user_id: String,
    pub session_id: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AuthSessionCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthSessionExpired {
    pub user_id: String,
    pub session_id: String,
    pub reason: String,
    #[serde(default = "chrono::Utc::now")]
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

impl AuthSessionExpired {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthSessionRevoked {
    pub user_id: String,
    pub session_id: String,
    pub revoked_by: String,
    pub reason: String,
    pub client_ip: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub revoked_at: chrono::DateTime<chrono::Utc>,
}

impl AuthSessionRevoked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthChallengeCreated {
    pub user_id: Option<String>,
    pub challenge_id: String,
    pub challenge_type: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub client_ip: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AuthChallengeCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthChallengeCompleted {
    pub user_id: Option<String>,
    pub challenge_id: String,
    pub challenge_type: String,
    pub success: bool,
    pub client_ip: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl AuthChallengeCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthAccountLocked {
    pub user_id: String,
    pub reason: String,
    pub failed_attempts: Option<i64>,
    pub locked_until: Option<chrono::DateTime<chrono::Utc>>,
    pub locked_by: String,
    #[serde(default = "chrono::Utc::now")]
    pub locked_at: chrono::DateTime<chrono::Utc>,
}

impl AuthAccountLocked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthAccountUnlocked {
    pub user_id: String,
    pub reason: String,
    pub unlocked_by: String,
    #[serde(default = "chrono::Utc::now")]
    pub unlocked_at: chrono::DateTime<chrono::Utc>,
}

impl AuthAccountUnlocked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthReauthenticationRequired {
    pub user_id: String,
    pub session_id: String,
    pub reason: String,
    pub action_requested: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub required_at: chrono::DateTime<chrono::Utc>,
}

impl AuthReauthenticationRequired {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthReauthenticationCompleted {
    pub user_id: String,
    pub session_id: String,
    pub passkey_credential_id: String,
    pub action_requested: Option<String>,
    pub client_ip: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl AuthReauthenticationCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


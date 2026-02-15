// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginAttempted {
    pub login_method: String,
    pub provider: Option<String>,
    #[serde(default)]
    pub success: bool,
    pub failure_reason: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub attempted_at: chrono::DateTime<chrono::Utc>,
}

impl LoginAttempted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogoutClicked {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub logout_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl LogoutClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasswordResetRequested {
    pub identifier: String,
    pub identifier_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub requested_at: chrono::DateTime<chrono::Utc>,
}

impl PasswordResetRequested {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignupInitiated {
    pub signup_method: String,
    pub provider: Option<String>,
    pub referral_source: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub initiated_at: chrono::DateTime<chrono::Utc>,
}

impl SignupInitiated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OauthProviderSelected {
    pub provider: String,
    pub context: String,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl OauthProviderSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


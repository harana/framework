// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Session {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub data: Option<String>,
    pub ip_address: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub last_activity_at: chrono::DateTime<chrono::Utc>,
    pub token: String,
    pub user_agent: Option<String>,
    /// Reference: user.id
    pub user_id: String,
}

impl Session {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// refresh_token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RefreshToken {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_revoked: bool,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: session.id
    pub session_id: String,
    pub token: String,
}

impl RefreshToken {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// session_activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionActivity {
    pub method: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
    pub metadata: Option<String>,
    /// Reference: session.id
    pub session_id: String,
}

impl SessionActivity {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


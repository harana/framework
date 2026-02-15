// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub user_id: String,
}

impl Session {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RefreshToken {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_revoked: bool,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub session_id: String,
    pub token: String,
}

impl RefreshToken {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionActivity {
    pub method: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
    pub metadata: Option<String>,
    pub session_id: String,
}

impl SessionActivity {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionData {
    pub values: std::collections::HashMap<String, String>,
}

impl SessionData {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl SessionInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


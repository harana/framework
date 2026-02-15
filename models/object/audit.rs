// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditEvent {
    pub action: String,
    pub actor_id: String,
    pub actor_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub metadata: Option<String>,
    pub outcome: String,
    pub resource_id: String,
    pub resource_type: String,
    pub user_agent: Option<String>,
}

impl AuditEvent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditAlert {
    pub action_pattern: Option<String>,
    pub actor_pattern: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub last_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub notification_channel: Option<String>,
    pub resource_pattern: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AuditAlert {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditExport {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub download_url: Option<String>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub file_size: Option<i64>,
    pub format: String,
    pub record_count: Option<i64>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

impl AuditExport {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditActivity {
    pub audit_id: String,
    pub method: String,
    pub resource_id: String,
    pub resource_type: String,
    pub outcome: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AuditActivity {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditFilter {
    pub method: String,
    pub actor_id: String,
    pub actor_type: String,
    pub outcome: String,
    pub resource_type: String,
}

impl AuditFilter {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditAlertCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

impl AuditAlertCondition {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditStatistic {
    pub key: String,
    pub count: i64,
    pub percentage: f64,
}

impl AuditStatistic {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


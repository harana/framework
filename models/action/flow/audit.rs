// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEventOutput {
    pub audit_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryLogsOutput {
    pub has_more: bool,
    pub logs: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEventOutput {
    pub method: String,
    pub actor_id: String,
    pub actor_type: String,
    pub audit_id: String,
    pub details: std::collections::HashMap<String, String>,
    pub ip_address: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub outcome: String,
    pub resource_id: String,
    pub resource_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetActorActivityOutput {
    pub activities: Vec<String>,
    pub first_activity: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetResourceHistoryOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub history: Vec<String>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExportLogsOutput {
    pub download_url: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub export_id: String,
    pub file_size: i64,
    pub record_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListAlertsOutput {
    pub alerts: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatisticsOutput {
    pub statistics: Vec<String>,
    pub total_events: i64,
    pub unique_actors: i64,
    pub unique_resources: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRetentionPolicyOutput {
    pub archive_to_cold_storage: bool,
    pub cold_storage_after_days: i64,
    pub policy_id: String,
    pub retention_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyIntegrityOutput {
    pub tampered_records: i64,
    pub total_records: i64,
    pub verification_hash: String,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditEvent {
    pub audit_id: String,
    pub method: String,
    pub actor_id: String,
    pub actor_type: String,
    pub resource_id: String,
    pub resource_type: String,
    pub outcome: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub user_agent: String,
    pub details: std::collections::HashMap<String, String>,
    pub metadata: std::collections::HashMap<String, String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditFilter {
    pub method: String,
    pub actor_id: String,
    pub actor_type: String,
    pub outcome: String,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditAlertCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditAlert {
    pub alert_id: String,
    pub name: String,
    pub description: String,
    pub conditions: String,
    pub enabled: bool,
    pub notification_channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditStatistic {
    pub key: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditExport {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub download_url: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub file_size: i64,
    pub format: String,
    pub record_count: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[async_trait]
pub trait AuditAction {
    async fn log_event(&self, method: String, actor_id: String, actor_type: String, details: std::collections::HashMap<String, String>, ip_address: String, metadata: std::collections::HashMap<String, String>, outcome: String, resource_id: String, resource_type: String, user_agent: String) -> Result<LogEventOutput, Box<dyn std::error::Error>>;
    async fn query_logs(&self, method: String, actor_id: String, actor_type: String, end_time: chrono::DateTime<chrono::Utc>, limit: i64, offset: i64, outcome: String, resource_id: String, resource_type: String, sort_order: String, start_time: chrono::DateTime<chrono::Utc>) -> Result<QueryLogsOutput, Box<dyn std::error::Error>>;
    async fn get_event(&self, audit_id: String) -> Result<GetEventOutput, Box<dyn std::error::Error>>;
    async fn get_actor_activity(&self, actor_id: String, end_time: chrono::DateTime<chrono::Utc>, limit: i64, start_time: chrono::DateTime<chrono::Utc>) -> Result<GetActorActivityOutput, Box<dyn std::error::Error>>;
    async fn get_resource_history(&self, end_time: chrono::DateTime<chrono::Utc>, limit: i64, resource_id: String, resource_type: String, start_time: chrono::DateTime<chrono::Utc>) -> Result<GetResourceHistoryOutput, Box<dyn std::error::Error>>;
    async fn export_logs(&self, end_time: chrono::DateTime<chrono::Utc>, filters: String, format: String, include_fields: Vec<String>, start_time: chrono::DateTime<chrono::Utc>) -> Result<ExportLogsOutput, Box<dyn std::error::Error>>;
    async fn create_alert(&self, conditions: String, description: String, enabled: bool, name: String, notification_channels: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_alert(&self, alert_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_alerts(&self, enabled: bool, limit: i64) -> Result<ListAlertsOutput, Box<dyn std::error::Error>>;
    async fn get_statistics(&self, end_time: chrono::DateTime<chrono::Utc>, group_by: String, start_time: chrono::DateTime<chrono::Utc>) -> Result<GetStatisticsOutput, Box<dyn std::error::Error>>;
    async fn set_retention_policy(&self, archive_to_cold_storage: bool, cold_storage_after_days: i64, retention_days: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_retention_policy(&self) -> Result<GetRetentionPolicyOutput, Box<dyn std::error::Error>>;
    async fn verify_integrity(&self, end_time: chrono::DateTime<chrono::Utc>, start_time: chrono::DateTime<chrono::Utc>) -> Result<VerifyIntegrityOutput, Box<dyn std::error::Error>>;
}

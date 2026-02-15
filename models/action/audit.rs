// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEventInput {
    pub method: String,
    pub actor_id: String,
    pub actor_type: String,
    pub details: std::collections::HashMap<String, String>,
    pub ip_address: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub outcome: String,
    pub resource_id: String,
    pub resource_type: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEventOutput {
    pub audit_id: String,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryLogsInput {
    pub method: String,
    pub actor_id: String,
    pub actor_type: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub limit: i64,
    pub offset: i64,
    pub outcome: String,
    pub resource_id: String,
    pub resource_type: String,
    pub sort_order: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
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
pub struct GetEventInput {
    pub audit_id: String,
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
pub struct GetActorActivityInput {
    pub actor_id: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub limit: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
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
pub struct GetResourceHistoryInput {
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub limit: i64,
    pub resource_id: String,
    pub resource_type: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
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
pub struct ExportLogsInput {
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub filters: String,
    pub format: String,
    pub include_fields: Vec<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
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
pub struct CreateAlertInput {
    pub conditions: String,
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    pub name: String,
    pub notification_channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAlertOutput {
    pub alert_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAlertInput {
    pub alert_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAlertOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListAlertsInput {
    pub enabled: bool,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListAlertsOutput {
    pub alerts: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatisticsInput {
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub group_by: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
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
pub struct SetRetentionPolicyInput {
    #[serde(default)]
    pub archive_to_cold_storage: bool,
    pub cold_storage_after_days: i64,
    pub retention_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetRetentionPolicyOutput {
    pub policy_id: String,
    pub success: bool,
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
pub struct VerifyIntegrityInput {
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub start_time: chrono::DateTime<chrono::Utc>,
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

#[async_trait]
pub trait AuditAction {
    async fn log_event(&self, input: LogEventInput) -> Result<LogEventOutput, Box<dyn std::error::Error>>;
    async fn query_logs(&self, input: QueryLogsInput) -> Result<QueryLogsOutput, Box<dyn std::error::Error>>;
    async fn get_event(&self, input: GetEventInput) -> Result<GetEventOutput, Box<dyn std::error::Error>>;
    async fn get_actor_activity(&self, input: GetActorActivityInput) -> Result<GetActorActivityOutput, Box<dyn std::error::Error>>;
    async fn get_resource_history(&self, input: GetResourceHistoryInput) -> Result<GetResourceHistoryOutput, Box<dyn std::error::Error>>;
    async fn export_logs(&self, input: ExportLogsInput) -> Result<ExportLogsOutput, Box<dyn std::error::Error>>;
    async fn create_alert(&self, input: CreateAlertInput) -> Result<CreateAlertOutput, Box<dyn std::error::Error>>;
    async fn delete_alert(&self, input: DeleteAlertInput) -> Result<DeleteAlertOutput, Box<dyn std::error::Error>>;
    async fn list_alerts(&self, input: ListAlertsInput) -> Result<ListAlertsOutput, Box<dyn std::error::Error>>;
    async fn get_statistics(&self, input: GetStatisticsInput) -> Result<GetStatisticsOutput, Box<dyn std::error::Error>>;
    async fn set_retention_policy(&self, input: SetRetentionPolicyInput) -> Result<SetRetentionPolicyOutput, Box<dyn std::error::Error>>;
    async fn get_retention_policy(&self) -> Result<GetRetentionPolicyOutput, Box<dyn std::error::Error>>;
    async fn verify_integrity(&self, input: VerifyIntegrityInput) -> Result<VerifyIntegrityOutput, Box<dyn std::error::Error>>;
}

// Harana Actions - Audit Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub audit_id: String,
    pub action: String,
    pub actor_id: String,
    pub actor_type: String,
    pub resource_id: String,
    pub resource_type: String,
    pub outcome: String,
    pub timestamp: DateTime<Utc>,
    pub details: Option<HashMap<String, Value>>,
    pub metadata: Option<HashMap<String, Value>>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditActivity {
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub timestamp: DateTime<Utc>,
    pub outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditFilter {
    pub action: Option<String>,
    pub actor_id: Option<String>,
    pub actor_type: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditAlert {
    pub alert_id: String,
    pub name: String,
    pub description: Option<String>,
    pub conditions: HashMap<String, Value>,
    pub notification_channels: Vec<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistic {
    pub key: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub policy_id: String,
    pub retention_days: i32,
    pub archive_to_cold_storage: bool,
    pub cold_storage_after_days: Option<i32>,
}

// log_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEventOutput {
    pub audit_id: String,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
}

// query_logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLogsOutput {
    pub logs: Vec<AuditEvent>,
    pub total: i32,
    pub has_more: bool,
}

// get_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventOutput {
    pub event: AuditEvent,
}

// get_actor_activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActorActivityOutput {
    pub activities: Vec<AuditActivity>,
    pub total: i32,
    pub first_activity: Option<DateTime<Utc>>,
    pub last_activity: Option<DateTime<Utc>>,
}

// get_resource_history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResourceHistoryOutput {
    pub history: Vec<AuditEvent>,
    pub total: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub last_modified: Option<DateTime<Utc>>,
}

// export_logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportLogsOutput {
    pub export_id: String,
    pub download_url: String,
    pub record_count: i32,
    pub file_size: i64,
    pub expires_at: DateTime<Utc>,
}

// create_alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlertOutput {
    pub alert_id: String,
    pub success: bool,
}

// delete_alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAlertOutput {
    pub success: bool,
}

// list_alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAlertsOutput {
    pub alerts: Vec<AuditAlert>,
    pub total: i32,
}

// get_statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatisticsOutput {
    pub statistics: Vec<AuditStatistic>,
    pub total_events: i32,
    pub unique_actors: i32,
    pub unique_resources: i32,
}

// set_retention_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRetentionPolicyOutput {
    pub policy_id: String,
    pub success: bool,
}

// get_retention_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRetentionPolicyOutput {
    pub policy: Option<RetentionPolicy>,
}

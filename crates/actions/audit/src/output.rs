// Harana Actions - Audit Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

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
    pub has_more: bool,
    pub logs: Vec<AuditEvent>,
    pub total: i32,
}

// get_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventOutput {
    pub action: String,
    pub actor_id: String,
    pub actor_type: String,
    pub audit_id: String,
    pub details: Option<HashMap<String, Value>>,
    pub ip_address: Option<String>,
    pub metadata: Option<HashMap<String, Value>>,
    pub outcome: String,
    pub resource_id: String,
    pub resource_type: String,
    pub timestamp: DateTime<Utc>,
    pub user_agent: Option<String>,
}

// get_actor_activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActorActivityOutput {
    pub activities: Vec<AuditActivity>,
    pub first_activity: Option<DateTime<Utc>>,
    pub last_activity: Option<DateTime<Utc>>,
    pub total: i32,
}

// get_resource_history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetResourceHistoryOutput {
    pub created_at: Option<DateTime<Utc>>,
    pub history: Vec<AuditEvent>,
    pub last_modified: Option<DateTime<Utc>>,
    pub total: i32,
}

// export_logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportLogsOutput {
    pub download_url: String,
    pub expires_at: DateTime<Utc>,
    pub export_id: String,
    pub file_size: i64,
    pub record_count: i32,
}

// create_alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlertOutput {
    pub alert_id: String,
    pub success: bool,
}

// Helper structs
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditActivity {
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub timestamp: DateTime<Utc>,
    pub outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFilter {
    pub action: Option<String>,
    pub actor_id: Option<String>,
    pub resource_type: Option<String>,
    pub outcome: Option<String>,
}

pub mod output;

use output::*;
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Global storage for audit events.
static AUDIT_LOGS: Lazy<DashMap<String, AuditEvent>> = Lazy::new(DashMap::new);
/// Global storage for audit alerts.
static ALERTS: Lazy<DashMap<String, AuditAlert>> = Lazy::new(DashMap::new);
/// Global storage for retention policy.
static RETENTION_POLICY: Lazy<RwLock<Option<RetentionPolicy>>> = Lazy::new(|| RwLock::new(None));
/// Global storage for exports.
static EXPORTS: Lazy<DashMap<String, ExportLogsOutput>> = Lazy::new(DashMap::new);

/// Log Audit Event.
pub async fn log_event(
    action: &str,
    actor_id: &str,
    outcome: &str,
    resource_id: &str,
    resource_type: &str,
    actor_type: Option<&str>,
    details: Option<HashMap<String, Value>>,
    ip_address: Option<&str>,
    metadata: Option<HashMap<String, Value>>,
    user_agent: Option<&str>,
) -> Result<LogEventOutput, String> {
    let audit_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now();
    
    let event = AuditEvent {
        audit_id: audit_id.clone(),
        action: action.to_string(),
        actor_id: actor_id.to_string(),
        actor_type: actor_type.unwrap_or("User").to_string(),
        resource_id: resource_id.to_string(),
        resource_type: resource_type.to_string(),
        outcome: outcome.to_string(),
        timestamp,
        details,
        metadata,
        ip_address: ip_address.map(String::from),
        user_agent: user_agent.map(String::from),
    };
    
    AUDIT_LOGS.insert(audit_id.clone(), event);
    
    Ok(LogEventOutput {
        audit_id,
        success: true,
        timestamp,
    })
}

/// Query Audit Logs.
pub async fn query_logs(
    start_time: DateTime<Utc>,
    action: Option<&str>,
    actor_id: Option<&str>,
    actor_type: Option<&str>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<i32>,
    offset: Option<i32>,
    outcome: Option<&str>,
    resource_id: Option<&str>,
    resource_type: Option<&str>,
    sort_order: Option<&str>,
) -> Result<QueryLogsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let offset = offset.unwrap_or(0) as usize;
    let end_time = end_time.unwrap_or_else(Utc::now);
    let sort_desc = sort_order.map_or(true, |s| s.to_lowercase() != "asc");
    
    let mut logs: Vec<AuditEvent> = AUDIT_LOGS
        .iter()
        .filter(|e| {
            // Time range filter
            if e.timestamp < start_time || e.timestamp > end_time {
                return false;
            }
            // Optional filters
            if let Some(a) = action {
                if e.action != a {
                    return false;
                }
            }
            if let Some(aid) = actor_id {
                if e.actor_id != aid {
                    return false;
                }
            }
            if let Some(at) = actor_type {
                if e.actor_type != at {
                    return false;
                }
            }
            if let Some(o) = outcome {
                if e.outcome != o {
                    return false;
                }
            }
            if let Some(rid) = resource_id {
                if e.resource_id != rid {
                    return false;
                }
            }
            if let Some(rt) = resource_type {
                if e.resource_type != rt {
                    return false;
                }
            }
            true
        })
        .map(|e| e.clone())
        .collect();
    
    // Sort by timestamp
    if sort_desc {
        logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    } else {
        logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }
    
    let total = logs.len() as i32;
    let has_more = offset + limit < logs.len();
    let logs: Vec<AuditEvent> = logs.into_iter().skip(offset).take(limit).collect();
    
    Ok(QueryLogsOutput {
        logs,
        total,
        has_more,
    })
}

/// Get Audit Event By ID.
pub async fn get_event(
    audit_id: &str,
) -> Result<GetEventOutput, String> {
    let event = AUDIT_LOGS
        .get(audit_id)
        .ok_or_else(|| format!("Audit event not found: {}", audit_id))?;
    
    Ok(GetEventOutput {
        event: event.clone(),
    })
}

/// Get Actor Activity.
pub async fn get_actor_activity(
    actor_id: &str,
    end_time: Option<DateTime<Utc>>,
    limit: Option<i32>,
    start_time: Option<DateTime<Utc>>,
) -> Result<GetActorActivityOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let end_time = end_time.unwrap_or_else(Utc::now);
    let start_time = start_time.unwrap_or_else(|| Utc::now() - Duration::days(30));
    
    let mut activities: Vec<AuditActivity> = AUDIT_LOGS
        .iter()
        .filter(|e| {
            e.actor_id == actor_id && e.timestamp >= start_time && e.timestamp <= end_time
        })
        .map(|e| AuditActivity {
            action: e.action.clone(),
            resource_type: e.resource_type.clone(),
            resource_id: e.resource_id.clone(),
            timestamp: e.timestamp,
            outcome: e.outcome.clone(),
        })
        .collect();
    
    activities.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    let first_activity = activities.last().map(|a| a.timestamp);
    let last_activity = activities.first().map(|a| a.timestamp);
    let total = activities.len() as i32;
    
    let activities: Vec<AuditActivity> = activities.into_iter().take(limit).collect();
    
    Ok(GetActorActivityOutput {
        activities,
        total,
        first_activity,
        last_activity,
    })
}

/// Get Resource History.
pub async fn get_resource_history(
    resource_id: &str,
    resource_type: &str,
    end_time: Option<DateTime<Utc>>,
    limit: Option<i32>,
    start_time: Option<DateTime<Utc>>,
) -> Result<GetResourceHistoryOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let end_time = end_time.unwrap_or_else(Utc::now);
    let start_time = start_time.unwrap_or_else(|| Utc::now() - Duration::days(365));
    
    let mut history: Vec<AuditEvent> = AUDIT_LOGS
        .iter()
        .filter(|e| {
            e.resource_id == resource_id
                && e.resource_type == resource_type
                && e.timestamp >= start_time
                && e.timestamp <= end_time
        })
        .map(|e| e.clone())
        .collect();
    
    history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    let last_modified = history.first().map(|e| e.timestamp);
    let created_at = history.last().map(|e| e.timestamp);
    let total = history.len() as i32;
    
    let history: Vec<AuditEvent> = history.into_iter().take(limit).collect();
    
    Ok(GetResourceHistoryOutput {
        history,
        total,
        created_at,
        last_modified,
    })
}

/// Export Audit Logs.
pub async fn export_logs(
    end_time: DateTime<Utc>,
    start_time: DateTime<Utc>,
    filters: Option<AuditFilter>,
    format: Option<&str>,
    _include_fields: Option<Vec<String>>,
) -> Result<ExportLogsOutput, String> {
    let format = format.unwrap_or("Json");
    let filters = filters.unwrap_or_default();
    
    let logs: Vec<AuditEvent> = AUDIT_LOGS
        .iter()
        .filter(|e| {
            if e.timestamp < start_time || e.timestamp > end_time {
                return false;
            }
            if let Some(ref a) = filters.action {
                if &e.action != a {
                    return false;
                }
            }
            if let Some(ref aid) = filters.actor_id {
                if &e.actor_id != aid {
                    return false;
                }
            }
            if let Some(ref at) = filters.actor_type {
                if &e.actor_type != at {
                    return false;
                }
            }
            if let Some(ref o) = filters.outcome {
                if &e.outcome != o {
                    return false;
                }
            }
            if let Some(ref rt) = filters.resource_type {
                if &e.resource_type != rt {
                    return false;
                }
            }
            if let Some(ref rid) = filters.resource_id {
                if &e.resource_id != rid {
                    return false;
                }
            }
            true
        })
        .map(|e| e.clone())
        .collect();
    
    let export_id = Uuid::new_v4().to_string();
    let record_count = logs.len() as i32;
    
    // Simulate export file size based on format
    let file_size = match format.to_lowercase().as_str() {
        "csv" => (record_count as i64) * 200,
        "parquet" => (record_count as i64) * 100,
        _ => (record_count as i64) * 500, // JSON
    };
    
    let expires_at = Utc::now() + Duration::hours(24);
    let download_url = format!("https://exports.example.com/audit/{}.{}", export_id, format.to_lowercase());
    
    let output = ExportLogsOutput {
        export_id: export_id.clone(),
        download_url,
        record_count,
        file_size,
        expires_at,
    };
    
    EXPORTS.insert(export_id, output.clone());
    
    Ok(output)
}

/// Create Audit Alert.
pub async fn create_alert(
    name: &str,
    conditions: HashMap<String, Value>,
    notification_channels: Vec<String>,
    description: Option<&str>,
    enabled: Option<bool>,
) -> Result<CreateAlertOutput, String> {
    let alert_id = Uuid::new_v4().to_string();
    
    let alert = AuditAlert {
        alert_id: alert_id.clone(),
        name: name.to_string(),
        description: description.map(String::from),
        conditions,
        notification_channels,
        enabled: enabled.unwrap_or(true),
        created_at: Utc::now(),
    };
    
    ALERTS.insert(alert_id.clone(), alert);
    
    Ok(CreateAlertOutput {
        alert_id,
        success: true,
    })
}

/// Delete Audit Alert.
pub async fn delete_alert(
    alert_id: &str,
) -> Result<DeleteAlertOutput, String> {
    ALERTS
        .remove(alert_id)
        .ok_or_else(|| format!("Alert not found: {}", alert_id))?;
    
    Ok(DeleteAlertOutput { success: true })
}

/// List Audit Alerts.
pub async fn list_alerts(
    enabled: Option<bool>,
    limit: Option<i32>,
) -> Result<ListAlertsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    
    let alerts: Vec<AuditAlert> = ALERTS
        .iter()
        .filter(|a| {
            if let Some(e) = enabled {
                a.enabled == e
            } else {
                true
            }
        })
        .take(limit)
        .map(|a| a.clone())
        .collect();
    
    let total = alerts.len() as i32;
    
    Ok(ListAlertsOutput { alerts, total })
}

/// Get Audit Statistics.
pub async fn get_statistics(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    group_by: Option<&str>,
) -> Result<GetStatisticsOutput, String> {
    let group_by = group_by.unwrap_or("Day");
    
    let logs: Vec<AuditEvent> = AUDIT_LOGS
        .iter()
        .filter(|e| e.timestamp >= start_time && e.timestamp <= end_time)
        .map(|e| e.clone())
        .collect();
    
    let total_events = logs.len() as i32;
    let unique_actors: HashSet<&str> = logs.iter().map(|e| e.actor_id.as_str()).collect();
    let unique_resources: HashSet<(&str, &str)> = logs
        .iter()
        .map(|e| (e.resource_type.as_str(), e.resource_id.as_str()))
        .collect();
    
    let mut counts: HashMap<String, i32> = HashMap::new();
    
    for event in &logs {
        let key = match group_by.to_lowercase().as_str() {
            "action" => event.action.clone(),
            "resourcetype" => event.resource_type.clone(),
            "actorid" => event.actor_id.clone(),
            "outcome" => event.outcome.clone(),
            "hour" => event.timestamp.format("%Y-%m-%d %H:00").to_string(),
            _ => event.timestamp.format("%Y-%m-%d").to_string(), // Day
        };
        *counts.entry(key).or_insert(0) += 1;
    }
    
    let statistics: Vec<AuditStatistic> = counts
        .into_iter()
        .map(|(key, count)| AuditStatistic { key, count })
        .collect();
    
    Ok(GetStatisticsOutput {
        statistics,
        total_events,
        unique_actors: unique_actors.len() as i32,
        unique_resources: unique_resources.len() as i32,
    })
}

/// Set Retention Policy.
pub async fn set_retention_policy(
    retention_days: i32,
    archive_to_cold_storage: Option<bool>,
    cold_storage_after_days: Option<i32>,
) -> Result<SetRetentionPolicyOutput, String> {
    let policy_id = Uuid::new_v4().to_string();
    
    let policy = RetentionPolicy {
        policy_id: policy_id.clone(),
        retention_days,
        archive_to_cold_storage: archive_to_cold_storage.unwrap_or(false),
        cold_storage_after_days,
    };
    
    *RETENTION_POLICY.write() = Some(policy);
    
    Ok(SetRetentionPolicyOutput {
        policy_id,
        success: true,
    })
}

/// Get Retention Policy.
pub async fn get_retention_policy() -> Result<GetRetentionPolicyOutput, String> {
    let policy = RETENTION_POLICY.read().clone();
    Ok(GetRetentionPolicyOutput { policy })
}

#[cfg(test)]
mod tests;

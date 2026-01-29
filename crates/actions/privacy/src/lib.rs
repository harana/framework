pub mod output;

use output::*;
use chrono::{Duration, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

/// Storage for consents.
static CONSENTS: Lazy<DashMap<String, StoredConsent>> = Lazy::new(DashMap::new);
/// Index by user_id -> consent_type -> consent_id.
static USER_CONSENTS: Lazy<DashMap<String, HashMap<String, String>>> = Lazy::new(DashMap::new);
/// Storage for consent history.
static CONSENT_HISTORY: Lazy<DashMap<String, Vec<ConsentHistoryEntry>>> = Lazy::new(DashMap::new);
/// Storage for data access logs.
static ACCESS_LOGS: Lazy<DashMap<String, Vec<AccessLogEntry>>> = Lazy::new(DashMap::new);
/// Storage for privacy policies.
static POLICIES: Lazy<DashMap<String, StoredPrivacyPolicy>> = Lazy::new(DashMap::new);
/// Storage for policy acceptances.
static POLICY_ACCEPTANCES: Lazy<DashMap<String, PolicyAcceptance>> = Lazy::new(DashMap::new);
/// Storage for data exports.
static EXPORTS: Lazy<DashMap<String, DataExport>> = Lazy::new(DashMap::new);
/// Storage for data deletions.
static DELETIONS: Lazy<DashMap<String, DataDeletion>> = Lazy::new(DashMap::new);
/// Storage for anonymizations.
static ANONYMIZATIONS: Lazy<DashMap<String, Anonymization>> = Lazy::new(DashMap::new);

#[derive(Debug, Clone)]
struct StoredConsent {
    consent_id: String,
    user_id: String,
    consent_type: String,
    granted: bool,
    version: String,
    metadata: Option<HashMap<String, Value>>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    timestamp: i64,
    revoked: bool,
    revoked_at: Option<i64>,
}

#[derive(Debug, Clone)]
struct ConsentHistoryEntry {
    consent_id: String,
    consent_type: String,
    action: String, // "granted", "revoked", "updated"
    timestamp: i64,
    details: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
struct AccessLogEntry {
    access_id: String,
    accessor_id: String,
    resource: String,
    access_type: String,
    purpose: Option<String>,
    timestamp: i64,
}

#[derive(Debug, Clone)]
struct StoredPrivacyPolicy {
    policy_id: String,
    policy_type: String,
    title: String,
    content: String,
    version: String,
    effective_date: i64,
    active: bool,
}

#[derive(Debug, Clone)]
struct PolicyAcceptance {
    acceptance_id: String,
    policy_id: String,
    user_id: String,
    timestamp: i64,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

#[derive(Debug, Clone)]
struct DataExport {
    export_id: String,
    user_id: String,
    status: String,
    format: String,
    download_url: String,
    created_at: i64,
    completed_at: Option<i64>,
    expires_at: i64,
}

#[derive(Debug, Clone)]
struct DataDeletion {
    deletion_id: String,
    user_id: String,
    status: String, // "scheduled", "processing", "completed", "cancelled"
    delete_type: String,
    reason: String,
    scheduled_at: i64,
    completed_at: Option<i64>,
    retain_sections: Vec<String>,
}

#[derive(Debug, Clone)]
struct Anonymization {
    anonymization_id: String,
    user_id: String,
    timestamp: i64,
    retain_sections: Vec<String>,
}

fn now_timestamp() -> i64 {
    Utc::now().timestamp()
}

/// Anonymize User Data.
pub async fn anonymize_data(
    user_id: &str,
    retain_sections: Option<Vec<String>>,
) -> Result<AnonymizeDataOutput, String> {
    let anonymization_id = Uuid::new_v4().to_string();
    
    let anon = Anonymization {
        anonymization_id: anonymization_id.clone(),
        user_id: user_id.to_string(),
        timestamp: now_timestamp(),
        retain_sections: retain_sections.unwrap_or_default(),
    };
    
    ANONYMIZATIONS.insert(anonymization_id.clone(), anon);
    
    Ok(AnonymizeDataOutput {
        anonymization_id,
        user_id: user_id.to_string(),
        success: true,
    })
}

/// Cancel Data Deletion.
pub async fn cancel_deletion(
    deletion_id: &str,
) -> Result<CancelDeletionOutput, String> {
    let mut deletion = DELETIONS
        .get_mut(deletion_id)
        .ok_or_else(|| format!("Deletion not found: {}", deletion_id))?;
    
    if deletion.status == "completed" {
        return Err("Cannot cancel completed deletion".to_string());
    }
    
    if deletion.status == "cancelled" {
        return Ok(CancelDeletionOutput {
            success: true,
            cancelled: false,
            deletion_id: deletion_id.to_string(),
        });
    }
    
    deletion.status = "cancelled".to_string();
    
    Ok(CancelDeletionOutput {
        success: true,
        cancelled: true,
        deletion_id: deletion_id.to_string(),
    })
}

/// Create Privacy Policy Version.
pub async fn create_policy_version(
    title: &str,
    effective_date: i32,
    content: &str,
    version: &str,
) -> Result<CreatePolicyVersionOutput, String> {
    let policy_id = Uuid::new_v4().to_string();
    
    let policy = StoredPrivacyPolicy {
        policy_id: policy_id.clone(),
        policy_type: "privacy".to_string(),
        title: title.to_string(),
        content: content.to_string(),
        version: version.to_string(),
        effective_date: effective_date as i64,
        active: true,
    };
    
    // Deactivate previous versions
    for mut p in POLICIES.iter_mut() {
        if p.policy_type == "privacy" && p.active {
            p.active = false;
        }
    }
    
    POLICIES.insert(policy_id.clone(), policy);
    
    Ok(CreatePolicyVersionOutput {
        policy_id,
        success: true,
        version: version.to_string(),
    })
}

/// Delete User Data.
pub async fn delete_data(
    user_id: &str,
    reason: &str,
    delete_type: Option<&str>,
    retain_sections: Option<Vec<String>>,
) -> Result<DeleteDataOutput, String> {
    let deletion_id = Uuid::new_v4().to_string();
    let scheduled_at = now_timestamp() + 86400 * 30; // 30 days from now
    
    let deletion = DataDeletion {
        deletion_id: deletion_id.clone(),
        user_id: user_id.to_string(),
        status: "scheduled".to_string(),
        delete_type: delete_type.unwrap_or("soft").to_string(),
        reason: reason.to_string(),
        scheduled_at,
        completed_at: None,
        retain_sections: retain_sections.unwrap_or_default(),
    };
    
    DELETIONS.insert(deletion_id.clone(), deletion);
    
    Ok(DeleteDataOutput {
        deletion_id,
        user_id: user_id.to_string(),
        status: "scheduled".to_string(),
        scheduled_at: scheduled_at as i32,
        success: true,
    })
}

/// Export User Data.
pub async fn export_data(
    user_id: &str,
    _exclude_sections: Option<Vec<String>>,
    _include_sections: Option<Vec<String>>,
    format: Option<&str>,
) -> Result<ExportDataOutput, String> {
    let export_id = Uuid::new_v4().to_string();
    let format = format.unwrap_or("json");
    let created_at = now_timestamp();
    let completed_at = created_at;
    let expires_at = created_at + 86400 * 7; // 7 days
    
    let download_url = format!(
        "https://exports.example.com/privacy/{}.{}",
        export_id, format
    );
    
    let export = DataExport {
        export_id: export_id.clone(),
        user_id: user_id.to_string(),
        status: "completed".to_string(),
        format: format.to_string(),
        download_url: download_url.clone(),
        created_at,
        completed_at: Some(completed_at),
        expires_at,
    };
    
    EXPORTS.insert(export_id.clone(), export);
    
    Ok(ExportDataOutput {
        export_id,
        user_id: user_id.to_string(),
        status: "completed".to_string(),
        download_url,
        expires_at: expires_at as i32,
    })
}

/// Get Data Access Log.
pub async fn get_access_log(
    user_id: &str,
    start_date: Option<i32>,
    end_date: Option<i32>,
    limit: Option<i32>,
) -> Result<GetAccessLogOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let start_date = start_date.map(|d| d as i64).unwrap_or(0);
    let end_date = end_date.map(|d| d as i64).unwrap_or(i64::MAX);
    
    let logs = ACCESS_LOGS
        .get(user_id)
        .map(|entries| entries.clone())
        .unwrap_or_default();
    
    let accesses: Vec<HashMap<String, Value>> = logs
        .iter()
        .filter(|e| e.timestamp >= start_date && e.timestamp <= end_date)
        .take(limit)
        .map(|e| {
            let mut map = HashMap::new();
            map.insert("access_id".to_string(), json!(e.access_id.clone()));
            map.insert("accessor_id".to_string(), json!(e.accessor_id.clone()));
            map.insert("resource".to_string(), json!(e.resource.clone()));
            map.insert("access_type".to_string(), json!(e.access_type.clone()));
            map.insert("purpose".to_string(), json!(e.purpose.clone()));
            map.insert("timestamp".to_string(), json!(e.timestamp));
            map
        })
        .collect();
    
    let total = accesses.len() as i32;
    
    Ok(GetAccessLogOutput {
        user_id: user_id.to_string(),
        accesses,
        total,
    })
}

/// Get Active Policy Version.
pub async fn get_active_policy(
    policy_type: Option<&str>,
) -> Result<GetActivePolicyOutput, String> {
    let policy_type = policy_type.unwrap_or("privacy");
    
    let policy = POLICIES
        .iter()
        .find(|p| p.policy_type == policy_type && p.active)
        .ok_or_else(|| format!("No active policy found for type: {}", policy_type))?;
    
    Ok(GetActivePolicyOutput {
        policy_id: policy.policy_id.clone(),
        version: policy.version.clone(),
        content: policy.content.clone(),
        effective_date: policy.effective_date as i32,
    })
}

/// Get Consent Status.
pub async fn get_consent_status(
    user_id: &str,
    consent_type: Option<&str>,
) -> Result<GetConsentStatusOutput, String> {
    let user_consents = USER_CONSENTS
        .get(user_id)
        .map(|c| c.clone())
        .unwrap_or_default();
    
    let consents: Vec<HashMap<String, Value>> = user_consents
        .iter()
        .filter(|(ct, _)| consent_type.map_or(true, |t| *ct == t))
        .filter_map(|(_, consent_id)| {
            CONSENTS.get(consent_id).map(|c| {
                let mut map = HashMap::new();
                map.insert("consent_id".to_string(), json!(c.consent_id.clone()));
                map.insert("consent_type".to_string(), json!(c.consent_type.clone()));
                map.insert("granted".to_string(), json!(c.granted));
                map.insert("version".to_string(), json!(c.version.clone()));
                map.insert("timestamp".to_string(), json!(c.timestamp));
                map.insert("revoked".to_string(), json!(c.revoked));
                map
            })
        })
        .collect();
    
    Ok(GetConsentStatusOutput {
        user_id: user_id.to_string(),
        consents,
    })
}

/// Get Deletion Status.
pub async fn get_deletion_status(
    deletion_id: &str,
) -> Result<GetDeletionStatusOutput, String> {
    let deletion = DELETIONS
        .get(deletion_id)
        .ok_or_else(|| format!("Deletion not found: {}", deletion_id))?;
    
    Ok(GetDeletionStatusOutput {
        deletion_id: deletion.deletion_id.clone(),
        user_id: deletion.user_id.clone(),
        status: deletion.status.clone(),
        scheduled_at: deletion.scheduled_at as i32,
        completed_at: deletion.completed_at.unwrap_or(0) as i32,
    })
}

/// Get Export Status.
pub async fn get_export_status(
    export_id: &str,
) -> Result<GetExportStatusOutput, String> {
    let export = EXPORTS
        .get(export_id)
        .ok_or_else(|| format!("Export not found: {}", export_id))?;
    
    Ok(GetExportStatusOutput {
        export_id: export.export_id.clone(),
        status: export.status.clone(),
        download_url: export.download_url.clone(),
        created_at: export.created_at as i32,
        completed_at: export.completed_at.unwrap_or(0) as i32,
    })
}

/// List Consent History.
pub async fn list_consent_history(
    user_id: &str,
    start_date: Option<i32>,
    end_date: Option<i32>,
    consent_type: Option<&str>,
    limit: Option<i32>,
) -> Result<ListConsentHistoryOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let start_date = start_date.map(|d| d as i64).unwrap_or(0);
    let end_date = end_date.map(|d| d as i64).unwrap_or(i64::MAX);
    
    let history = CONSENT_HISTORY
        .get(user_id)
        .map(|h| h.clone())
        .unwrap_or_default();
    
    let entries: Vec<HashMap<String, Value>> = history
        .iter()
        .filter(|e| {
            e.timestamp >= start_date
                && e.timestamp <= end_date
                && consent_type.map_or(true, |t| e.consent_type == t)
        })
        .take(limit)
        .map(|e| {
            let mut map = HashMap::new();
            map.insert("consent_id".to_string(), json!(e.consent_id.clone()));
            map.insert("consent_type".to_string(), json!(e.consent_type.clone()));
            map.insert("action".to_string(), json!(e.action.clone()));
            map.insert("timestamp".to_string(), json!(e.timestamp));
            map.insert("details".to_string(), json!(e.details.clone()));
            map
        })
        .collect();
    
    let total = entries.len() as i32;
    
    Ok(ListConsentHistoryOutput {
        user_id: user_id.to_string(),
        history: entries,
        total,
    })
}

/// Record Consent.
pub async fn record_consent(
    user_id: &str,
    consent_type: &str,
    granted: bool,
    version: &str,
    metadata: Option<HashMap<String, Value>>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<RecordConsentOutput, String> {
    let consent_id = Uuid::new_v4().to_string();
    let timestamp = now_timestamp();
    
    let consent = StoredConsent {
        consent_id: consent_id.clone(),
        user_id: user_id.to_string(),
        consent_type: consent_type.to_string(),
        granted,
        version: version.to_string(),
        metadata,
        ip_address: ip_address.map(String::from),
        user_agent: user_agent.map(String::from),
        timestamp,
        revoked: false,
        revoked_at: None,
    };
    
    CONSENTS.insert(consent_id.clone(), consent);
    
    // Update user consent index
    USER_CONSENTS
        .entry(user_id.to_string())
        .or_insert_with(HashMap::new)
        .insert(consent_type.to_string(), consent_id.clone());
    
    // Add to history
    let history_entry = ConsentHistoryEntry {
        consent_id: consent_id.clone(),
        consent_type: consent_type.to_string(),
        action: if granted { "granted" } else { "denied" }.to_string(),
        timestamp,
        details: HashMap::new(),
    };
    
    CONSENT_HISTORY
        .entry(user_id.to_string())
        .or_insert_with(Vec::new)
        .push(history_entry);
    
    Ok(RecordConsentOutput {
        consent_id,
        user_id: user_id.to_string(),
        success: true,
        timestamp: timestamp as i32,
    })
}

/// Record Data Access.
pub async fn record_data_access(
    accessor_id: &str,
    resource: &str,
    access_type: &str,
    user_id: &str,
    purpose: Option<&str>,
) -> Result<RecordDataAccessOutput, String> {
    let access_id = Uuid::new_v4().to_string();
    let timestamp = now_timestamp();
    
    let entry = AccessLogEntry {
        access_id: access_id.clone(),
        accessor_id: accessor_id.to_string(),
        resource: resource.to_string(),
        access_type: access_type.to_string(),
        purpose: purpose.map(String::from),
        timestamp,
    };
    
    ACCESS_LOGS
        .entry(user_id.to_string())
        .or_insert_with(Vec::new)
        .push(entry);
    
    Ok(RecordDataAccessOutput {
        access_id,
        success: true,
        timestamp: timestamp as i32,
    })
}

/// Record Policy Acceptance.
pub async fn record_policy_acceptance(
    policy_id: &str,
    user_id: &str,
    user_agent: Option<&str>,
    ip_address: Option<&str>,
) -> Result<RecordPolicyAcceptanceOutput, String> {
    // Verify policy exists
    if !POLICIES.contains_key(policy_id) {
        return Err(format!("Policy not found: {}", policy_id));
    }
    
    let acceptance_id = Uuid::new_v4().to_string();
    let timestamp = now_timestamp();
    
    let acceptance = PolicyAcceptance {
        acceptance_id: acceptance_id.clone(),
        policy_id: policy_id.to_string(),
        user_id: user_id.to_string(),
        timestamp,
        ip_address: ip_address.map(String::from),
        user_agent: user_agent.map(String::from),
    };
    
    POLICY_ACCEPTANCES.insert(acceptance_id.clone(), acceptance);
    
    Ok(RecordPolicyAcceptanceOutput {
        acceptance_id,
        user_id: user_id.to_string(),
        success: true,
        timestamp: timestamp as i32,
    })
}

/// Revoke Consent.
pub async fn revoke_consent(
    consent_type: &str,
    user_id: &str,
    reason: Option<&str>,
) -> Result<RevokeConsentOutput, String> {
    let consent_id = USER_CONSENTS
        .get(user_id)
        .and_then(|c| c.get(consent_type).cloned())
        .ok_or_else(|| format!("No consent found for type: {}", consent_type))?;
    
    let mut consent = CONSENTS
        .get_mut(&consent_id)
        .ok_or_else(|| format!("Consent not found: {}", consent_id))?;
    
    let revoked_at = now_timestamp();
    consent.revoked = true;
    consent.revoked_at = Some(revoked_at);
    
    // Add to history
    let mut details = HashMap::new();
    if let Some(r) = reason {
        details.insert("reason".to_string(), json!(r));
    }
    
    let history_entry = ConsentHistoryEntry {
        consent_id: consent_id.clone(),
        consent_type: consent_type.to_string(),
        action: "revoked".to_string(),
        timestamp: revoked_at,
        details,
    };
    
    CONSENT_HISTORY
        .entry(user_id.to_string())
        .or_insert_with(Vec::new)
        .push(history_entry);
    
    Ok(RevokeConsentOutput {
        consent_id,
        user_id: user_id.to_string(),
        revoked_at: revoked_at as i32,
        success: true,
    })
}

/// Update Consent.
pub async fn update_consent(
    consent_id: &str,
    granted: bool,
    version: Option<&str>,
) -> Result<UpdateConsentOutput, String> {
    let mut consent = CONSENTS
        .get_mut(consent_id)
        .ok_or_else(|| format!("Consent not found: {}", consent_id))?;
    
    consent.granted = granted;
    if let Some(v) = version {
        consent.version = v.to_string();
    }
    consent.timestamp = now_timestamp();
    
    // Add to history
    let history_entry = ConsentHistoryEntry {
        consent_id: consent_id.to_string(),
        consent_type: consent.consent_type.clone(),
        action: "updated".to_string(),
        timestamp: consent.timestamp,
        details: HashMap::new(),
    };
    
    CONSENT_HISTORY
        .entry(consent.user_id.clone())
        .or_insert_with(Vec::new)
        .push(history_entry);
    
    Ok(UpdateConsentOutput {
        consent_id: consent_id.to_string(),
        success: true,
    })
}

/// Verify Right To Be Forgotten Eligibility.
pub async fn verify_rtbf_eligibility(
    user_id: &str,
) -> Result<VerifyRtbfEligibilityOutput, String> {
    let mut blocking_factors = Vec::new();
    let mut reasons = Vec::new();
    
    // Check for pending deletions
    let has_pending_deletion = DELETIONS.iter().any(|d| {
        d.user_id == user_id && (d.status == "scheduled" || d.status == "processing")
    });
    
    if has_pending_deletion {
        blocking_factors.push("Pending deletion request already exists".to_string());
    }
    
    // Check for active consents (simulated business rules)
    let active_consents = USER_CONSENTS
        .get(user_id)
        .map(|c| c.len())
        .unwrap_or(0);
    
    // Add eligibility reasons
    reasons.push(format!("User has {} recorded consent(s)", active_consents));
    reasons.push("No legal hold detected".to_string());
    reasons.push("Account is not under investigation".to_string());
    
    let eligible = blocking_factors.is_empty();
    
    Ok(VerifyRtbfEligibilityOutput {
        user_id: user_id.to_string(),
        eligible,
        reasons,
        blocking_factors,
    })
}

#[cfg(test)]
mod tests;

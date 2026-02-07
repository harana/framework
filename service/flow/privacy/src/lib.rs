pub mod output;

#[cfg(test)]
mod tests;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use chrono::{Utc, Duration};
use uuid::Uuid;

use output::*;

// In-memory storage
static CONSENTS: Lazy<DashMap<String, StoredConsent>> = Lazy::new(DashMap::new);
static CONSENT_HISTORY: Lazy<DashMap<String, Vec<StoredConsentHistory>>> = Lazy::new(DashMap::new);
static EXPORTS: Lazy<DashMap<String, StoredExport>> = Lazy::new(DashMap::new);
static DELETIONS: Lazy<DashMap<String, StoredDeletion>> = Lazy::new(DashMap::new);
static DATA_ACCESSES: Lazy<DashMap<String, Vec<StoredDataAccess>>> = Lazy::new(DashMap::new);
static POLICIES: Lazy<DashMap<String, StoredPolicy>> = Lazy::new(DashMap::new);
static POLICY_ACCEPTANCES: Lazy<DashMap<String, StoredPolicyAcceptance>> = Lazy::new(DashMap::new);

fn generate_id(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4().to_string().replace("-", "")[..16].to_string())
}

fn consent_key(user_id: &str, consent_type: ConsentType) -> String {
    format!("{}:{:?}", user_id, consent_type)
}

/// Record user consent
pub async fn record_consent(
    user_id: String,
    consent_type: ConsentType,
    granted: bool,
    version: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    _metadata: Option<serde_json::Value>,
) -> RecordConsentOutput {
    let consent_id = generate_id("con");
    let now = Utc::now();

    let stored = StoredConsent {
        consent_id: consent_id.clone(),
        user_id: user_id.clone(),
        consent_type,
        granted,
        version: version.clone(),
        ip_address,
        user_agent,
        timestamp: now,
        revoked_at: None,
    };

    CONSENTS.insert(consent_key(&user_id, consent_type), stored);

    // Record in history
    let history_entry = StoredConsentHistory {
        consent_id: consent_id.clone(),
        user_id: user_id.clone(),
        consent_type,
        granted,
        version,
        action: if granted { "granted".to_string() } else { "denied".to_string() },
        timestamp: now,
    };

    CONSENT_HISTORY
        .entry(user_id.clone())
        .or_insert_with(Vec::new)
        .push(history_entry);

    RecordConsentOutput {
        consent_id,
        user_id,
        timestamp: now.timestamp(),
        success: true,
    }
}

/// Revoke user consent
pub async fn revoke_consent(
    user_id: String,
    consent_type: ConsentType,
    _reason: Option<String>,
) -> RevokeConsentOutput {
    let key = consent_key(&user_id, consent_type);
    let now = Utc::now();

    let consent_id = if let Some(mut entry) = CONSENTS.get_mut(&key) {
        entry.granted = false;
        entry.revoked_at = Some(now);
        
        // Record in history
        let history_entry = StoredConsentHistory {
            consent_id: entry.consent_id.clone(),
            user_id: user_id.clone(),
            consent_type,
            granted: false,
            version: entry.version.clone(),
            action: "revoked".to_string(),
            timestamp: now,
        };

        CONSENT_HISTORY
            .entry(user_id.clone())
            .or_insert_with(Vec::new)
            .push(history_entry);

        entry.consent_id.clone()
    } else {
        String::new()
    };

    RevokeConsentOutput {
        consent_id,
        user_id,
        revoked_at: now.timestamp(),
        success: !consent_id.is_empty(),
    }
}

/// Get consent status for a user
pub async fn get_consent_status(
    user_id: String,
    consent_type: Option<ConsentType>,
) -> GetConsentStatusOutput {
    let consents: Vec<PrivacyConsent> = CONSENTS
        .iter()
        .filter(|entry| {
            let key = entry.key();
            key.starts_with(&format!("{}:", user_id)) && {
                if let Some(ct) = consent_type {
                    entry.consent_type == ct
                } else {
                    true
                }
            }
        })
        .map(|entry| {
            let c = entry.value();
            PrivacyConsent {
                consent_id: c.consent_id.clone(),
                consent_type: c.consent_type.to_string(),
                granted: c.granted,
                version: c.version.clone(),
                timestamp: c.timestamp,
            }
        })
        .collect();

    GetConsentStatusOutput { user_id, consents }
}

/// Update an existing consent
pub async fn update_consent(
    consent_id: String,
    granted: bool,
    version: Option<String>,
) -> UpdateConsentOutput {
    let mut success = false;

    for mut entry in CONSENTS.iter_mut() {
        if entry.consent_id == consent_id {
            entry.granted = granted;
            if let Some(v) = version {
                entry.version = v;
            }
            success = true;
            break;
        }
    }

    UpdateConsentOutput { consent_id, success }
}

/// Export user data
pub async fn export_data(
    user_id: String,
    format: Option<ExportFormat>,
    include_sections: Option<Vec<String>>,
    exclude_sections: Option<Vec<String>>,
) -> ExportDataOutput {
    let export_id = generate_id("exp");
    let now = Utc::now();

    let stored = StoredExport {
        export_id: export_id.clone(),
        user_id: user_id.clone(),
        format: format.unwrap_or(ExportFormat::Json),
        status: ExportStatus::Pending,
        include_sections: include_sections.unwrap_or_default(),
        exclude_sections: exclude_sections.unwrap_or_default(),
        download_url: None,
        created_at: now,
        completed_at: None,
    };

    EXPORTS.insert(export_id.clone(), stored);

    ExportDataOutput {
        export_id,
        user_id,
        status: "pending".to_string(),
        download_url: None,
        expires_at: None,
    }
}

/// Get export status
pub async fn get_export_status(export_id: String) -> GetExportStatusOutput {
    if let Some(entry) = EXPORTS.get(&export_id) {
        // Simulate completion for demo purposes
        let status = if entry.created_at < Utc::now() - Duration::seconds(5) {
            ExportStatus::Completed
        } else {
            entry.status
        };

        let download_url = if status == ExportStatus::Completed {
            Some(format!("https://exports.example.com/{}", export_id))
        } else {
            None
        };

        GetExportStatusOutput {
            export_id,
            status,
            created_at: entry.created_at.timestamp(),
            completed_at: entry.completed_at.map(|t| t.timestamp()),
            download_url,
        }
    } else {
        GetExportStatusOutput {
            export_id,
            status: ExportStatus::Failed,
            created_at: 0,
            completed_at: None,
            download_url: None,
        }
    }
}

/// Delete user data
pub async fn delete_data(
    user_id: String,
    reason: String,
    delete_type: Option<DeleteType>,
    retain_sections: Option<Vec<String>>,
) -> DeleteDataOutput {
    let deletion_id = generate_id("del");
    let now = Utc::now();

    let stored = StoredDeletion {
        deletion_id: deletion_id.clone(),
        user_id: user_id.clone(),
        delete_type: delete_type.unwrap_or(DeleteType::Soft),
        reason,
        retain_sections: retain_sections.unwrap_or_default(),
        status: DeletionStatus::Pending,
        scheduled_at: now,
        completed_at: None,
    };

    DELETIONS.insert(deletion_id.clone(), stored);

    DeleteDataOutput {
        deletion_id,
        user_id,
        status: "pending".to_string(),
        scheduled_at: now.timestamp(),
        success: true,
    }
}

/// Get deletion status
pub async fn get_deletion_status(deletion_id: String) -> GetDeletionStatusOutput {
    if let Some(entry) = DELETIONS.get(&deletion_id) {
        GetDeletionStatusOutput {
            deletion_id,
            user_id: entry.user_id.clone(),
            status: entry.status,
            scheduled_at: entry.scheduled_at.timestamp(),
            completed_at: entry.completed_at.map(|t| t.timestamp()),
        }
    } else {
        GetDeletionStatusOutput {
            deletion_id,
            user_id: String::new(),
            status: DeletionStatus::Failed,
            scheduled_at: 0,
            completed_at: None,
        }
    }
}

/// Cancel a scheduled deletion
pub async fn cancel_deletion(deletion_id: String) -> CancelDeletionOutput {
    let mut cancelled = false;

    if let Some(mut entry) = DELETIONS.get_mut(&deletion_id) {
        if entry.status == DeletionStatus::Pending {
            entry.status = DeletionStatus::Failed; // Mark as failed/cancelled
            cancelled = true;
        }
    }

    CancelDeletionOutput {
        deletion_id,
        cancelled,
        success: cancelled,
    }
}

/// Anonymize user data
pub async fn anonymize_data(
    user_id: String,
    _retain_sections: Option<Vec<String>>,
) -> AnonymizeDataOutput {
    let anonymization_id = generate_id("anon");

    // In a real implementation, this would anonymize user data
    // For demo, we just return success

    AnonymizeDataOutput {
        anonymization_id,
        user_id,
        success: true,
    }
}

/// List consent history for a user
pub async fn list_consent_history(
    user_id: String,
    consent_type: Option<ConsentType>,
    start_date: Option<i64>,
    end_date: Option<i64>,
    limit: Option<i64>,
) -> ListConsentHistoryOutput {
    let limit = limit.unwrap_or(100) as usize;
    
    let history: Vec<PrivacyConsentHistory> = CONSENT_HISTORY
        .get(&user_id)
        .map(|entries| {
            entries
                .iter()
                .filter(|e| {
                    let type_match = consent_type.map(|ct| e.consent_type == ct).unwrap_or(true);
                    let start_match = start_date.map(|s| e.timestamp.timestamp() >= s).unwrap_or(true);
                    let end_match = end_date.map(|end| e.timestamp.timestamp() <= end).unwrap_or(true);
                    type_match && start_match && end_match
                })
                .take(limit)
                .map(|e| PrivacyConsentHistory {
                    consent_id: e.consent_id.clone(),
                    consent_type: e.consent_type.to_string(),
                    granted: e.granted,
                    version: e.version.clone(),
                    action: e.action.clone(),
                    timestamp: e.timestamp,
                })
                .collect()
        })
        .unwrap_or_default();

    let total = history.len() as i64;

    ListConsentHistoryOutput {
        user_id,
        history,
        total,
    }
}

/// Record data access event
pub async fn record_data_access(
    user_id: String,
    accessor_id: String,
    access_type: AccessType,
    resource: String,
    purpose: Option<String>,
) -> RecordDataAccessOutput {
    let access_id = generate_id("acc");
    let now = Utc::now();

    let stored = StoredDataAccess {
        access_id: access_id.clone(),
        user_id: user_id.clone(),
        accessor_id,
        access_type,
        resource,
        purpose,
        timestamp: now,
    };

    DATA_ACCESSES
        .entry(user_id)
        .or_insert_with(Vec::new)
        .push(stored);

    RecordDataAccessOutput {
        access_id,
        timestamp: now.timestamp(),
        success: true,
    }
}

/// Get data access log for a user
pub async fn get_access_log(
    user_id: String,
    start_date: Option<i64>,
    end_date: Option<i64>,
    limit: Option<i64>,
) -> GetAccessLogOutput {
    let limit = limit.unwrap_or(100) as usize;

    let accesses: Vec<PrivacyDataAccess> = DATA_ACCESSES
        .get(&user_id)
        .map(|entries| {
            entries
                .iter()
                .filter(|e| {
                    let start_match = start_date.map(|s| e.timestamp.timestamp() >= s).unwrap_or(true);
                    let end_match = end_date.map(|end| e.timestamp.timestamp() <= end).unwrap_or(true);
                    start_match && end_match
                })
                .take(limit)
                .map(|e| PrivacyDataAccess {
                    access_id: e.access_id.clone(),
                    accessor_id: e.accessor_id.clone(),
                    access_type: format!("{:?}", e.access_type),
                    resource: e.resource.clone(),
                    purpose: e.purpose.clone(),
                    timestamp: e.timestamp,
                })
                .collect()
        })
        .unwrap_or_default();

    let total = accesses.len() as i64;

    GetAccessLogOutput {
        user_id,
        accesses,
        total,
    }
}

/// Create a new policy version
pub async fn create_policy_version(
    title: String,
    content: String,
    version: String,
    effective_date: i64,
) -> CreatePolicyVersionOutput {
    let policy_id = generate_id("pol");
    let now = Utc::now();

    let effective = chrono::DateTime::from_timestamp(effective_date, 0)
        .unwrap_or(now);

    let stored = StoredPolicy {
        policy_id: policy_id.clone(),
        policy_type: PolicyType::Privacy,
        version: version.clone(),
        title,
        content,
        effective_date: effective,
        created_at: now,
        is_active: true,
    };

    POLICIES.insert(policy_id.clone(), stored);

    CreatePolicyVersionOutput {
        policy_id,
        version,
        success: true,
    }
}

/// Get the active policy
pub async fn get_active_policy(policy_type: Option<PolicyType>) -> GetActivePolicyOutput {
    let policy_type = policy_type.unwrap_or(PolicyType::Privacy);
    
    // Find the most recent active policy of the given type
    let policy = POLICIES
        .iter()
        .filter(|entry| entry.policy_type == policy_type && entry.is_active)
        .max_by_key(|entry| entry.effective_date);

    if let Some(p) = policy {
        GetActivePolicyOutput {
            policy_id: p.policy_id.clone(),
            version: p.version.clone(),
            effective_date: p.effective_date.timestamp(),
            content: p.content.clone(),
        }
    } else {
        GetActivePolicyOutput {
            policy_id: String::new(),
            version: String::new(),
            effective_date: 0,
            content: String::new(),
        }
    }
}

/// Record policy acceptance
pub async fn record_policy_acceptance(
    user_id: String,
    policy_id: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> RecordPolicyAcceptanceOutput {
    let acceptance_id = generate_id("acc");
    let now = Utc::now();

    let stored = StoredPolicyAcceptance {
        acceptance_id: acceptance_id.clone(),
        user_id: user_id.clone(),
        policy_id,
        ip_address,
        user_agent,
        timestamp: now,
    };

    POLICY_ACCEPTANCES.insert(acceptance_id.clone(), stored);

    RecordPolicyAcceptanceOutput {
        acceptance_id,
        user_id,
        timestamp: now.timestamp(),
        success: true,
    }
}

/// Verify eligibility for Right to Be Forgotten
pub async fn verify_rtbf_eligibility(user_id: String) -> VerifyRtbfEligibilityOutput {
    // In a real implementation, this would check various factors
    // For demo, we assume eligibility with some sample reasons
    
    let mut blocking_factors = Vec::new();
    let mut reasons = Vec::new();

    // Check for pending transactions, legal holds, etc.
    // This is a mock implementation
    
    // Example: Check if there are active subscriptions
    let has_active_subscription = false; // Would check actual data
    
    if has_active_subscription {
        blocking_factors.push("Active subscription".to_string());
    }

    let eligible = blocking_factors.is_empty();

    if eligible {
        reasons.push("No active contracts".to_string());
        reasons.push("No legal holds".to_string());
        reasons.push("Data retention period expired".to_string());
    }

    VerifyRtbfEligibilityOutput {
        user_id,
        eligible,
        reasons,
        blocking_factors,
    }
}

// Utility functions for testing
#[cfg(test)]
pub fn clear_all_data() {
    CONSENTS.clear();
    CONSENT_HISTORY.clear();
    EXPORTS.clear();
    DELETIONS.clear();
    DATA_ACCESSES.clear();
    POLICIES.clear();
    POLICY_ACCEPTANCES.clear();
}

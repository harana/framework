// Harana Actions - Approval Module
// This module provides approval workflow actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use output::*;

// In-memory storage for approval requests
static REQUESTS: Lazy<DashMap<String, ApprovalRequest>> = Lazy::new(DashMap::new);
static HISTORY: Lazy<DashMap<String, Vec<ApprovalHistoryEntry>>> = Lazy::new(DashMap::new);

#[derive(Debug, Clone)]
struct ApprovalRequest {
    request_id: String,
    title: String,
    description: Option<String>,
    request_type: String,
    resource_id: String,
    resource_type: String,
    requester_id: String,
    approvers: Vec<ApproverState>,
    status: String, // pending, approved, rejected, cancelled
    priority: String,
    due_date: Option<DateTime<Utc>>,
    metadata: Option<HashMap<String, Value>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct ApproverState {
    approver_id: String,
    status: String, // pending, approved, rejected
    required: bool,
    responded_at: Option<DateTime<Utc>>,
    comment: Option<String>,
}

fn add_history_entry(request_id: &str, action: &str, actor_id: &str, comment: Option<String>) {
    let entry = ApprovalHistoryEntry {
        action: action.to_string(),
        actor_id: actor_id.to_string(),
        timestamp: Utc::now(),
        comment,
    };
    HISTORY.entry(request_id.to_string())
        .or_insert_with(Vec::new)
        .push(entry);
}

fn check_approval_status(request: &ApprovalRequest) -> String {
    let required_approvers: Vec<_> = request.approvers.iter().filter(|a| a.required).collect();
    let all_approvers = &request.approvers;

    // Check if any required approver rejected
    if required_approvers.iter().any(|a| a.status == "rejected") {
        return "rejected".to_string();
    }

    // Check if any approver rejected (for non-required)
    if all_approvers.iter().any(|a| a.status == "rejected") {
        return "rejected".to_string();
    }

    // Check if all required approvers approved
    let required_approved = required_approvers.iter().all(|a| a.status == "approved");
    
    // If there are required approvers, they must all approve
    if !required_approvers.is_empty() {
        if required_approved {
            return "approved".to_string();
        }
        return "pending".to_string();
    }

    // If no required approvers, at least one must approve
    if all_approvers.iter().any(|a| a.status == "approved") {
        return "approved".to_string();
    }

    "pending".to_string()
}

/// Create Approval Request
pub async fn create_request(
    approvers: Vec<String>,
    request_type: &str,
    resource_id: &str,
    resource_type: &str,
    title: &str,
    description: Option<&str>,
    due_date: Option<DateTime<Utc>>,
    metadata: Option<HashMap<String, Value>>,
    priority: Option<&str>,
) -> Result<CreateRequestOutput, String> {
    if approvers.is_empty() {
        return Err("At least one approver is required".to_string());
    }

    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }

    let request_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let approver_states: Vec<ApproverState> = approvers.iter().enumerate().map(|(i, id)| {
        ApproverState {
            approver_id: id.clone(),
            status: "pending".to_string(),
            required: i == 0, // First approver is required by default
            responded_at: None,
            comment: None,
        }
    }).collect();

    let request = ApprovalRequest {
        request_id: request_id.clone(),
        title: title.to_string(),
        description: description.map(|s| s.to_string()),
        request_type: request_type.to_string(),
        resource_id: resource_id.to_string(),
        resource_type: resource_type.to_string(),
        requester_id: "system".to_string(), // Would come from auth context in production
        approvers: approver_states,
        status: "pending".to_string(),
        priority: priority.unwrap_or("normal").to_string(),
        due_date,
        metadata,
        created_at: now,
        updated_at: now,
    };

    REQUESTS.insert(request_id.clone(), request);
    add_history_entry(&request_id, "created", "system", None);

    Ok(CreateRequestOutput {
        request_id,
        status: "pending".to_string(),
        success: true,
    })
}

/// Get Approval Request
pub async fn get_request(
    request_id: &str,
) -> Result<GetRequestOutput, String> {
    let request = REQUESTS.get(request_id)
        .ok_or_else(|| format!("Request not found: {}", request_id))?;

    let approvers: Vec<ApproverInfo> = request.approvers.iter().map(|a| {
        ApproverInfo {
            approver_id: a.approver_id.clone(),
            status: a.status.clone(),
            required: a.required,
            responded_at: a.responded_at,
        }
    }).collect();

    Ok(GetRequestOutput {
        approvers,
        created_at: request.created_at,
        description: request.description.clone(),
        due_date: request.due_date,
        request_id: request.request_id.clone(),
        requester_id: request.requester_id.clone(),
        resource_id: request.resource_id.clone(),
        resource_type: request.resource_type.clone(),
        status: request.status.clone(),
        title: request.title.clone(),
        updated_at: request.updated_at,
    })
}

/// List Approval Requests
pub async fn list_requests(
    approver_id: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
    _requester_id: Option<&str>,
    resource_type: Option<&str>,
    status: Option<&str>,
) -> Result<ListRequestsOutput, String> {
    let limit = limit.unwrap_or(50).min(100).max(1) as usize;
    let offset = offset.unwrap_or(0).max(0) as usize;

    let mut filtered: Vec<ApprovalRequestInfo> = REQUESTS.iter()
        .filter(|entry| {
            let request = entry.value();

            // Filter by approver
            if let Some(aid) = approver_id {
                if !request.approvers.iter().any(|a| a.approver_id == aid) {
                    return false;
                }
            }

            // Filter by resource_type
            if let Some(rt) = resource_type {
                if request.resource_type != rt {
                    return false;
                }
            }

            // Filter by status
            if let Some(s) = status {
                if request.status != s {
                    return false;
                }
            }

            true
        })
        .map(|entry| {
            let request = entry.value();
            ApprovalRequestInfo {
                request_id: request.request_id.clone(),
                title: request.title.clone(),
                status: request.status.clone(),
                resource_type: request.resource_type.clone(),
                created_at: request.created_at,
            }
        })
        .collect();

    let total = filtered.len() as i32;

    // Apply pagination
    filtered = filtered.into_iter().skip(offset).take(limit).collect();

    Ok(ListRequestsOutput {
        requests: filtered,
        total,
    })
}

/// Approve Request
pub async fn approve(
    approver_id: &str,
    request_id: &str,
    comment: Option<&str>,
) -> Result<ApproveOutput, String> {
    let mut request = REQUESTS.get_mut(request_id)
        .ok_or_else(|| format!("Request not found: {}", request_id))?;

    if request.status != "pending" {
        return Err(format!("Request is already {}", request.status));
    }

    // Find the approver
    let approver = request.approvers.iter_mut()
        .find(|a| a.approver_id == approver_id)
        .ok_or_else(|| format!("Approver {} is not assigned to this request", approver_id))?;

    if approver.status != "pending" {
        return Err(format!("Approver has already {}", approver.status));
    }

    let now = Utc::now();
    approver.status = "approved".to_string();
    approver.responded_at = Some(now);
    approver.comment = comment.map(|s| s.to_string());

    // Check if request is now fully approved
    let new_status = check_approval_status(&request);
    request.status = new_status.clone();
    request.updated_at = now;

    drop(request); // Release the mutable borrow

    add_history_entry(request_id, "approved", approver_id, comment.map(|s| s.to_string()));

    Ok(ApproveOutput {
        approved_at: now,
        request_id: request_id.to_string(),
        status: new_status,
        success: true,
    })
}

/// Reject Request
pub async fn reject(
    approver_id: &str,
    reason: &str,
    request_id: &str,
) -> Result<RejectOutput, String> {
    let mut request = REQUESTS.get_mut(request_id)
        .ok_or_else(|| format!("Request not found: {}", request_id))?;

    if request.status != "pending" {
        return Err(format!("Request is already {}", request.status));
    }

    // Find the approver
    let approver = request.approvers.iter_mut()
        .find(|a| a.approver_id == approver_id)
        .ok_or_else(|| format!("Approver {} is not assigned to this request", approver_id))?;

    if approver.status != "pending" {
        return Err(format!("Approver has already {}", approver.status));
    }

    let now = Utc::now();
    approver.status = "rejected".to_string();
    approver.responded_at = Some(now);
    approver.comment = Some(reason.to_string());

    // Check if request is now rejected
    let new_status = check_approval_status(&request);
    request.status = new_status.clone();
    request.updated_at = now;

    drop(request);

    add_history_entry(request_id, "rejected", approver_id, Some(reason.to_string()));

    Ok(RejectOutput {
        rejected_at: now,
        request_id: request_id.to_string(),
        status: new_status,
        success: true,
    })
}

/// Cancel Approval Request
pub async fn cancel(
    request_id: &str,
    reason: Option<&str>,
) -> Result<CancelOutput, String> {
    let mut request = REQUESTS.get_mut(request_id)
        .ok_or_else(|| format!("Request not found: {}", request_id))?;

    if request.status != "pending" {
        return Err(format!("Request is already {} and cannot be cancelled", request.status));
    }

    let now = Utc::now();
    request.status = "cancelled".to_string();
    request.updated_at = now;

    drop(request);

    add_history_entry(request_id, "cancelled", "system", reason.map(|s| s.to_string()));

    Ok(CancelOutput {
        cancelled_at: now,
        request_id: request_id.to_string(),
        success: true,
    })
}

/// Add Approver To Request
pub async fn add_approver(
    approver_id: &str,
    request_id: &str,
    required: Option<bool>,
) -> Result<AddApproverOutput, String> {
    let mut request = REQUESTS.get_mut(request_id)
        .ok_or_else(|| format!("Request not found: {}", request_id))?;

    if request.status != "pending" {
        return Err("Cannot add approver to a non-pending request".to_string());
    }

    // Check if approver already exists
    if request.approvers.iter().any(|a| a.approver_id == approver_id) {
        return Err(format!("Approver {} is already assigned to this request", approver_id));
    }

    let new_approver = ApproverState {
        approver_id: approver_id.to_string(),
        status: "pending".to_string(),
        required: required.unwrap_or(false),
        responded_at: None,
        comment: None,
    };

    request.approvers.push(new_approver);
    request.updated_at = Utc::now();

    drop(request);

    add_history_entry(request_id, "approver_added", "system", Some(format!("Added approver: {}", approver_id)));

    Ok(AddApproverOutput { success: true })
}

/// Remove Approver From Request
pub async fn remove_approver(
    approver_id: &str,
    request_id: &str,
) -> Result<RemoveApproverOutput, String> {
    let mut request = REQUESTS.get_mut(request_id)
        .ok_or_else(|| format!("Request not found: {}", request_id))?;

    if request.status != "pending" {
        return Err("Cannot remove approver from a non-pending request".to_string());
    }

    let initial_count = request.approvers.len();
    request.approvers.retain(|a| a.approver_id != approver_id);

    if request.approvers.len() == initial_count {
        return Err(format!("Approver {} is not assigned to this request", approver_id));
    }

    if request.approvers.is_empty() {
        return Err("Cannot remove the last approver".to_string());
    }

    request.updated_at = Utc::now();

    drop(request);

    add_history_entry(request_id, "approver_removed", "system", Some(format!("Removed approver: {}", approver_id)));

    Ok(RemoveApproverOutput { success: true })
}

/// Get Approval History
pub async fn get_history(
    request_id: &str,
) -> Result<GetHistoryOutput, String> {
    // Check request exists
    if !REQUESTS.contains_key(request_id) {
        return Err(format!("Request not found: {}", request_id));
    }

    let history = HISTORY.get(request_id)
        .map(|h| h.clone())
        .unwrap_or_default();

    let total = history.len() as i32;

    Ok(GetHistoryOutput { history, total })
}

#[cfg(test)]
mod tests;

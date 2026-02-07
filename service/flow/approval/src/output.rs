// Harana Actions - Approval Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// create_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestOutput {
    pub request_id: String,
    pub status: String,
    pub success: bool,
}

// get_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestOutput {
    pub approvers: Vec<ApproverInfo>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub request_id: String,
    pub requester_id: String,
    pub resource_id: String,
    pub resource_type: String,
    pub status: String,
    pub title: String,
    pub updated_at: DateTime<Utc>,
}

// list_requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestsOutput {
    pub requests: Vec<ApprovalRequestInfo>,
    pub total: i32,
}

// approve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveOutput {
    pub approved_at: DateTime<Utc>,
    pub request_id: String,
    pub status: String,
    pub success: bool,
}

// reject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectOutput {
    pub rejected_at: DateTime<Utc>,
    pub request_id: String,
    pub status: String,
    pub success: bool,
}

// cancel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOutput {
    pub cancelled_at: DateTime<Utc>,
    pub request_id: String,
    pub success: bool,
}

// add_approver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddApproverOutput {
    pub success: bool,
}

// remove_approver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveApproverOutput {
    pub success: bool,
}

// get_history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHistoryOutput {
    pub history: Vec<ApprovalHistoryEntry>,
    pub total: i32,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproverInfo {
    pub approver_id: String,
    pub status: String,
    pub required: bool,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequestInfo {
    pub request_id: String,
    pub title: String,
    pub status: String,
    pub resource_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalHistoryEntry {
    pub action: String,
    pub actor_id: String,
    pub timestamp: DateTime<Utc>,
    pub comment: Option<String>,
}

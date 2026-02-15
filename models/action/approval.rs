// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateApprovalRequestInput {
    pub approvers: String,
    pub description: String,
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub priority: String,
    pub request_type: String,
    pub resource_id: String,
    pub resource_type: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateApprovalRequestOutput {
    pub request_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetApprovalRequestInput {
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetApprovalRequestOutput {
    pub approvers: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub requester_id: String,
    pub resource_id: String,
    pub resource_type: String,
    pub status: String,
    pub title: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListApprovalRequestsInput {
    pub approver_id: String,
    pub limit: i64,
    pub offset: i64,
    pub requester_id: String,
    pub resource_type: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListApprovalRequestsOutput {
    pub requests: String,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApproveRequestInput {
    pub approver_id: String,
    pub comment: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApproveRequestOutput {
    pub approved_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RejectRequestInput {
    pub approver_id: String,
    pub reason: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RejectRequestOutput {
    pub rejected_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelApprovalRequestInput {
    pub reason: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelApprovalRequestOutput {
    pub cancelled_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddApproverToRequestInput {
    pub approver_id: String,
    pub request_id: String,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddApproverToRequestOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveApproverFromRequestInput {
    pub approver_id: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveApproverFromRequestOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetApprovalHistoryInput {
    pub limit: i64,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetApprovalHistoryOutput {
    pub events: String,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EscalateApprovalRequestInput {
    pub escalate_to: String,
    pub reason: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EscalateApprovalRequestOutput {
    pub escalated_at: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DelegateApprovalInput {
    pub approver_id: String,
    pub delegate_to: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DelegateApprovalOutput {
    pub delegated_at: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalRequest {
    pub request_id: String,
    pub title: String,
    pub description: String,
    pub requester_id: String,
    pub approvers: String,
    pub status: String,
    pub resource_type: String,
    pub resource_id: String,
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApproverInfo {
    pub approver_id: String,
    pub status: String,
    pub responded_at: chrono::DateTime<chrono::Utc>,
    pub comment: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalRequestInfo {
    pub request_id: String,
    pub title: String,
    pub status: String,
    pub requester_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub due_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalEvent {
    pub event_id: String,
    pub event_type: String,
    pub actor_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub details: String,
}

#[async_trait]
pub trait ApprovalAction {
    async fn create _approval _request(&self, input: CreateApprovalRequestInput) -> Result<CreateApprovalRequestOutput, Box<dyn std::error::Error>>;
    async fn get _approval _request(&self, input: GetApprovalRequestInput) -> Result<GetApprovalRequestOutput, Box<dyn std::error::Error>>;
    async fn list _approval _requests(&self, input: ListApprovalRequestsInput) -> Result<ListApprovalRequestsOutput, Box<dyn std::error::Error>>;
    async fn approve _request(&self, input: ApproveRequestInput) -> Result<ApproveRequestOutput, Box<dyn std::error::Error>>;
    async fn reject _request(&self, input: RejectRequestInput) -> Result<RejectRequestOutput, Box<dyn std::error::Error>>;
    async fn cancel _approval _request(&self, input: CancelApprovalRequestInput) -> Result<CancelApprovalRequestOutput, Box<dyn std::error::Error>>;
    async fn add _approver _to _request(&self, input: AddApproverToRequestInput) -> Result<AddApproverToRequestOutput, Box<dyn std::error::Error>>;
    async fn remove _approver _from _request(&self, input: RemoveApproverFromRequestInput) -> Result<RemoveApproverFromRequestOutput, Box<dyn std::error::Error>>;
    async fn get _approval _history(&self, input: GetApprovalHistoryInput) -> Result<GetApprovalHistoryOutput, Box<dyn std::error::Error>>;
    async fn escalate _approval _request(&self, input: EscalateApprovalRequestInput) -> Result<EscalateApprovalRequestOutput, Box<dyn std::error::Error>>;
    async fn delegate _approval(&self, input: DelegateApprovalInput) -> Result<DelegateApprovalOutput, Box<dyn std::error::Error>>;
}

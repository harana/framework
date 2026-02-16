// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRequestOutput {
    pub request_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRequestOutput {
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
pub struct ListRequestsOutput {
    pub requests: String,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApproveOutput {
    pub approved_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RejectOutput {
    pub rejected_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOutput {
    pub cancelled_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHistoryOutput {
    pub events: String,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalRequest {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub priority: String,
    pub request_type: String,
    pub requester_id: String,
    pub resource_id: String,
    pub resource_type: String,
    pub status: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalApprover {
    pub approver_id: String,
    pub comment: String,
    pub decided_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_required: bool,
    pub reason: String,
    pub request_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalHistory {
    pub action: String,
    pub actor_id: String,
    pub comment: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
}

#[async_trait]
pub trait ApprovalAction {
    async fn create_request(&self, approvers: String, description: String, due_date: chrono::DateTime<chrono::Utc>, metadata: String, priority: String, request_type: String, resource_id: String, resource_type: String, title: String) -> Result<CreateRequestOutput, Box<dyn std::error::Error>>;
    async fn get_request(&self, request_id: String) -> Result<GetRequestOutput, Box<dyn std::error::Error>>;
    async fn list_requests(&self, approver_id: String, limit: i64, offset: i64, requester_id: String, resource_type: String, status: String) -> Result<ListRequestsOutput, Box<dyn std::error::Error>>;
    async fn approve(&self, approver_id: String, comment: String, request_id: String) -> Result<ApproveOutput, Box<dyn std::error::Error>>;
    async fn reject(&self, approver_id: String, reason: String, request_id: String) -> Result<RejectOutput, Box<dyn std::error::Error>>;
    async fn cancel(&self, reason: String, request_id: String) -> Result<CancelOutput, Box<dyn std::error::Error>>;
    async fn add_approver(&self, approver_id: String, request_id: String, required: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_approver(&self, approver_id: String, request_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_history(&self, limit: i64, request_id: String) -> Result<GetHistoryOutput, Box<dyn std::error::Error>>;
    async fn escalate(&self, escalate_to: String, reason: String, request_id: String) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>>;
    async fn delegate(&self, approver_id: String, delegate_to: String, request_id: String) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>>;
}

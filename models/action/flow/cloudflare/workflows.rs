// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusOutput {
    pub error: String,
    pub output: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepWaitForEventOutput {
    pub payload: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflow {
    pub account_id: String,
    pub binding: String,
    pub class_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub script_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workflow_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflowInstance {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error: String,
    pub instance_id: String,
    pub output: String,
    pub params: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workflow_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflowStep {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    pub result: String,
    pub retries_backoff: String,
    pub retries_delay: String,
    pub retries_limit: i64,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub step_name: String,
    pub step_type: String,
    pub timeout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflowEvent {
    pub event_type: String,
    pub instance_id: String,
    pub payload: String,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait WorkflowsAction {
    async fn create(&self, binding: String, id: String, params: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_batch(&self, binding: String, instances: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn get(&self, binding: String, id: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn status(&self, binding: String, id: String) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn pause(&self, binding: String, id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn resume(&self, binding: String, id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn restart(&self, binding: String, id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn terminate(&self, binding: String, id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_event(&self, binding: String, id: String, payload: String, type: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn step_do(&self, callback: String, name: String, retries_backoff: String, retries_delay: String, retries_limit: i64, timeout: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn step_sleep(&self, duration: String, name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn step_sleep_until(&self, name: String, timestamp: chrono::DateTime<chrono::Utc>) -> Result<(), Box<dyn std::error::Error>>;
    async fn step_wait_for_event(&self, name: String, timeout: String, type: String) -> Result<StepWaitForEventOutput, Box<dyn std::error::Error>>;
}

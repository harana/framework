// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub actions: Vec<String>,
    pub conditions: String,
    pub description: String,
    pub effect: String,
    pub name: String,
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPoliciesOutput {
    pub policies: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluateOutput {
    pub allowed: bool,
    pub evaluated_conditions: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Policy {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub effect: String,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
    pub conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyConditions {
    pub conditions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyInfo {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyEvaluationContext {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyEvaluatedConditions {
    pub results: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyAttachment {
    #[serde(default = "chrono::Utc::now")]
    pub attached_at: chrono::DateTime<chrono::Utc>,
    pub attached_by: String,
    pub entity_id: String,
    pub entity_type: String,
    pub policy_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyEvaluationLog {
    pub action: String,
    #[serde(default)]
    pub allowed: bool,
    #[serde(default = "chrono::Utc::now")]
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
    pub policy_id: String,
    pub reason: String,
    pub resource: String,
    pub user_id: String,
}

#[async_trait]
pub trait PolicyAction {
    async fn create(&self, conditions: String, description: String, effect: String, name: String, resources: Vec<String>, actions: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn update(&self, conditions: String, description: String, effect: String, name: String, policy_id: String, resources: Vec<String>, actions: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, policy_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get(&self, policy_id: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn list_policies(&self, limit: i64, offset: i64) -> Result<ListPoliciesOutput, Box<dyn std::error::Error>>;
    async fn attach_to_user(&self, policy_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_from_user(&self, policy_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn attach_to_role(&self, policy_id: String, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_from_role(&self, policy_id: String, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn evaluate(&self, method: String, context: String, policy_id: String, resource: String) -> Result<EvaluateOutput, Box<dyn std::error::Error>>;
}

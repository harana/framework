// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInput {
    pub conditions: String,
    pub description: String,
    pub effect: String,
    pub name: String,
    pub resources: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOutput {
    pub policy_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateInput {
    pub conditions: String,
    pub description: String,
    pub effect: String,
    pub name: String,
    pub policy_id: String,
    pub resources: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub policy_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub policy_id: String,
}

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
pub struct ListPoliciesInput {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPoliciesOutput {
    pub policies: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachToUserInput {
    pub policy_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachToUserOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachFromUserInput {
    pub policy_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachFromUserOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachToRoleInput {
    pub policy_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachToRoleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachFromRoleInput {
    pub policy_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachFromRoleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluateInput {
    pub method: String,
    pub context: String,
    pub policy_id: String,
    pub resource: String,
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

#[async_trait]
pub trait PolicyAction {
    async fn create(&self, input: CreateInput) -> Result<CreateOutput, Box<dyn std::error::Error>>;
    async fn update(&self, input: UpdateInput) -> Result<UpdateOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn list_policies(&self, input: ListPoliciesInput) -> Result<ListPoliciesOutput, Box<dyn std::error::Error>>;
    async fn attach_to_user(&self, input: AttachToUserInput) -> Result<AttachToUserOutput, Box<dyn std::error::Error>>;
    async fn detach_from_user(&self, input: DetachFromUserInput) -> Result<DetachFromUserOutput, Box<dyn std::error::Error>>;
    async fn attach_to_role(&self, input: AttachToRoleInput) -> Result<AttachToRoleOutput, Box<dyn std::error::Error>>;
    async fn detach_from_role(&self, input: DetachFromRoleInput) -> Result<DetachFromRoleOutput, Box<dyn std::error::Error>>;
    async fn evaluate(&self, input: EvaluateInput) -> Result<EvaluateOutput, Box<dyn std::error::Error>>;
}

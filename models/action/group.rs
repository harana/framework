// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInput {
    pub description: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOutput {
    pub group_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateInput {
    pub description: String,
    pub group_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub description: String,
    pub member_count: i64,
    pub name: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsInput {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsOutput {
    pub groups: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddUserToInput {
    pub group_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddUserToOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveUserFromInput {
    pub group_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveUserFromOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMembersInput {
    pub group_id: String,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMembersOutput {
    pub members: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssignRoleToInput {
    pub group_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssignRoleToOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveRoleFromInput {
    pub group_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveRoleFromOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRolesInput {
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRolesOutput {
    pub roles: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckPermissionInput {
    pub method: String,
    pub group_id: String,
    pub resource: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckPermissionOutput {
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    pub group_id: String,
    pub name: String,
    pub description: String,
    pub member_count: i64,
    pub roles: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupRole {
    pub role_id: String,
    pub name: String,
    pub permissions: Vec<String>,
}

#[async_trait]
pub trait GroupAction {
    async fn create(&self, input: CreateInput) -> Result<CreateOutput, Box<dyn std::error::Error>>;
    async fn update(&self, input: UpdateInput) -> Result<UpdateOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn lists(&self, input: ListsInput) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn add_user_to(&self, input: AddUserToInput) -> Result<AddUserToOutput, Box<dyn std::error::Error>>;
    async fn remove_user_from(&self, input: RemoveUserFromInput) -> Result<RemoveUserFromOutput, Box<dyn std::error::Error>>;
    async fn list_members(&self, input: ListMembersInput) -> Result<ListMembersOutput, Box<dyn std::error::Error>>;
    async fn assign_role_to(&self, input: AssignRoleToInput) -> Result<AssignRoleToOutput, Box<dyn std::error::Error>>;
    async fn remove_role_from(&self, input: RemoveRoleFromInput) -> Result<RemoveRoleFromOutput, Box<dyn std::error::Error>>;
    async fn list_roles(&self, input: ListRolesInput) -> Result<ListRolesOutput, Box<dyn std::error::Error>>;
    async fn check_permission(&self, input: CheckPermissionInput) -> Result<CheckPermissionOutput, Box<dyn std::error::Error>>;
}

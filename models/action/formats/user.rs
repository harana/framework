// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckPermissionInput {
    pub method: String,
    pub resource: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckPermissionOutput {
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GrantPermissionInput {
    pub method: String,
    pub conditions: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub resource: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GrantPermissionOutput {
    pub permission_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokePermissionInput {
    pub permission_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokePermissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPermissionsInput {
    pub resource: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPermissionsOutput {
    pub permissions: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssignRoleInput {
    pub role_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssignRoleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveRoleInput {
    pub role_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveRoleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRolesInput {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRolesOutput {
    pub roles: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEffectivePermissionsInput {
    pub resource: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEffectivePermissionsOutput {
    pub permissions: Vec<String>,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermission {
    pub permission_id: String,
    pub user_id: String,
    pub method: String,
    pub resource: String,
    pub conditions: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermissionConditions {
    pub conditions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermissionInfo {
    pub permission_id: String,
    pub method: String,
    pub resource: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserRoleInfo {
    pub role_id: String,
    pub name: String,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EffectivePermission {
    pub method: String,
    pub resource: String,
    pub allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PermissionSource {
    pub source_type: String,
    pub source_id: String,
    pub name: String,
}

#[async_trait]
pub trait UserAction {
    async fn check_permission(&self, input: CheckPermissionInput) -> Result<CheckPermissionOutput, Box<dyn std::error::Error>>;
    async fn grant_permission(&self, input: GrantPermissionInput) -> Result<GrantPermissionOutput, Box<dyn std::error::Error>>;
    async fn revoke_permission(&self, input: RevokePermissionInput) -> Result<RevokePermissionOutput, Box<dyn std::error::Error>>;
    async fn list_permissions(&self, input: ListPermissionsInput) -> Result<ListPermissionsOutput, Box<dyn std::error::Error>>;
    async fn assign_role(&self, input: AssignRoleInput) -> Result<AssignRoleOutput, Box<dyn std::error::Error>>;
    async fn remove_role(&self, input: RemoveRoleInput) -> Result<RemoveRoleOutput, Box<dyn std::error::Error>>;
    async fn list_roles(&self, input: ListRolesInput) -> Result<ListRolesOutput, Box<dyn std::error::Error>>;
    async fn get_effective_permissions(&self, input: GetEffectivePermissionsInput) -> Result<GetEffectivePermissionsOutput, Box<dyn std::error::Error>>;
}

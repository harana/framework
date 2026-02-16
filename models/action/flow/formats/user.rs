// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckPermissionOutput {
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPermissionsOutput {
    pub permissions: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRolesOutput {
    pub roles: Vec<String>,
    pub total: i64,
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
    async fn check_permission(&self, method: String, resource: String, user_id: String) -> Result<CheckPermissionOutput, Box<dyn std::error::Error>>;
    async fn grant_permission(&self, method: String, conditions: String, expires_at: chrono::DateTime<chrono::Utc>, resource: String, user_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn revoke_permission(&self, permission_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_permissions(&self, resource: String, user_id: String) -> Result<ListPermissionsOutput, Box<dyn std::error::Error>>;
    async fn assign_role(&self, role_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_role(&self, role_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_roles(&self, user_id: String) -> Result<ListRolesOutput, Box<dyn std::error::Error>>;
    async fn get_effective_permissions(&self, resource: String, user_id: String) -> Result<GetEffectivePermissionsOutput, Box<dyn std::error::Error>>;
}

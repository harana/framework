// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
pub struct ListsOutput {
    pub groups: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMembersOutput {
    pub members: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupMember {
    #[serde(default = "chrono::Utc::now")]
    pub added_at: chrono::DateTime<chrono::Utc>,
    pub added_by: String,
    pub group_id: String,
    #[serde(default)]
    pub is_admin: bool,
    pub user_id: String,
}

#[async_trait]
pub trait GroupAction {
    async fn create(&self, description: String, name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn update(&self, description: String, group_id: String, name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, group_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get(&self, group_id: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn lists(&self, limit: i64, offset: i64) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn add_user_to(&self, group_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_user_from(&self, group_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_members(&self, group_id: String, limit: i64, offset: i64) -> Result<ListMembersOutput, Box<dyn std::error::Error>>;
    async fn assign_role_to(&self, group_id: String, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_role_from(&self, group_id: String, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_roles(&self, group_id: String) -> Result<ListRolesOutput, Box<dyn std::error::Error>>;
    async fn check_permission(&self, method: String, group_id: String, resource: String) -> Result<CheckPermissionOutput, Box<dyn std::error::Error>>;
}

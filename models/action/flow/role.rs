// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListOutput {
    pub roles: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub description: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub user_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Role {
    pub role_id: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub user_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolePermission {
    pub method: String,
    pub resource: String,
    pub conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RoleInfo {
    pub role_id: String,
    pub name: String,
    pub description: String,
    pub user_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolePermissionConditions {
    pub conditions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RoleAssignment {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: String,
    pub entity_id: String,
    pub entity_type: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub role_id: String,
}

#[async_trait]
pub trait RoleAction {
    async fn create(&self, description: String, name: String, permissions: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn update(&self, description: String, name: String, permissions: Vec<String>, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list(&self, limit: i64, offset: i64) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn get(&self, role_id: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn check_permission(&self, method: String, resource: String, role_id: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn add_permission(&self, method: String, conditions: String, resource: String, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_permission(&self, method: String, resource: String, role_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn clone(&self, new_name: String, source_id: String) -> Result<String, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub password_last_used: chrono::DateTime<chrono::Utc>,
    pub path: String,
    pub permissions_boundary: String,
    pub tags: Vec<String>,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub group_id: String,
    pub group_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListGroupsOutput {
    pub groups: Vec<String>,
    pub is_truncated: bool,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListGroupsForUserOutput {
    pub groups: Vec<String>,
    pub is_truncated: bool,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRoleOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub role_id: String,
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRoleOutput {
    pub arn: String,
    pub assume_role_policy_document: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub max_session_duration: i64,
    pub path: String,
    pub permissions_boundary: String,
    pub role_id: String,
    pub role_name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRolesOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePolicyOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub policy_id: String,
    pub policy_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPolicyOutput {
    pub arn: String,
    pub attachment_count: i64,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub default_version_id: String,
    pub description: String,
    pub is_attachable: bool,
    pub path: String,
    pub permissions_boundary_usage_count: i64,
    pub policy_id: String,
    pub policy_name: String,
    pub tags: Vec<String>,
    pub update_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPoliciesOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub policies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAccessKeyOutput {
    pub access_key_id: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub secret_access_key: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListAccessKeysOutput {
    pub access_key_metadata: Vec<String>,
    pub is_truncated: bool,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCredentialReportOutput {
    pub content: String,
    pub generated_time: chrono::DateTime<chrono::Utc>,
    pub report_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamUser {
    pub account_id: String,
    pub arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub password_last_used: chrono::DateTime<chrono::Utc>,
    pub path: String,
    pub permissions_boundary_arn: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamGroup {
    pub account_id: String,
    pub arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub group_id: String,
    pub group_name: String,
    pub path: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamGroupMembership {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub group_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamRole {
    pub account_id: String,
    pub arn: String,
    pub assume_role_policy_document: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub max_session_duration: i64,
    pub path: String,
    pub permissions_boundary_arn: String,
    pub role_id: String,
    pub role_name: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamPolicy {
    pub account_id: String,
    pub arn: String,
    pub attachment_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_version_id: String,
    pub description: String,
    #[serde(default)]
    pub is_attachable: bool,
    pub path: String,
    pub policy_arn: String,
    pub policy_id: String,
    pub policy_name: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamPolicyAttachment {
    #[serde(default = "chrono::Utc::now")]
    pub attached_at: chrono::DateTime<chrono::Utc>,
    pub entity_name: String,
    pub entity_type: String,
    pub policy_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamAccessKey {
    pub access_key_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub last_used_region: String,
    pub last_used_service: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[async_trait]
pub trait IamAction {
    async fn create_user(&self, path: String, permissions_boundary: String, tags: String, user_name: String) -> Result<CreateUserOutput, Box<dyn std::error::Error>>;
    async fn delete_user(&self, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_user(&self, user_name: String) -> Result<GetUserOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, max_items: i64, path_prefix: String) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
    async fn update_user(&self, new_path: String, new_user_name: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_group(&self, group_name: String, path: String) -> Result<CreateGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_group(&self, group_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_groups(&self, max_items: i64, path_prefix: String) -> Result<ListGroupsOutput, Box<dyn std::error::Error>>;
    async fn add_user_to_group(&self, group_name: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_user_from_group(&self, group_name: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_groups_for_user(&self, max_items: i64, user_name: String) -> Result<ListGroupsForUserOutput, Box<dyn std::error::Error>>;
    async fn create_role(&self, assume_role_policy_document: String, description: String, max_session_duration: i64, path: String, permissions_boundary: String, role_name: String, tags: String) -> Result<CreateRoleOutput, Box<dyn std::error::Error>>;
    async fn delete_role(&self, role_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_role(&self, role_name: String) -> Result<GetRoleOutput, Box<dyn std::error::Error>>;
    async fn list_roles(&self, max_items: i64, path_prefix: String) -> Result<ListRolesOutput, Box<dyn std::error::Error>>;
    async fn create_policy(&self, description: String, path: String, policy_document: String, policy_name: String, tags: String) -> Result<CreatePolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_policy(&self, policy_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_policy(&self, policy_arn: String) -> Result<GetPolicyOutput, Box<dyn std::error::Error>>;
    async fn list_policies(&self, max_items: i64, only_attached: bool, path_prefix: String, scope: String) -> Result<ListPoliciesOutput, Box<dyn std::error::Error>>;
    async fn attach_user_policy(&self, policy_arn: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_user_policy(&self, policy_arn: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn attach_group_policy(&self, group_name: String, policy_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_group_policy(&self, group_name: String, policy_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn attach_role_policy(&self, policy_arn: String, role_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_role_policy(&self, policy_arn: String, role_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn put_user_policy(&self, policy_document: String, policy_name: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn put_group_policy(&self, group_name: String, policy_document: String, policy_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn put_role_policy(&self, policy_document: String, policy_name: String, role_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_access_key(&self, user_name: String) -> Result<CreateAccessKeyOutput, Box<dyn std::error::Error>>;
    async fn delete_access_key(&self, access_key_id: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_access_keys(&self, max_items: i64, user_name: String) -> Result<ListAccessKeysOutput, Box<dyn std::error::Error>>;
    async fn update_access_key(&self, access_key_id: String, status: String, user_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_account_summary(&self) -> Result<String, Box<dyn std::error::Error>>;
    async fn generate_credential_report(&self) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_credential_report(&self) -> Result<GetCredentialReportOutput, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserInput {
    pub path: String,
    pub permissions_boundary: String,
    pub tags: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteUserInput {
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteUserOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserInput {
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
pub struct ListUsersInput {
    pub max_items: i64,
    pub path_prefix: String,
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
pub struct UpdateUserInput {
    pub new_path: String,
    pub new_user_name: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateUserOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupInput {
    pub group_name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub group_id: String,
    pub group_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteGroupInput {
    pub group_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListGroupsInput {
    pub max_items: i64,
    pub path_prefix: String,
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
pub struct AddUserToGroupInput {
    pub group_name: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddUserToGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveUserFromGroupInput {
    pub group_name: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveUserFromGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListGroupsForUserInput {
    pub max_items: i64,
    pub user_name: String,
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
pub struct CreateRoleInput {
    pub assume_role_policy_document: String,
    pub description: String,
    pub max_session_duration: i64,
    pub path: String,
    pub permissions_boundary: String,
    pub role_name: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRoleOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub role_id: String,
    pub role_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRoleInput {
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRoleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRoleInput {
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
pub struct ListRolesInput {
    pub max_items: i64,
    pub path_prefix: String,
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
pub struct CreatePolicyInput {
    pub description: String,
    pub path: String,
    pub policy_document: String,
    pub policy_name: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePolicyOutput {
    pub arn: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub policy_id: String,
    pub policy_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePolicyInput {
    pub policy_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPolicyInput {
    pub policy_arn: String,
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
pub struct ListPoliciesInput {
    pub max_items: i64,
    #[serde(default)]
    pub only_attached: bool,
    pub path_prefix: String,
    pub scope: String,
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
pub struct AttachUserPolicyInput {
    pub policy_arn: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachUserPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachUserPolicyInput {
    pub policy_arn: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachUserPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachGroupPolicyInput {
    pub group_name: String,
    pub policy_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachGroupPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachGroupPolicyInput {
    pub group_name: String,
    pub policy_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachGroupPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachRolePolicyInput {
    pub policy_arn: String,
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachRolePolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachRolePolicyInput {
    pub policy_arn: String,
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachRolePolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutUserPolicyInput {
    pub policy_document: String,
    pub policy_name: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutUserPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutGroupPolicyInput {
    pub group_name: String,
    pub policy_document: String,
    pub policy_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutGroupPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutRolePolicyInput {
    pub policy_document: String,
    pub policy_name: String,
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutRolePolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAccessKeyInput {
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAccessKeyOutput {
    pub access_key_id: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub secret_access_key: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAccessKeyInput {
    pub access_key_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAccessKeyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListAccessKeysInput {
    pub max_items: i64,
    pub user_name: String,
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
pub struct UpdateAccessKeyInput {
    pub access_key_id: String,
    pub status: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateAccessKeyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountSummaryOutput {
    pub summary_map: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateCredentialReportOutput {
    pub state: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCredentialReportOutput {
    pub content: String,
    pub generated_time: chrono::DateTime<chrono::Utc>,
    pub report_format: String,
}

#[async_trait]
pub trait IamAction {
    async fn create_user(&self, input: CreateUserInput) -> Result<CreateUserOutput, Box<dyn std::error::Error>>;
    async fn delete_user(&self, input: DeleteUserInput) -> Result<DeleteUserOutput, Box<dyn std::error::Error>>;
    async fn get_user(&self, input: GetUserInput) -> Result<GetUserOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, input: ListUsersInput) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<UpdateUserOutput, Box<dyn std::error::Error>>;
    async fn create_group(&self, input: CreateGroupInput) -> Result<CreateGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_group(&self, input: DeleteGroupInput) -> Result<DeleteGroupOutput, Box<dyn std::error::Error>>;
    async fn list_groups(&self, input: ListGroupsInput) -> Result<ListGroupsOutput, Box<dyn std::error::Error>>;
    async fn add_user_to_group(&self, input: AddUserToGroupInput) -> Result<AddUserToGroupOutput, Box<dyn std::error::Error>>;
    async fn remove_user_from_group(&self, input: RemoveUserFromGroupInput) -> Result<RemoveUserFromGroupOutput, Box<dyn std::error::Error>>;
    async fn list_groups_for_user(&self, input: ListGroupsForUserInput) -> Result<ListGroupsForUserOutput, Box<dyn std::error::Error>>;
    async fn create_role(&self, input: CreateRoleInput) -> Result<CreateRoleOutput, Box<dyn std::error::Error>>;
    async fn delete_role(&self, input: DeleteRoleInput) -> Result<DeleteRoleOutput, Box<dyn std::error::Error>>;
    async fn get_role(&self, input: GetRoleInput) -> Result<GetRoleOutput, Box<dyn std::error::Error>>;
    async fn list_roles(&self, input: ListRolesInput) -> Result<ListRolesOutput, Box<dyn std::error::Error>>;
    async fn create_policy(&self, input: CreatePolicyInput) -> Result<CreatePolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_policy(&self, input: DeletePolicyInput) -> Result<DeletePolicyOutput, Box<dyn std::error::Error>>;
    async fn get_policy(&self, input: GetPolicyInput) -> Result<GetPolicyOutput, Box<dyn std::error::Error>>;
    async fn list_policies(&self, input: ListPoliciesInput) -> Result<ListPoliciesOutput, Box<dyn std::error::Error>>;
    async fn attach_user_policy(&self, input: AttachUserPolicyInput) -> Result<AttachUserPolicyOutput, Box<dyn std::error::Error>>;
    async fn detach_user_policy(&self, input: DetachUserPolicyInput) -> Result<DetachUserPolicyOutput, Box<dyn std::error::Error>>;
    async fn attach_group_policy(&self, input: AttachGroupPolicyInput) -> Result<AttachGroupPolicyOutput, Box<dyn std::error::Error>>;
    async fn detach_group_policy(&self, input: DetachGroupPolicyInput) -> Result<DetachGroupPolicyOutput, Box<dyn std::error::Error>>;
    async fn attach_role_policy(&self, input: AttachRolePolicyInput) -> Result<AttachRolePolicyOutput, Box<dyn std::error::Error>>;
    async fn detach_role_policy(&self, input: DetachRolePolicyInput) -> Result<DetachRolePolicyOutput, Box<dyn std::error::Error>>;
    async fn put_user_policy(&self, input: PutUserPolicyInput) -> Result<PutUserPolicyOutput, Box<dyn std::error::Error>>;
    async fn put_group_policy(&self, input: PutGroupPolicyInput) -> Result<PutGroupPolicyOutput, Box<dyn std::error::Error>>;
    async fn put_role_policy(&self, input: PutRolePolicyInput) -> Result<PutRolePolicyOutput, Box<dyn std::error::Error>>;
    async fn create_access_key(&self, input: CreateAccessKeyInput) -> Result<CreateAccessKeyOutput, Box<dyn std::error::Error>>;
    async fn delete_access_key(&self, input: DeleteAccessKeyInput) -> Result<DeleteAccessKeyOutput, Box<dyn std::error::Error>>;
    async fn list_access_keys(&self, input: ListAccessKeysInput) -> Result<ListAccessKeysOutput, Box<dyn std::error::Error>>;
    async fn update_access_key(&self, input: UpdateAccessKeyInput) -> Result<UpdateAccessKeyOutput, Box<dyn std::error::Error>>;
    async fn get_account_summary(&self) -> Result<GetAccountSummaryOutput, Box<dyn std::error::Error>>;
    async fn generate_credential_report(&self) -> Result<GenerateCredentialReportOutput, Box<dyn std::error::Error>>;
    async fn get_credential_report(&self) -> Result<GetCredentialReportOutput, Box<dyn std::error::Error>>;
}

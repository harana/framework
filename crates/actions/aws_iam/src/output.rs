// Harana Actions - Aws Iam Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add_user_to_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUserToGroupOutput {
    pub success: bool
}

// attach_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachGroupPolicyOutput {
    pub success: bool
}

// attach_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachRolePolicyOutput {
    pub success: bool
}

// attach_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachUserPolicyOutput {
    pub success: bool
}

// create_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccessKeyOutput {
    pub secret_access_key: String,
    pub status: String,
    pub create_date: String,
    pub access_key_id: String,
    pub success: bool
}

// create_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupOutput {
    pub success: bool,
    pub create_date: String,
    pub arn: String,
    pub group_id: String,
    pub group_name: String
}

// create_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyOutput {
    pub arn: String,
    pub create_date: String,
    pub policy_id: String,
    pub success: bool,
    pub policy_name: String
}

// create_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleOutput {
    pub arn: String,
    pub success: bool,
    pub create_date: String,
    pub role_name: String,
    pub role_id: String
}

// create_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub arn: String,
    pub success: bool,
    pub user_id: String,
    pub create_date: String,
    pub user_name: String
}

// delete_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccessKeyOutput {
    pub success: bool
}

// delete_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGroupOutput {
    pub success: bool
}

// delete_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyOutput {
    pub success: bool
}

// delete_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleOutput {
    pub success: bool
}

// delete_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserOutput {
    pub success: bool
}

// detach_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachGroupPolicyOutput {
    pub success: bool
}

// detach_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachRolePolicyOutput {
    pub success: bool
}

// detach_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachUserPolicyOutput {
    pub success: bool
}

// generate_credential_report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCredentialReportOutput {
    pub state: String,
    pub success: bool
}

// get_account_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountSummaryOutput {
    pub summary_map: HashMap<String, Value>
}

// get_credential_report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCredentialReportOutput {
    pub generated_time: String,
    pub content: String,
    pub report_format: String
}

// get_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPolicyOutput {
    pub policy_name: String,
    pub attachment_count: i32,
    pub permissions_boundary_usage_count: i32,
    pub path: String,
    pub default_version_id: String,
    pub update_date: String,
    pub policy_id: String,
    pub tags: Vec<HashMap<String, Value>>,
    pub arn: String,
    pub is_attachable: bool,
    pub description: String,
    pub create_date: String
}

// get_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleOutput {
    pub assume_role_policy_document: String,
    pub permissions_boundary: HashMap<String, Value>,
    pub arn: String,
    pub role_name: String,
    pub role_id: String,
    pub description: String,
    pub path: String,
    pub tags: Vec<HashMap<String, Value>>,
    pub create_date: String,
    pub max_session_duration: i32
}

// get_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    pub arn: String,
    pub create_date: String,
    pub password_last_used: String,
    pub user_id: String,
    pub path: String,
    pub permissions_boundary: HashMap<String, Value>,
    pub tags: Vec<HashMap<String, Value>>,
    pub user_name: String
}

// list_access_keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccessKeysOutput {
    pub access_key_metadata: Vec<HashMap<String, Value>>,
    pub is_truncated: bool,
    pub marker: String
}

// list_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsOutput {
    pub marker: String,
    pub is_truncated: bool,
    pub groups: Vec<HashMap<String, Value>>
}

// list_groups_for_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsForUserOutput {
    pub marker: String,
    pub groups: Vec<HashMap<String, Value>>,
    pub is_truncated: bool
}

// list_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPoliciesOutput {
    pub marker: String,
    pub is_truncated: bool,
    pub policies: Vec<HashMap<String, Value>>
}

// list_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub roles: Vec<HashMap<String, Value>>
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub users: Vec<HashMap<String, Value>>
}

// put_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutGroupPolicyOutput {
    pub success: bool
}

// put_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutRolePolicyOutput {
    pub success: bool
}

// put_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutUserPolicyOutput {
    pub success: bool
}

// remove_user_from_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserFromGroupOutput {
    pub success: bool
}

// update_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccessKeyOutput {
    pub success: bool
}

// update_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserOutput {
    pub success: bool
}

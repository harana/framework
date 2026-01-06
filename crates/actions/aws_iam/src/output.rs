// Harana Actions - AWS IAM Module Output Types
// Auto-generated output structs for AWS IAM action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub arn: String,
    pub create_date: String,
    pub success: bool,
    pub user_id: String,
    pub user_name: String,
}

// delete_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserOutput {
    pub success: bool,
}

// get_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    pub arn: String,
    pub create_date: String,
    pub password_last_used: String,
    pub path: String,
    pub permissions_boundary: HashMap<String, Value>,
    pub tags: Vec<HashMap<String, Value>>,
    pub user_id: String,
    pub user_name: String,
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub users: Vec<HashMap<String, Value>>,
}

// update_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserOutput {
    pub success: bool,
}


// create_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupOutput {
    pub success: bool,
    pub group_id: String,
    pub create_date: String,
    pub group_name: String,
    pub arn: String
}

// delete_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGroupOutput {
    pub success: bool
}

// list_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsOutput {
    pub groups: Vec<HashMap<String, Value>>,
    pub is_truncated: bool,
    pub marker: String
}

// add_user_to_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUserToGroupOutput {
    pub success: bool
}

// remove_user_from_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserFromGroupOutput {
    pub success: bool
}

// list_groups_for_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsForUserOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub groups: Vec<HashMap<String, Value>>
}

// create_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleOutput {
    pub create_date: String,
    pub arn: String,
    pub success: bool,
    pub role_name: String,
    pub role_id: String
}

// delete_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleOutput {
    pub success: bool
}

// get_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleOutput {
    pub path: String,
    pub arn: String,
    pub assume_role_policy_document: String,
    pub role_id: String,
    pub max_session_duration: i32,
    pub description: String,
    pub create_date: String,
    pub role_name: String,
    pub permissions_boundary: HashMap<String, Value>,
    pub tags: Vec<HashMap<String, Value>>
}

// list_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesOutput {
    pub marker: String,
    pub is_truncated: bool,
    pub roles: Vec<HashMap<String, Value>>
}

// create_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyOutput {
    pub policy_name: String,
    pub arn: String,
    pub policy_id: String,
    pub create_date: String,
    pub success: bool
}

// delete_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyOutput {
    pub success: bool
}

// get_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPolicyOutput {
    pub arn: String,
    pub tags: Vec<HashMap<String, Value>>,
    pub path: String,
    pub update_date: String,
    pub policy_name: String,
    pub description: String,
    pub is_attachable: bool,
    pub default_version_id: String,
    pub permissions_boundary_usage_count: i32,
    pub attachment_count: i32,
    pub create_date: String,
    pub policy_id: String
}

// list_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPoliciesOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub policies: Vec<HashMap<String, Value>>
}

// attach_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachUserPolicyOutput {
    pub success: bool
}

// detach_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachUserPolicyOutput {
    pub success: bool
}

// attach_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachGroupPolicyOutput {
    pub success: bool
}

// detach_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachGroupPolicyOutput {
    pub success: bool
}

// attach_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachRolePolicyOutput {
    pub success: bool
}

// detach_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachRolePolicyOutput {
    pub success: bool
}

// put_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutUserPolicyOutput {
    pub success: bool
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

// create_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccessKeyOutput {
    pub create_date: String,
    pub success: bool,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub status: String
}

// delete_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccessKeyOutput {
    pub success: bool
}

// list_access_keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccessKeysOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub access_key_metadata: Vec<HashMap<String, Value>>
}

// update_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccessKeyOutput {
    pub success: bool
}

// get_account_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountSummaryOutput {
    pub summary_map: HashMap<String, Value>
}

// generate_credential_report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCredentialReportOutput {
    pub success: bool,
    pub state: String
}

// get_credential_report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCredentialReportOutput {
    pub content: String,
    pub generated_time: String,
    pub report_format: String
}
// TODO: Add remaining output types - see core/schema/actions/aws_iam.yml

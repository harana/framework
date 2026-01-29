//! Output types for AWS IAM actions
//!
//! This module contains all the output structs and helper types used by the AWS IAM actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output for add_user_to_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUserToGroupOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for attach_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachGroupPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for attach_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachRolePolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for attach_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachUserPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for create_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccessKeyOutput {
    /// The secret access key
    pub secret_access_key: String,
    /// The status of the access key
    pub status: String,
    /// The date when the access key was created
    pub create_date: String,
    /// The access key ID
    pub access_key_id: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for create_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// The date when the group was created
    pub create_date: String,
    /// The Amazon Resource Name (ARN) of the group
    pub arn: String,
    /// The unique ID of the group
    pub group_id: String,
    /// The name of the group
    pub group_name: String,
}

/// Output for create_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyOutput {
    /// The Amazon Resource Name (ARN) of the policy
    pub arn: String,
    /// The date when the policy was created
    pub create_date: String,
    /// The unique ID of the policy
    pub policy_id: String,
    /// Whether the operation was successful
    pub success: bool,
    /// The name of the policy
    pub policy_name: String,
}

/// Output for create_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleOutput {
    /// The Amazon Resource Name (ARN) of the role
    pub arn: String,
    /// Whether the operation was successful
    pub success: bool,
    /// The date when the role was created
    pub create_date: String,
    /// The name of the role
    pub role_name: String,
    /// The unique ID of the role
    pub role_id: String,
}

/// Output for create_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    /// The Amazon Resource Name (ARN) of the user
    pub arn: String,
    /// Whether the operation was successful
    pub success: bool,
    /// The unique ID of the user
    pub user_id: String,
    /// The date when the user was created
    pub create_date: String,
    /// The name of the user
    pub user_name: String,
}

/// Output for delete_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccessKeyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGroupOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for detach_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachGroupPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for detach_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachRolePolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for detach_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachUserPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for generate_credential_report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCredentialReportOutput {
    /// The state of the credential report generation
    pub state: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_account_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountSummaryOutput {
    /// A map of summary metrics for the AWS account
    pub summary_map: HashMap<String, Value>,
}

/// Output for get_credential_report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCredentialReportOutput {
    /// The time when the credential report was generated
    pub generated_time: String,
    /// The content of the credential report (CSV format)
    pub content: String,
    /// The format of the credential report
    pub report_format: String,
}

/// Output for get_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPolicyOutput {
    /// The name of the policy
    pub policy_name: String,
    /// The number of entities attached to this policy
    pub attachment_count: i32,
    /// The number of entities using this policy as a permissions boundary
    pub permissions_boundary_usage_count: i32,
    /// The path to the policy
    pub path: String,
    /// The identifier for the version of the policy that is the default version
    pub default_version_id: String,
    /// The date when the policy was last updated
    pub update_date: String,
    /// The unique ID of the policy
    pub policy_id: String,
    /// Tags attached to the policy
    pub tags: Vec<HashMap<String, Value>>,
    /// The Amazon Resource Name (ARN) of the policy
    pub arn: String,
    /// Whether the policy can be attached to resources
    pub is_attachable: bool,
    /// A friendly description of the policy
    pub description: String,
    /// The date when the policy was created
    pub create_date: String,
}

/// Output for get_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleOutput {
    /// The policy that grants an entity permission to assume the role
    pub assume_role_policy_document: String,
    /// The ARN of the policy that is used to set the permissions boundary
    pub permissions_boundary: HashMap<String, Value>,
    /// The Amazon Resource Name (ARN) of the role
    pub arn: String,
    /// The name of the role
    pub role_name: String,
    /// The unique ID of the role
    pub role_id: String,
    /// A description of the role
    pub description: String,
    /// The path to the role
    pub path: String,
    /// Tags attached to the role
    pub tags: Vec<HashMap<String, Value>>,
    /// The date when the role was created
    pub create_date: String,
    /// The maximum session duration (in seconds) for the role
    pub max_session_duration: i32,
}

/// Output for get_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    /// The Amazon Resource Name (ARN) of the user
    pub arn: String,
    /// The date when the user was created
    pub create_date: String,
    /// The date and time when the user's password was last used
    pub password_last_used: String,
    /// The unique ID of the user
    pub user_id: String,
    /// The path to the user
    pub path: String,
    /// The ARN of the policy that is used to set the permissions boundary
    pub permissions_boundary: HashMap<String, Value>,
    /// Tags attached to the user
    pub tags: Vec<HashMap<String, Value>>,
    /// The name of the user
    pub user_name: String,
}

/// Output for list_access_keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccessKeysOutput {
    /// A list of access key metadata
    pub access_key_metadata: Vec<HashMap<String, Value>>,
    /// A flag that indicates whether there are more items to return
    pub is_truncated: bool,
    /// When truncated, this is the value to use for the Marker parameter
    pub marker: String,
}

/// Output for list_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsOutput {
    /// When truncated, this is the value to use for the Marker parameter
    pub marker: String,
    /// A flag that indicates whether there are more items to return
    pub is_truncated: bool,
    /// A list of groups
    pub groups: Vec<HashMap<String, Value>>,
}

/// Output for list_groups_for_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsForUserOutput {
    /// When truncated, this is the value to use for the Marker parameter
    pub marker: String,
    /// A list of groups the user belongs to
    pub groups: Vec<HashMap<String, Value>>,
    /// A flag that indicates whether there are more items to return
    pub is_truncated: bool,
}

/// Output for list_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPoliciesOutput {
    /// When truncated, this is the value to use for the Marker parameter
    pub marker: String,
    /// A flag that indicates whether there are more items to return
    pub is_truncated: bool,
    /// A list of policies
    pub policies: Vec<HashMap<String, Value>>,
}

/// Output for list_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesOutput {
    /// A flag that indicates whether there are more items to return
    pub is_truncated: bool,
    /// When truncated, this is the value to use for the Marker parameter
    pub marker: String,
    /// A list of roles
    pub roles: Vec<HashMap<String, Value>>,
}

/// Output for list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    /// A flag that indicates whether there are more items to return
    pub is_truncated: bool,
    /// When truncated, this is the value to use for the Marker parameter
    pub marker: String,
    /// A list of users
    pub users: Vec<HashMap<String, Value>>,
}

/// Output for put_group_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutGroupPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for put_role_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutRolePolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for put_user_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutUserPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for remove_user_from_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserFromGroupOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for update_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccessKeyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for update_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserOutput {
    /// Whether the operation was successful
    pub success: bool,
}

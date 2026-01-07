// Harana Actions - Aws Iam Module
// This module provides aws iam actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Add User To Group
pub async fn add_user_to_group(
    group_name: &str,
    user_name: &str,
) -> Result<AddUserToGroupOutput, String> {
    unimplemented!("add_user_to_group")
}

/// Attach Group Policy
pub async fn attach_group_policy(
    policy_arn: &str,
    group_name: &str,
) -> Result<AttachGroupPolicyOutput, String> {
    unimplemented!("attach_group_policy")
}

/// Attach Role Policy
pub async fn attach_role_policy(
    role_name: &str,
    policy_arn: &str,
) -> Result<AttachRolePolicyOutput, String> {
    unimplemented!("attach_role_policy")
}

/// Attach User Policy
pub async fn attach_user_policy(
    user_name: &str,
    policy_arn: &str,
) -> Result<AttachUserPolicyOutput, String> {
    unimplemented!("attach_user_policy")
}

/// Create Access Key
pub async fn create_access_key(
    user_name: Option<&str>,
) -> Result<CreateAccessKeyOutput, String> {
    unimplemented!("create_access_key")
}

/// Create IAM Group
pub async fn create_group(
    group_name: &str,
    path: Option<&str>,
) -> Result<CreateGroupOutput, String> {
    unimplemented!("create_group")
}

/// Create IAM Policy
pub async fn create_policy(
    policy_name: &str,
    policy_document: &str,
    description: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    path: Option<&str>,
) -> Result<CreatePolicyOutput, String> {
    unimplemented!("create_policy")
}

/// Create IAM Role
pub async fn create_role(
    role_name: &str,
    assume_role_policy_document: &str,
    permissions_boundary: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    path: Option<&str>,
    description: Option<&str>,
    max_session_duration: Option<i32>,
) -> Result<CreateRoleOutput, String> {
    unimplemented!("create_role")
}

/// Create IAM User
pub async fn create_user(
    user_name: &str,
    path: Option<&str>,
    permissions_boundary: Option<&str>,
    tags: Option<HashMap<String, Value>>,
) -> Result<CreateUserOutput, String> {
    unimplemented!("create_user")
}

/// Delete Access Key
pub async fn delete_access_key(
    access_key_id: &str,
    user_name: Option<&str>,
) -> Result<DeleteAccessKeyOutput, String> {
    unimplemented!("delete_access_key")
}

/// Delete IAM Group
pub async fn delete_group(
    group_name: &str,
) -> Result<DeleteGroupOutput, String> {
    unimplemented!("delete_group")
}

/// Delete IAM Policy
pub async fn delete_policy(
    policy_arn: &str,
) -> Result<DeletePolicyOutput, String> {
    unimplemented!("delete_policy")
}

/// Delete IAM Role
pub async fn delete_role(
    role_name: &str,
) -> Result<DeleteRoleOutput, String> {
    unimplemented!("delete_role")
}

/// Delete IAM User
pub async fn delete_user(
    user_name: &str,
) -> Result<DeleteUserOutput, String> {
    unimplemented!("delete_user")
}

/// Detach Group Policy
pub async fn detach_group_policy(
    group_name: &str,
    policy_arn: &str,
) -> Result<DetachGroupPolicyOutput, String> {
    unimplemented!("detach_group_policy")
}

/// Detach Role Policy
pub async fn detach_role_policy(
    role_name: &str,
    policy_arn: &str,
) -> Result<DetachRolePolicyOutput, String> {
    unimplemented!("detach_role_policy")
}

/// Detach User Policy
pub async fn detach_user_policy(
    policy_arn: &str,
    user_name: &str,
) -> Result<DetachUserPolicyOutput, String> {
    unimplemented!("detach_user_policy")
}

/// Generate Credential Report
pub async fn generate_credential_report() -> Result<GenerateCredentialReportOutput, String> {
    unimplemented!("generate_credential_report")
}

/// Get Account Summary
pub async fn get_account_summary() -> Result<GetAccountSummaryOutput, String> {
    unimplemented!("get_account_summary")
}

/// Get Credential Report
pub async fn get_credential_report() -> Result<GetCredentialReportOutput, String> {
    unimplemented!("get_credential_report")
}

/// Get IAM Policy
pub async fn get_policy(
    policy_arn: &str,
) -> Result<GetPolicyOutput, String> {
    unimplemented!("get_policy")
}

/// Get IAM Role
pub async fn get_role(
    role_name: &str,
) -> Result<GetRoleOutput, String> {
    unimplemented!("get_role")
}

/// Get IAM User
pub async fn get_user(
    user_name: Option<&str>,
) -> Result<GetUserOutput, String> {
    unimplemented!("get_user")
}

/// List Access Keys
pub async fn list_access_keys(
    max_items: Option<i32>,
    user_name: Option<&str>,
) -> Result<ListAccessKeysOutput, String> {
    unimplemented!("list_access_keys")
}

/// List IAM Groups
pub async fn list_groups(
    max_items: Option<i32>,
    path_prefix: Option<&str>,
) -> Result<ListGroupsOutput, String> {
    unimplemented!("list_groups")
}

/// List Groups For User
pub async fn list_groups_for_user(
    user_name: &str,
    max_items: Option<i32>,
) -> Result<ListGroupsForUserOutput, String> {
    unimplemented!("list_groups_for_user")
}

/// List IAM Policies
pub async fn list_policies(
    scope: Option<&str>,
    path_prefix: Option<&str>,
    max_items: Option<i32>,
    only_attached: Option<bool>,
) -> Result<ListPoliciesOutput, String> {
    unimplemented!("list_policies")
}

/// List IAM Roles
pub async fn list_roles(
    max_items: Option<i32>,
    path_prefix: Option<&str>,
) -> Result<ListRolesOutput, String> {
    unimplemented!("list_roles")
}

/// List IAM Users
pub async fn list_users(
    path_prefix: Option<&str>,
    max_items: Option<i32>,
) -> Result<ListUsersOutput, String> {
    unimplemented!("list_users")
}

/// Put Group Policy
pub async fn put_group_policy(
    group_name: &str,
    policy_name: &str,
    policy_document: &str,
) -> Result<PutGroupPolicyOutput, String> {
    unimplemented!("put_group_policy")
}

/// Put Role Policy
pub async fn put_role_policy(
    policy_name: &str,
    policy_document: &str,
    role_name: &str,
) -> Result<PutRolePolicyOutput, String> {
    unimplemented!("put_role_policy")
}

/// Put User Policy
pub async fn put_user_policy(
    policy_name: &str,
    policy_document: &str,
    user_name: &str,
) -> Result<PutUserPolicyOutput, String> {
    unimplemented!("put_user_policy")
}

/// Remove User From Group
pub async fn remove_user_from_group(
    user_name: &str,
    group_name: &str,
) -> Result<RemoveUserFromGroupOutput, String> {
    unimplemented!("remove_user_from_group")
}

/// Update Access Key
pub async fn update_access_key(
    access_key_id: &str,
    status: &str,
    user_name: Option<&str>,
) -> Result<UpdateAccessKeyOutput, String> {
    unimplemented!("update_access_key")
}

/// Update IAM User
pub async fn update_user(
    user_name: &str,
    new_user_name: Option<&str>,
    new_path: Option<&str>,
) -> Result<UpdateUserOutput, String> {
    unimplemented!("update_user")
}

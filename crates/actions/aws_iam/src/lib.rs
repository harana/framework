//! Harana Actions - AWS IAM Module
//!
//! This module provides AWS Identity and Access Management (IAM) actions for managing
//! users, groups, roles, policies, and access keys.

#![warn(missing_docs)]

/// Output types for AWS IAM actions
pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_iam::{
    config::Region,
    types::{StatusType, Tag},
    Client,
};
use output::*;
use serde_json::Value;
use std::collections::HashMap;

/// Creates an IAM client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let iam_config = if let Some(region_str) = region {
        aws_sdk_iam::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_iam::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(iam_config))
}

/// Converts a HashMap of tags to AWS SDK Tag format
fn tags_to_aws_tags(tags: &HashMap<String, Value>) -> Vec<Tag> {
    tags.iter()
        .map(|(k, v)| {
            Tag::builder()
                .key(k)
                .value(v.as_str().unwrap_or(&v.to_string()))
                .build()
                .expect("Failed to build tag")
        })
        .collect()
}

/// Converts AWS SDK Tags to a Vec of HashMaps
fn aws_tags_to_vec(tags: &[aws_sdk_iam::types::Tag]) -> Vec<HashMap<String, Value>> {
    tags.iter()
        .map(|t| {
            let mut map = HashMap::new();
            map.insert(
                "key".to_string(),
                Value::String(t.key().to_string()),
            );
            map.insert(
                "value".to_string(),
                Value::String(t.value().to_string()),
            );
            map
        })
        .collect()
}

/// Add User To Group
///
/// Adds the specified user to the specified group.
///
pub async fn add_user_to_group(
    group_name: &str,
    user_name: &str,
    region: Option<&str>,
) -> Result<AddUserToGroupOutput, String> {
    let client = create_client(region).await?;

    client
        .add_user_to_group()
        .group_name(group_name)
        .user_name(user_name)
        .send()
        .await
        .map_err(|e| format!("Failed to add user to group: {}", e))?;

    Ok(AddUserToGroupOutput { success: true })
}

/// Attach Group Policy
///
/// Attaches the specified managed policy to the specified IAM group.
///
pub async fn attach_group_policy(
    policy_arn: &str,
    group_name: &str,
    region: Option<&str>,
) -> Result<AttachGroupPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .attach_group_policy()
        .group_name(group_name)
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to attach group policy: {}", e))?;

    Ok(AttachGroupPolicyOutput { success: true })
}

/// Attach Role Policy
///
/// Attaches the specified managed policy to the specified IAM role.
///
pub async fn attach_role_policy(
    role_name: &str,
    policy_arn: &str,
    region: Option<&str>,
) -> Result<AttachRolePolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .attach_role_policy()
        .role_name(role_name)
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to attach role policy: {}", e))?;

    Ok(AttachRolePolicyOutput { success: true })
}

/// Attach User Policy
///
/// Attaches the specified managed policy to the specified user.
///
pub async fn attach_user_policy(
    user_name: &str,
    policy_arn: &str,
    region: Option<&str>,
) -> Result<AttachUserPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .attach_user_policy()
        .user_name(user_name)
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to attach user policy: {}", e))?;

    Ok(AttachUserPolicyOutput { success: true })
}

/// Create Access Key
///
/// Creates a new AWS secret access key and corresponding AWS access key ID for the specified user.
///
pub async fn create_access_key(
    user_name: Option<&str>,
    region: Option<&str>,
) -> Result<CreateAccessKeyOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_access_key();

    if let Some(name) = user_name {
        request = request.user_name(name);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create access key: {}", e))?;

    let access_key = response
        .access_key()
        .ok_or("No access key in response")?;

    Ok(CreateAccessKeyOutput {
        access_key_id: access_key.access_key_id().to_string(),
        secret_access_key: access_key.secret_access_key().to_string(),
        status: access_key
            .status()
            .as_str()
            .to_string(),
        create_date: access_key
            .create_date()
            .map(|d| {
                chrono::DateTime::from_timestamp(d.secs(), d.subsec_nanos())
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            })
            .unwrap_or_default(),
        success: true,
    })
}

/// Create IAM Group
///
/// Creates a new IAM group.
///
pub async fn create_group(
    group_name: &str,
    path: Option<&str>,
    region: Option<&str>,
) -> Result<CreateGroupOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_group().group_name(group_name);

    if let Some(p) = path {
        request = request.path(p);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create group: {}", e))?;

    let group = response.group().ok_or("No group in response")?;

    Ok(CreateGroupOutput {
        group_name: group.group_name().to_string(),
        group_id: group.group_id().to_string(),
        arn: group.arn().to_string(),
        create_date: chrono::DateTime::from_timestamp(
            group.create_date().secs(),
            group.create_date().subsec_nanos(),
        )
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default(),
        success: true,
    })
}

/// Create IAM Policy
///
/// Creates a new IAM policy for your AWS account.
///
pub async fn create_policy(
    policy_name: &str,
    policy_document: &str,
    description: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    path: Option<&str>,
    region: Option<&str>,
) -> Result<CreatePolicyOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .create_policy()
        .policy_name(policy_name)
        .policy_document(policy_document);

    if let Some(desc) = description {
        request = request.description(desc);
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    if let Some(p) = path {
        request = request.path(p);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create policy: {}", e))?;

    let policy = response.policy().ok_or("No policy in response")?;

    Ok(CreatePolicyOutput {
        policy_name: policy.policy_name().unwrap_or_default().to_string(),
        policy_id: policy.policy_id().unwrap_or_default().to_string(),
        arn: policy.arn().unwrap_or_default().to_string(),
        create_date: policy
            .create_date()
            .map(|d| {
                chrono::DateTime::from_timestamp(d.secs(), d.subsec_nanos())
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            })
            .unwrap_or_default(),
        success: true,
    })
}

/// Create IAM Role
///
/// Creates a new role for your AWS account.
///
pub async fn create_role(
    role_name: &str,
    assume_role_policy_document: &str,
    permissions_boundary: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    path: Option<&str>,
    description: Option<&str>,
    max_session_duration: Option<i32>,
    region: Option<&str>,
) -> Result<CreateRoleOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .create_role()
        .role_name(role_name)
        .assume_role_policy_document(assume_role_policy_document);

    if let Some(boundary) = permissions_boundary {
        request = request.permissions_boundary(boundary);
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    if let Some(p) = path {
        request = request.path(p);
    }

    if let Some(desc) = description {
        request = request.description(desc);
    }

    if let Some(duration) = max_session_duration {
        request = request.max_session_duration(duration);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create role: {}", e))?;

    let role = response.role().ok_or("No role in response")?;

    Ok(CreateRoleOutput {
        role_name: role.role_name().to_string(),
        role_id: role.role_id().to_string(),
        arn: role.arn().to_string(),
        create_date: chrono::DateTime::from_timestamp(
            role.create_date().secs(),
            role.create_date().subsec_nanos(),
        )
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default(),
        success: true,
    })
}

/// Create IAM User
///
/// Creates a new IAM user for your AWS account.
///
pub async fn create_user(
    user_name: &str,
    path: Option<&str>,
    permissions_boundary: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    region: Option<&str>,
) -> Result<CreateUserOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_user().user_name(user_name);

    if let Some(p) = path {
        request = request.path(p);
    }

    if let Some(boundary) = permissions_boundary {
        request = request.permissions_boundary(boundary);
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

    let user = response.user().ok_or("No user in response")?;

    Ok(CreateUserOutput {
        user_name: user.user_name().to_string(),
        user_id: user.user_id().to_string(),
        arn: user.arn().to_string(),
        create_date: chrono::DateTime::from_timestamp(
            user.create_date().secs(),
            user.create_date().subsec_nanos(),
        )
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default(),
        success: true,
    })
}

/// Delete Access Key
///
/// Deletes the access key pair associated with the specified IAM user.
///
pub async fn delete_access_key(
    access_key_id: &str,
    user_name: Option<&str>,
    region: Option<&str>,
) -> Result<DeleteAccessKeyOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.delete_access_key().access_key_id(access_key_id);

    if let Some(name) = user_name {
        request = request.user_name(name);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to delete access key: {}", e))?;

    Ok(DeleteAccessKeyOutput { success: true })
}

/// Delete IAM Group
///
/// Deletes the specified IAM group. The group must not contain any users or have any attached policies.
///
pub async fn delete_group(
    group_name: &str,
    region: Option<&str>,
) -> Result<DeleteGroupOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_group()
        .group_name(group_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete group: {}", e))?;

    Ok(DeleteGroupOutput { success: true })
}

/// Delete IAM Policy
///
/// Deletes the specified managed policy.
///
pub async fn delete_policy(
    policy_arn: &str,
    region: Option<&str>,
) -> Result<DeletePolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_policy()
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to delete policy: {}", e))?;

    Ok(DeletePolicyOutput { success: true })
}

/// Delete IAM Role
///
/// Deletes the specified role. The role must not have any policies attached.
///
pub async fn delete_role(
    role_name: &str,
    region: Option<&str>,
) -> Result<DeleteRoleOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_role()
        .role_name(role_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete role: {}", e))?;

    Ok(DeleteRoleOutput { success: true })
}

/// Delete IAM User
///
/// Deletes the specified IAM user. The user must not belong to any groups or have any access keys, signing certificates, or attached policies.
///
pub async fn delete_user(
    user_name: &str,
    region: Option<&str>,
) -> Result<DeleteUserOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_user()
        .user_name(user_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete user: {}", e))?;

    Ok(DeleteUserOutput { success: true })
}

/// Detach Group Policy
///
/// Removes the specified managed policy from the specified IAM group.
///
pub async fn detach_group_policy(
    group_name: &str,
    policy_arn: &str,
    region: Option<&str>,
) -> Result<DetachGroupPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .detach_group_policy()
        .group_name(group_name)
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to detach group policy: {}", e))?;

    Ok(DetachGroupPolicyOutput { success: true })
}

/// Detach Role Policy
///
/// Removes the specified managed policy from the specified role.
///
pub async fn detach_role_policy(
    role_name: &str,
    policy_arn: &str,
    region: Option<&str>,
) -> Result<DetachRolePolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .detach_role_policy()
        .role_name(role_name)
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to detach role policy: {}", e))?;

    Ok(DetachRolePolicyOutput { success: true })
}

/// Detach User Policy
///
/// Removes the specified managed policy from the specified user.
///
pub async fn detach_user_policy(
    policy_arn: &str,
    user_name: &str,
    region: Option<&str>,
) -> Result<DetachUserPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .detach_user_policy()
        .user_name(user_name)
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to detach user policy: {}", e))?;

    Ok(DetachUserPolicyOutput { success: true })
}

/// Generate Credential Report
///
/// Generates a credential report for the AWS account.
///
pub async fn generate_credential_report(
    region: Option<&str>,
) -> Result<GenerateCredentialReportOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .generate_credential_report()
        .send()
        .await
        .map_err(|e| format!("Failed to generate credential report: {}", e))?;

    Ok(GenerateCredentialReportOutput {
        state: response
            .state()
            .map(|s| s.as_str().to_string())
            .unwrap_or_default(),
        success: true,
    })
}

/// Get Account Summary
///
/// Retrieves information about IAM entity usage and IAM quotas in the AWS account.
///
pub async fn get_account_summary(
    region: Option<&str>,
) -> Result<GetAccountSummaryOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_account_summary()
        .send()
        .await
        .map_err(|e| format!("Failed to get account summary: {}", e))?;

    let summary_map: HashMap<String, Value> = response
        .summary_map()
        .unwrap_or(&std::collections::HashMap::new())
        .iter()
        .map(|(k, v)| (k.as_str().to_string(), Value::Number((*v).into())))
        .collect();

    Ok(GetAccountSummaryOutput { summary_map })
}

/// Get Credential Report
///
/// Retrieves a credential report for the AWS account.
///
pub async fn get_credential_report(
    region: Option<&str>,
) -> Result<GetCredentialReportOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_credential_report()
        .send()
        .await
        .map_err(|e| format!("Failed to get credential report: {}", e))?;

    let content = response
        .content()
        .map(|b| String::from_utf8_lossy(b.as_ref()).to_string())
        .unwrap_or_default();

    Ok(GetCredentialReportOutput {
        content,
        generated_time: response
            .generated_time()
            .map(|t| {
                chrono::DateTime::from_timestamp(t.secs(), t.subsec_nanos())
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            })
            .unwrap_or_default(),
        report_format: response
            .report_format()
            .map(|f| f.as_str().to_string())
            .unwrap_or_default(),
    })
}

/// Get IAM Policy
///
/// Retrieves information about the specified managed policy.
///
pub async fn get_policy(
    policy_arn: &str,
    region: Option<&str>,
) -> Result<GetPolicyOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_policy()
        .policy_arn(policy_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to get policy: {}", e))?;

    let policy = response.policy().ok_or("No policy in response")?;

    Ok(GetPolicyOutput {
        policy_name: policy.policy_name().unwrap_or_default().to_string(),
        policy_id: policy.policy_id().unwrap_or_default().to_string(),
        arn: policy.arn().unwrap_or_default().to_string(),
        path: policy.path().unwrap_or_default().to_string(),
        default_version_id: policy.default_version_id().unwrap_or_default().to_string(),
        attachment_count: policy.attachment_count().unwrap_or(0),
        permissions_boundary_usage_count: policy.permissions_boundary_usage_count().unwrap_or(0),
        is_attachable: policy.is_attachable(),
        description: policy.description().unwrap_or_default().to_string(),
        create_date: policy
            .create_date()
            .map(|d| {
                chrono::DateTime::from_timestamp(d.secs(), d.subsec_nanos())
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            })
            .unwrap_or_default(),
        update_date: policy
            .update_date()
            .map(|d| {
                chrono::DateTime::from_timestamp(d.secs(), d.subsec_nanos())
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            })
            .unwrap_or_default(),
        tags: aws_tags_to_vec(policy.tags()),
    })
}

/// Get IAM Role
///
/// Retrieves information about the specified role.
///
pub async fn get_role(
    role_name: &str,
    region: Option<&str>,
) -> Result<GetRoleOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_role()
        .role_name(role_name)
        .send()
        .await
        .map_err(|e| format!("Failed to get role: {}", e))?;

    let role = response.role().ok_or("No role in response")?;

    let permissions_boundary: HashMap<String, Value> = role
        .permissions_boundary()
        .map(|pb| {
            let mut map = HashMap::new();
            if let Some(t) = pb.permissions_boundary_type() {
                map.insert(
                    "permissions_boundary_type".to_string(),
                    Value::String(t.as_str().to_string()),
                );
            }
            if let Some(arn) = pb.permissions_boundary_arn() {
                map.insert(
                    "permissions_boundary_arn".to_string(),
                    Value::String(arn.to_string()),
                );
            }
            map
        })
        .unwrap_or_default();

    Ok(GetRoleOutput {
        role_name: role.role_name().to_string(),
        role_id: role.role_id().to_string(),
        arn: role.arn().to_string(),
        path: role.path().to_string(),
        assume_role_policy_document: role
            .assume_role_policy_document()
            .map(|d| urlencoding::decode(d).unwrap_or_default().to_string())
            .unwrap_or_default(),
        description: role.description().unwrap_or_default().to_string(),
        max_session_duration: role.max_session_duration().unwrap_or(0),
        permissions_boundary,
        create_date: chrono::DateTime::from_timestamp(
            role.create_date().secs(),
            role.create_date().subsec_nanos(),
        )
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default(),
        tags: aws_tags_to_vec(role.tags()),
    })
}

/// Get IAM User
///
/// Retrieves information about the specified IAM user.
///
pub async fn get_user(
    user_name: Option<&str>,
    region: Option<&str>,
) -> Result<GetUserOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_user();

    if let Some(name) = user_name {
        request = request.user_name(name);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get user: {}", e))?;

    let user = response.user().ok_or("No user in response")?;

    let permissions_boundary: HashMap<String, Value> = user
        .permissions_boundary()
        .map(|pb| {
            let mut map = HashMap::new();
            if let Some(t) = pb.permissions_boundary_type() {
                map.insert(
                    "permissions_boundary_type".to_string(),
                    Value::String(t.as_str().to_string()),
                );
            }
            if let Some(arn) = pb.permissions_boundary_arn() {
                map.insert(
                    "permissions_boundary_arn".to_string(),
                    Value::String(arn.to_string()),
                );
            }
            map
        })
        .unwrap_or_default();

    Ok(GetUserOutput {
        user_name: user.user_name().to_string(),
        user_id: user.user_id().to_string(),
        arn: user.arn().to_string(),
        path: user.path().to_string(),
        create_date: chrono::DateTime::from_timestamp(
            user.create_date().secs(),
            user.create_date().subsec_nanos(),
        )
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default(),
        password_last_used: user
            .password_last_used()
            .map(|d| {
                chrono::DateTime::from_timestamp(d.secs(), d.subsec_nanos())
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            })
            .unwrap_or_default(),
        permissions_boundary,
        tags: aws_tags_to_vec(user.tags()),
    })
}

/// List Access Keys
///
/// Returns information about the access key IDs associated with the specified IAM user.
///
pub async fn list_access_keys(
    max_items: Option<i32>,
    user_name: Option<&str>,
    region: Option<&str>,
) -> Result<ListAccessKeysOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_access_keys();

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    if let Some(name) = user_name {
        request = request.user_name(name);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list access keys: {}", e))?;

    let access_key_metadata: Vec<HashMap<String, Value>> = response
        .access_key_metadata()
        .iter()
        .map(|ak| {
            let mut map = HashMap::new();
            map.insert(
                "user_name".to_string(),
                Value::String(ak.user_name().unwrap_or("").to_string()),
            );
            map.insert(
                "access_key_id".to_string(),
                Value::String(ak.access_key_id().unwrap_or("").to_string()),
            );
            map.insert(
                "status".to_string(),
                Value::String(
                    ak.status()
                        .map(|s| s.as_str().to_string())
                        .unwrap_or_default(),
                ),
            );
            if let Some(create_date) = ak.create_date() {
                map.insert(
                    "create_date".to_string(),
                    Value::String(
                        chrono::DateTime::from_timestamp(create_date.secs(), create_date.subsec_nanos())
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    ),
                );
            }
            map
        })
        .collect();

    Ok(ListAccessKeysOutput {
        access_key_metadata,
        is_truncated: response.is_truncated(),
        marker: response.marker().unwrap_or_default().to_string(),
    })
}

/// List IAM Groups
///
/// Lists the IAM groups that have the specified path prefix.
///
pub async fn list_groups(
    max_items: Option<i32>,
    path_prefix: Option<&str>,
    region: Option<&str>,
) -> Result<ListGroupsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_groups();

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    if let Some(prefix) = path_prefix {
        request = request.path_prefix(prefix);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list groups: {}", e))?;

    let groups: Vec<HashMap<String, Value>> = response
        .groups()
        .iter()
        .map(|g| {
            let mut map = HashMap::new();
            map.insert(
                "group_name".to_string(),
                Value::String(g.group_name().to_string()),
            );
            map.insert(
                "group_id".to_string(),
                Value::String(g.group_id().to_string()),
            );
            map.insert("arn".to_string(), Value::String(g.arn().to_string()));
            map.insert("path".to_string(), Value::String(g.path().to_string()));
            map.insert(
                "create_date".to_string(),
                Value::String(
                    chrono::DateTime::from_timestamp(g.create_date().secs(), g.create_date().subsec_nanos())
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                ),
            );
            map
        })
        .collect();

    Ok(ListGroupsOutput {
        groups,
        is_truncated: response.is_truncated(),
        marker: response.marker().unwrap_or_default().to_string(),
    })
}

/// List Groups For User
///
/// Lists the IAM groups that the specified IAM user belongs to.
///
pub async fn list_groups_for_user(
    user_name: &str,
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListGroupsForUserOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_groups_for_user().user_name(user_name);

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list groups for user: {}", e))?;

    let groups: Vec<HashMap<String, Value>> = response
        .groups()
        .iter()
        .map(|g| {
            let mut map = HashMap::new();
            map.insert(
                "group_name".to_string(),
                Value::String(g.group_name().to_string()),
            );
            map.insert(
                "group_id".to_string(),
                Value::String(g.group_id().to_string()),
            );
            map.insert("arn".to_string(), Value::String(g.arn().to_string()));
            map.insert("path".to_string(), Value::String(g.path().to_string()));
            map.insert(
                "create_date".to_string(),
                Value::String(
                    chrono::DateTime::from_timestamp(g.create_date().secs(), g.create_date().subsec_nanos())
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                ),
            );
            map
        })
        .collect();

    Ok(ListGroupsForUserOutput {
        groups,
        is_truncated: response.is_truncated(),
        marker: response.marker().unwrap_or_default().to_string(),
    })
}

/// List IAM Policies
///
/// Lists all the managed policies that are available in your AWS account.
///
pub async fn list_policies(
    scope: Option<&str>,
    path_prefix: Option<&str>,
    max_items: Option<i32>,
    only_attached: Option<bool>,
    region: Option<&str>,
) -> Result<ListPoliciesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_policies();

    if let Some(s) = scope {
        request = request.scope(aws_sdk_iam::types::PolicyScopeType::from(s));
    }

    if let Some(prefix) = path_prefix {
        request = request.path_prefix(prefix);
    }

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    if let Some(attached) = only_attached {
        request = request.only_attached(attached);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list policies: {}", e))?;

    let policies: Vec<HashMap<String, Value>> = response
        .policies()
        .iter()
        .map(|p| {
            let mut map = HashMap::new();
            map.insert(
                "policy_name".to_string(),
                Value::String(p.policy_name().unwrap_or_default().to_string()),
            );
            map.insert(
                "policy_id".to_string(),
                Value::String(p.policy_id().unwrap_or_default().to_string()),
            );
            map.insert(
                "arn".to_string(),
                Value::String(p.arn().unwrap_or_default().to_string()),
            );
            map.insert(
                "path".to_string(),
                Value::String(p.path().unwrap_or_default().to_string()),
            );
            map.insert(
                "default_version_id".to_string(),
                Value::String(p.default_version_id().unwrap_or_default().to_string()),
            );
            map.insert(
                "attachment_count".to_string(),
                Value::Number(p.attachment_count().unwrap_or(0).into()),
            );
            map.insert(
                "is_attachable".to_string(),
                Value::Bool(p.is_attachable()),
            );
            if let Some(create_date) = p.create_date() {
                map.insert(
                    "create_date".to_string(),
                    Value::String(
                        chrono::DateTime::from_timestamp(create_date.secs(), create_date.subsec_nanos())
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    ),
                );
            }
            if let Some(update_date) = p.update_date() {
                map.insert(
                    "update_date".to_string(),
                    Value::String(
                        chrono::DateTime::from_timestamp(update_date.secs(), update_date.subsec_nanos())
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    ),
                );
            }
            map
        })
        .collect();

    Ok(ListPoliciesOutput {
        policies,
        is_truncated: response.is_truncated(),
        marker: response.marker().unwrap_or_default().to_string(),
    })
}

/// List IAM Roles
///
/// Lists the IAM roles that have the specified path prefix.
///
pub async fn list_roles(
    max_items: Option<i32>,
    path_prefix: Option<&str>,
    region: Option<&str>,
) -> Result<ListRolesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_roles();

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    if let Some(prefix) = path_prefix {
        request = request.path_prefix(prefix);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list roles: {}", e))?;

    let roles: Vec<HashMap<String, Value>> = response
        .roles()
        .iter()
        .map(|r| {
            let mut map = HashMap::new();
            map.insert(
                "role_name".to_string(),
                Value::String(r.role_name().to_string()),
            );
            map.insert(
                "role_id".to_string(),
                Value::String(r.role_id().to_string()),
            );
            map.insert("arn".to_string(), Value::String(r.arn().to_string()));
            map.insert("path".to_string(), Value::String(r.path().to_string()));
            map.insert(
                "assume_role_policy_document".to_string(),
                Value::String(
                    r.assume_role_policy_document()
                        .map(|d| urlencoding::decode(d).unwrap_or_default().to_string())
                        .unwrap_or_default(),
                ),
            );
            map.insert(
                "description".to_string(),
                Value::String(r.description().unwrap_or_default().to_string()),
            );
            map.insert(
                "max_session_duration".to_string(),
                Value::Number(r.max_session_duration().unwrap_or(0).into()),
            );
            map.insert(
                "create_date".to_string(),
                Value::String(
                    chrono::DateTime::from_timestamp(r.create_date().secs(), r.create_date().subsec_nanos())
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                ),
            );
            map
        })
        .collect();

    Ok(ListRolesOutput {
        roles,
        is_truncated: response.is_truncated(),
        marker: response.marker().unwrap_or_default().to_string(),
    })
}

/// List IAM Users
///
/// Lists the IAM users that have the specified path prefix.
///
pub async fn list_users(
    path_prefix: Option<&str>,
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListUsersOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_users();

    if let Some(prefix) = path_prefix {
        request = request.path_prefix(prefix);
    }

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list users: {}", e))?;

    let users: Vec<HashMap<String, Value>> = response
        .users()
        .iter()
        .map(|u| {
            let mut map = HashMap::new();
            map.insert(
                "user_name".to_string(),
                Value::String(u.user_name().to_string()),
            );
            map.insert(
                "user_id".to_string(),
                Value::String(u.user_id().to_string()),
            );
            map.insert("arn".to_string(), Value::String(u.arn().to_string()));
            map.insert("path".to_string(), Value::String(u.path().to_string()));
            map.insert(
                "create_date".to_string(),
                Value::String(
                    chrono::DateTime::from_timestamp(u.create_date().secs(), u.create_date().subsec_nanos())
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                ),
            );
            if let Some(password_last_used) = u.password_last_used() {
                map.insert(
                    "password_last_used".to_string(),
                    Value::String(
                        chrono::DateTime::from_timestamp(password_last_used.secs(), password_last_used.subsec_nanos())
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    ),
                );
            }
            map
        })
        .collect();

    Ok(ListUsersOutput {
        users,
        is_truncated: response.is_truncated(),
        marker: response.marker().unwrap_or_default().to_string(),
    })
}

/// Put Group Policy
///
/// Adds or updates an inline policy document that is embedded in the specified IAM group.
///
pub async fn put_group_policy(
    group_name: &str,
    policy_name: &str,
    policy_document: &str,
    region: Option<&str>,
) -> Result<PutGroupPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .put_group_policy()
        .group_name(group_name)
        .policy_name(policy_name)
        .policy_document(policy_document)
        .send()
        .await
        .map_err(|e| format!("Failed to put group policy: {}", e))?;

    Ok(PutGroupPolicyOutput { success: true })
}

/// Put Role Policy
///
/// Adds or updates an inline policy document that is embedded in the specified IAM role.
///
pub async fn put_role_policy(
    policy_name: &str,
    policy_document: &str,
    role_name: &str,
    region: Option<&str>,
) -> Result<PutRolePolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .put_role_policy()
        .role_name(role_name)
        .policy_name(policy_name)
        .policy_document(policy_document)
        .send()
        .await
        .map_err(|e| format!("Failed to put role policy: {}", e))?;

    Ok(PutRolePolicyOutput { success: true })
}

/// Put User Policy
///
/// Adds or updates an inline policy document that is embedded in the specified IAM user.
///
pub async fn put_user_policy(
    policy_name: &str,
    policy_document: &str,
    user_name: &str,
    region: Option<&str>,
) -> Result<PutUserPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .put_user_policy()
        .user_name(user_name)
        .policy_name(policy_name)
        .policy_document(policy_document)
        .send()
        .await
        .map_err(|e| format!("Failed to put user policy: {}", e))?;

    Ok(PutUserPolicyOutput { success: true })
}

/// Remove User From Group
///
/// Removes the specified user from the specified group.
///
pub async fn remove_user_from_group(
    user_name: &str,
    group_name: &str,
    region: Option<&str>,
) -> Result<RemoveUserFromGroupOutput, String> {
    let client = create_client(region).await?;

    client
        .remove_user_from_group()
        .group_name(group_name)
        .user_name(user_name)
        .send()
        .await
        .map_err(|e| format!("Failed to remove user from group: {}", e))?;

    Ok(RemoveUserFromGroupOutput { success: true })
}

/// Update Access Key
///
/// Changes the status of the specified access key from Active to Inactive, or vice versa.
///
pub async fn update_access_key(
    access_key_id: &str,
    status: &str,
    user_name: Option<&str>,
    region: Option<&str>,
) -> Result<UpdateAccessKeyOutput, String> {
    let client = create_client(region).await?;

    let status_type = match status.to_lowercase().as_str() {
        "active" => StatusType::Active,
        "inactive" => StatusType::Inactive,
        _ => return Err(format!("Invalid status: {}. Must be 'Active' or 'Inactive'", status)),
    };

    let mut request = client
        .update_access_key()
        .access_key_id(access_key_id)
        .status(status_type);

    if let Some(name) = user_name {
        request = request.user_name(name);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to update access key: {}", e))?;

    Ok(UpdateAccessKeyOutput { success: true })
}

/// Update IAM User
///
/// Updates the name and/or the path of the specified IAM user.
///
pub async fn update_user(
    user_name: &str,
    new_user_name: Option<&str>,
    new_path: Option<&str>,
    region: Option<&str>,
) -> Result<UpdateUserOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.update_user().user_name(user_name);

    if let Some(new_name) = new_user_name {
        request = request.new_user_name(new_name);
    }

    if let Some(path) = new_path {
        request = request.new_path(path);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to update user: {}", e))?;

    Ok(UpdateUserOutput { success: true })
}

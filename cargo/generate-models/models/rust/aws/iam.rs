// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_iam_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamUser {
    pub account_id: String,
    pub arn: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub password_last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub path: String,
    pub permissions_boundary_arn: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    pub user_name: String,
}

impl AwsIamUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_iam_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamGroup {
    pub account_id: String,
    pub arn: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub group_id: Option<String>,
    pub group_name: String,
    pub path: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsIamGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_iam_group_membership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamGroupMembership {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_iam_group.id
    pub group_id: String,
    /// Reference: aws_iamuser.id
    pub user_id: String,
}

impl AwsIamGroupMembership {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_iam_role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamRole {
    pub account_id: String,
    pub arn: Option<String>,
    pub assume_role_policy_document: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub max_session_duration: i64,
    pub path: String,
    pub permissions_boundary_arn: Option<String>,
    pub role_id: Option<String>,
    pub role_name: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsIamRole {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_iam_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamPolicy {
    pub account_id: String,
    pub arn: Option<String>,
    pub attachment_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_version_id: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_attachable: bool,
    pub path: String,
    pub policy_arn: String,
    pub policy_id: Option<String>,
    pub policy_name: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsIamPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_iam_policy_attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamPolicyAttachment {
    #[serde(default = "chrono::Utc::now")]
    pub attached_at: chrono::DateTime<chrono::Utc>,
    pub entity_name: String,
    pub entity_type: String,
    /// Reference: aws_iam_policy.id
    pub policy_id: String,
}

impl AwsIamPolicyAttachment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_iam_access_key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsIamAccessKey {
    pub access_key_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used_region: Option<String>,
    pub last_used_service: Option<String>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_iamuser.id
    pub user_id: String,
}

impl AwsIamAccessKey {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


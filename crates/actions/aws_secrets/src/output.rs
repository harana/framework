//! Output types for AWS Secrets Manager actions
//!
//! This module contains all the output structs and helper types used by the AWS Secrets Manager actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output for cancel_rotate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRotateOutput {
    /// The ARN of the secret
    pub arn: String,
    /// The friendly name of the secret
    pub name: String,
    /// The unique identifier of the version of the secret
    pub version_id: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for create_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// The ARN of the secret
    pub arn: String,
    /// The unique identifier associated with the version of the secret
    pub version_id: String,
    /// The friendly name of the secret
    pub name: String,
}

/// Output for delete_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    /// The ARN of the secret
    pub arn: String,
    /// The date and time after which this secret will be deleted (Unix timestamp)
    pub deletion_date: i64,
    /// The friendly name of the secret
    pub name: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for describe_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeOutput {
    /// The ARN of the secret
    pub arn: String,
    /// The KMS key ID used to encrypt the secret
    pub kms_key_id: String,
    /// The date and time the secret was last rotated (Unix timestamp)
    pub last_rotated_date: i64,
    /// The date and time the secret was last changed (Unix timestamp)
    pub last_changed_date: i64,
    /// The user-provided description of the secret
    pub description: String,
    /// Whether rotation is enabled for this secret
    pub rotation_enabled: bool,
    /// The friendly name of the secret
    pub name: String,
    /// The tags attached to the secret
    pub tags: HashMap<String, Value>,
}

/// Output for get_secret_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValueOutput {
    /// The unique identifier of the version of the secret
    pub version_id: String,
    /// The friendly name of the secret
    pub name: String,
    /// The decrypted secret value
    pub secret_value: String,
    /// The date and time this version was created (Unix timestamp)
    pub created_date: i64,
    /// The ARN of the secret
    pub arn: String,
}

/// Output for list_secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
    /// Token for pagination to retrieve the next set of results
    pub next_token: String,
    /// List of secrets in the account
    pub secrets: Vec<HashMap<String, Value>>,
}

/// Output for put_secret_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutValueOutput {
    /// The unique identifier of the version of the secret
    pub version_id: String,
    /// The ARN of the secret
    pub arn: String,
    /// The friendly name of the secret
    pub name: String,
    /// The list of staging labels attached to this version
    pub version_stages: Vec<String>,
}

/// Output for remove_regions_from_replication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRegionsFromReplicationOutput {
    /// The ARN of the secret
    pub arn: String,
    /// Whether the operation was successful
    pub success: bool,
    /// The status of replicas after the removal
    pub replication_status: Vec<HashMap<String, Value>>,
}

/// Output for replicate_secret_to_regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicateOutput {
    /// The ARN of the secret
    pub arn: String,
    /// The status of replication to each region
    pub replication_status: Vec<HashMap<String, Value>>,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for restore_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOutput {
    /// The ARN of the secret
    pub arn: String,
    /// Whether the operation was successful
    pub success: bool,
    /// The friendly name of the secret
    pub name: String,
}

/// Output for rotate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateOutput {
    /// The ARN of the secret
    pub arn: String,
    /// The friendly name of the secret
    pub name: String,
    /// The unique identifier of the new version of the secret
    pub version_id: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for tag_resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for untag_resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntagOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for update_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// The friendly name of the secret
    pub name: String,
    /// The ARN of the secret
    pub arn: String,
    /// The unique identifier of the version of the secret
    pub version_id: String,
}

/// Output for validate_resource_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    /// Whether the secret is scheduled for deletion
    pub scheduled_for_deletion: bool,
    /// Whether the secret exists
    pub exists: bool,
    /// Whether the secret is valid and accessible
    pub valid: bool,
}

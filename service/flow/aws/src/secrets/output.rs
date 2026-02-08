//! Output types for AWS Secrets Manager actions
//!
//! This module contains all the output structs and helper types used by the AWS Secrets Manager actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRotateOutput {
        pub arn: String,
        pub name: String,
        pub version_id: String,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
        pub success: bool,
        pub arn: String,
        pub version_id: String,
        pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
        pub arn: String,
        pub deletion_date: i64,
        pub name: String,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeOutput {
        pub arn: String,
        pub kms_key_id: String,
        pub last_rotated_date: i64,
        pub last_changed_date: i64,
        pub description: String,
        pub rotation_enabled: bool,
        pub name: String,
        pub tags: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValueOutput {
        pub version_id: String,
        pub name: String,
        pub secret_value: String,
        pub created_date: i64,
        pub arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
        pub next_token: String,
        pub secrets: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutValueOutput {
        pub version_id: String,
        pub arn: String,
        pub name: String,
        pub version_stages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRegionsFromReplicationOutput {
        pub arn: String,
        pub success: bool,
        pub replication_status: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicateOutput {
        pub arn: String,
        pub replication_status: Vec<HashMap<String, Value>>,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOutput {
        pub arn: String,
        pub success: bool,
        pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateOutput {
        pub arn: String,
        pub name: String,
        pub version_id: String,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntagOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
        pub success: bool,
        pub name: String,
        pub arn: String,
        pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
        pub scheduled_for_deletion: bool,
        pub exists: bool,
        pub valid: bool,
}

// Harana Actions - Aws Secrets Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// cancel_rotate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRotateOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
    pub success: bool
}

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub success: bool,
    pub arn: String,
    pub version_id: String,
    pub name: String
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub arn: String,
    pub deletion_date: i32,
    pub name: String,
    pub success: bool
}

// describe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeOutput {
    pub arn: String,
    pub kms_key_id: String,
    pub last_rotated_date: i32,
    pub last_changed_date: i32,
    pub description: String,
    pub rotation_enabled: bool,
    pub name: String,
    pub tags: HashMap<String, Value>
}

// get_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValueOutput {
    pub version_id: String,
    pub name: String,
    pub secret_value: String,
    pub created_date: i32,
    pub arn: String
}

// lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
    pub next_token: String,
    pub secrets: Vec<HashMap<String, Value>>
}

// put_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutValueOutput {
    pub version_id: String,
    pub arn: String,
    pub name: String,
    pub version_stages: Vec<String>
}

// remove_regions_from_replication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRegionsFromReplicationOutput {
    pub arn: String,
    pub success: bool,
    pub replication_status: Vec<HashMap<String, Value>>
}

// replicate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicateOutput {
    pub arn: String,
    pub replication_status: Vec<HashMap<String, Value>>,
    pub success: bool
}

// restore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOutput {
    pub arn: String,
    pub success: bool,
    pub name: String
}

// rotate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
    pub success: bool
}

// tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagOutput {
    pub success: bool
}

// untag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntagOutput {
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool,
    pub name: String,
    pub arn: String,
    pub version_id: String
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub scheduled_for_deletion: bool,
    pub exists: bool,
    pub valid: bool
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInput {
    pub description: String,
    pub kms_key_id: String,
    pub name: String,
    pub secret_value: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOutput {
    pub arn: String,
    pub name: String,
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetValueInput {
    pub secret_id: String,
    pub version_id: String,
    pub version_stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetValueOutput {
    pub arn: String,
    pub created_date: i64,
    pub name: String,
    pub secret_value: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateInput {
    pub description: String,
    pub kms_key_id: String,
    pub secret_id: String,
    pub secret_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOutput {
    pub arn: String,
    pub name: String,
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    #[serde(default)]
    pub force_delete: bool,
    pub recovery_window_days: i64,
    pub secret_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub arn: String,
    pub deletion_date: i64,
    pub name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreInput {
    pub secret_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreOutput {
    pub arn: String,
    pub name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsInput {
    pub filters: Vec<String>,
    pub max_results: i64,
    pub sort_order: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsOutput {
    pub next_token: String,
    pub secrets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInput {
    pub secret_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOutput {
    pub arn: String,
    pub description: String,
    pub kms_key_id: String,
    pub last_changed_date: i64,
    pub last_rotated_date: i64,
    pub name: String,
    pub rotation_enabled: bool,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutValueInput {
    pub secret_id: String,
    pub secret_value: String,
    pub version_stages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutValueOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
    pub version_stages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateInput {
    pub rotation_lambda_arn: String,
    pub rotation_rules: String,
    pub secret_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateOutput {
    pub arn: String,
    pub name: String,
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelRotateInput {
    pub secret_id: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelRotateOutput {
    pub arn: String,
    pub name: String,
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagInput {
    pub secret_id: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntagInput {
    pub secret_id: String,
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntagOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplicateInput {
    pub kms_key_ids: String,
    pub regions: Vec<String>,
    pub secret_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplicateOutput {
    pub arn: String,
    pub replication_status: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveRegionsFromReplicationInput {
    pub regions: Vec<String>,
    pub secret_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveRegionsFromReplicationOutput {
    pub arn: String,
    pub replication_status: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateInput {
    pub secret_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub exists: bool,
    pub scheduled_for_deletion: bool,
    pub valid: bool,
}

#[async_trait]
pub trait SecretsAction {
    async fn create(&self, input: CreateInput) -> Result<CreateOutput, Box<dyn std::error::Error>>;
    async fn get_value(&self, input: GetValueInput) -> Result<GetValueOutput, Box<dyn std::error::Error>>;
    async fn update(&self, input: UpdateInput) -> Result<UpdateOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn restore(&self, input: RestoreInput) -> Result<RestoreOutput, Box<dyn std::error::Error>>;
    async fn lists(&self, input: ListsInput) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn describe(&self, input: DescribeInput) -> Result<DescribeOutput, Box<dyn std::error::Error>>;
    async fn put_value(&self, input: PutValueInput) -> Result<PutValueOutput, Box<dyn std::error::Error>>;
    async fn rotate(&self, input: RotateInput) -> Result<RotateOutput, Box<dyn std::error::Error>>;
    async fn cancel_rotate(&self, input: CancelRotateInput) -> Result<CancelRotateOutput, Box<dyn std::error::Error>>;
    async fn tag(&self, input: TagInput) -> Result<TagOutput, Box<dyn std::error::Error>>;
    async fn untag(&self, input: UntagInput) -> Result<UntagOutput, Box<dyn std::error::Error>>;
    async fn replicate(&self, input: ReplicateInput) -> Result<ReplicateOutput, Box<dyn std::error::Error>>;
    async fn remove_regions_from_replication(&self, input: RemoveRegionsFromReplicationInput) -> Result<RemoveRegionsFromReplicationOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}

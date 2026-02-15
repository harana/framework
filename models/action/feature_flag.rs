// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFlagInput {
    pub default_variation: i64,
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    pub key: String,
    pub name: String,
    pub tags: Vec<String>,
    pub variations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFlagOutput {
    pub flag_id: String,
    pub key: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFlagInput {
    pub description: String,
    pub enabled: bool,
    pub flag_id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub variations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFlagOutput {
    pub flag_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToggleFlagInput {
    pub enabled: bool,
    pub flag_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToggleFlagOutput {
    pub enabled: bool,
    pub flag_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFlagInput {
    pub flag_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFlagOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFlagInput {
    pub flag_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFlagOutput {
    pub created_at: i64,
    pub description: String,
    pub enabled: bool,
    pub flag_id: String,
    pub key: String,
    pub name: String,
    pub updated_at: i64,
    pub variations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFlagsInput {
    pub enabled: bool,
    pub limit: i64,
    pub offset: i64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFlagsOutput {
    pub flags: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluateFlagInput {
    pub context: String,
    pub flag_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluateFlagOutput {
    pub enabled: bool,
    pub flag_id: String,
    pub reason: String,
    pub variation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTargetingRuleInput {
    pub conditions: Vec<String>,
    pub flag_id: String,
    pub name: String,
    pub priority: i64,
    pub variation: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTargetingRuleOutput {
    pub flag_id: String,
    pub rule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTargetingRuleInput {
    pub conditions: Vec<String>,
    pub name: String,
    pub priority: i64,
    pub rule_id: String,
    pub variation: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTargetingRuleOutput {
    pub rule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTargetingRuleInput {
    pub rule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTargetingRuleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRolloutInput {
    pub flag_id: String,
    pub percentages: std::collections::HashMap<String, String>,
    pub seed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRolloutOutput {
    pub flag_id: String,
    pub rollout_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateRolloutInput {
    pub percentages: std::collections::HashMap<String, String>,
    pub rollout_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateRolloutOutput {
    pub rollout_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEvaluationCountInput {
    pub end_date: i64,
    pub flag_id: String,
    pub start_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEvaluationCountOutput {
    pub flag_id: String,
    pub total_evaluations: i64,
    pub variation_counts: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEnvironmentInput {
    pub description: String,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEnvironmentOutput {
    pub environment_id: String,
    pub key: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloneFlagInput {
    pub flag_id: String,
    pub source_environment: String,
    pub target_environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloneFlagOutput {
    pub flag_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveFlagInput {
    pub flag_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveFlagOutput {
    pub archived: bool,
    pub flag_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreFlagInput {
    pub flag_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreFlagOutput {
    pub archived: bool,
    pub flag_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlag {
    pub flag_id: String,
    pub key: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub variations: Vec<String>,
    pub default_variation: i64,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagVariation {
    pub name: String,
    pub value: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagEvaluationContext {
    pub user_id: String,
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagRuleCondition {
    pub attribute: String,
    pub operator: String,
    pub value: String,
}

#[async_trait]
pub trait FeatureFlagAction {
    async fn create_flag(&self, input: CreateFlagInput) -> Result<CreateFlagOutput, Box<dyn std::error::Error>>;
    async fn update_flag(&self, input: UpdateFlagInput) -> Result<UpdateFlagOutput, Box<dyn std::error::Error>>;
    async fn toggle_flag(&self, input: ToggleFlagInput) -> Result<ToggleFlagOutput, Box<dyn std::error::Error>>;
    async fn delete_flag(&self, input: DeleteFlagInput) -> Result<DeleteFlagOutput, Box<dyn std::error::Error>>;
    async fn get_flag(&self, input: GetFlagInput) -> Result<GetFlagOutput, Box<dyn std::error::Error>>;
    async fn list_flags(&self, input: ListFlagsInput) -> Result<ListFlagsOutput, Box<dyn std::error::Error>>;
    async fn evaluate_flag(&self, input: EvaluateFlagInput) -> Result<EvaluateFlagOutput, Box<dyn std::error::Error>>;
    async fn create_targeting_rule(&self, input: CreateTargetingRuleInput) -> Result<CreateTargetingRuleOutput, Box<dyn std::error::Error>>;
    async fn update_targeting_rule(&self, input: UpdateTargetingRuleInput) -> Result<UpdateTargetingRuleOutput, Box<dyn std::error::Error>>;
    async fn delete_targeting_rule(&self, input: DeleteTargetingRuleInput) -> Result<DeleteTargetingRuleOutput, Box<dyn std::error::Error>>;
    async fn create_rollout(&self, input: CreateRolloutInput) -> Result<CreateRolloutOutput, Box<dyn std::error::Error>>;
    async fn update_rollout(&self, input: UpdateRolloutInput) -> Result<UpdateRolloutOutput, Box<dyn std::error::Error>>;
    async fn get_evaluation_count(&self, input: GetEvaluationCountInput) -> Result<GetEvaluationCountOutput, Box<dyn std::error::Error>>;
    async fn create_environment(&self, input: CreateEnvironmentInput) -> Result<CreateEnvironmentOutput, Box<dyn std::error::Error>>;
    async fn clone_flag(&self, input: CloneFlagInput) -> Result<CloneFlagOutput, Box<dyn std::error::Error>>;
    async fn archive_flag(&self, input: ArchiveFlagInput) -> Result<ArchiveFlagOutput, Box<dyn std::error::Error>>;
    async fn restore_flag(&self, input: RestoreFlagInput) -> Result<RestoreFlagOutput, Box<dyn std::error::Error>>;
}

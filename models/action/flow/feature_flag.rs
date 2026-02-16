// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFlagOutput {
    pub flag_id: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToggleFlagOutput {
    pub enabled: bool,
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
pub struct ListFlagsOutput {
    pub flags: Vec<String>,
    pub total: i64,
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
pub struct CreateTargetingRuleOutput {
    pub flag_id: String,
    pub rule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRolloutOutput {
    pub flag_id: String,
    pub rollout_id: String,
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
pub struct CreateEnvironmentOutput {
    pub environment_id: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveFlagOutput {
    pub archived: bool,
    pub flag_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreFlagOutput {
    pub archived: bool,
    pub flag_id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagVariation {
    pub description: String,
    pub flag_id: String,
    pub sort_order: i64,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagTargetingRule {
    pub conditions: String,
    pub flag_id: String,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub priority: i64,
    pub variation_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagEvaluationLog {
    #[serde(default = "chrono::Utc::now")]
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
    pub flag_id: String,
    pub reason: String,
    pub user_id: String,
    pub variation_index: i64,
}

#[async_trait]
pub trait FeatureFlagAction {
    async fn create_flag(&self, default_variation: i64, description: String, enabled: bool, key: String, name: String, tags: Vec<String>, variations: Vec<String>) -> Result<CreateFlagOutput, Box<dyn std::error::Error>>;
    async fn update_flag(&self, description: String, enabled: bool, flag_id: String, name: String, tags: Vec<String>, variations: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn toggle_flag(&self, enabled: bool, flag_id: String) -> Result<ToggleFlagOutput, Box<dyn std::error::Error>>;
    async fn delete_flag(&self, flag_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_flag(&self, flag_id: String) -> Result<GetFlagOutput, Box<dyn std::error::Error>>;
    async fn list_flags(&self, enabled: bool, limit: i64, offset: i64, tags: Vec<String>) -> Result<ListFlagsOutput, Box<dyn std::error::Error>>;
    async fn evaluate_flag(&self, context: String, flag_id: String, user_id: String) -> Result<EvaluateFlagOutput, Box<dyn std::error::Error>>;
    async fn create_targeting_rule(&self, conditions: Vec<String>, flag_id: String, name: String, priority: i64, variation: i64) -> Result<CreateTargetingRuleOutput, Box<dyn std::error::Error>>;
    async fn update_targeting_rule(&self, conditions: Vec<String>, name: String, priority: i64, rule_id: String, variation: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_targeting_rule(&self, rule_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_rollout(&self, flag_id: String, percentages: std::collections::HashMap<String, String>, seed: String) -> Result<CreateRolloutOutput, Box<dyn std::error::Error>>;
    async fn update_rollout(&self, percentages: std::collections::HashMap<String, String>, rollout_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_evaluation_count(&self, end_date: i64, flag_id: String, start_date: i64) -> Result<GetEvaluationCountOutput, Box<dyn std::error::Error>>;
    async fn create_environment(&self, description: String, key: String, name: String) -> Result<CreateEnvironmentOutput, Box<dyn std::error::Error>>;
    async fn clone_flag(&self, flag_id: String, source_environment: String, target_environment: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn archive_flag(&self, flag_id: String) -> Result<ArchiveFlagOutput, Box<dyn std::error::Error>>;
    async fn restore_flag(&self, flag_id: String) -> Result<RestoreFlagOutput, Box<dyn std::error::Error>>;
}

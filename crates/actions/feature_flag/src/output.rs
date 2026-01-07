// Harana Actions - Feature Flag Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// archive_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveFlagOutput {
    pub success: bool,
    pub archived: bool,
    pub flag_id: String
}

// clone_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneFlagOutput {
    pub success: bool,
    pub flag_id: String
}

// create_environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEnvironmentOutput {
    pub success: bool,
    pub environment_id: String,
    pub key: String
}

// create_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFlagOutput {
    pub flag_id: String,
    pub key: String,
    pub success: bool
}

// create_rollout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRolloutOutput {
    pub flag_id: String,
    pub rollout_id: String,
    pub success: bool
}

// create_targeting_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTargetingRuleOutput {
    pub flag_id: String,
    pub success: bool,
    pub rule_id: String
}

// delete_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFlagOutput {
    pub success: bool
}

// delete_targeting_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTargetingRuleOutput {
    pub success: bool
}

// evaluate_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateFlagOutput {
    pub variation: HashMap<String, Value>,
    pub enabled: bool,
    pub reason: String,
    pub flag_id: String
}

// get_evaluation_count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvaluationCountOutput {
    pub variation_counts: HashMap<String, Value>,
    pub flag_id: String,
    pub total_evaluations: i32
}

// get_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFlagOutput {
    pub updated_at: i32,
    pub enabled: bool,
    pub name: String,
    pub variations: Vec<HashMap<String, Value>>,
    pub description: String,
    pub created_at: i32,
    pub key: String,
    pub flag_id: String
}

// list_flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFlagsOutput {
    pub flags: Vec<HashMap<String, Value>>,
    pub total: i32
}

// restore_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreFlagOutput {
    pub archived: bool,
    pub success: bool,
    pub flag_id: String
}

// toggle_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleFlagOutput {
    pub enabled: bool,
    pub success: bool,
    pub flag_id: String
}

// update_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFlagOutput {
    pub flag_id: String,
    pub success: bool
}

// update_rollout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRolloutOutput {
    pub success: bool,
    pub rollout_id: String
}

// update_targeting_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTargetingRuleOutput {
    pub success: bool,
    pub rule_id: String
}

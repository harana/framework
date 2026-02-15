// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// feature_flag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlag {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_variation: i64,
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub key: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl FeatureFlag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// feature_flag_variation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagVariation {
    pub description: Option<String>,
    /// Reference: feature_flag.id
    pub flag_id: String,
    pub sort_order: i64,
    pub value: String,
}

impl FeatureFlagVariation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// feature_flag_targeting_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagTargetingRule {
    pub conditions: String,
    /// Reference: feature_flag.id
    pub flag_id: String,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub priority: i64,
    pub variation_index: i64,
}

impl FeatureFlagTargetingRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// feature_flag_evaluation_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagEvaluationLog {
    #[serde(default = "chrono::Utc::now")]
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: feature_flag.id
    pub flag_id: String,
    pub reason: Option<String>,
    pub user_id: Option<String>,
    pub variation_index: i64,
}

impl FeatureFlagEvaluationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// flag_variation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagVariation {
    pub name: String,
    pub value: String,
    pub description: String,
}

impl FlagVariation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// flag_evaluation_context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagEvaluationContext {
    pub user_id: String,
    pub attributes: String,
}

impl FlagEvaluationContext {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// flag_rule_condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagRuleCondition {
    pub attribute: String,
    pub operator: String,
    pub value: String,
}

impl FlagRuleCondition {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


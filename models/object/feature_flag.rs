// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagVariation {
    pub description: Option<String>,
    pub flag_id: String,
    pub sort_order: i64,
    pub value: String,
}

impl FeatureFlagVariation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl FeatureFlagTargetingRule {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagEvaluationLog {
    #[serde(default = "chrono::Utc::now")]
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
    pub flag_id: String,
    pub reason: Option<String>,
    pub user_id: Option<String>,
    pub variation_index: i64,
}

impl FeatureFlagEvaluationLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagVariation {
    pub name: String,
    pub value: String,
    pub description: String,
}

impl FlagVariation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagEvaluationContext {
    pub user_id: String,
    pub attributes: std::collections::HashMap<String, String>,
}

impl FlagEvaluationContext {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlagRuleCondition {
    pub attribute: String,
    pub operator: String,
    pub value: String,
}

impl FlagRuleCondition {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


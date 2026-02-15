// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_eventbridge_event_bus
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeEventBus {
    pub account_id: String,
    pub arn: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub policy: Option<String>,
    pub region: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEventbridgeEventBus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_eventbridge_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeRule {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    /// Reference: aws_eventbridge_event_bus.id
    pub event_bus_id: String,
    pub event_pattern: Option<String>,
    pub name: String,
    pub role_arn: Option<String>,
    pub rule_arn: Option<String>,
    pub schedule_expression: Option<String>,
    pub state: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEventbridgeRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_eventbridge_target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeTarget {
    pub arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub input: Option<String>,
    pub input_path: Option<String>,
    pub input_transformer: Option<String>,
    pub role_arn: Option<String>,
    /// Reference: aws_eventbridge_rule.id
    pub rule_id: String,
    pub target_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEventbridgeTarget {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_eventbridge_archive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeArchive {
    pub account_id: String,
    pub archive_arn: Option<String>,
    pub archive_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub event_count: i64,
    pub event_pattern: Option<String>,
    pub event_source_arn: String,
    pub region: Option<String>,
    pub retention_days: Option<i64>,
    pub size_bytes: i64,
    pub state: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEventbridgeArchive {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_eventbridge_replay
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeReplay {
    pub account_id: String,
    pub description: Option<String>,
    pub event_end_time: chrono::DateTime<chrono::Utc>,
    pub event_source_arn: String,
    pub event_start_time: chrono::DateTime<chrono::Utc>,
    pub region: Option<String>,
    pub replay_arn: Option<String>,
    pub replay_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub replay_name: String,
    pub replay_start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub state: String,
    pub state_reason: Option<String>,
}

impl AwsEventbridgeReplay {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


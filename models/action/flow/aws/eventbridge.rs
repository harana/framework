// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutEventsOutput {
    pub failed_entry_count: i64,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRuleOutput {
    pub name: String,
    pub arn: String,
    pub event_pattern: String,
    pub schedule_expression: String,
    pub state: String,
    pub description: String,
    pub role_arn: String,
    pub event_bus_name: String,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRulesOutput {
    pub rules: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutTargetsOutput {
    pub failed_entry_count: i64,
    pub failed_entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTargetsOutput {
    pub failed_entry_count: i64,
    pub failed_entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTargetsByRuleOutput {
    pub targets: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeEventBusOutput {
    pub name: String,
    pub arn: String,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEventBusesOutput {
    pub event_buses: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateArchiveOutput {
    pub archive_arn: String,
    pub state: String,
    pub state_reason: String,
    pub creation_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeArchiveOutput {
    pub archive_arn: String,
    pub archive_name: String,
    pub event_source_arn: String,
    pub description: String,
    pub event_pattern: String,
    pub state: String,
    pub retention_days: i64,
    pub size_bytes: i64,
    pub event_count: i64,
    pub creation_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartReplayOutput {
    pub replay_arn: String,
    pub state: String,
    pub state_reason: String,
    pub replay_start_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelReplayOutput {
    pub state: String,
    pub state_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeReplayOutput {
    pub replay_name: String,
    pub replay_arn: String,
    pub description: String,
    pub state: String,
    pub state_reason: String,
    pub event_source_arn: String,
    pub destination: String,
    pub event_start_time: chrono::DateTime<chrono::Utc>,
    pub event_end_time: chrono::DateTime<chrono::Utc>,
    pub replay_start_time: chrono::DateTime<chrono::Utc>,
    pub replay_end_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeEventBus {
    pub account_id: String,
    pub arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub policy: String,
    pub region: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeRule {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub event_bus_id: String,
    pub event_pattern: String,
    pub name: String,
    pub role_arn: String,
    pub rule_arn: String,
    pub schedule_expression: String,
    pub state: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeTarget {
    pub arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub input: String,
    pub input_path: String,
    pub input_transformer: String,
    pub role_arn: String,
    pub rule_id: String,
    pub target_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeArchive {
    pub account_id: String,
    pub archive_arn: String,
    pub archive_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub event_count: i64,
    pub event_pattern: String,
    pub event_source_arn: String,
    pub region: String,
    pub retention_days: i64,
    pub size_bytes: i64,
    pub state: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEventbridgeReplay {
    pub account_id: String,
    pub description: String,
    pub event_end_time: chrono::DateTime<chrono::Utc>,
    pub event_source_arn: String,
    pub event_start_time: chrono::DateTime<chrono::Utc>,
    pub region: String,
    pub replay_arn: String,
    pub replay_end_time: chrono::DateTime<chrono::Utc>,
    pub replay_name: String,
    pub replay_start_time: chrono::DateTime<chrono::Utc>,
    pub state: String,
    pub state_reason: String,
}

#[async_trait]
pub trait EventbridgeAction {
    async fn put_events(&self, entries: Vec<String>, endpoint_id: String) -> Result<PutEventsOutput, Box<dyn std::error::Error>>;
    async fn create_rule(&self, name: String, event_bus_name: String, schedule_expression: String, event_pattern: String, state: String, description: String, role_arn: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_rule(&self, name: String, event_bus_name: String, force: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_rule(&self, name: String, event_bus_name: String) -> Result<DescribeRuleOutput, Box<dyn std::error::Error>>;
    async fn list_rules(&self, event_bus_name: String, name_prefix: String, limit: i64, next_token: String) -> Result<ListRulesOutput, Box<dyn std::error::Error>>;
    async fn enable_rule(&self, name: String, event_bus_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn disable_rule(&self, name: String, event_bus_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn put_targets(&self, rule: String, event_bus_name: String, targets: Vec<String>) -> Result<PutTargetsOutput, Box<dyn std::error::Error>>;
    async fn remove_targets(&self, rule: String, event_bus_name: String, ids: Vec<String>, force: bool) -> Result<RemoveTargetsOutput, Box<dyn std::error::Error>>;
    async fn list_targets_by_rule(&self, rule: String, event_bus_name: String, limit: i64, next_token: String) -> Result<ListTargetsByRuleOutput, Box<dyn std::error::Error>>;
    async fn create_event_bus(&self, name: String, event_source_name: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_event_bus(&self, name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_event_bus(&self, name: String) -> Result<DescribeEventBusOutput, Box<dyn std::error::Error>>;
    async fn list_event_buses(&self, name_prefix: String, limit: i64, next_token: String) -> Result<ListEventBusesOutput, Box<dyn std::error::Error>>;
    async fn put_permission(&self, event_bus_name: String, method: String, principal: String, statement_id: String, condition: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_permission(&self, event_bus_name: String, statement_id: String, remove_all_permissions: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_archive(&self, archive_name: String, event_source_arn: String, description: String, event_pattern: String, retention_days: i64) -> Result<CreateArchiveOutput, Box<dyn std::error::Error>>;
    async fn delete_archive(&self, archive_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_archive(&self, archive_name: String) -> Result<DescribeArchiveOutput, Box<dyn std::error::Error>>;
    async fn start_replay(&self, replay_name: String, event_source_arn: String, destination: String, event_start_time: chrono::DateTime<chrono::Utc>, event_end_time: chrono::DateTime<chrono::Utc>, description: String) -> Result<StartReplayOutput, Box<dyn std::error::Error>>;
    async fn cancel_replay(&self, replay_name: String) -> Result<CancelReplayOutput, Box<dyn std::error::Error>>;
    async fn describe_replay(&self, replay_name: String) -> Result<DescribeReplayOutput, Box<dyn std::error::Error>>;
    async fn test_event_pattern(&self, event_pattern: String, event: String) -> Result<bool, Box<dyn std::error::Error>>;
}

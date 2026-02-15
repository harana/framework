// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutEventsInput {
    pub entries: Vec<String>,
    pub endpoint_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutEventsOutput {
    pub failed_entry_count: i64,
    pub entries: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRuleInput {
    pub name: String,
    pub event_bus_name: String,
    pub schedule_expression: String,
    pub event_pattern: String,
    pub state: String,
    pub description: String,
    pub role_arn: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRuleOutput {
    pub rule_arn: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRuleInput {
    pub name: String,
    pub event_bus_name: String,
    #[serde(default)]
    pub force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRuleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRuleInput {
    pub name: String,
    pub event_bus_name: String,
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
pub struct ListRulesInput {
    pub event_bus_name: String,
    pub name_prefix: String,
    pub limit: i64,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListRulesOutput {
    pub rules: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableRuleInput {
    pub name: String,
    pub event_bus_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableRuleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableRuleInput {
    pub name: String,
    pub event_bus_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableRuleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutTargetsInput {
    pub rule: String,
    pub event_bus_name: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutTargetsOutput {
    pub failed_entry_count: i64,
    pub failed_entries: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTargetsInput {
    pub rule: String,
    pub event_bus_name: String,
    pub ids: Vec<String>,
    #[serde(default)]
    pub force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTargetsOutput {
    pub failed_entry_count: i64,
    pub failed_entries: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTargetsByRuleInput {
    pub rule: String,
    pub event_bus_name: String,
    pub limit: i64,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTargetsByRuleOutput {
    pub targets: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEventBusInput {
    pub name: String,
    pub event_source_name: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEventBusOutput {
    pub event_bus_arn: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEventBusInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEventBusOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeEventBusInput {
    pub name: String,
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
pub struct ListEventBusesInput {
    pub name_prefix: String,
    pub limit: i64,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEventBusesOutput {
    pub event_buses: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutPermissionInput {
    pub event_bus_name: String,
    pub method: String,
    pub principal: String,
    pub statement_id: String,
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutPermissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemovePermissionInput {
    pub event_bus_name: String,
    pub statement_id: String,
    #[serde(default)]
    pub remove_all_permissions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemovePermissionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateArchiveInput {
    pub archive_name: String,
    pub event_source_arn: String,
    pub description: String,
    pub event_pattern: String,
    pub retention_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateArchiveOutput {
    pub archive_arn: String,
    pub state: String,
    pub state_reason: String,
    pub creation_time: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteArchiveInput {
    pub archive_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteArchiveOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeArchiveInput {
    pub archive_name: String,
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
pub struct StartReplayInput {
    pub replay_name: String,
    pub event_source_arn: String,
    pub destination: String,
    pub event_start_time: chrono::DateTime<chrono::Utc>,
    pub event_end_time: chrono::DateTime<chrono::Utc>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartReplayOutput {
    pub replay_arn: String,
    pub state: String,
    pub state_reason: String,
    pub replay_start_time: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelReplayInput {
    pub replay_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelReplayOutput {
    pub state: String,
    pub state_reason: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeReplayInput {
    pub replay_name: String,
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
pub struct TestEventPatternInput {
    pub event_pattern: String,
    pub event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestEventPatternOutput {
    pub result: bool,
}

#[async_trait]
pub trait EventbridgeAction {
    async fn put_events(&self, input: PutEventsInput) -> Result<PutEventsOutput, Box<dyn std::error::Error>>;
    async fn create_rule(&self, input: CreateRuleInput) -> Result<CreateRuleOutput, Box<dyn std::error::Error>>;
    async fn delete_rule(&self, input: DeleteRuleInput) -> Result<DeleteRuleOutput, Box<dyn std::error::Error>>;
    async fn describe_rule(&self, input: DescribeRuleInput) -> Result<DescribeRuleOutput, Box<dyn std::error::Error>>;
    async fn list_rules(&self, input: ListRulesInput) -> Result<ListRulesOutput, Box<dyn std::error::Error>>;
    async fn enable_rule(&self, input: EnableRuleInput) -> Result<EnableRuleOutput, Box<dyn std::error::Error>>;
    async fn disable_rule(&self, input: DisableRuleInput) -> Result<DisableRuleOutput, Box<dyn std::error::Error>>;
    async fn put_targets(&self, input: PutTargetsInput) -> Result<PutTargetsOutput, Box<dyn std::error::Error>>;
    async fn remove_targets(&self, input: RemoveTargetsInput) -> Result<RemoveTargetsOutput, Box<dyn std::error::Error>>;
    async fn list_targets_by_rule(&self, input: ListTargetsByRuleInput) -> Result<ListTargetsByRuleOutput, Box<dyn std::error::Error>>;
    async fn create_event_bus(&self, input: CreateEventBusInput) -> Result<CreateEventBusOutput, Box<dyn std::error::Error>>;
    async fn delete_event_bus(&self, input: DeleteEventBusInput) -> Result<DeleteEventBusOutput, Box<dyn std::error::Error>>;
    async fn describe_event_bus(&self, input: DescribeEventBusInput) -> Result<DescribeEventBusOutput, Box<dyn std::error::Error>>;
    async fn list_event_buses(&self, input: ListEventBusesInput) -> Result<ListEventBusesOutput, Box<dyn std::error::Error>>;
    async fn put_permission(&self, input: PutPermissionInput) -> Result<PutPermissionOutput, Box<dyn std::error::Error>>;
    async fn remove_permission(&self, input: RemovePermissionInput) -> Result<RemovePermissionOutput, Box<dyn std::error::Error>>;
    async fn create_archive(&self, input: CreateArchiveInput) -> Result<CreateArchiveOutput, Box<dyn std::error::Error>>;
    async fn delete_archive(&self, input: DeleteArchiveInput) -> Result<DeleteArchiveOutput, Box<dyn std::error::Error>>;
    async fn describe_archive(&self, input: DescribeArchiveInput) -> Result<DescribeArchiveOutput, Box<dyn std::error::Error>>;
    async fn start_replay(&self, input: StartReplayInput) -> Result<StartReplayOutput, Box<dyn std::error::Error>>;
    async fn cancel_replay(&self, input: CancelReplayInput) -> Result<CancelReplayOutput, Box<dyn std::error::Error>>;
    async fn describe_replay(&self, input: DescribeReplayInput) -> Result<DescribeReplayOutput, Box<dyn std::error::Error>>;
    async fn test_event_pattern(&self, input: TestEventPatternInput) -> Result<TestEventPatternOutput, Box<dyn std::error::Error>>;
}

// Harana Actions - AWS EventBridge Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// put_events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutEventsOutput {
    pub failed_entry_count: i32,
    pub entries: Vec<PutEventsResultEntry>,
    pub success: bool,
}

// create_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRuleOutput {
    pub rule_arn: String,
    pub success: bool,
}

// delete_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRuleOutput {
    pub success: bool,
}

// describe_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeRuleOutput {
    pub name: String,
    pub arn: String,
    pub event_pattern: Option<String>,
    pub schedule_expression: Option<String>,
    pub state: String,
    pub description: Option<String>,
    pub role_arn: Option<String>,
    pub event_bus_name: String,
    pub created_by: Option<String>,
}

// list_rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRulesOutput {
    pub rules: Vec<EventBridgeRule>,
    pub next_token: Option<String>,
}

// enable_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnableRuleOutput {
    pub success: bool,
}

// disable_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableRuleOutput {
    pub success: bool,
}

// put_targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutTargetsOutput {
    pub failed_entry_count: i32,
    pub failed_entries: Vec<TargetFailure>,
    pub success: bool,
}

// remove_targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTargetsOutput {
    pub failed_entry_count: i32,
    pub failed_entries: Vec<TargetFailure>,
    pub success: bool,
}

// list_targets_by_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTargetsByRuleOutput {
    pub targets: Vec<EventBridgeTarget>,
    pub next_token: Option<String>,
}

// create_event_bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventBusOutput {
    pub event_bus_arn: String,
    pub success: bool,
}

// delete_event_bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEventBusOutput {
    pub success: bool,
}

// describe_event_bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeEventBusOutput {
    pub name: String,
    pub arn: String,
    pub policy: Option<String>,
}

// list_event_buses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEventBusesOutput {
    pub event_buses: Vec<EventBus>,
    pub next_token: Option<String>,
}

// put_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutPermissionOutput {
    pub success: bool,
}

// remove_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePermissionOutput {
    pub success: bool,
}

// create_archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArchiveOutput {
    pub archive_arn: String,
    pub state: String,
    pub state_reason: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub success: bool,
}

// delete_archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteArchiveOutput {
    pub success: bool,
}

// describe_archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeArchiveOutput {
    pub archive_arn: String,
    pub archive_name: String,
    pub event_source_arn: String,
    pub description: Option<String>,
    pub event_pattern: Option<String>,
    pub state: String,
    pub retention_days: Option<i32>,
    pub size_bytes: Option<i64>,
    pub event_count: Option<i64>,
    pub creation_time: DateTime<Utc>,
}

// start_replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartReplayOutput {
    pub replay_arn: String,
    pub state: String,
    pub state_reason: Option<String>,
    pub replay_start_time: DateTime<Utc>,
    pub success: bool,
}

// cancel_replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelReplayOutput {
    pub state: String,
    pub state_reason: Option<String>,
    pub success: bool,
}

// describe_replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeReplayOutput {
    pub replay_name: String,
    pub replay_arn: String,
    pub description: Option<String>,
    pub state: String,
    pub state_reason: Option<String>,
    pub event_source_arn: String,
    pub destination: EventDestination,
    pub event_start_time: DateTime<Utc>,
    pub event_end_time: DateTime<Utc>,
    pub replay_start_time: Option<DateTime<Utc>>,
    pub replay_end_time: Option<DateTime<Utc>>,
}

// test_event_pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEventPatternOutput {
    pub result: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBridgeEntry {
    pub time: Option<DateTime<Utc>>,
    pub source: String,
    pub resources: Option<Vec<String>>,
    pub detail_type: String,
    pub detail: String,
    pub event_bus_name: Option<String>,
    pub trace_header: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutEventsResultEntry {
    pub event_id: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBridgeRule {
    pub name: String,
    pub arn: String,
    pub event_pattern: Option<String>,
    pub state: String,
    pub description: Option<String>,
    pub schedule_expression: Option<String>,
    pub role_arn: Option<String>,
    pub managed_by: Option<String>,
    pub event_bus_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBridgeTarget {
    pub id: String,
    pub arn: String,
    pub role_arn: Option<String>,
    pub input: Option<String>,
    pub input_path: Option<String>,
    pub input_transformer: Option<InputTransformer>,
    pub kinesis_parameters: Option<Value>,
    pub run_command_parameters: Option<Value>,
    pub ecs_parameters: Option<Value>,
    pub batch_parameters: Option<Value>,
    pub sqs_parameters: Option<Value>,
    pub http_parameters: Option<Value>,
    pub redshift_data_parameters: Option<Value>,
    pub sage_maker_pipeline_parameters: Option<Value>,
    pub dead_letter_config: Option<DeadLetterConfig>,
    pub retry_policy: Option<RetryPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTransformer {
    pub input_paths_map: Option<HashMap<String, String>>,
    pub input_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterConfig {
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub maximum_retry_attempts: Option<i32>,
    pub maximum_event_age_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetFailure {
    pub target_id: String,
    pub error_code: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBus {
    pub name: String,
    pub arn: String,
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub condition_type: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDestination {
    pub arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBridgeTags {
    pub tags: HashMap<String, String>,
}

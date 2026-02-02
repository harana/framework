//! Harana Actions - AWS EventBridge Module
//!
//! This module provides AWS EventBridge actions.

pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_eventbridge::{
    config::Region,
    types::{
        PutEventsRequestEntry, RuleState, Tag, Target,
        InputTransformer as AwsInputTransformer,
        DeadLetterConfig as AwsDeadLetterConfig,
        RetryPolicy as AwsRetryPolicy,
        ReplayDestination,
        Condition,
    },
    Client,
};
use chrono::{DateTime, Utc};
use output::*;

/// Creates an EventBridge client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    
    let eventbridge_config = if let Some(region_str) = region {
        aws_sdk_eventbridge::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_eventbridge::config::Builder::from(&config).build()
    };
    
    Ok(Client::from_conf(eventbridge_config))
}

/// Converts EventBridgeTags to AWS SDK Tag format
fn tags_to_aws_tags(tags: &EventBridgeTags) -> Vec<Tag> {
    tags.tags
        .iter()
        .map(|(k, v)| {
            Tag::builder()
                .key(k)
                .value(v)
                .build()
                .expect("Failed to build tag")
        })
        .collect()
}

/// Converts EventBridgeEntry to AWS SDK PutEventsRequestEntry format
fn entry_to_aws_entry(entry: &EventBridgeEntry) -> PutEventsRequestEntry {
    let mut builder = PutEventsRequestEntry::builder()
        .source(&entry.source)
        .detail_type(&entry.detail_type)
        .detail(&entry.detail);
    
    if let Some(time) = entry.time {
        builder = builder.time(aws_sdk_eventbridge::primitives::DateTime::from_secs(time.timestamp()));
    }
    
    if let Some(ref resources) = entry.resources {
        for resource in resources {
            builder = builder.resources(resource);
        }
    }
    
    if let Some(ref event_bus_name) = entry.event_bus_name {
        builder = builder.event_bus_name(event_bus_name);
    }
    
    if let Some(ref trace_header) = entry.trace_header {
        builder = builder.trace_header(trace_header);
    }
    
    builder.build()
}

/// Converts EventBridgeTarget to AWS SDK Target format
fn target_to_aws_target(target: &EventBridgeTarget) -> Target {
    let mut builder = Target::builder()
        .id(&target.id)
        .arn(&target.arn);
    
    if let Some(ref role_arn) = target.role_arn {
        builder = builder.role_arn(role_arn);
    }
    
    if let Some(ref input) = target.input {
        builder = builder.input(input);
    }
    
    if let Some(ref input_path) = target.input_path {
        builder = builder.input_path(input_path);
    }
    
    if let Some(ref input_transformer) = target.input_transformer {
        let mut it_builder = AwsInputTransformer::builder()
            .input_template(&input_transformer.input_template);
        
        if let Some(ref paths_map) = input_transformer.input_paths_map {
            for (k, v) in paths_map {
                it_builder = it_builder.input_paths_map(k, v);
            }
        }
        
        builder = builder.input_transformer(it_builder.build().expect("Failed to build input transformer"));
    }
    
    if let Some(ref dlc) = target.dead_letter_config {
        let mut dlc_builder = AwsDeadLetterConfig::builder();
        if let Some(ref arn) = dlc.arn {
            dlc_builder = dlc_builder.arn(arn);
        }
        builder = builder.dead_letter_config(dlc_builder.build());
    }
    
    if let Some(ref retry_policy) = target.retry_policy {
        let mut rp_builder = AwsRetryPolicy::builder();
        if let Some(max_retry) = retry_policy.maximum_retry_attempts {
            rp_builder = rp_builder.maximum_retry_attempts(max_retry);
        }
        if let Some(max_age) = retry_policy.maximum_event_age_in_seconds {
            rp_builder = rp_builder.maximum_event_age_in_seconds(max_age);
        }
        builder = builder.retry_policy(rp_builder.build());
    }
    
    builder.build().expect("Failed to build target")
}

/// Converts AWS SDK Target to our EventBridgeTarget type
fn convert_target(target: &aws_sdk_eventbridge::types::Target) -> EventBridgeTarget {
    EventBridgeTarget {
        id: target.id().to_string(),
        arn: target.arn().to_string(),
        role_arn: target.role_arn().map(|s| s.to_string()),
        input: target.input().map(|s| s.to_string()),
        input_path: target.input_path().map(|s| s.to_string()),
        input_transformer: target.input_transformer().map(|it| InputTransformer {
            input_paths_map: {
                let map = it.input_paths_map();
                if let Some(m) = map {
                    if m.is_empty() {
                        None
                    } else {
                        Some(m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    }
                } else {
                    None
                }
            },
            input_template: it.input_template().to_string(),
        }),
        kinesis_parameters: target.kinesis_parameters().map(|_| serde_json::Value::Null),
        run_command_parameters: target.run_command_parameters().map(|_| serde_json::Value::Null),
        ecs_parameters: target.ecs_parameters().map(|_| serde_json::Value::Null),
        batch_parameters: target.batch_parameters().map(|_| serde_json::Value::Null),
        sqs_parameters: target.sqs_parameters().map(|_| serde_json::Value::Null),
        http_parameters: target.http_parameters().map(|_| serde_json::Value::Null),
        redshift_data_parameters: target.redshift_data_parameters().map(|_| serde_json::Value::Null),
        sage_maker_pipeline_parameters: target.sage_maker_pipeline_parameters().map(|_| serde_json::Value::Null),
        dead_letter_config: target.dead_letter_config().map(|dlc| DeadLetterConfig {
            arn: dlc.arn().map(|s| s.to_string()),
        }),
        retry_policy: target.retry_policy().map(|rp| RetryPolicy {
            maximum_retry_attempts: rp.maximum_retry_attempts(),
            maximum_event_age_in_seconds: rp.maximum_event_age_in_seconds(),
        }),
    }
}

/// Converts AWS SDK Rule to our EventBridgeRule type
fn convert_rule(rule: &aws_sdk_eventbridge::types::Rule) -> EventBridgeRule {
    EventBridgeRule {
        name: rule.name().unwrap_or_default().to_string(),
        arn: rule.arn().unwrap_or_default().to_string(),
        event_pattern: rule.event_pattern().map(|s| s.to_string()),
        state: rule.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
        description: rule.description().map(|s| s.to_string()),
        schedule_expression: rule.schedule_expression().map(|s| s.to_string()),
        role_arn: rule.role_arn().map(|s| s.to_string()),
        managed_by: rule.managed_by().map(|s| s.to_string()),
        event_bus_name: rule.event_bus_name().map(|s| s.to_string()),
    }
}

/// Converts AWS SDK EventBus to our EventBus type
fn convert_event_bus(event_bus: &aws_sdk_eventbridge::types::EventBus) -> EventBus {
    EventBus {
        name: event_bus.name().unwrap_or_default().to_string(),
        arn: event_bus.arn().unwrap_or_default().to_string(),
        policy: event_bus.policy().map(|s| s.to_string()),
    }
}

/// Put EventBridge Events
pub async fn put_events(
    entries: Vec<EventBridgeEntry>,
    endpoint_id: Option<&str>,
    region: Option<&str>,
) -> Result<PutEventsOutput, String> {
    let client = create_client(region).await?;
    
    let aws_entries: Vec<PutEventsRequestEntry> = entries
        .iter()
        .map(entry_to_aws_entry)
        .collect();
    
    let mut request = client.put_events()
        .set_entries(Some(aws_entries));
    
    if let Some(endpoint) = endpoint_id {
        request = request.endpoint_id(endpoint);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to put events: {}", e))?;
    
    let result_entries: Vec<PutEventsResultEntry> = response
        .entries()
        .iter()
        .map(|e| PutEventsResultEntry {
            event_id: e.event_id().map(|s| s.to_string()),
            error_code: e.error_code().map(|s| s.to_string()),
            error_message: e.error_message().map(|s| s.to_string()),
        })
        .collect();
    
    Ok(PutEventsOutput {
        failed_entry_count: response.failed_entry_count(),
        entries: result_entries,
        success: response.failed_entry_count() == 0,
    })
}

/// Create EventBridge Rule
pub async fn create_rule(
    name: &str,
    event_bus_name: Option<&str>,
    schedule_expression: Option<&str>,
    event_pattern: Option<&str>,
    state: Option<&str>,
    description: Option<&str>,
    role_arn: Option<&str>,
    tags: Option<EventBridgeTags>,
    region: Option<&str>,
) -> Result<CreateRuleOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.put_rule()
        .name(name);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    if let Some(schedule) = schedule_expression {
        request = request.schedule_expression(schedule);
    }
    
    if let Some(pattern) = event_pattern {
        request = request.event_pattern(pattern);
    }
    
    if let Some(s) = state {
        let rule_state = match s.to_uppercase().as_str() {
            "ENABLED" => RuleState::Enabled,
            "DISABLED" => RuleState::Disabled,
            _ => RuleState::Enabled,
        };
        request = request.state(rule_state);
    }
    
    if let Some(desc) = description {
        request = request.description(desc);
    }
    
    if let Some(role) = role_arn {
        request = request.role_arn(role);
    }
    
    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create rule: {}", e))?;
    
    Ok(CreateRuleOutput {
        rule_arn: response.rule_arn().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete EventBridge Rule
pub async fn delete_rule(
    name: &str,
    event_bus_name: Option<&str>,
    force: Option<bool>,
    region: Option<&str>,
) -> Result<DeleteRuleOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.delete_rule()
        .name(name);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    if let Some(f) = force {
        request = request.force(f);
    }
    
    request
        .send()
        .await
        .map_err(|e| format!("Failed to delete rule: {}", e))?;
    
    Ok(DeleteRuleOutput { success: true })
}

/// Describe EventBridge Rule
pub async fn describe_rule(
    name: &str,
    event_bus_name: Option<&str>,
    region: Option<&str>,
) -> Result<DescribeRuleOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.describe_rule()
        .name(name);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe rule: {}", e))?;
    
    Ok(DescribeRuleOutput {
        name: response.name().unwrap_or_default().to_string(),
        arn: response.arn().unwrap_or_default().to_string(),
        event_pattern: response.event_pattern().map(|s| s.to_string()),
        schedule_expression: response.schedule_expression().map(|s| s.to_string()),
        state: response.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
        description: response.description().map(|s| s.to_string()),
        role_arn: response.role_arn().map(|s| s.to_string()),
        event_bus_name: response.event_bus_name().unwrap_or_default().to_string(),
        created_by: response.created_by().map(|s| s.to_string()),
    })
}

/// List EventBridge Rules
pub async fn list_rules(
    event_bus_name: Option<&str>,
    name_prefix: Option<&str>,
    limit: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListRulesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.list_rules();
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    if let Some(prefix) = name_prefix {
        request = request.name_prefix(prefix);
    }
    
    if let Some(l) = limit {
        request = request.limit(l);
    }
    
    if let Some(token) = next_token {
        request = request.next_token(token);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list rules: {}", e))?;
    
    let rules: Vec<EventBridgeRule> = response
        .rules()
        .iter()
        .map(convert_rule)
        .collect();
    
    Ok(ListRulesOutput {
        rules,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Enable EventBridge Rule
pub async fn enable_rule(
    name: &str,
    event_bus_name: Option<&str>,
    region: Option<&str>,
) -> Result<EnableRuleOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.enable_rule()
        .name(name);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    request
        .send()
        .await
        .map_err(|e| format!("Failed to enable rule: {}", e))?;
    
    Ok(EnableRuleOutput { success: true })
}

/// Disable EventBridge Rule
pub async fn disable_rule(
    name: &str,
    event_bus_name: Option<&str>,
    region: Option<&str>,
) -> Result<DisableRuleOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.disable_rule()
        .name(name);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    request
        .send()
        .await
        .map_err(|e| format!("Failed to disable rule: {}", e))?;
    
    Ok(DisableRuleOutput { success: true })
}

/// Put EventBridge Targets
pub async fn put_targets(
    rule: &str,
    targets: Vec<EventBridgeTarget>,
    event_bus_name: Option<&str>,
    region: Option<&str>,
) -> Result<PutTargetsOutput, String> {
    let client = create_client(region).await?;
    
    let aws_targets: Vec<Target> = targets
        .iter()
        .map(target_to_aws_target)
        .collect();
    
    let mut request = client.put_targets()
        .rule(rule)
        .set_targets(Some(aws_targets));
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to put targets: {}", e))?;
    
    let failed_entries: Vec<TargetFailure> = response
        .failed_entries()
        .iter()
        .map(|e| TargetFailure {
            target_id: e.target_id().unwrap_or_default().to_string(),
            error_code: e.error_code().unwrap_or_default().to_string(),
            error_message: e.error_message().unwrap_or_default().to_string(),
        })
        .collect();
    
    Ok(PutTargetsOutput {
        failed_entry_count: response.failed_entry_count(),
        failed_entries,
        success: response.failed_entry_count() == 0,
    })
}

/// Remove EventBridge Targets
pub async fn remove_targets(
    rule: &str,
    ids: Vec<String>,
    event_bus_name: Option<&str>,
    force: Option<bool>,
    region: Option<&str>,
) -> Result<RemoveTargetsOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.remove_targets()
        .rule(rule)
        .set_ids(Some(ids));
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    if let Some(f) = force {
        request = request.force(f);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to remove targets: {}", e))?;
    
    let failed_entries: Vec<TargetFailure> = response
        .failed_entries()
        .iter()
        .map(|e| TargetFailure {
            target_id: e.target_id().unwrap_or_default().to_string(),
            error_code: e.error_code().unwrap_or_default().to_string(),
            error_message: e.error_message().unwrap_or_default().to_string(),
        })
        .collect();
    
    Ok(RemoveTargetsOutput {
        failed_entry_count: response.failed_entry_count(),
        failed_entries,
        success: response.failed_entry_count() == 0,
    })
}

/// List EventBridge Targets By Rule
pub async fn list_targets_by_rule(
    rule: &str,
    event_bus_name: Option<&str>,
    limit: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListTargetsByRuleOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.list_targets_by_rule()
        .rule(rule);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    if let Some(l) = limit {
        request = request.limit(l);
    }
    
    if let Some(token) = next_token {
        request = request.next_token(token);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list targets by rule: {}", e))?;
    
    let targets: Vec<EventBridgeTarget> = response
        .targets()
        .iter()
        .map(convert_target)
        .collect();
    
    Ok(ListTargetsByRuleOutput {
        targets,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Create Event Bus
pub async fn create_event_bus(
    name: &str,
    event_source_name: Option<&str>,
    tags: Option<EventBridgeTags>,
    region: Option<&str>,
) -> Result<CreateEventBusOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.create_event_bus()
        .name(name);
    
    if let Some(source_name) = event_source_name {
        request = request.event_source_name(source_name);
    }
    
    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create event bus: {}", e))?;
    
    Ok(CreateEventBusOutput {
        event_bus_arn: response.event_bus_arn().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete Event Bus
pub async fn delete_event_bus(
    name: &str,
    region: Option<&str>,
) -> Result<DeleteEventBusOutput, String> {
    let client = create_client(region).await?;
    
    client.delete_event_bus()
        .name(name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete event bus: {}", e))?;
    
    Ok(DeleteEventBusOutput { success: true })
}

/// Describe Event Bus
pub async fn describe_event_bus(
    name: Option<&str>,
    region: Option<&str>,
) -> Result<DescribeEventBusOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.describe_event_bus();
    
    if let Some(n) = name {
        request = request.name(n);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe event bus: {}", e))?;
    
    Ok(DescribeEventBusOutput {
        name: response.name().unwrap_or_default().to_string(),
        arn: response.arn().unwrap_or_default().to_string(),
        policy: response.policy().map(|s| s.to_string()),
    })
}

/// List Event Buses
pub async fn list_event_buses(
    name_prefix: Option<&str>,
    limit: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListEventBusesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.list_event_buses();
    
    if let Some(prefix) = name_prefix {
        request = request.name_prefix(prefix);
    }
    
    if let Some(l) = limit {
        request = request.limit(l);
    }
    
    if let Some(token) = next_token {
        request = request.next_token(token);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list event buses: {}", e))?;
    
    let event_buses: Vec<EventBus> = response
        .event_buses()
        .iter()
        .map(convert_event_bus)
        .collect();
    
    Ok(ListEventBusesOutput {
        event_buses,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Put Event Bus Permission
pub async fn put_permission(
    principal: &str,
    statement_id: &str,
    event_bus_name: Option<&str>,
    action: Option<&str>,
    condition: Option<RuleCondition>,
    region: Option<&str>,
) -> Result<PutPermissionOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.put_permission()
        .principal(principal)
        .statement_id(statement_id);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    if let Some(a) = action {
        request = request.action(a);
    }
    
    if let Some(cond) = condition {
        let aws_condition = Condition::builder()
            .r#type(&cond.condition_type)
            .key(&cond.key)
            .value(&cond.value)
            .build()
            .expect("Failed to build condition");
        request = request.condition(aws_condition);
    }
    
    request
        .send()
        .await
        .map_err(|e| format!("Failed to put permission: {}", e))?;
    
    Ok(PutPermissionOutput { success: true })
}

/// Remove Event Bus Permission
pub async fn remove_permission(
    statement_id: &str,
    event_bus_name: Option<&str>,
    remove_all_permissions: Option<bool>,
    region: Option<&str>,
) -> Result<RemovePermissionOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.remove_permission()
        .statement_id(statement_id);
    
    if let Some(bus_name) = event_bus_name {
        request = request.event_bus_name(bus_name);
    }
    
    if let Some(remove_all) = remove_all_permissions {
        request = request.remove_all_permissions(remove_all);
    }
    
    request
        .send()
        .await
        .map_err(|e| format!("Failed to remove permission: {}", e))?;
    
    Ok(RemovePermissionOutput { success: true })
}

/// Create Archive
pub async fn create_archive(
    archive_name: &str,
    event_source_arn: &str,
    description: Option<&str>,
    event_pattern: Option<&str>,
    retention_days: Option<i32>,
    region: Option<&str>,
) -> Result<CreateArchiveOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.create_archive()
        .archive_name(archive_name)
        .event_source_arn(event_source_arn);
    
    if let Some(desc) = description {
        request = request.description(desc);
    }
    
    if let Some(pattern) = event_pattern {
        request = request.event_pattern(pattern);
    }
    
    if let Some(days) = retention_days {
        request = request.retention_days(days);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create archive: {}", e))?;
    
    let creation_time = response
        .creation_time()
        .map(|t| DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()))
        .flatten()
        .unwrap_or_else(Utc::now);
    
    Ok(CreateArchiveOutput {
        archive_arn: response.archive_arn().unwrap_or_default().to_string(),
        state: response.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
        state_reason: response.state_reason().map(|s| s.to_string()),
        creation_time,
        success: true,
    })
}

/// Delete Archive
pub async fn delete_archive(
    archive_name: &str,
    region: Option<&str>,
) -> Result<DeleteArchiveOutput, String> {
    let client = create_client(region).await?;
    
    client.delete_archive()
        .archive_name(archive_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete archive: {}", e))?;
    
    Ok(DeleteArchiveOutput { success: true })
}

/// Describe Archive
pub async fn describe_archive(
    archive_name: &str,
    region: Option<&str>,
) -> Result<DescribeArchiveOutput, String> {
    let client = create_client(region).await?;
    
    let response = client.describe_archive()
        .archive_name(archive_name)
        .send()
        .await
        .map_err(|e| format!("Failed to describe archive: {}", e))?;
    
    let creation_time = response
        .creation_time()
        .map(|t| DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()))
        .flatten()
        .unwrap_or_else(Utc::now);
    
    Ok(DescribeArchiveOutput {
        archive_arn: response.archive_arn().unwrap_or_default().to_string(),
        archive_name: response.archive_name().unwrap_or_default().to_string(),
        event_source_arn: response.event_source_arn().unwrap_or_default().to_string(),
        description: response.description().map(|s| s.to_string()),
        event_pattern: response.event_pattern().map(|s| s.to_string()),
        state: response.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
        retention_days: response.retention_days(),
        size_bytes: Some(response.size_bytes()),
        event_count: Some(response.event_count()),
        creation_time,
    })
}

/// Start Replay
pub async fn start_replay(
    replay_name: &str,
    event_source_arn: &str,
    destination: EventDestination,
    event_start_time: &str,
    event_end_time: &str,
    description: Option<&str>,
    region: Option<&str>,
) -> Result<StartReplayOutput, String> {
    let client = create_client(region).await?;
    
    let start_time: DateTime<Utc> = event_start_time
        .parse()
        .map_err(|e| format!("Invalid event_start_time: {}", e))?;
    
    let end_time: DateTime<Utc> = event_end_time
        .parse()
        .map_err(|e| format!("Invalid event_end_time: {}", e))?;
    
    let replay_destination = ReplayDestination::builder()
        .arn(&destination.arn)
        .build()
        .expect("Failed to build replay destination");
    
    let mut request = client.start_replay()
        .replay_name(replay_name)
        .event_source_arn(event_source_arn)
        .destination(replay_destination)
        .event_start_time(aws_sdk_eventbridge::primitives::DateTime::from_secs(start_time.timestamp()))
        .event_end_time(aws_sdk_eventbridge::primitives::DateTime::from_secs(end_time.timestamp()));
    
    if let Some(desc) = description {
        request = request.description(desc);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to start replay: {}", e))?;
    
    let replay_start_time = response
        .replay_start_time()
        .map(|t| DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()))
        .flatten()
        .unwrap_or_else(Utc::now);
    
    Ok(StartReplayOutput {
        replay_arn: response.replay_arn().unwrap_or_default().to_string(),
        state: response.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
        state_reason: response.state_reason().map(|s| s.to_string()),
        replay_start_time,
        success: true,
    })
}

/// Cancel Replay
pub async fn cancel_replay(
    replay_name: &str,
    region: Option<&str>,
) -> Result<CancelReplayOutput, String> {
    let client = create_client(region).await?;
    
    let response = client.cancel_replay()
        .replay_name(replay_name)
        .send()
        .await
        .map_err(|e| format!("Failed to cancel replay: {}", e))?;
    
    Ok(CancelReplayOutput {
        state: response.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
        state_reason: response.state_reason().map(|s| s.to_string()),
        success: true,
    })
}

/// Describe Replay
pub async fn describe_replay(
    replay_name: &str,
    region: Option<&str>,
) -> Result<DescribeReplayOutput, String> {
    let client = create_client(region).await?;
    
    let response = client.describe_replay()
        .replay_name(replay_name)
        .send()
        .await
        .map_err(|e| format!("Failed to describe replay: {}", e))?;
    
    let event_start_time = response
        .event_start_time()
        .map(|t| DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()))
        .flatten()
        .unwrap_or_else(Utc::now);
    
    let event_end_time = response
        .event_end_time()
        .map(|t| DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()))
        .flatten()
        .unwrap_or_else(Utc::now);
    
    let replay_start_time = response
        .replay_start_time()
        .map(|t| DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()))
        .flatten();
    
    let replay_end_time = response
        .replay_end_time()
        .map(|t| DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()))
        .flatten();
    
    let destination = response
        .destination()
        .map(|d| EventDestination {
            arn: d.arn().to_string(),
        })
        .unwrap_or(EventDestination { arn: String::new() });
    
    Ok(DescribeReplayOutput {
        replay_name: response.replay_name().unwrap_or_default().to_string(),
        replay_arn: response.replay_arn().unwrap_or_default().to_string(),
        description: response.description().map(|s| s.to_string()),
        state: response.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
        state_reason: response.state_reason().map(|s| s.to_string()),
        event_source_arn: response.event_source_arn().unwrap_or_default().to_string(),
        destination,
        event_start_time,
        event_end_time,
        replay_start_time,
        replay_end_time,
    })
}

/// Test Event Pattern
pub async fn test_event_pattern(
    event_pattern: &str,
    event: &str,
    region: Option<&str>,
) -> Result<TestEventPatternOutput, String> {
    let client = create_client(region).await?;
    
    let response = client.test_event_pattern()
        .event_pattern(event_pattern)
        .event(event)
        .send()
        .await
        .map_err(|e| format!("Failed to test event pattern: {}", e))?;
    
    Ok(TestEventPatternOutput {
        result: response.result(),
    })
}

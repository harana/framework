// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingGroup {
    pub account_id: String,
    pub auto_scaling_group_arn: Option<String>,
    pub auto_scaling_group_name: String,
    pub availability_zones: Option<String>,
    #[serde(default)]
    pub capacity_rebalance: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_cooldown: i64,
    pub desired_capacity: i64,
    pub health_check_grace_period: i64,
    pub health_check_type: String,
    pub launch_configuration_name: Option<String>,
    pub launch_template_id: Option<String>,
    pub launch_template_version: Option<String>,
    pub max_instance_lifetime: Option<i64>,
    pub max_size: i64,
    pub min_size: i64,
    #[serde(default)]
    pub new_instances_protected_from_scale_in: bool,
    pub placement_group: Option<String>,
    pub region: Option<String>,
    pub service_linked_role_arn: Option<String>,
    pub status: Option<String>,
    pub tags: Option<String>,
    pub target_group_arns: Option<String>,
    pub termination_policies: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_zone_identifier: Option<String>,
}

impl AwsAutoscalingGroup {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingLaunchConfiguration {
    pub account_id: String,
    #[serde(default)]
    pub associate_public_ip_address: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub ebs_optimized: bool,
    pub iam_instance_profile: Option<String>,
    pub image_id: Option<String>,
    #[serde(default)]
    pub instance_monitoring_enabled: bool,
    pub instance_type: Option<String>,
    pub kernel_id: Option<String>,
    pub key_name: Option<String>,
    pub launch_configuration_arn: Option<String>,
    pub launch_configuration_name: String,
    pub region: Option<String>,
    pub security_groups: Option<String>,
    pub spot_price: Option<String>,
}

impl AwsAutoscalingLaunchConfiguration {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingPolicy {
    pub adjustment_type: String,
    pub auto_scaling_group_id: String,
    pub cooldown: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub enabled: bool,
    pub estimated_instance_warmup: Option<i64>,
    pub metric_aggregation_type: Option<String>,
    pub min_adjustment_magnitude: Option<i64>,
    pub policy_arn: Option<String>,
    pub policy_name: String,
    pub policy_type: String,
    pub scaling_adjustment: Option<i64>,
    pub target_tracking_configuration: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsAutoscalingPolicy {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingScheduledAction {
    pub auto_scaling_group_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub desired_capacity: Option<i64>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub max_size: Option<i64>,
    pub min_size: Option<i64>,
    pub recurrence: Option<String>,
    pub scheduled_action_arn: Option<String>,
    pub scheduled_action_name: String,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub time_zone: Option<String>,
}

impl AwsAutoscalingScheduledAction {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingLifecycleHook {
    pub auto_scaling_group_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_result: String,
    pub heartbeat_timeout: i64,
    pub lifecycle_hook_name: String,
    pub lifecycle_transition: String,
    pub notification_metadata: Option<String>,
    pub notification_target_arn: Option<String>,
    pub role_arn: Option<String>,
}

impl AwsAutoscalingLifecycleHook {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


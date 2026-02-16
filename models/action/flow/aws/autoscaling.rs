// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingGroupsOutput {
    pub auto_scaling_groups: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingInstancesOutput {
    pub auto_scaling_instances: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLaunchConfigurationsOutput {
    pub launch_configurations: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutScalingPolicyOutput {
    pub alarms: Vec<String>,
    pub policy_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribePoliciesOutput {
    pub next_token: String,
    pub scaling_policies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScheduledActionsOutput {
    pub next_token: String,
    pub scheduled_update_group_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNotificationConfigurationsOutput {
    pub next_token: String,
    pub notification_configurations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancersOutput {
    pub load_balancers: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancerTargetGroupsOutput {
    pub load_balancer_target_groups: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTrafficSourcesOutput {
    pub next_token: String,
    pub traffic_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScalingActivitiesOutput {
    pub activities: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstanceRefreshesOutput {
    pub instance_refreshes: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeWarmPoolOutput {
    pub instances: Vec<String>,
    pub next_token: String,
    pub warm_pool_configuration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAccountLimitsOutput {
    pub max_number_of_auto_scaling_groups: i64,
    pub max_number_of_launch_configurations: i64,
    pub number_of_auto_scaling_groups: i64,
    pub number_of_launch_configurations: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeMetricCollectionTypesOutput {
    pub granularities: Vec<String>,
    pub metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPredictiveScalingForecastOutput {
    pub capacity_forecast: String,
    pub load_forecast: Vec<String>,
    pub update_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTagsOutput {
    pub next_token: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingGroup {
    pub account_id: String,
    pub auto_scaling_group_arn: String,
    pub auto_scaling_group_name: String,
    pub availability_zones: String,
    #[serde(default)]
    pub capacity_rebalance: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_cooldown: i64,
    pub desired_capacity: i64,
    pub health_check_grace_period: i64,
    pub health_check_type: String,
    pub launch_configuration_name: String,
    pub launch_template_id: String,
    pub launch_template_version: String,
    pub max_instance_lifetime: i64,
    pub max_size: i64,
    pub min_size: i64,
    #[serde(default)]
    pub new_instances_protected_from_scale_in: bool,
    pub placement_group: String,
    pub region: String,
    pub service_linked_role_arn: String,
    pub status: String,
    pub tags: String,
    pub target_group_arns: String,
    pub termination_policies: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_zone_identifier: String,
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
    pub iam_instance_profile: String,
    pub image_id: String,
    #[serde(default)]
    pub instance_monitoring_enabled: bool,
    pub instance_type: String,
    pub kernel_id: String,
    pub key_name: String,
    pub launch_configuration_arn: String,
    pub launch_configuration_name: String,
    pub region: String,
    pub security_groups: String,
    pub spot_price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingPolicy {
    pub adjustment_type: String,
    pub auto_scaling_group_id: String,
    pub cooldown: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub enabled: bool,
    pub estimated_instance_warmup: i64,
    pub metric_aggregation_type: String,
    pub min_adjustment_magnitude: i64,
    pub policy_arn: String,
    pub policy_name: String,
    pub policy_type: String,
    pub scaling_adjustment: i64,
    pub target_tracking_configuration: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAutoscalingScheduledAction {
    pub auto_scaling_group_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub desired_capacity: i64,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub max_size: i64,
    pub min_size: i64,
    pub recurrence: String,
    pub scheduled_action_arn: String,
    pub scheduled_action_name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub time_zone: String,
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
    pub notification_metadata: String,
    pub notification_target_arn: String,
    pub role_arn: String,
}

#[async_trait]
pub trait AutoscalingAction {
    async fn create_auto_scaling_group(&self, auto_scaling_group_name: String, availability_zones: Vec<String>, capacity_rebalance: bool, context: String, default_cooldown: i64, default_instance_warmup: i64, desired_capacity: i64, desired_capacity_type: String, health_check_grace_period: i64, health_check_type: String, instance_id: String, instance_maintenance_policy: String, launch_configuration_name: String, launch_template: String, lifecycle_hook_specification_list: Vec<String>, load_balancer_names: Vec<String>, max_instance_lifetime: i64, max_size: i64, min_size: i64, mixed_instances_policy: String, new_instances_protected_from_scale_in: bool, placement_group: String, region: String, service_linked_role_arn: String, tags: Vec<String>, target_group_arns: Vec<String>, termination_policies: Vec<String>, traffic_sources: Vec<String>, vpc_zone_identifier: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_auto_scaling_group(&self, auto_scaling_group_name: String, force_delete: bool, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_auto_scaling_groups(&self, auto_scaling_group_names: Vec<String>, filters: Vec<String>, max_records: i64, next_token: String, region: String) -> Result<DescribeAutoScalingGroupsOutput, Box<dyn std::error::Error>>;
    async fn update_auto_scaling_group(&self, auto_scaling_group_name: String, availability_zones: Vec<String>, capacity_rebalance: bool, context: String, default_cooldown: i64, default_instance_warmup: i64, desired_capacity: i64, desired_capacity_type: String, health_check_grace_period: i64, health_check_type: String, instance_maintenance_policy: String, launch_configuration_name: String, launch_template: String, max_instance_lifetime: i64, max_size: i64, min_size: i64, mixed_instances_policy: String, new_instances_protected_from_scale_in: bool, placement_group: String, region: String, service_linked_role_arn: String, termination_policies: Vec<String>, vpc_zone_identifier: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_desired_capacity(&self, auto_scaling_group_name: String, desired_capacity: i64, honor_cooldown: bool, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn attach_instances(&self, auto_scaling_group_name: String, instance_ids: Vec<String>, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_instances(&self, auto_scaling_group_name: String, instance_ids: Vec<String>, region: String, should_decrement_desired_capacity: bool) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn enter_standby(&self, auto_scaling_group_name: String, instance_ids: Vec<String>, region: String, should_decrement_desired_capacity: bool) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn exit_standby(&self, auto_scaling_group_name: String, instance_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn set_instance_health(&self, health_status: String, instance_id: String, region: String, should_respect_grace_period: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_instance_protection(&self, auto_scaling_group_name: String, instance_ids: Vec<String>, protected_from_scale_in: bool, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn terminate_instance_in_auto_scaling_group(&self, instance_id: String, region: String, should_decrement_desired_capacity: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_auto_scaling_instances(&self, instance_ids: Vec<String>, max_records: i64, next_token: String, region: String) -> Result<DescribeAutoScalingInstancesOutput, Box<dyn std::error::Error>>;
    async fn create_launch_configuration(&self, associate_public_ip_address: bool, block_device_mappings: Vec<String>, classic_link_vpc_id: String, classic_link_vpc_security_groups: Vec<String>, ebs_optimized: bool, iam_instance_profile: String, image_id: String, instance_id: String, instance_monitoring: String, instance_type: String, kernel_id: String, key_name: String, launch_configuration_name: String, metadata_options: String, placement_tenancy: String, ramdisk_id: String, region: String, security_groups: Vec<String>, spot_price: String, user_data: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_launch_configuration(&self, launch_configuration_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_launch_configurations(&self, launch_configuration_names: Vec<String>, max_records: i64, next_token: String, region: String) -> Result<DescribeLaunchConfigurationsOutput, Box<dyn std::error::Error>>;
    async fn put_scaling_policy(&self, adjustment_type: String, auto_scaling_group_name: String, cooldown: i64, enabled: bool, estimated_instance_warmup: i64, metric_aggregation_type: String, min_adjustment_magnitude: i64, min_adjustment_step: i64, policy_name: String, policy_type: String, predictive_scaling_configuration: String, region: String, scaling_adjustment: i64, step_adjustments: Vec<String>, target_tracking_configuration: String) -> Result<PutScalingPolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_policy(&self, auto_scaling_group_name: String, policy_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_policies(&self, auto_scaling_group_name: String, max_records: i64, next_token: String, policy_names: Vec<String>, policy_types: Vec<String>, region: String) -> Result<DescribePoliciesOutput, Box<dyn std::error::Error>>;
    async fn execute_policy(&self, auto_scaling_group_name: String, breach_threshold: String, honor_cooldown: bool, metric_value: String, policy_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn put_scheduled_update_group_action(&self, auto_scaling_group_name: String, desired_capacity: i64, end_time: chrono::DateTime<chrono::Utc>, max_size: i64, min_size: i64, recurrence: String, region: String, scheduled_action_name: String, start_time: chrono::DateTime<chrono::Utc>, time: chrono::DateTime<chrono::Utc>, time_zone: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_scheduled_action(&self, auto_scaling_group_name: String, region: String, scheduled_action_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_scheduled_actions(&self, auto_scaling_group_name: String, end_time: chrono::DateTime<chrono::Utc>, max_records: i64, next_token: String, region: String, scheduled_action_names: Vec<String>, start_time: chrono::DateTime<chrono::Utc>) -> Result<DescribeScheduledActionsOutput, Box<dyn std::error::Error>>;
    async fn batch_delete_scheduled_action(&self, auto_scaling_group_name: String, region: String, scheduled_action_names: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn batch_put_scheduled_update_group_action(&self, auto_scaling_group_name: String, region: String, scheduled_update_group_actions: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn put_lifecycle_hook(&self, auto_scaling_group_name: String, default_result: String, heartbeat_timeout: i64, lifecycle_hook_name: String, lifecycle_transition: String, notification_metadata: String, notification_target_arn: String, region: String, role_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_lifecycle_hook(&self, auto_scaling_group_name: String, lifecycle_hook_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_lifecycle_hooks(&self, auto_scaling_group_name: String, lifecycle_hook_names: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_lifecycle_hook_types(&self, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn complete_lifecycle_action(&self, auto_scaling_group_name: String, instance_id: String, lifecycle_action_result: String, lifecycle_action_token: String, lifecycle_hook_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn record_lifecycle_action_heartbeat(&self, auto_scaling_group_name: String, instance_id: String, lifecycle_action_token: String, lifecycle_hook_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn put_notification_configuration(&self, auto_scaling_group_name: String, notification_types: Vec<String>, region: String, topic_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_notification_configuration(&self, auto_scaling_group_name: String, region: String, topic_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_notification_configurations(&self, auto_scaling_group_names: Vec<String>, max_records: i64, next_token: String, region: String) -> Result<DescribeNotificationConfigurationsOutput, Box<dyn std::error::Error>>;
    async fn describe_auto_scaling_notification_types(&self, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn attach_load_balancers(&self, auto_scaling_group_name: String, load_balancer_names: Vec<String>, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_load_balancers(&self, auto_scaling_group_name: String, load_balancer_names: Vec<String>, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_load_balancers(&self, auto_scaling_group_name: String, max_records: i64, next_token: String, region: String) -> Result<DescribeLoadBalancersOutput, Box<dyn std::error::Error>>;
    async fn attach_load_balancer_target_groups(&self, auto_scaling_group_name: String, region: String, target_group_arns: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_load_balancer_target_groups(&self, auto_scaling_group_name: String, region: String, target_group_arns: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_load_balancer_target_groups(&self, auto_scaling_group_name: String, max_records: i64, next_token: String, region: String) -> Result<DescribeLoadBalancerTargetGroupsOutput, Box<dyn std::error::Error>>;
    async fn attach_traffic_sources(&self, auto_scaling_group_name: String, region: String, traffic_sources: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_traffic_sources(&self, auto_scaling_group_name: String, region: String, traffic_sources: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_traffic_sources(&self, auto_scaling_group_name: String, max_records: i64, next_token: String, region: String, traffic_source_type: String) -> Result<DescribeTrafficSourcesOutput, Box<dyn std::error::Error>>;
    async fn describe_scaling_activities(&self, activity_ids: Vec<String>, auto_scaling_group_name: String, include_deleted_groups: bool, max_records: i64, next_token: String, region: String) -> Result<DescribeScalingActivitiesOutput, Box<dyn std::error::Error>>;
    async fn describe_scaling_process_types(&self, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn suspend_processes(&self, auto_scaling_group_name: String, region: String, scaling_processes: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn resume_processes(&self, auto_scaling_group_name: String, region: String, scaling_processes: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn start_instance_refresh(&self, auto_scaling_group_name: String, desired_configuration: String, preferences: String, region: String, strategy: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn cancel_instance_refresh(&self, auto_scaling_group_name: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_instance_refreshes(&self, auto_scaling_group_name: String, instance_refresh_ids: Vec<String>, max_records: i64, next_token: String, region: String) -> Result<DescribeInstanceRefreshesOutput, Box<dyn std::error::Error>>;
    async fn rollback_instance_refresh(&self, auto_scaling_group_name: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn put_warm_pool(&self, auto_scaling_group_name: String, instance_reuse_policy: String, max_group_prepared_capacity: i64, min_size: i64, pool_state: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_warm_pool(&self, auto_scaling_group_name: String, force_delete: bool, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_warm_pool(&self, auto_scaling_group_name: String, max_records: i64, next_token: String, region: String) -> Result<DescribeWarmPoolOutput, Box<dyn std::error::Error>>;
    async fn describe_account_limits(&self, region: String) -> Result<DescribeAccountLimitsOutput, Box<dyn std::error::Error>>;
    async fn describe_adjustment_types(&self, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_termination_policy_types(&self, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_metric_collection_types(&self, region: String) -> Result<DescribeMetricCollectionTypesOutput, Box<dyn std::error::Error>>;
    async fn enable_metrics_collection(&self, auto_scaling_group_name: String, granularity: String, metrics: Vec<String>, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn disable_metrics_collection(&self, auto_scaling_group_name: String, metrics: Vec<String>, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_predictive_scaling_forecast(&self, auto_scaling_group_name: String, end_time: chrono::DateTime<chrono::Utc>, policy_name: String, region: String, start_time: chrono::DateTime<chrono::Utc>) -> Result<GetPredictiveScalingForecastOutput, Box<dyn std::error::Error>>;
    async fn create_or_update_tags(&self, region: String, tags: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_tags(&self, region: String, tags: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_tags(&self, filters: Vec<String>, max_records: i64, next_token: String, region: String) -> Result<DescribeTagsOutput, Box<dyn std::error::Error>>;
}

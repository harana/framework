// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAutoScalingGroupInput {
    pub auto_scaling_group_name: String,
    pub availability_zones: Vec<String>,
    #[serde(default)]
    pub capacity_rebalance: bool,
    pub context: String,
    pub default_cooldown: i64,
    pub default_instance_warmup: i64,
    pub desired_capacity: i64,
    pub desired_capacity_type: String,
    pub health_check_grace_period: i64,
    pub health_check_type: String,
    pub instance_id: String,
    pub instance_maintenance_policy: String,
    pub launch_configuration_name: String,
    pub launch_template: String,
    pub lifecycle_hook_specification_list: Vec<String>,
    pub load_balancer_names: Vec<String>,
    pub max_instance_lifetime: i64,
    pub max_size: i64,
    pub min_size: i64,
    pub mixed_instances_policy: String,
    #[serde(default)]
    pub new_instances_protected_from_scale_in: bool,
    pub placement_group: String,
    pub region: String,
    pub service_linked_role_arn: String,
    pub tags: Vec<String>,
    pub target_group_arns: Vec<String>,
    pub termination_policies: Vec<String>,
    pub traffic_sources: Vec<String>,
    pub vpc_zone_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAutoScalingGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAutoScalingGroupInput {
    pub auto_scaling_group_name: String,
    #[serde(default)]
    pub force_delete: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAutoScalingGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingGroupsInput {
    pub auto_scaling_group_names: Vec<String>,
    pub filters: Vec<String>,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingGroupsOutput {
    pub auto_scaling_groups: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateAutoScalingGroupInput {
    pub auto_scaling_group_name: String,
    pub availability_zones: Vec<String>,
    pub capacity_rebalance: bool,
    pub context: String,
    pub default_cooldown: i64,
    pub default_instance_warmup: i64,
    pub desired_capacity: i64,
    pub desired_capacity_type: String,
    pub health_check_grace_period: i64,
    pub health_check_type: String,
    pub instance_maintenance_policy: String,
    pub launch_configuration_name: String,
    pub launch_template: String,
    pub max_instance_lifetime: i64,
    pub max_size: i64,
    pub min_size: i64,
    pub mixed_instances_policy: String,
    pub new_instances_protected_from_scale_in: bool,
    pub placement_group: String,
    pub region: String,
    pub service_linked_role_arn: String,
    pub termination_policies: Vec<String>,
    pub vpc_zone_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateAutoScalingGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetDesiredCapacityInput {
    pub auto_scaling_group_name: String,
    pub desired_capacity: i64,
    #[serde(default)]
    pub honor_cooldown: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetDesiredCapacityOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachInstancesInput {
    pub auto_scaling_group_name: String,
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachInstancesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachInstancesInput {
    pub auto_scaling_group_name: String,
    pub instance_ids: Vec<String>,
    pub region: String,
    pub should_decrement_desired_capacity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachInstancesOutput {
    pub activities: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnterStandbyInput {
    pub auto_scaling_group_name: String,
    pub instance_ids: Vec<String>,
    pub region: String,
    pub should_decrement_desired_capacity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnterStandbyOutput {
    pub activities: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExitStandbyInput {
    pub auto_scaling_group_name: String,
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExitStandbyOutput {
    pub activities: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetInstanceHealthInput {
    pub health_status: String,
    pub instance_id: String,
    pub region: String,
    #[serde(default)]
    pub should_respect_grace_period: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetInstanceHealthOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetInstanceProtectionInput {
    pub auto_scaling_group_name: String,
    pub instance_ids: Vec<String>,
    pub protected_from_scale_in: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetInstanceProtectionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateInstanceInAutoScalingGroupInput {
    pub instance_id: String,
    pub region: String,
    pub should_decrement_desired_capacity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateInstanceInAutoScalingGroupOutput {
    pub activity: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingInstancesInput {
    pub instance_ids: Vec<String>,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingInstancesOutput {
    pub auto_scaling_instances: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLaunchConfigurationInput {
    pub associate_public_ip_address: bool,
    pub block_device_mappings: Vec<String>,
    pub classic_link_vpc_id: String,
    pub classic_link_vpc_security_groups: Vec<String>,
    #[serde(default)]
    pub ebs_optimized: bool,
    pub iam_instance_profile: String,
    pub image_id: String,
    pub instance_id: String,
    pub instance_monitoring: String,
    pub instance_type: String,
    pub kernel_id: String,
    pub key_name: String,
    pub launch_configuration_name: String,
    pub metadata_options: String,
    pub placement_tenancy: String,
    pub ramdisk_id: String,
    pub region: String,
    pub security_groups: Vec<String>,
    pub spot_price: String,
    pub user_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLaunchConfigurationOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchConfigurationInput {
    pub launch_configuration_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchConfigurationOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLaunchConfigurationsInput {
    pub launch_configuration_names: Vec<String>,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLaunchConfigurationsOutput {
    pub launch_configurations: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutScalingPolicyInput {
    pub adjustment_type: String,
    pub auto_scaling_group_name: String,
    pub cooldown: i64,
    #[serde(default)]
    pub enabled: bool,
    pub estimated_instance_warmup: i64,
    pub metric_aggregation_type: String,
    pub min_adjustment_magnitude: i64,
    pub min_adjustment_step: i64,
    pub policy_name: String,
    pub policy_type: String,
    pub predictive_scaling_configuration: String,
    pub region: String,
    pub scaling_adjustment: i64,
    pub step_adjustments: Vec<String>,
    pub target_tracking_configuration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutScalingPolicyOutput {
    pub alarms: Vec<String>,
    pub policy_arn: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePolicyInput {
    pub auto_scaling_group_name: String,
    pub policy_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribePoliciesInput {
    pub auto_scaling_group_name: String,
    pub max_records: i64,
    pub next_token: String,
    pub policy_names: Vec<String>,
    pub policy_types: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribePoliciesOutput {
    pub next_token: String,
    pub scaling_policies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecutePolicyInput {
    pub auto_scaling_group_name: String,
    pub breach_threshold: String,
    #[serde(default)]
    pub honor_cooldown: bool,
    pub metric_value: String,
    pub policy_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecutePolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutScheduledUpdateGroupActionInput {
    pub auto_scaling_group_name: String,
    pub desired_capacity: i64,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub max_size: i64,
    pub min_size: i64,
    pub recurrence: String,
    pub region: String,
    pub scheduled_action_name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub time_zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutScheduledUpdateGroupActionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteScheduledActionInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub scheduled_action_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteScheduledActionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScheduledActionsInput {
    pub auto_scaling_group_name: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
    pub scheduled_action_names: Vec<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScheduledActionsOutput {
    pub next_token: String,
    pub scheduled_update_group_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchDeleteScheduledActionInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub scheduled_action_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchDeleteScheduledActionOutput {
    pub failed_scheduled_actions: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchPutScheduledUpdateGroupActionInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub scheduled_update_group_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchPutScheduledUpdateGroupActionOutput {
    pub failed_scheduled_update_group_actions: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutLifecycleHookInput {
    pub auto_scaling_group_name: String,
    pub default_result: String,
    pub heartbeat_timeout: i64,
    pub lifecycle_hook_name: String,
    pub lifecycle_transition: String,
    pub notification_metadata: String,
    pub notification_target_arn: String,
    pub region: String,
    pub role_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutLifecycleHookOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLifecycleHookInput {
    pub auto_scaling_group_name: String,
    pub lifecycle_hook_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLifecycleHookOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLifecycleHooksInput {
    pub auto_scaling_group_name: String,
    pub lifecycle_hook_names: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLifecycleHooksOutput {
    pub lifecycle_hooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLifecycleHookTypesInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLifecycleHookTypesOutput {
    pub lifecycle_hook_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompleteLifecycleActionInput {
    pub auto_scaling_group_name: String,
    pub instance_id: String,
    pub lifecycle_action_result: String,
    pub lifecycle_action_token: String,
    pub lifecycle_hook_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompleteLifecycleActionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordLifecycleActionHeartbeatInput {
    pub auto_scaling_group_name: String,
    pub instance_id: String,
    pub lifecycle_action_token: String,
    pub lifecycle_hook_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordLifecycleActionHeartbeatOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutNotificationConfigurationInput {
    pub auto_scaling_group_name: String,
    pub notification_types: Vec<String>,
    pub region: String,
    pub topic_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutNotificationConfigurationOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNotificationConfigurationInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub topic_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNotificationConfigurationOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNotificationConfigurationsInput {
    pub auto_scaling_group_names: Vec<String>,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNotificationConfigurationsOutput {
    pub next_token: String,
    pub notification_configurations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingNotificationTypesInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAutoScalingNotificationTypesOutput {
    pub auto_scaling_notification_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachLoadBalancersInput {
    pub auto_scaling_group_name: String,
    pub load_balancer_names: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachLoadBalancersOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachLoadBalancersInput {
    pub auto_scaling_group_name: String,
    pub load_balancer_names: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachLoadBalancersOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancersInput {
    pub auto_scaling_group_name: String,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancersOutput {
    pub load_balancers: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachLoadBalancerTargetGroupsInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub target_group_arns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachLoadBalancerTargetGroupsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachLoadBalancerTargetGroupsInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub target_group_arns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachLoadBalancerTargetGroupsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancerTargetGroupsInput {
    pub auto_scaling_group_name: String,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancerTargetGroupsOutput {
    pub load_balancer_target_groups: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachTrafficSourcesInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub traffic_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachTrafficSourcesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachTrafficSourcesInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub traffic_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachTrafficSourcesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTrafficSourcesInput {
    pub auto_scaling_group_name: String,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
    pub traffic_source_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTrafficSourcesOutput {
    pub next_token: String,
    pub traffic_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScalingActivitiesInput {
    pub activity_ids: Vec<String>,
    pub auto_scaling_group_name: String,
    #[serde(default)]
    pub include_deleted_groups: bool,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScalingActivitiesOutput {
    pub activities: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScalingProcessTypesInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScalingProcessTypesOutput {
    pub processes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SuspendProcessesInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub scaling_processes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SuspendProcessesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeProcessesInput {
    pub auto_scaling_group_name: String,
    pub region: String,
    pub scaling_processes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeProcessesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartInstanceRefreshInput {
    pub auto_scaling_group_name: String,
    pub desired_configuration: String,
    pub preferences: String,
    pub region: String,
    pub strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartInstanceRefreshOutput {
    pub instance_refresh_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelInstanceRefreshInput {
    pub auto_scaling_group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelInstanceRefreshOutput {
    pub instance_refresh_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstanceRefreshesInput {
    pub auto_scaling_group_name: String,
    pub instance_refresh_ids: Vec<String>,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstanceRefreshesOutput {
    pub instance_refreshes: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RollbackInstanceRefreshInput {
    pub auto_scaling_group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RollbackInstanceRefreshOutput {
    pub instance_refresh_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutWarmPoolInput {
    pub auto_scaling_group_name: String,
    pub instance_reuse_policy: String,
    pub max_group_prepared_capacity: i64,
    pub min_size: i64,
    pub pool_state: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutWarmPoolOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteWarmPoolInput {
    pub auto_scaling_group_name: String,
    #[serde(default)]
    pub force_delete: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteWarmPoolOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeWarmPoolInput {
    pub auto_scaling_group_name: String,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
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
pub struct DescribeAccountLimitsInput {
    pub region: String,
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
pub struct DescribeAdjustmentTypesInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAdjustmentTypesOutput {
    pub adjustment_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTerminationPolicyTypesInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTerminationPolicyTypesOutput {
    pub termination_policy_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeMetricCollectionTypesInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeMetricCollectionTypesOutput {
    pub granularities: Vec<String>,
    pub metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableMetricsCollectionInput {
    pub auto_scaling_group_name: String,
    pub granularity: String,
    pub metrics: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableMetricsCollectionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableMetricsCollectionInput {
    pub auto_scaling_group_name: String,
    pub metrics: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableMetricsCollectionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPredictiveScalingForecastInput {
    pub auto_scaling_group_name: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub policy_name: String,
    pub region: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
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
pub struct CreateOrUpdateTagsInput {
    pub region: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOrUpdateTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTagsInput {
    pub region: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTagsInput {
    pub filters: Vec<String>,
    pub max_records: i64,
    pub next_token: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTagsOutput {
    pub next_token: String,
    pub tags: Vec<String>,
}

#[async_trait]
pub trait AutoscalingAction {
    async fn create_auto_scaling_group(&self, input: CreateAutoScalingGroupInput) -> Result<CreateAutoScalingGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_auto_scaling_group(&self, input: DeleteAutoScalingGroupInput) -> Result<DeleteAutoScalingGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_auto_scaling_groups(&self, input: DescribeAutoScalingGroupsInput) -> Result<DescribeAutoScalingGroupsOutput, Box<dyn std::error::Error>>;
    async fn update_auto_scaling_group(&self, input: UpdateAutoScalingGroupInput) -> Result<UpdateAutoScalingGroupOutput, Box<dyn std::error::Error>>;
    async fn set_desired_capacity(&self, input: SetDesiredCapacityInput) -> Result<SetDesiredCapacityOutput, Box<dyn std::error::Error>>;
    async fn attach_instances(&self, input: AttachInstancesInput) -> Result<AttachInstancesOutput, Box<dyn std::error::Error>>;
    async fn detach_instances(&self, input: DetachInstancesInput) -> Result<DetachInstancesOutput, Box<dyn std::error::Error>>;
    async fn enter_standby(&self, input: EnterStandbyInput) -> Result<EnterStandbyOutput, Box<dyn std::error::Error>>;
    async fn exit_standby(&self, input: ExitStandbyInput) -> Result<ExitStandbyOutput, Box<dyn std::error::Error>>;
    async fn set_instance_health(&self, input: SetInstanceHealthInput) -> Result<SetInstanceHealthOutput, Box<dyn std::error::Error>>;
    async fn set_instance_protection(&self, input: SetInstanceProtectionInput) -> Result<SetInstanceProtectionOutput, Box<dyn std::error::Error>>;
    async fn terminate_instance_in_auto_scaling_group(&self, input: TerminateInstanceInAutoScalingGroupInput) -> Result<TerminateInstanceInAutoScalingGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_auto_scaling_instances(&self, input: DescribeAutoScalingInstancesInput) -> Result<DescribeAutoScalingInstancesOutput, Box<dyn std::error::Error>>;
    async fn create_launch_configuration(&self, input: CreateLaunchConfigurationInput) -> Result<CreateLaunchConfigurationOutput, Box<dyn std::error::Error>>;
    async fn delete_launch_configuration(&self, input: DeleteLaunchConfigurationInput) -> Result<DeleteLaunchConfigurationOutput, Box<dyn std::error::Error>>;
    async fn describe_launch_configurations(&self, input: DescribeLaunchConfigurationsInput) -> Result<DescribeLaunchConfigurationsOutput, Box<dyn std::error::Error>>;
    async fn put_scaling_policy(&self, input: PutScalingPolicyInput) -> Result<PutScalingPolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_policy(&self, input: DeletePolicyInput) -> Result<DeletePolicyOutput, Box<dyn std::error::Error>>;
    async fn describe_policies(&self, input: DescribePoliciesInput) -> Result<DescribePoliciesOutput, Box<dyn std::error::Error>>;
    async fn execute_policy(&self, input: ExecutePolicyInput) -> Result<ExecutePolicyOutput, Box<dyn std::error::Error>>;
    async fn put_scheduled_update_group_action(&self, input: PutScheduledUpdateGroupActionInput) -> Result<PutScheduledUpdateGroupActionOutput, Box<dyn std::error::Error>>;
    async fn delete_scheduled_action(&self, input: DeleteScheduledActionInput) -> Result<DeleteScheduledActionOutput, Box<dyn std::error::Error>>;
    async fn describe_scheduled_actions(&self, input: DescribeScheduledActionsInput) -> Result<DescribeScheduledActionsOutput, Box<dyn std::error::Error>>;
    async fn batch_delete_scheduled_action(&self, input: BatchDeleteScheduledActionInput) -> Result<BatchDeleteScheduledActionOutput, Box<dyn std::error::Error>>;
    async fn batch_put_scheduled_update_group_action(&self, input: BatchPutScheduledUpdateGroupActionInput) -> Result<BatchPutScheduledUpdateGroupActionOutput, Box<dyn std::error::Error>>;
    async fn put_lifecycle_hook(&self, input: PutLifecycleHookInput) -> Result<PutLifecycleHookOutput, Box<dyn std::error::Error>>;
    async fn delete_lifecycle_hook(&self, input: DeleteLifecycleHookInput) -> Result<DeleteLifecycleHookOutput, Box<dyn std::error::Error>>;
    async fn describe_lifecycle_hooks(&self, input: DescribeLifecycleHooksInput) -> Result<DescribeLifecycleHooksOutput, Box<dyn std::error::Error>>;
    async fn describe_lifecycle_hook_types(&self, input: DescribeLifecycleHookTypesInput) -> Result<DescribeLifecycleHookTypesOutput, Box<dyn std::error::Error>>;
    async fn complete_lifecycle_action(&self, input: CompleteLifecycleActionInput) -> Result<CompleteLifecycleActionOutput, Box<dyn std::error::Error>>;
    async fn record_lifecycle_action_heartbeat(&self, input: RecordLifecycleActionHeartbeatInput) -> Result<RecordLifecycleActionHeartbeatOutput, Box<dyn std::error::Error>>;
    async fn put_notification_configuration(&self, input: PutNotificationConfigurationInput) -> Result<PutNotificationConfigurationOutput, Box<dyn std::error::Error>>;
    async fn delete_notification_configuration(&self, input: DeleteNotificationConfigurationInput) -> Result<DeleteNotificationConfigurationOutput, Box<dyn std::error::Error>>;
    async fn describe_notification_configurations(&self, input: DescribeNotificationConfigurationsInput) -> Result<DescribeNotificationConfigurationsOutput, Box<dyn std::error::Error>>;
    async fn describe_auto_scaling_notification_types(&self, input: DescribeAutoScalingNotificationTypesInput) -> Result<DescribeAutoScalingNotificationTypesOutput, Box<dyn std::error::Error>>;
    async fn attach_load_balancers(&self, input: AttachLoadBalancersInput) -> Result<AttachLoadBalancersOutput, Box<dyn std::error::Error>>;
    async fn detach_load_balancers(&self, input: DetachLoadBalancersInput) -> Result<DetachLoadBalancersOutput, Box<dyn std::error::Error>>;
    async fn describe_load_balancers(&self, input: DescribeLoadBalancersInput) -> Result<DescribeLoadBalancersOutput, Box<dyn std::error::Error>>;
    async fn attach_load_balancer_target_groups(&self, input: AttachLoadBalancerTargetGroupsInput) -> Result<AttachLoadBalancerTargetGroupsOutput, Box<dyn std::error::Error>>;
    async fn detach_load_balancer_target_groups(&self, input: DetachLoadBalancerTargetGroupsInput) -> Result<DetachLoadBalancerTargetGroupsOutput, Box<dyn std::error::Error>>;
    async fn describe_load_balancer_target_groups(&self, input: DescribeLoadBalancerTargetGroupsInput) -> Result<DescribeLoadBalancerTargetGroupsOutput, Box<dyn std::error::Error>>;
    async fn attach_traffic_sources(&self, input: AttachTrafficSourcesInput) -> Result<AttachTrafficSourcesOutput, Box<dyn std::error::Error>>;
    async fn detach_traffic_sources(&self, input: DetachTrafficSourcesInput) -> Result<DetachTrafficSourcesOutput, Box<dyn std::error::Error>>;
    async fn describe_traffic_sources(&self, input: DescribeTrafficSourcesInput) -> Result<DescribeTrafficSourcesOutput, Box<dyn std::error::Error>>;
    async fn describe_scaling_activities(&self, input: DescribeScalingActivitiesInput) -> Result<DescribeScalingActivitiesOutput, Box<dyn std::error::Error>>;
    async fn describe_scaling_process_types(&self, input: DescribeScalingProcessTypesInput) -> Result<DescribeScalingProcessTypesOutput, Box<dyn std::error::Error>>;
    async fn suspend_processes(&self, input: SuspendProcessesInput) -> Result<SuspendProcessesOutput, Box<dyn std::error::Error>>;
    async fn resume_processes(&self, input: ResumeProcessesInput) -> Result<ResumeProcessesOutput, Box<dyn std::error::Error>>;
    async fn start_instance_refresh(&self, input: StartInstanceRefreshInput) -> Result<StartInstanceRefreshOutput, Box<dyn std::error::Error>>;
    async fn cancel_instance_refresh(&self, input: CancelInstanceRefreshInput) -> Result<CancelInstanceRefreshOutput, Box<dyn std::error::Error>>;
    async fn describe_instance_refreshes(&self, input: DescribeInstanceRefreshesInput) -> Result<DescribeInstanceRefreshesOutput, Box<dyn std::error::Error>>;
    async fn rollback_instance_refresh(&self, input: RollbackInstanceRefreshInput) -> Result<RollbackInstanceRefreshOutput, Box<dyn std::error::Error>>;
    async fn put_warm_pool(&self, input: PutWarmPoolInput) -> Result<PutWarmPoolOutput, Box<dyn std::error::Error>>;
    async fn delete_warm_pool(&self, input: DeleteWarmPoolInput) -> Result<DeleteWarmPoolOutput, Box<dyn std::error::Error>>;
    async fn describe_warm_pool(&self, input: DescribeWarmPoolInput) -> Result<DescribeWarmPoolOutput, Box<dyn std::error::Error>>;
    async fn describe_account_limits(&self, input: DescribeAccountLimitsInput) -> Result<DescribeAccountLimitsOutput, Box<dyn std::error::Error>>;
    async fn describe_adjustment_types(&self, input: DescribeAdjustmentTypesInput) -> Result<DescribeAdjustmentTypesOutput, Box<dyn std::error::Error>>;
    async fn describe_termination_policy_types(&self, input: DescribeTerminationPolicyTypesInput) -> Result<DescribeTerminationPolicyTypesOutput, Box<dyn std::error::Error>>;
    async fn describe_metric_collection_types(&self, input: DescribeMetricCollectionTypesInput) -> Result<DescribeMetricCollectionTypesOutput, Box<dyn std::error::Error>>;
    async fn enable_metrics_collection(&self, input: EnableMetricsCollectionInput) -> Result<EnableMetricsCollectionOutput, Box<dyn std::error::Error>>;
    async fn disable_metrics_collection(&self, input: DisableMetricsCollectionInput) -> Result<DisableMetricsCollectionOutput, Box<dyn std::error::Error>>;
    async fn get_predictive_scaling_forecast(&self, input: GetPredictiveScalingForecastInput) -> Result<GetPredictiveScalingForecastOutput, Box<dyn std::error::Error>>;
    async fn create_or_update_tags(&self, input: CreateOrUpdateTagsInput) -> Result<CreateOrUpdateTagsOutput, Box<dyn std::error::Error>>;
    async fn delete_tags(&self, input: DeleteTagsInput) -> Result<DeleteTagsOutput, Box<dyn std::error::Error>>;
    async fn describe_tags(&self, input: DescribeTagsInput) -> Result<DescribeTagsOutput, Box<dyn std::error::Error>>;
}

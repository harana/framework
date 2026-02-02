// Harana Actions - AWS Auto Scaling Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// create_auto_scaling_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutoScalingGroupOutput {
    pub success: bool,
}

// delete_auto_scaling_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAutoScalingGroupOutput {
    pub success: bool,
}

// describe_auto_scaling_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeAutoScalingGroupsOutput {
    pub auto_scaling_groups: Vec<AutoScalingGroup>,
    pub next_token: Option<String>,
}

// update_auto_scaling_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAutoScalingGroupOutput {
    pub success: bool,
}

// set_desired_capacity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDesiredCapacityOutput {
    pub success: bool,
}

// attach_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachInstancesOutput {
    pub success: bool,
}

// detach_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachInstancesOutput {
    pub activities: Vec<ScalingActivity>,
    pub success: bool,
}

// create_scaling_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScalingPolicyOutput {
    pub alarms: Vec<Alarm>,
    pub policy_arn: String,
    pub success: bool,
}

// delete_scaling_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScalingPolicyOutput {
    pub success: bool,
}

// describe_scaling_activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeScalingActivitiesOutput {
    pub activities: Vec<ScalingActivity>,
    pub next_token: Option<String>,
}

// suspend_processes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendProcessesOutput {
    pub success: bool,
}

// resume_processes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeProcessesOutput {
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingGroup {
    pub auto_scaling_group_name: String,
    pub auto_scaling_group_arn: String,
    pub launch_configuration_name: Option<String>,
    pub launch_template: Option<LaunchTemplateSpecification>,
    pub min_size: i32,
    pub max_size: i32,
    pub desired_capacity: i32,
    pub default_cooldown: i32,
    pub availability_zones: Vec<String>,
    pub load_balancer_names: Vec<String>,
    pub target_group_arns: Vec<String>,
    pub health_check_type: String,
    pub health_check_grace_period: i32,
    pub instances: Vec<Instance>,
    pub created_time: String,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub instance_id: String,
    pub instance_type: Option<String>,
    pub availability_zone: String,
    pub lifecycle_state: String,
    pub health_status: String,
    pub launch_configuration_name: Option<String>,
    pub launch_template: Option<LaunchTemplateSpecification>,
    pub protected_from_scale_in: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchTemplateSpecification {
    pub launch_template_id: Option<String>,
    pub launch_template_name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingActivity {
    pub activity_id: String,
    pub auto_scaling_group_name: String,
    pub description: Option<String>,
    pub cause: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub status_code: String,
    pub status_message: Option<String>,
    pub progress: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alarm {
    pub alarm_name: String,
    pub alarm_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingTag {
    pub key: String,
    pub propagate_at_launch: bool,
    pub resource_id: String,
    pub resource_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingFilter {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHookSpecification {
    pub lifecycle_hook_name: String,
    pub lifecycle_transition: String,
    pub default_result: Option<String>,
    pub heartbeat_timeout: Option<i32>,
    pub notification_metadata: Option<String>,
    pub notification_target_arn: Option<String>,
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedInstancesPolicy {
    pub launch_template: Option<LaunchTemplateSpecification>,
    pub instances_distribution: Option<InstancesDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancesDistribution {
    pub on_demand_allocation_strategy: Option<String>,
    pub on_demand_base_capacity: Option<i32>,
    pub on_demand_percentage_above_base_capacity: Option<i32>,
    pub spot_allocation_strategy: Option<String>,
    pub spot_instance_pools: Option<i32>,
    pub spot_max_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceMaintenancePolicy {
    pub min_healthy_percentage: Option<i32>,
    pub max_healthy_percentage: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSourceIdentifier {
    pub identifier: String,
    pub traffic_source_type: Option<String>,
}

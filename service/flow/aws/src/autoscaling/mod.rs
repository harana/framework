// Harana Actions - AWS Auto Scaling Module
//
// This module provides AWS Auto Scaling actions.

pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_autoscaling::{
    Client,
    config::Region,
    types::{
        Filter, InstancesDistribution as AwsInstancesDistribution, LaunchTemplate as AwsLaunchTemplate,
        LaunchTemplateSpecification as AwsLaunchTemplateSpecification, MixedInstancesPolicy as AwsMixedInstancesPolicy,
        Tag as AutoScalingTagType,
    },
};
use output::*;

/// Creates an Auto Scaling client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let autoscaling_config = if let Some(region_str) = region {
        aws_sdk_autoscaling::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_autoscaling::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(autoscaling_config))
}

/// Converts LaunchTemplateSpecification to AWS SDK format
fn launch_template_to_aws(spec: &LaunchTemplateSpecification) -> AwsLaunchTemplateSpecification {
    let mut builder = AwsLaunchTemplateSpecification::builder();

    if let Some(ref id) = spec.launch_template_id {
        builder = builder.launch_template_id(id);
    }
    if let Some(ref name) = spec.launch_template_name {
        builder = builder.launch_template_name(name);
    }
    if let Some(ref version) = spec.version {
        builder = builder.version(version);
    }

    builder.build()
}

/// Converts MixedInstancesPolicy to AWS SDK format
fn mixed_instances_policy_to_aws(policy: &MixedInstancesPolicy) -> AwsMixedInstancesPolicy {
    let mut builder = AwsMixedInstancesPolicy::builder();

    if let Some(ref template) = policy.launch_template {
        let aws_template = AwsLaunchTemplate::builder()
            .launch_template_specification(launch_template_to_aws(template))
            .build();
        builder = builder.launch_template(aws_template);
    }

    if let Some(ref dist) = policy.instances_distribution {
        let mut dist_builder = AwsInstancesDistribution::builder();

        if let Some(ref strategy) = dist.on_demand_allocation_strategy {
            dist_builder = dist_builder.on_demand_allocation_strategy(strategy.as_str());
        }
        if let Some(capacity) = dist.on_demand_base_capacity {
            dist_builder = dist_builder.on_demand_base_capacity(capacity);
        }
        if let Some(percentage) = dist.on_demand_percentage_above_base_capacity {
            dist_builder = dist_builder.on_demand_percentage_above_base_capacity(percentage);
        }
        if let Some(ref strategy) = dist.spot_allocation_strategy {
            dist_builder = dist_builder.spot_allocation_strategy(strategy.as_str());
        }
        if let Some(pools) = dist.spot_instance_pools {
            dist_builder = dist_builder.spot_instance_pools(pools);
        }
        if let Some(ref price) = dist.spot_max_price {
            dist_builder = dist_builder.spot_max_price(price);
        }

        builder = builder.instances_distribution(dist_builder.build());
    }

    builder.build()
}

/// Converts AutoScalingTag to AWS SDK format
fn tag_to_aws(tag: &AutoScalingTag) -> AutoScalingTagType {
    AutoScalingTagType::builder()
        .key(&tag.key)
        .value(&tag.value)
        .propagate_at_launch(tag.propagate_at_launch)
        .resource_id(&tag.resource_id)
        .resource_type(&tag.resource_type)
        .build()
}

/// Converts AutoScalingFilter to AWS SDK format
fn filter_to_aws(filter: &AutoScalingFilter) -> Filter {
    Filter::builder()
        .name(&filter.name)
        .set_values(Some(filter.values.clone()))
        .build()
}

/// Converts AWS SDK AutoScalingGroup to our AutoScalingGroup type
fn convert_auto_scaling_group(group: &aws_sdk_autoscaling::types::AutoScalingGroup) -> AutoScalingGroup {
    AutoScalingGroup {
        auto_scaling_group_name: group.auto_scaling_group_name().unwrap_or_default().to_string(),
        auto_scaling_group_arn: group.auto_scaling_group_arn().unwrap_or_default().to_string(),
        launch_configuration_name: group.launch_configuration_name().map(|s| s.to_string()),
        launch_template: group.launch_template().map(|lt| LaunchTemplateSpecification {
            launch_template_id: lt.launch_template_id().map(|s| s.to_string()),
            launch_template_name: lt.launch_template_name().map(|s| s.to_string()),
            version: lt.version().map(|s| s.to_string()),
        }),
        min_size: group.min_size().unwrap_or(0),
        max_size: group.max_size().unwrap_or(0),
        desired_capacity: group.desired_capacity().unwrap_or(0),
        default_cooldown: group.default_cooldown().unwrap_or(0),
        availability_zones: group.availability_zones().iter().map(|s| s.to_string()).collect(),
        load_balancer_names: group.load_balancer_names().iter().map(|s| s.to_string()).collect(),
        target_group_arns: group.target_group_arns().iter().map(|s| s.to_string()).collect(),
        health_check_type: group.health_check_type().unwrap_or_default().to_string(),
        health_check_grace_period: group.health_check_grace_period().unwrap_or(0),
        instances: group
            .instances()
            .iter()
            .map(|inst| Instance {
                instance_id: inst.instance_id().unwrap_or_default().to_string(),
                instance_type: inst.instance_type().map(|s| s.to_string()),
                availability_zone: inst.availability_zone().unwrap_or_default().to_string(),
                lifecycle_state: inst
                    .lifecycle_state()
                    .map(|s| s.as_str().to_string())
                    .unwrap_or_default(),
                health_status: inst.health_status().unwrap_or_default().to_string(),
                launch_configuration_name: inst.launch_configuration_name().map(|s| s.to_string()),
                launch_template: inst.launch_template().map(|lt| LaunchTemplateSpecification {
                    launch_template_id: lt.launch_template_id().map(|s| s.to_string()),
                    launch_template_name: lt.launch_template_name().map(|s| s.to_string()),
                    version: lt.version().map(|s| s.to_string()),
                }),
                protected_from_scale_in: inst.protected_from_scale_in().unwrap_or(false),
            })
            .collect(),
        created_time: group.created_time().map(|t| t.to_string()).unwrap_or_default(),
        status: group.status().map(|s| s.to_string()),
    }
}

/// Converts AWS SDK Activity to our ScalingActivity type
fn convert_scaling_activity(activity: &aws_sdk_autoscaling::types::Activity) -> ScalingActivity {
    ScalingActivity {
        activity_id: activity.activity_id().unwrap_or_default().to_string(),
        auto_scaling_group_name: activity.auto_scaling_group_name().unwrap_or_default().to_string(),
        description: activity.description().map(|s| s.to_string()),
        cause: activity.cause().unwrap_or_default().to_string(),
        start_time: activity.start_time().map(|t| t.to_string()).unwrap_or_default(),
        end_time: activity.end_time().map(|t| t.to_string()),
        status_code: activity
            .status_code()
            .map(|s| s.as_str().to_string())
            .unwrap_or_default(),
        status_message: activity.status_message().map(|s| s.to_string()),
        progress: activity.progress(),
    }
}

/// Converts AWS SDK Alarm to our Alarm type
fn convert_alarm(alarm: &aws_sdk_autoscaling::types::Alarm) -> Alarm {
    Alarm {
        alarm_name: alarm.alarm_name().unwrap_or_default().to_string(),
        alarm_arn: alarm.alarm_arn().unwrap_or_default().to_string(),
    }
}

/// Create Auto Scaling Group
pub async fn create_auto_scaling_group(
    auto_scaling_group_name: &str,
    max_size: i32,
    min_size: i32,
    availability_zones: Option<Vec<String>>,
    default_cooldown: Option<i32>,
    desired_capacity: Option<i32>,
    health_check_grace_period: Option<i32>,
    health_check_type: Option<&str>,
    launch_configuration_name: Option<&str>,
    launch_template: Option<LaunchTemplateSpecification>,
    load_balancer_names: Option<Vec<String>>,
    mixed_instances_policy: Option<MixedInstancesPolicy>,
    region: Option<&str>,
    tags: Option<Vec<AutoScalingTag>>,
    target_group_arns: Option<Vec<String>>,
    vpc_zone_identifier: Option<&str>,
) -> Result<CreateAutoScalingGroupOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .create_auto_scaling_group()
        .auto_scaling_group_name(auto_scaling_group_name)
        .max_size(max_size)
        .min_size(min_size);

    if let Some(zones) = availability_zones {
        request = request.set_availability_zones(Some(zones));
    }

    if let Some(cooldown) = default_cooldown {
        request = request.default_cooldown(cooldown);
    }

    if let Some(capacity) = desired_capacity {
        request = request.desired_capacity(capacity);
    }

    if let Some(grace_period) = health_check_grace_period {
        request = request.health_check_grace_period(grace_period);
    }

    if let Some(health_type) = health_check_type {
        request = request.health_check_type(health_type);
    }

    if let Some(launch_config) = launch_configuration_name {
        request = request.launch_configuration_name(launch_config);
    }

    if let Some(ref template) = launch_template {
        request = request.launch_template(launch_template_to_aws(template));
    }

    if let Some(lb_names) = load_balancer_names {
        request = request.set_load_balancer_names(Some(lb_names));
    }

    if let Some(ref policy) = mixed_instances_policy {
        request = request.mixed_instances_policy(mixed_instances_policy_to_aws(policy));
    }

    if let Some(tag_list) = tags {
        let aws_tags: Vec<AutoScalingTagType> = tag_list.iter().map(tag_to_aws).collect();
        request = request.set_tags(Some(aws_tags));
    }

    if let Some(target_arns) = target_group_arns {
        request = request.set_target_group_arns(Some(target_arns));
    }

    if let Some(vpc_zones) = vpc_zone_identifier {
        request = request.vpc_zone_identifier(vpc_zones);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to create auto scaling group: {}", e))?;

    Ok(CreateAutoScalingGroupOutput { success: true })
}

/// Delete Auto Scaling Group
pub async fn delete_auto_scaling_group(
    auto_scaling_group_name: &str,
    force_delete: Option<bool>,
    region: Option<&str>,
) -> Result<DeleteAutoScalingGroupOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .delete_auto_scaling_group()
        .auto_scaling_group_name(auto_scaling_group_name);

    if let Some(force) = force_delete {
        request = request.force_delete(force);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to delete auto scaling group: {}", e))?;

    Ok(DeleteAutoScalingGroupOutput { success: true })
}

/// Describe Auto Scaling Groups
pub async fn describe_auto_scaling_groups(
    auto_scaling_group_names: Option<Vec<String>>,
    filters: Option<Vec<AutoScalingFilter>>,
    max_records: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<DescribeAutoScalingGroupsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_auto_scaling_groups();

    if let Some(names) = auto_scaling_group_names {
        request = request.set_auto_scaling_group_names(Some(names));
    }

    if let Some(filter_list) = filters {
        let aws_filters: Vec<Filter> = filter_list.iter().map(filter_to_aws).collect();
        request = request.set_filters(Some(aws_filters));
    }

    if let Some(max) = max_records {
        request = request.max_records(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe auto scaling groups: {}", e))?;

    let groups: Vec<AutoScalingGroup> = response
        .auto_scaling_groups()
        .iter()
        .map(convert_auto_scaling_group)
        .collect();

    Ok(DescribeAutoScalingGroupsOutput {
        auto_scaling_groups: groups,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Update Auto Scaling Group
pub async fn update_auto_scaling_group(
    auto_scaling_group_name: &str,
    availability_zones: Option<Vec<String>>,
    default_cooldown: Option<i32>,
    desired_capacity: Option<i32>,
    health_check_grace_period: Option<i32>,
    health_check_type: Option<&str>,
    launch_configuration_name: Option<&str>,
    launch_template: Option<LaunchTemplateSpecification>,
    max_size: Option<i32>,
    min_size: Option<i32>,
    mixed_instances_policy: Option<MixedInstancesPolicy>,
    region: Option<&str>,
    vpc_zone_identifier: Option<&str>,
) -> Result<UpdateAutoScalingGroupOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .update_auto_scaling_group()
        .auto_scaling_group_name(auto_scaling_group_name);

    if let Some(zones) = availability_zones {
        request = request.set_availability_zones(Some(zones));
    }

    if let Some(cooldown) = default_cooldown {
        request = request.default_cooldown(cooldown);
    }

    if let Some(capacity) = desired_capacity {
        request = request.desired_capacity(capacity);
    }

    if let Some(grace_period) = health_check_grace_period {
        request = request.health_check_grace_period(grace_period);
    }

    if let Some(health_type) = health_check_type {
        request = request.health_check_type(health_type);
    }

    if let Some(launch_config) = launch_configuration_name {
        request = request.launch_configuration_name(launch_config);
    }

    if let Some(ref template) = launch_template {
        request = request.launch_template(launch_template_to_aws(template));
    }

    if let Some(max) = max_size {
        request = request.max_size(max);
    }

    if let Some(min) = min_size {
        request = request.min_size(min);
    }

    if let Some(ref policy) = mixed_instances_policy {
        request = request.mixed_instances_policy(mixed_instances_policy_to_aws(policy));
    }

    if let Some(vpc_zones) = vpc_zone_identifier {
        request = request.vpc_zone_identifier(vpc_zones);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to update auto scaling group: {}", e))?;

    Ok(UpdateAutoScalingGroupOutput { success: true })
}

/// Set Desired Capacity
pub async fn set_desired_capacity(
    auto_scaling_group_name: &str,
    desired_capacity: i32,
    honor_cooldown: Option<bool>,
    region: Option<&str>,
) -> Result<SetDesiredCapacityOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .set_desired_capacity()
        .auto_scaling_group_name(auto_scaling_group_name)
        .desired_capacity(desired_capacity);

    if let Some(honor) = honor_cooldown {
        request = request.honor_cooldown(honor);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to set desired capacity: {}", e))?;

    Ok(SetDesiredCapacityOutput { success: true })
}

/// Attach Instances
pub async fn attach_instances(
    auto_scaling_group_name: &str,
    instance_ids: Vec<String>,
    region: Option<&str>,
) -> Result<AttachInstancesOutput, String> {
    let client = create_client(region).await?;

    client
        .attach_instances()
        .auto_scaling_group_name(auto_scaling_group_name)
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .map_err(|e| format!("Failed to attach instances: {}", e))?;

    Ok(AttachInstancesOutput { success: true })
}

/// Detach Instances
pub async fn detach_instances(
    auto_scaling_group_name: &str,
    instance_ids: Vec<String>,
    should_decrement_desired_capacity: bool,
    region: Option<&str>,
) -> Result<DetachInstancesOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .detach_instances()
        .auto_scaling_group_name(auto_scaling_group_name)
        .set_instance_ids(Some(instance_ids))
        .should_decrement_desired_capacity(should_decrement_desired_capacity)
        .send()
        .await
        .map_err(|e| format!("Failed to detach instances: {}", e))?;

    let activities: Vec<ScalingActivity> = response.activities().iter().map(convert_scaling_activity).collect();

    Ok(DetachInstancesOutput {
        activities,
        success: true,
    })
}

/// Create Scaling Policy
pub async fn create_scaling_policy(
    auto_scaling_group_name: &str,
    policy_name: &str,
    policy_type: Option<&str>,
    adjustment_type: Option<&str>,
    scaling_adjustment: Option<i32>,
    cooldown: Option<i32>,
    region: Option<&str>,
) -> Result<CreateScalingPolicyOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .put_scaling_policy()
        .auto_scaling_group_name(auto_scaling_group_name)
        .policy_name(policy_name);

    if let Some(p_type) = policy_type {
        request = request.policy_type(p_type);
    }

    if let Some(adj_type) = adjustment_type {
        request = request.adjustment_type(adj_type);
    }

    if let Some(adjustment) = scaling_adjustment {
        request = request.scaling_adjustment(adjustment);
    }

    if let Some(cd) = cooldown {
        request = request.cooldown(cd);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create scaling policy: {}", e))?;

    let alarms: Vec<Alarm> = response.alarms().iter().map(convert_alarm).collect();

    Ok(CreateScalingPolicyOutput {
        alarms,
        policy_arn: response.policy_arn().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete Scaling Policy
pub async fn delete_scaling_policy(
    auto_scaling_group_name: &str,
    policy_name: &str,
    region: Option<&str>,
) -> Result<DeleteScalingPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_policy()
        .auto_scaling_group_name(auto_scaling_group_name)
        .policy_name(policy_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete scaling policy: {}", e))?;

    Ok(DeleteScalingPolicyOutput { success: true })
}

/// Describe Scaling Activities
pub async fn describe_scaling_activities(
    auto_scaling_group_name: Option<&str>,
    activity_ids: Option<Vec<String>>,
    max_records: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<DescribeScalingActivitiesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_scaling_activities();

    if let Some(name) = auto_scaling_group_name {
        request = request.auto_scaling_group_name(name);
    }

    if let Some(ids) = activity_ids {
        request = request.set_activity_ids(Some(ids));
    }

    if let Some(max) = max_records {
        request = request.max_records(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe scaling activities: {}", e))?;

    let activities: Vec<ScalingActivity> = response.activities().iter().map(convert_scaling_activity).collect();

    Ok(DescribeScalingActivitiesOutput {
        activities,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Suspend Processes
pub async fn suspend_processes(
    auto_scaling_group_name: &str,
    scaling_processes: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<SuspendProcessesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .suspend_processes()
        .auto_scaling_group_name(auto_scaling_group_name);

    if let Some(processes) = scaling_processes {
        request = request.set_scaling_processes(Some(processes));
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to suspend processes: {}", e))?;

    Ok(SuspendProcessesOutput { success: true })
}

/// Resume Processes
pub async fn resume_processes(
    auto_scaling_group_name: &str,
    scaling_processes: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<ResumeProcessesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .resume_processes()
        .auto_scaling_group_name(auto_scaling_group_name);

    if let Some(processes) = scaling_processes {
        request = request.set_scaling_processes(Some(processes));
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to resume processes: {}", e))?;

    Ok(ResumeProcessesOutput { success: true })
}

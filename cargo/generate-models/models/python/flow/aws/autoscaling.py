# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateAutoScalingGroup(BaseModel):
    """
    create_auto_scaling_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteAutoScalingGroup(BaseModel):
    """
    delete_auto_scaling_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAutoScalingGroups(BaseModel):
    """
    describe_auto_scaling_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateAutoScalingGroup(BaseModel):
    """
    update_auto_scaling_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetDesiredCapacity(BaseModel):
    """
    set_desired_capacity
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachInstances(BaseModel):
    """
    attach_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachInstances(BaseModel):
    """
    detach_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EnterStandby(BaseModel):
    """
    enter_standby
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExitStandby(BaseModel):
    """
    exit_standby
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetInstanceHealth(BaseModel):
    """
    set_instance_health
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SetInstanceProtection(BaseModel):
    """
    set_instance_protection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TerminateInstanceInAutoScalingGroup(BaseModel):
    """
    terminate_instance_in_auto_scaling_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAutoScalingInstances(BaseModel):
    """
    describe_auto_scaling_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateLaunchConfiguration(BaseModel):
    """
    create_launch_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteLaunchConfiguration(BaseModel):
    """
    delete_launch_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLaunchConfigurations(BaseModel):
    """
    describe_launch_configurations
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutScalingPolicy(BaseModel):
    """
    put_scaling_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeletePolicy(BaseModel):
    """
    delete_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribePolicies(BaseModel):
    """
    describe_policies
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExecutePolicy(BaseModel):
    """
    execute_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutScheduledUpdateGroupAction(BaseModel):
    """
    put_scheduled_update_group_action
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteScheduledAction(BaseModel):
    """
    delete_scheduled_action
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeScheduledActions(BaseModel):
    """
    describe_scheduled_actions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BatchDeleteScheduledAction(BaseModel):
    """
    batch_delete_scheduled_action
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BatchPutScheduledUpdateGroupAction(BaseModel):
    """
    batch_put_scheduled_update_group_action
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutLifecycleHook(BaseModel):
    """
    put_lifecycle_hook
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteLifecycleHook(BaseModel):
    """
    delete_lifecycle_hook
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLifecycleHooks(BaseModel):
    """
    describe_lifecycle_hooks
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLifecycleHookTypes(BaseModel):
    """
    describe_lifecycle_hook_types
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CompleteLifecycleAction(BaseModel):
    """
    complete_lifecycle_action
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RecordLifecycleActionHeartbeat(BaseModel):
    """
    record_lifecycle_action_heartbeat
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutNotificationConfiguration(BaseModel):
    """
    put_notification_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteNotificationConfiguration(BaseModel):
    """
    delete_notification_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeNotificationConfigurations(BaseModel):
    """
    describe_notification_configurations
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAutoScalingNotificationTypes(BaseModel):
    """
    describe_auto_scaling_notification_types
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachLoadBalancers(BaseModel):
    """
    attach_load_balancers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachLoadBalancers(BaseModel):
    """
    detach_load_balancers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLoadBalancers(BaseModel):
    """
    describe_load_balancers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachLoadBalancerTargetGroups(BaseModel):
    """
    attach_load_balancer_target_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachLoadBalancerTargetGroups(BaseModel):
    """
    detach_load_balancer_target_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLoadBalancerTargetGroups(BaseModel):
    """
    describe_load_balancer_target_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachTrafficSources(BaseModel):
    """
    attach_traffic_sources
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachTrafficSources(BaseModel):
    """
    detach_traffic_sources
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeTrafficSources(BaseModel):
    """
    describe_traffic_sources
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeScalingActivities(BaseModel):
    """
    describe_scaling_activities
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeScalingProcessTypes(BaseModel):
    """
    describe_scaling_process_types
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SuspendProcesses(BaseModel):
    """
    suspend_processes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ResumeProcesses(BaseModel):
    """
    resume_processes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StartInstanceRefresh(BaseModel):
    """
    start_instance_refresh
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelInstanceRefresh(BaseModel):
    """
    cancel_instance_refresh
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeInstanceRefreshes(BaseModel):
    """
    describe_instance_refreshes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RollbackInstanceRefresh(BaseModel):
    """
    rollback_instance_refresh
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutWarmPool(BaseModel):
    """
    put_warm_pool
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteWarmPool(BaseModel):
    """
    delete_warm_pool
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeWarmPool(BaseModel):
    """
    describe_warm_pool
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAccountLimits(BaseModel):
    """
    describe_account_limits
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAdjustmentTypes(BaseModel):
    """
    describe_adjustment_types
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeTerminationPolicyTypes(BaseModel):
    """
    describe_termination_policy_types
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeMetricCollectionTypes(BaseModel):
    """
    describe_metric_collection_types
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EnableMetricsCollection(BaseModel):
    """
    enable_metrics_collection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DisableMetricsCollection(BaseModel):
    """
    disable_metrics_collection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetPredictiveScalingForecast(BaseModel):
    """
    get_predictive_scaling_forecast
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateOrUpdateTags(BaseModel):
    """
    create_or_update_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTags(BaseModel):
    """
    delete_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeTags(BaseModel):
    """
    describe_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True



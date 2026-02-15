# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsAutoscalingGroup(BaseModel):
    """
    aws_autoscaling_group
    
    ID fields: account_id, auto_scaling_group_name
    """

    account_id: str
    auto_scaling_group_arn: Optional[str] = None
    auto_scaling_group_name: str
    availability_zones: Optional[str] = None
    capacity_rebalance: bool = Field(default=False)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_cooldown: int = Field(default=300)
    desired_capacity: int = Field(default=0)
    health_check_grace_period: int = Field(default=300)
    health_check_type: str = Field(default="ec2")
    launch_configuration_name: Optional[str] = None
    launch_template_id: Optional[str] = None
    launch_template_version: Optional[str] = None
    max_instance_lifetime: Optional[int] = None
    max_size: int
    min_size: int
    new_instances_protected_from_scale_in: bool = Field(default=False)
    placement_group: Optional[str] = None
    region: Optional[str] = None
    service_linked_role_arn: Optional[str] = None
    status: Optional[str] = None
    tags: Optional[str] = None
    target_group_arns: Optional[str] = None
    termination_policies: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_zone_identifier: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsAutoscalingLaunchConfiguration(BaseModel):
    """
    aws_autoscaling_launch_configuration
    
    ID fields: account_id, launch_configuration_name
    """

    account_id: str
    associate_public_ip_address: bool = Field(default=False)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    ebs_optimized: bool = Field(default=False)
    iam_instance_profile: Optional[str] = None
    image_id: Optional[str] = None
    instance_monitoring_enabled: bool = Field(default=False)
    instance_type: Optional[str] = None
    kernel_id: Optional[str] = None
    key_name: Optional[str] = None
    launch_configuration_arn: Optional[str] = None
    launch_configuration_name: str
    region: Optional[str] = None
    security_groups: Optional[str] = None
    spot_price: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsAutoscalingPolicy(BaseModel):
    """
    aws_autoscaling_policy
    
    ID fields: auto_scaling_group_id, policy_name
    """

    adjustment_type: str = Field(default="change_in_capacity")
    auto_scaling_group_id: str = Field(description="Reference: aws_autoscaling_group.id")
    cooldown: Optional[int] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    enabled: bool = Field(default=True)
    estimated_instance_warmup: Optional[int] = None
    metric_aggregation_type: Optional[str] = None
    min_adjustment_magnitude: Optional[int] = None
    policy_arn: Optional[str] = None
    policy_name: str
    policy_type: str = Field(default="simple_scaling")
    scaling_adjustment: Optional[int] = None
    target_tracking_configuration: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsAutoscalingScheduledAction(BaseModel):
    """
    aws_autoscaling_scheduled_action
    
    ID fields: auto_scaling_group_id, scheduled_action_name
    """

    auto_scaling_group_id: str = Field(description="Reference: aws_autoscaling_group.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    desired_capacity: Optional[int] = None
    end_time: Optional[datetime] = None
    max_size: Optional[int] = None
    min_size: Optional[int] = None
    recurrence: Optional[str] = None
    scheduled_action_arn: Optional[str] = None
    scheduled_action_name: str
    start_time: Optional[datetime] = None
    time_zone: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsAutoscalingLifecycleHook(BaseModel):
    """
    aws_autoscaling_lifecycle_hook
    
    ID fields: auto_scaling_group_id, lifecycle_hook_name
    """

    auto_scaling_group_id: str = Field(description="Reference: aws_autoscaling_group.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_result: str = Field(default="continue")
    heartbeat_timeout: int = Field(default=3600)
    lifecycle_hook_name: str
    lifecycle_transition: str
    notification_metadata: Optional[str] = None
    notification_target_arn: Optional[str] = None
    role_arn: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True



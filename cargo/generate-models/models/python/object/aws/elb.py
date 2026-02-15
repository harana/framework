# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsElbLoadBalancer(BaseModel):
    """
    aws_elb_load_balancer
    
    ID fields: account_id, load_balancer_arn
    """

    account_id: str
    availability_zones: Optional[str] = None
    canonical_hosted_zone_id: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    dns_name: Optional[str] = None
    ip_address_type: str = Field(default="ipv4")
    load_balancer_arn: str
    name: str
    region: Optional[str] = None
    scheme: str = Field(default="internet_facing")
    security_groups: Optional[str] = None
    state: str = Field(default="provisioning")
    subnets: Optional[str] = None
    tags: Optional[str] = None
    type: str = Field(default="application")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsElbTargetGroup(BaseModel):
    """
    aws_elb_target_group
    
    ID fields: account_id, target_group_arn
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    health_check_enabled: bool = Field(default=True)
    health_check_interval_seconds: int = Field(default=30)
    health_check_path: Optional[str] = None
    health_check_port: Optional[str] = None
    health_check_protocol: str = Field(default="http")
    health_check_timeout_seconds: Optional[int] = None
    healthy_threshold_count: int = Field(default=5)
    ip_address_type: str = Field(default="ipv4")
    load_balancer_arns: Optional[str] = None
    name: str
    port: Optional[int] = None
    protocol: str = Field(default="http")
    protocol_version: str = Field(default="http1")
    region: Optional[str] = None
    tags: Optional[str] = None
    target_group_arn: str
    target_type: str = Field(default="instance")
    unhealthy_threshold_count: int = Field(default=2)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsElbListener(BaseModel):
    """
    aws_elb_listener
    
    ID fields: load_balancer_id, listener_arn
    """

    certificates: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_actions: Optional[str] = None
    listener_arn: str
    load_balancer_id: str = Field(description="Reference: aws_elb_load_balancer.id")
    port: int
    protocol: str = Field(default="http")
    ssl_policy: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsElbListenerRule(BaseModel):
    """
    aws_elb_listener_rule
    
    ID fields: listener_id, rule_arn
    """

    actions: Optional[str] = None
    conditions: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_default: bool = Field(default=False)
    listener_id: str = Field(description="Reference: aws_elb_listener.id")
    priority: Optional[int] = None
    rule_arn: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True



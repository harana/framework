# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployEcs(BaseModel):
    """
    deploy_ecs
    
    ID fields: id
    """

    assign_public_ip: bool = Field(default=True)
    auto_rollback: bool = Field(default=True)
    cluster: str
    container_name: str
    cpu: str = Field(default=512)
    deployment: Optional[str] = None
    desired_count: int = Field(default=2)
    enabled: bool = Field(default=True)
    health_check_grace_period: int = Field(default=60)
    image: str
    launch_type: str = Field(default="fargate")
    load_balancer: Optional[str] = None
    memory: str = Field(default=1024)
    network_mode: str = Field(default="awsvpc")
    region: str
    security_groups: List[str]
    service: str
    subnets: List[str]
    task_definition: str
    wait_for_stable: bool = Field(default=True)
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployEcsDeployment(BaseModel):
    """
    deploy_ecs_deployment
    
    ID fields: id
    """

    maximum_percent: int = Field(default=200)
    minimum_healthy_percent: int = Field(default=100)
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployEcsLoadBalancer(BaseModel):
    """
    deploy_ecs_load_balancer
    
    ID fields: id
    """

    container_port: int
    target_group_arn: str
    class Config:
        from_attributes = True
        populate_by_name = True



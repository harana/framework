# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsEcsCluster(BaseModel):
    """
    aws_ecs_cluster
    
    ID fields: account_id, cluster_name
    """

    account_id: str
    active_services_count: int = Field(default=0)
    capacity_providers: Optional[str] = None
    cluster_arn: Optional[str] = None
    cluster_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    pending_tasks_count: int = Field(default=0)
    region: Optional[str] = None
    registered_container_instances_count: int = Field(default=0)
    running_tasks_count: int = Field(default=0)
    settings: Optional[str] = None
    status: str = Field(default="active")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEcsService(BaseModel):
    """
    aws_ecs_service
    
    ID fields: cluster_id, service_name
    """

    cluster_id: str = Field(description="Reference: aws_ecs_cluster.id")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    desired_count: int = Field(default=1)
    launch_type: str = Field(default="fargate")
    load_balancers: Optional[str] = None
    network_configuration: Optional[str] = None
    pending_count: int = Field(default=0)
    platform_version: Optional[str] = None
    running_count: int = Field(default=0)
    scheduling_strategy: str = Field(default="replica")
    service_arn: Optional[str] = None
    service_name: str
    status: str = Field(default="active")
    tags: Optional[str] = None
    task_definition: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEcsTaskDefinition(BaseModel):
    """
    aws_ecs_task_definition
    
    ID fields: account_id, family, revision
    """

    account_id: str
    container_definitions: Optional[str] = None
    cpu: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    execution_role_arn: Optional[str] = None
    family: str
    memory: Optional[str] = None
    network_mode: str = Field(default="awsvpc")
    region: Optional[str] = None
    requires_compatibilities: Optional[str] = None
    revision: int
    status: str = Field(default="active")
    tags: Optional[str] = None
    task_definition_arn: Optional[str] = None
    task_role_arn: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEcsTask(BaseModel):
    """
    aws_ecs_task
    
    ID fields: cluster_id, task_arn
    """

    cluster_id: str = Field(description="Reference: aws_ecs_cluster.id")
    connectivity: str = Field(default="connected")
    container_instance_arn: Optional[str] = None
    cpu: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    desired_status: str = Field(default="running")
    group: Optional[str] = None
    last_status: str = Field(default="provisioning")
    launch_type: str = Field(default="fargate")
    memory: Optional[str] = None
    platform_version: Optional[str] = None
    started_at: Optional[datetime] = None
    stopped_at: Optional[datetime] = None
    stopped_reason: Optional[str] = None
    tags: Optional[str] = None
    task_arn: str
    task_definition_arn: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True



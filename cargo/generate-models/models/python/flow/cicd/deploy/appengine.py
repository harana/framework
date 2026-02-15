# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DeployAppengine(BaseModel):
    """
    deploy_appengine
    
    ID fields: id
    """

    enabled: bool = Field(default=True)
    env_variables: str
    instance_type: str = Field(default="f2")
    project_id: str
    promote_traffic: bool = Field(default=True)
    region: str
    runtime: str = Field(default="nodejs20")
    scaling: Optional[str] = None
    service: str = Field(default="default")
    stop_previous_version: bool = Field(default=True)
    version: str = Field(default="auto")
    vpc_access_connector: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DeployAppengineScaling(BaseModel):
    """
    deploy_appengine_scaling
    
    ID fields: id
    """

    max_instances: int = Field(default=10)
    min_instances: int = Field(default=1)
    target_cpu_utilization: float = Field(default="0.65")
    target_throughput_utilization: float = Field(default="0.6")
    type: str = Field(default="automatic")
    class Config:
        from_attributes = True
        populate_by_name = True



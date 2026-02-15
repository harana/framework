# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class DockerImage(BaseModel):
    """
    docker_image
    
    ID fields: image_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    digest: Optional[str] = None
    image_id: str
    platform: Optional[str] = None
    repository: Optional[str] = None
    size: Optional[int] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerContainer(BaseModel):
    """
    docker_container
    
    ID fields: container_id
    """

    command: Optional[str] = None
    container_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    exit_code: Optional[int] = None
    image_id: str
    labels: Optional[str] = None
    name: Optional[str] = None
    network: Optional[str] = None
    ports: Optional[str] = None
    restart_policy: str = Field(default="no")
    started_at: Optional[datetime] = None
    status: str = Field(default="created")
    stopped_at: Optional[datetime] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetwork(BaseModel):
    """
    docker_network
    
    ID fields: network_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    driver: str = Field(default="bridge")
    labels: Optional[str] = None
    name: str
    network_id: str
    subnet: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerVolume(BaseModel):
    """
    docker_volume
    
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    driver: str = Field(default="local")
    labels: Optional[str] = None
    mountpoint: Optional[str] = None
    name: str
    class Config:
        from_attributes = True
        populate_by_name = True



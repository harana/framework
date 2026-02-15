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


class DockerContainerState(BaseModel):
    """
    docker_container_state
    
    ID fields: id
    """

    status: str
    running: bool
    paused: bool
    restarting: bool
    exit_code: int
    started_at: datetime
    finished_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerContainerConfig(BaseModel):
    """
    docker_container_config
    
    ID fields: id
    """

    hostname: str
    user: str
    env: List[str]
    cmd: List[str]
    working_dir: str
    entrypoint: List[str]
    labels: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetworkSettings(BaseModel):
    """
    docker_network_settings
    
    ID fields: id
    """

    networks: str
    ip_address: str
    gateway: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetworkEndpoint(BaseModel):
    """
    docker_network_endpoint
    
    ID fields: id
    """

    network_id: str
    ip_address: str
    gateway: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerPortMapping(BaseModel):
    """
    docker_port_mapping
    
    ID fields: id
    """

    container_port: int
    host_port: int
    protocol: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerVolumeMount(BaseModel):
    """
    docker_volume_mount
    
    ID fields: id
    """

    source: str
    target: str
    read_only: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerImageFilters(BaseModel):
    """
    docker_image_filters
    
    ID fields: id
    """

    dangling: bool
    label: List[str]
    reference: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerContainerFilters(BaseModel):
    """
    docker_container_filters
    
    ID fields: id
    """

    id: List[str]
    name: List[str]
    status: List[str]
    label: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetworkFilters(BaseModel):
    """
    docker_network_filters
    
    ID fields: id
    """

    driver: List[str]
    id: List[str]
    name: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class DockerVolumeFilters(BaseModel):
    """
    docker_volume_filters
    
    ID fields: id
    """

    dangling: bool
    driver: List[str]
    name: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True



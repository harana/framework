# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Build(BaseModel):
    """
    build
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Push(BaseModel):
    """
    push
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Pull(BaseModel):
    """
    pull
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListImages(BaseModel):
    """
    list_images
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveImage(BaseModel):
    """
    remove_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Tag(BaseModel):
    """
    tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Run(BaseModel):
    """
    run
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Start(BaseModel):
    """
    start
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Stop(BaseModel):
    """
    stop
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Restart(BaseModel):
    """
    restart
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveContainer(BaseModel):
    """
    remove_container
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListContainers(BaseModel):
    """
    list_containers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Logs(BaseModel):
    """
    logs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Exec(BaseModel):
    """
    exec
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class InspectContainer(BaseModel):
    """
    inspect_container
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateNetwork(BaseModel):
    """
    create_network
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveNetwork(BaseModel):
    """
    remove_network
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListNetworks(BaseModel):
    """
    list_networks
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateVolume(BaseModel):
    """
    create_volume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveVolume(BaseModel):
    """
    remove_volume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListVolumes(BaseModel):
    """
    list_volumes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Login(BaseModel):
    """
    login
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Logout(BaseModel):
    """
    logout
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerImage(BaseModel):
    """
    docker_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerContainer(BaseModel):
    """
    docker_container
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerContainerState(BaseModel):
    """
    docker_container_state
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerContainerConfig(BaseModel):
    """
    docker_container_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetworkSettings(BaseModel):
    """
    docker_network_settings
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetworkEndpoint(BaseModel):
    """
    docker_network_endpoint
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerPortMapping(BaseModel):
    """
    docker_port_mapping
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerVolumeMount(BaseModel):
    """
    docker_volume_mount
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetwork(BaseModel):
    """
    docker_network
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerVolume(BaseModel):
    """
    docker_volume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerImageFilters(BaseModel):
    """
    docker_image_filters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerContainerFilters(BaseModel):
    """
    docker_container_filters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerNetworkFilters(BaseModel):
    """
    docker_network_filters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DockerVolumeFilters(BaseModel):
    """
    docker_volume_filters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True



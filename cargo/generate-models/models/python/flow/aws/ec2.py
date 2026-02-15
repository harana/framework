# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class RunInstances(BaseModel):
    """
    run_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StartInstances(BaseModel):
    """
    start_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StopInstances(BaseModel):
    """
    stop_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RebootInstances(BaseModel):
    """
    reboot_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TerminateInstances(BaseModel):
    """
    terminate_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeInstances(BaseModel):
    """
    describe_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeInstanceStatus(BaseModel):
    """
    describe_instance_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyInstanceAttribute(BaseModel):
    """
    modify_instance_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetConsoleOutput(BaseModel):
    """
    get_console_output
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetPasswordData(BaseModel):
    """
    get_password_data
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateImage(BaseModel):
    """
    create_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeregisterImage(BaseModel):
    """
    deregister_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeImages(BaseModel):
    """
    describe_images
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CopyImage(BaseModel):
    """
    copy_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyImageAttribute(BaseModel):
    """
    modify_image_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateSnapshot(BaseModel):
    """
    create_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSnapshot(BaseModel):
    """
    delete_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeSnapshots(BaseModel):
    """
    describe_snapshots
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CopySnapshot(BaseModel):
    """
    copy_snapshot
    
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


class DeleteVolume(BaseModel):
    """
    delete_volume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeVolumes(BaseModel):
    """
    describe_volumes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachVolume(BaseModel):
    """
    attach_volume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachVolume(BaseModel):
    """
    detach_volume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyVolume(BaseModel):
    """
    modify_volume
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateKeyPair(BaseModel):
    """
    create_key_pair
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteKeyPair(BaseModel):
    """
    delete_key_pair
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeKeyPairs(BaseModel):
    """
    describe_key_pairs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImportKeyPair(BaseModel):
    """
    import_key_pair
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateLaunchTemplate(BaseModel):
    """
    create_launch_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteLaunchTemplate(BaseModel):
    """
    delete_launch_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeLaunchTemplates(BaseModel):
    """
    describe_launch_templates
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateLaunchTemplateVersion(BaseModel):
    """
    create_launch_template_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteLaunchTemplateVersions(BaseModel):
    """
    delete_launch_template_versions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyLaunchTemplate(BaseModel):
    """
    modify_launch_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateNetworkInterface(BaseModel):
    """
    create_network_interface
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteNetworkInterface(BaseModel):
    """
    delete_network_interface
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeNetworkInterfaces(BaseModel):
    """
    describe_network_interfaces
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachNetworkInterface(BaseModel):
    """
    attach_network_interface
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachNetworkInterface(BaseModel):
    """
    detach_network_interface
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyNetworkInterfaceAttribute(BaseModel):
    """
    modify_network_interface_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreatePlacementGroup(BaseModel):
    """
    create_placement_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeletePlacementGroup(BaseModel):
    """
    delete_placement_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribePlacementGroups(BaseModel):
    """
    describe_placement_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeInstanceTypes(BaseModel):
    """
    describe_instance_types
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAvailabilityZones(BaseModel):
    """
    describe_availability_zones
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeRegions(BaseModel):
    """
    describe_regions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTags(BaseModel):
    """
    create_tags
    
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


class RequestSpotInstances(BaseModel):
    """
    request_spot_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelSpotInstanceRequests(BaseModel):
    """
    cancel_spot_instance_requests
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeSpotInstanceRequests(BaseModel):
    """
    describe_spot_instance_requests
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeSpotPriceHistory(BaseModel):
    """
    describe_spot_price_history
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateFleet(BaseModel):
    """
    create_fleet
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteFleets(BaseModel):
    """
    delete_fleets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeFleets(BaseModel):
    """
    describe_fleets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyFleet(BaseModel):
    """
    modify_fleet
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True



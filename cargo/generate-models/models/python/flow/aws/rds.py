# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateDbInstance(BaseModel):
    """
    create_db_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDbInstance(BaseModel):
    """
    delete_db_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbInstances(BaseModel):
    """
    describe_db_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyDbInstance(BaseModel):
    """
    modify_db_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StartDbInstance(BaseModel):
    """
    start_db_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StopDbInstance(BaseModel):
    """
    stop_db_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RebootDbInstance(BaseModel):
    """
    reboot_db_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDbCluster(BaseModel):
    """
    create_db_cluster
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDbCluster(BaseModel):
    """
    delete_db_cluster
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbClusters(BaseModel):
    """
    describe_db_clusters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyDbCluster(BaseModel):
    """
    modify_db_cluster
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StartDbCluster(BaseModel):
    """
    start_db_cluster
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StopDbCluster(BaseModel):
    """
    stop_db_cluster
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FailoverDbCluster(BaseModel):
    """
    failover_db_cluster
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDbSnapshot(BaseModel):
    """
    create_db_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDbSnapshot(BaseModel):
    """
    delete_db_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbSnapshots(BaseModel):
    """
    describe_db_snapshots
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CopyDbSnapshot(BaseModel):
    """
    copy_db_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RestoreDbInstanceFromDbSnapshot(BaseModel):
    """
    restore_db_instance_from_db_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RestoreDbInstanceToPointInTime(BaseModel):
    """
    restore_db_instance_to_point_in_time
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDbClusterSnapshot(BaseModel):
    """
    create_db_cluster_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDbClusterSnapshot(BaseModel):
    """
    delete_db_cluster_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbClusterSnapshots(BaseModel):
    """
    describe_db_cluster_snapshots
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CopyDbClusterSnapshot(BaseModel):
    """
    copy_db_cluster_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RestoreDbClusterFromSnapshot(BaseModel):
    """
    restore_db_cluster_from_snapshot
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RestoreDbClusterToPointInTime(BaseModel):
    """
    restore_db_cluster_to_point_in_time
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDbSubnetGroup(BaseModel):
    """
    create_db_subnet_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDbSubnetGroup(BaseModel):
    """
    delete_db_subnet_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbSubnetGroups(BaseModel):
    """
    describe_db_subnet_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyDbSubnetGroup(BaseModel):
    """
    modify_db_subnet_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDbParameterGroup(BaseModel):
    """
    create_db_parameter_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDbParameterGroup(BaseModel):
    """
    delete_db_parameter_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbParameterGroups(BaseModel):
    """
    describe_db_parameter_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyDbParameterGroup(BaseModel):
    """
    modify_db_parameter_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ResetDbParameterGroup(BaseModel):
    """
    reset_db_parameter_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbParameters(BaseModel):
    """
    describe_db_parameters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDbClusterParameterGroup(BaseModel):
    """
    create_db_cluster_parameter_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDbClusterParameterGroup(BaseModel):
    """
    delete_db_cluster_parameter_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbClusterParameterGroups(BaseModel):
    """
    describe_db_cluster_parameter_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyDbClusterParameterGroup(BaseModel):
    """
    modify_db_cluster_parameter_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateOptionGroup(BaseModel):
    """
    create_option_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteOptionGroup(BaseModel):
    """
    delete_option_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeOptionGroups(BaseModel):
    """
    describe_option_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyOptionGroup(BaseModel):
    """
    modify_option_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDbInstanceReadReplica(BaseModel):
    """
    create_db_instance_read_replica
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PromoteReadReplica(BaseModel):
    """
    promote_read_replica
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PromoteReadReplicaDbCluster(BaseModel):
    """
    promote_read_replica_db_cluster
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDbEngineVersions(BaseModel):
    """
    describe_db_engine_versions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeOrderableDbInstanceOptions(BaseModel):
    """
    describe_orderable_db_instance_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AddTagsToResource(BaseModel):
    """
    add_tags_to_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RemoveTagsFromResource(BaseModel):
    """
    remove_tags_from_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTagsForResource(BaseModel):
    """
    list_tags_for_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True



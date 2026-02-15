# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsRdsInstance(BaseModel):
    """
    aws_rds_instance
    
    ID fields: account_id, db_instance_identifier
    """

    account_id: str
    allocated_storage: Optional[int] = None
    auto_minor_version_upgrade: bool = Field(default=True)
    availability_zone: Optional[str] = None
    backup_retention_period: int = Field(default=1)
    ca_certificate_identifier: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    db_cluster_identifier: Optional[str] = None
    db_instance_class: str
    db_instance_identifier: str
    db_name: Optional[str] = None
    db_subnet_group_name: Optional[str] = None
    deletion_protection: bool = Field(default=False)
    endpoint_address: Optional[str] = None
    endpoint_port: Optional[int] = None
    engine: str
    engine_version: Optional[str] = None
    iam_auth_enabled: bool = Field(default=False)
    iops: Optional[int] = None
    kms_key_id: Optional[str] = None
    master_username: Optional[str] = None
    multi_az: bool = Field(default=False)
    performance_insights_enabled: bool = Field(default=False)
    port: Optional[int] = None
    publicly_accessible: bool = Field(default=False)
    region: Optional[str] = None
    status: str = Field(default="creating")
    storage_encrypted: bool = Field(default=False)
    storage_type: str = Field(default="gp2")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsRdsCluster(BaseModel):
    """
    aws_rds_cluster
    
    ID fields: account_id, db_cluster_identifier
    """

    account_id: str
    availability_zones: Optional[str] = None
    backup_retention_period: int = Field(default=1)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    database_name: Optional[str] = None
    db_cluster_identifier: str
    deletion_protection: bool = Field(default=False)
    endpoint: Optional[str] = None
    engine: str
    engine_mode: str = Field(default="provisioned")
    engine_version: Optional[str] = None
    iam_auth_enabled: bool = Field(default=False)
    kms_key_id: Optional[str] = None
    master_username: Optional[str] = None
    multi_az: bool = Field(default=False)
    port: Optional[int] = None
    reader_endpoint: Optional[str] = None
    region: Optional[str] = None
    status: str = Field(default="creating")
    storage_encrypted: bool = Field(default=False)
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsRdsSubnetGroup(BaseModel):
    """
    aws_rds_subnet_group
    
    ID fields: account_id, db_subnet_group_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    db_subnet_group_description: Optional[str] = None
    db_subnet_group_name: str
    region: Optional[str] = None
    status: Optional[str] = None
    subnet_ids: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsRdsParameterGroup(BaseModel):
    """
    aws_rds_parameter_group
    
    ID fields: account_id, db_parameter_group_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    db_parameter_group_family: Optional[str] = None
    db_parameter_group_name: str
    description: Optional[str] = None
    region: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsRdsSnapshot(BaseModel):
    """
    aws_rds_snapshot
    
    ID fields: account_id, db_snapshot_identifier
    """

    account_id: str
    allocated_storage: Optional[int] = None
    availability_zone: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    db_instance_identifier: Optional[str] = None
    db_snapshot_identifier: str
    encrypted: bool = Field(default=False)
    engine: Optional[str] = None
    engine_version: Optional[str] = None
    kms_key_id: Optional[str] = None
    region: Optional[str] = None
    snapshot_type: str = Field(default="manual")
    status: str = Field(default="creating")
    storage_type: Optional[str] = None
    tags: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True



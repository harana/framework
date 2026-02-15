# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsEc2Instance(BaseModel):
    """
    aws_ec2_instance
    
    ID fields: account_id, instance_id
    """

    account_id: str
    ami_id: Optional[str] = None
    availability_zone: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    ebs_optimized: bool = Field(default=False)
    iam_instance_profile: Optional[str] = None
    instance_id: str
    instance_type: str
    key_name: Optional[str] = None
    launch_time: Optional[datetime] = None
    monitoring_enabled: bool = Field(default=False)
    platform: Optional[str] = None
    private_dns_name: Optional[str] = None
    private_ip_address: Optional[str] = None
    public_dns_name: Optional[str] = None
    public_ip_address: Optional[str] = None
    region: Optional[str] = None
    state: str = Field(default="pending")
    subnet_id: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEc2Ami(BaseModel):
    """
    aws_ec2_ami
    
    ID fields: account_id, image_id
    """

    account_id: str
    architecture: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    image_id: str
    is_public: bool = Field(default=False)
    name: Optional[str] = None
    owner_id: Optional[str] = None
    platform: Optional[str] = None
    region: Optional[str] = None
    root_device_type: str = Field(default="ebs")
    state: str = Field(default="pending")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    virtualization_type: str = Field(default="hvm")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEc2SecurityGroup(BaseModel):
    """
    aws_ec2_security_group
    
    ID fields: account_id, group_id
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    group_id: str
    group_name: str
    region: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEc2SecurityGroupRule(BaseModel):
    """
    aws_ec2_security_group_rule
    
    ID fields: security_group_id, direction, ip_protocol, from_port, to_port
    """

    cidr_ipv4: Optional[str] = None
    cidr_ipv6: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    direction: str
    from_port: Optional[int] = None
    ip_protocol: str
    referenced_group_id: Optional[str] = None
    security_group_id: str = Field(description="Reference: aws_ec2_security_group.id")
    to_port: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEc2KeyPair(BaseModel):
    """
    aws_ec2_key_pair
    
    ID fields: account_id, key_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    fingerprint: Optional[str] = None
    key_name: str
    key_pair_id: Optional[str] = None
    key_type: str = Field(default="rsa")
    region: Optional[str] = None
    tags: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEc2ElasticIp(BaseModel):
    """
    aws_ec2_elastic_ip
    
    ID fields: account_id, allocation_id
    """

    account_id: str
    allocation_id: str
    association_id: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    domain: str = Field(default="vpc")
    instance_id: Optional[str] = None
    network_interface_id: Optional[str] = None
    private_ip_address: Optional[str] = None
    public_ip: Optional[str] = None
    region: Optional[str] = None
    tags: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEc2Volume(BaseModel):
    """
    aws_ec2_volume
    
    ID fields: account_id, volume_id
    """

    account_id: str
    availability_zone: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    encrypted: bool = Field(default=False)
    iops: Optional[int] = None
    kms_key_id: Optional[str] = None
    region: Optional[str] = None
    size: int
    snapshot_id: Optional[str] = None
    state: str = Field(default="creating")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    volume_id: str
    volume_type: str = Field(default="gp3")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsEc2Snapshot(BaseModel):
    """
    aws_ec2_snapshot
    
    ID fields: account_id, snapshot_id
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    encrypted: bool = Field(default=False)
    kms_key_id: Optional[str] = None
    owner_id: Optional[str] = None
    progress: Optional[str] = None
    region: Optional[str] = None
    snapshot_id: str
    state: str = Field(default="pending")
    tags: Optional[str] = None
    volume_id: Optional[str] = None
    volume_size: Optional[int] = None
    class Config:
        from_attributes = True
        populate_by_name = True



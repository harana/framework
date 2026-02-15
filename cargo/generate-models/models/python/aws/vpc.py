# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class AwsVpc(BaseModel):
    """
    aws_vpc
    
    ID fields: account_id, vpc_id
    """

    account_id: str
    cidr_block: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    enable_dns_hostnames: bool = Field(default=True)
    enable_dns_support: bool = Field(default=True)
    instance_tenancy: str = Field(default="default")
    ipv6_cidr_block: Optional[str] = None
    is_default: bool = Field(default=False)
    owner_id: Optional[str] = None
    region: Optional[str] = None
    state: str = Field(default="pending")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcSubnet(BaseModel):
    """
    aws_vpc_subnet
    
    ID fields: vpc_id, subnet_id
    """

    availability_zone: Optional[str] = None
    availability_zone_id: Optional[str] = None
    available_ip_address_count: Optional[int] = None
    cidr_block: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    default_for_az: bool = Field(default=False)
    ipv6_cidr_block: Optional[str] = None
    map_public_ip_on_launch: bool = Field(default=False)
    owner_id: Optional[str] = None
    state: str = Field(default="pending")
    subnet_arn: Optional[str] = None
    subnet_id: str
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: str = Field(description="Reference: aws_vpc.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcInternetGateway(BaseModel):
    """
    aws_vpc_internet_gateway
    
    ID fields: account_id, internet_gateway_id
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    internet_gateway_id: str
    owner_id: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcInternetGatewayAttachment(BaseModel):
    """
    aws_vpc_internet_gateway_attachment
    
    ID fields: internet_gateway_id, vpc_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    internet_gateway_id: str = Field(description="Reference: aws_vpc_internet_gateway.id")
    state: str = Field(default="attaching")
    vpc_id: str = Field(description="Reference: aws_vpc.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcNatGateway(BaseModel):
    """
    aws_vpc_nat_gateway
    
    ID fields: account_id, nat_gateway_id
    """

    account_id: str
    allocation_id: Optional[str] = None
    connectivity_type: str = Field(default="public")
    created_at: datetime = Field(default_factory=datetime.utcnow)
    nat_gateway_id: str
    public_ip: Optional[str] = None
    private_ip: Optional[str] = None
    state: str = Field(default="pending")
    subnet_id: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcRouteTable(BaseModel):
    """
    aws_vpc_route_table
    
    ID fields: vpc_id, route_table_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    owner_id: Optional[str] = None
    route_table_id: str
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: str = Field(description="Reference: aws_vpc.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcRoute(BaseModel):
    """
    aws_vpc_route
    
    ID fields: route_table_id, destination_cidr_block
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    destination_cidr_block: Optional[str] = None
    destination_ipv6_cidr_block: Optional[str] = None
    gateway_id: Optional[str] = None
    instance_id: Optional[str] = None
    nat_gateway_id: Optional[str] = None
    network_interface_id: Optional[str] = None
    origin: str = Field(default="create_route")
    route_table_id: str = Field(description="Reference: aws_vpc_route_table.id")
    state: str = Field(default="active")
    transit_gateway_id: Optional[str] = None
    vpc_peering_connection_id: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcNetworkAcl(BaseModel):
    """
    aws_vpc_network_acl
    
    ID fields: vpc_id, network_acl_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_default: bool = Field(default=False)
    network_acl_id: str
    owner_id: Optional[str] = None
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_id: str = Field(description="Reference: aws_vpc.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsVpcPeeringConnection(BaseModel):
    """
    aws_vpc_peering_connection
    
    ID fields: account_id, vpc_peering_connection_id
    """

    accepter_vpc_id: Optional[str] = None
    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    requester_vpc_id: Optional[str] = None
    status: str = Field(default="initiating_request")
    tags: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    vpc_peering_connection_id: str
    class Config:
        from_attributes = True
        populate_by_name = True



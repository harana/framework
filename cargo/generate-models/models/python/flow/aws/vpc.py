# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateVpc(BaseModel):
    """
    create_vpc
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteVpc(BaseModel):
    """
    delete_vpc
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeVpcs(BaseModel):
    """
    describe_vpcs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyVpcAttribute(BaseModel):
    """
    modify_vpc_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateSubnet(BaseModel):
    """
    create_subnet
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSubnet(BaseModel):
    """
    delete_subnet
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeSubnets(BaseModel):
    """
    describe_subnets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifySubnetAttribute(BaseModel):
    """
    modify_subnet_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateInternetGateway(BaseModel):
    """
    create_internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteInternetGateway(BaseModel):
    """
    delete_internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachInternetGateway(BaseModel):
    """
    attach_internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DetachInternetGateway(BaseModel):
    """
    detach_internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeInternetGateways(BaseModel):
    """
    describe_internet_gateways
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateNatGateway(BaseModel):
    """
    create_nat_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteNatGateway(BaseModel):
    """
    delete_nat_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeNatGateways(BaseModel):
    """
    describe_nat_gateways
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRouteTable(BaseModel):
    """
    create_route_table
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRouteTable(BaseModel):
    """
    delete_route_table
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeRouteTables(BaseModel):
    """
    describe_route_tables
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRoute(BaseModel):
    """
    create_route
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteRoute(BaseModel):
    """
    delete_route
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReplaceRoute(BaseModel):
    """
    replace_route
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssociateRouteTable(BaseModel):
    """
    associate_route_table
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DisassociateRouteTable(BaseModel):
    """
    disassociate_route_table
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateSecurityGroup(BaseModel):
    """
    create_security_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteSecurityGroup(BaseModel):
    """
    delete_security_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeSecurityGroups(BaseModel):
    """
    describe_security_groups
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthorizeSecurityGroupIngress(BaseModel):
    """
    authorize_security_group_ingress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AuthorizeSecurityGroupEgress(BaseModel):
    """
    authorize_security_group_egress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RevokeSecurityGroupIngress(BaseModel):
    """
    revoke_security_group_ingress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RevokeSecurityGroupEgress(BaseModel):
    """
    revoke_security_group_egress
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateNetworkAcl(BaseModel):
    """
    create_network_acl
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteNetworkAcl(BaseModel):
    """
    delete_network_acl
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeNetworkAcls(BaseModel):
    """
    describe_network_acls
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateNetworkAclEntry(BaseModel):
    """
    create_network_acl_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteNetworkAclEntry(BaseModel):
    """
    delete_network_acl_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReplaceNetworkAclAssociation(BaseModel):
    """
    replace_network_acl_association
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AllocateAddress(BaseModel):
    """
    allocate_address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReleaseAddress(BaseModel):
    """
    release_address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssociateAddress(BaseModel):
    """
    associate_address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DisassociateAddress(BaseModel):
    """
    disassociate_address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeAddresses(BaseModel):
    """
    describe_addresses
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateVpcPeeringConnection(BaseModel):
    """
    create_vpc_peering_connection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AcceptVpcPeeringConnection(BaseModel):
    """
    accept_vpc_peering_connection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteVpcPeeringConnection(BaseModel):
    """
    delete_vpc_peering_connection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RejectVpcPeeringConnection(BaseModel):
    """
    reject_vpc_peering_connection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeVpcPeeringConnections(BaseModel):
    """
    describe_vpc_peering_connections
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateVpcEndpoint(BaseModel):
    """
    create_vpc_endpoint
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteVpcEndpoints(BaseModel):
    """
    delete_vpc_endpoints
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeVpcEndpoints(BaseModel):
    """
    describe_vpc_endpoints
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ModifyVpcEndpoint(BaseModel):
    """
    modify_vpc_endpoint
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateDhcpOptions(BaseModel):
    """
    create_dhcp_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteDhcpOptions(BaseModel):
    """
    delete_dhcp_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeDhcpOptions(BaseModel):
    """
    describe_dhcp_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssociateDhcpOptions(BaseModel):
    """
    associate_dhcp_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateEgressOnlyInternetGateway(BaseModel):
    """
    create_egress_only_internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteEgressOnlyInternetGateway(BaseModel):
    """
    delete_egress_only_internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DescribeEgressOnlyInternetGateways(BaseModel):
    """
    describe_egress_only_internet_gateways
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True



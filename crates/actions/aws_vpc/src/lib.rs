// Harana Actions - AWS VPC Module
// This module provides AWS VPC actions.

#![warn(missing_docs)]

pub mod output;

use output::*;

/// Create VPC
pub async fn create_vpc(
    cidr_block: &str,
    enable_dns_hostnames: Option<bool>,
    enable_dns_support: Option<bool>,
    instance_tenancy: Option<&str>,
    ipv6_cidr_block: Option<&str>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateVpcOutput, String> {
    unimplemented!("create_vpc")
}

/// Delete VPC
pub async fn delete_vpc(
    vpc_id: &str,
    region: Option<&str>,
) -> Result<DeleteVpcOutput, String> {
    unimplemented!("delete_vpc")
}

/// Describe VPCs
pub async fn describe_vpcs(
    filters: Option<Vec<VpcFilter>>,
    region: Option<&str>,
    vpc_ids: Option<Vec<String>>,
) -> Result<DescribeVpcsOutput, String> {
    unimplemented!("describe_vpcs")
}

/// Modify VPC Attribute
pub async fn modify_vpc_attribute(
    vpc_id: &str,
    enable_dns_hostnames: Option<bool>,
    enable_dns_support: Option<bool>,
    region: Option<&str>,
) -> Result<ModifyVpcAttributeOutput, String> {
    unimplemented!("modify_vpc_attribute")
}

/// Create Subnet
pub async fn create_subnet(
    cidr_block: &str,
    vpc_id: &str,
    availability_zone: Option<&str>,
    availability_zone_id: Option<&str>,
    ipv6_cidr_block: Option<&str>,
    map_public_ip_on_launch: Option<bool>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateSubnetOutput, String> {
    unimplemented!("create_subnet")
}

/// Delete Subnet
pub async fn delete_subnet(
    subnet_id: &str,
    region: Option<&str>,
) -> Result<DeleteSubnetOutput, String> {
    unimplemented!("delete_subnet")
}

/// Describe Subnets
pub async fn describe_subnets(
    filters: Option<Vec<VpcFilter>>,
    region: Option<&str>,
    subnet_ids: Option<Vec<String>>,
) -> Result<DescribeSubnetsOutput, String> {
    unimplemented!("describe_subnets")
}

/// Modify Subnet Attribute
pub async fn modify_subnet_attribute(
    subnet_id: &str,
    assign_ipv6_address_on_creation: Option<bool>,
    map_public_ip_on_launch: Option<bool>,
    region: Option<&str>,
) -> Result<ModifySubnetAttributeOutput, String> {
    unimplemented!("modify_subnet_attribute")
}

/// Create Internet Gateway
pub async fn create_internet_gateway(
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateInternetGatewayOutput, String> {
    unimplemented!("create_internet_gateway")
}

/// Delete Internet Gateway
pub async fn delete_internet_gateway(
    internet_gateway_id: &str,
    region: Option<&str>,
) -> Result<DeleteInternetGatewayOutput, String> {
    unimplemented!("delete_internet_gateway")
}

/// Attach Internet Gateway
pub async fn attach_internet_gateway(
    internet_gateway_id: &str,
    vpc_id: &str,
    region: Option<&str>,
) -> Result<AttachInternetGatewayOutput, String> {
    unimplemented!("attach_internet_gateway")
}

/// Detach Internet Gateway
pub async fn detach_internet_gateway(
    internet_gateway_id: &str,
    vpc_id: &str,
    region: Option<&str>,
) -> Result<DetachInternetGatewayOutput, String> {
    unimplemented!("detach_internet_gateway")
}

/// Describe Internet Gateways
pub async fn describe_internet_gateways(
    filters: Option<Vec<VpcFilter>>,
    internet_gateway_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeInternetGatewaysOutput, String> {
    unimplemented!("describe_internet_gateways")
}

/// Create NAT Gateway
pub async fn create_nat_gateway(
    subnet_id: &str,
    allocation_id: Option<&str>,
    connectivity_type: Option<&str>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateNatGatewayOutput, String> {
    unimplemented!("create_nat_gateway")
}

/// Delete NAT Gateway
pub async fn delete_nat_gateway(
    nat_gateway_id: &str,
    region: Option<&str>,
) -> Result<DeleteNatGatewayOutput, String> {
    unimplemented!("delete_nat_gateway")
}

/// Describe NAT Gateways
pub async fn describe_nat_gateways(
    filters: Option<Vec<VpcFilter>>,
    nat_gateway_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeNatGatewaysOutput, String> {
    unimplemented!("describe_nat_gateways")
}

/// Create Route Table
pub async fn create_route_table(
    vpc_id: &str,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateRouteTableOutput, String> {
    unimplemented!("create_route_table")
}

/// Delete Route Table
pub async fn delete_route_table(
    route_table_id: &str,
    region: Option<&str>,
) -> Result<DeleteRouteTableOutput, String> {
    unimplemented!("delete_route_table")
}

/// Describe Route Tables
pub async fn describe_route_tables(
    filters: Option<Vec<VpcFilter>>,
    region: Option<&str>,
    route_table_ids: Option<Vec<String>>,
) -> Result<DescribeRouteTablesOutput, String> {
    unimplemented!("describe_route_tables")
}

/// Create Route
pub async fn create_route(
    route_table_id: &str,
    destination_cidr_block: Option<&str>,
    destination_ipv6_cidr_block: Option<&str>,
    destination_prefix_list_id: Option<&str>,
    egress_only_internet_gateway_id: Option<&str>,
    gateway_id: Option<&str>,
    instance_id: Option<&str>,
    local_gateway_id: Option<&str>,
    nat_gateway_id: Option<&str>,
    network_interface_id: Option<&str>,
    region: Option<&str>,
    transit_gateway_id: Option<&str>,
    vpc_peering_connection_id: Option<&str>,
    carrier_gateway_id: Option<&str>,
    core_network_arn: Option<&str>,
) -> Result<CreateRouteOutput, String> {
    unimplemented!("create_route")
}

/// Delete Route
pub async fn delete_route(
    route_table_id: &str,
    destination_cidr_block: Option<&str>,
    destination_ipv6_cidr_block: Option<&str>,
    destination_prefix_list_id: Option<&str>,
    region: Option<&str>,
) -> Result<DeleteRouteOutput, String> {
    unimplemented!("delete_route")
}

/// Replace Route
pub async fn replace_route(
    route_table_id: &str,
    destination_cidr_block: Option<&str>,
    destination_ipv6_cidr_block: Option<&str>,
    destination_prefix_list_id: Option<&str>,
    egress_only_internet_gateway_id: Option<&str>,
    gateway_id: Option<&str>,
    instance_id: Option<&str>,
    local_gateway_id: Option<&str>,
    nat_gateway_id: Option<&str>,
    network_interface_id: Option<&str>,
    region: Option<&str>,
    transit_gateway_id: Option<&str>,
    vpc_peering_connection_id: Option<&str>,
    carrier_gateway_id: Option<&str>,
    core_network_arn: Option<&str>,
) -> Result<ReplaceRouteOutput, String> {
    unimplemented!("replace_route")
}

/// Associate Route Table
pub async fn associate_route_table(
    route_table_id: &str,
    gateway_id: Option<&str>,
    region: Option<&str>,
    subnet_id: Option<&str>,
) -> Result<AssociateRouteTableOutput, String> {
    unimplemented!("associate_route_table")
}

/// Disassociate Route Table
pub async fn disassociate_route_table(
    association_id: &str,
    region: Option<&str>,
) -> Result<DisassociateRouteTableOutput, String> {
    unimplemented!("disassociate_route_table")
}

/// Allocate Address
pub async fn allocate_address(
    address: Option<&str>,
    customer_owned_ipv4_pool: Option<&str>,
    domain: Option<&str>,
    network_border_group: Option<&str>,
    public_ipv4_pool: Option<&str>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<AllocateAddressOutput, String> {
    unimplemented!("allocate_address")
}

/// Release Address
pub async fn release_address(
    allocation_id: Option<&str>,
    network_border_group: Option<&str>,
    public_ip: Option<&str>,
    region: Option<&str>,
) -> Result<ReleaseAddressOutput, String> {
    unimplemented!("release_address")
}

/// Describe Addresses
pub async fn describe_addresses(
    allocation_ids: Option<Vec<String>>,
    filters: Option<Vec<VpcFilter>>,
    public_ips: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeAddressesOutput, String> {
    unimplemented!("describe_addresses")
}

/// Associate Address
pub async fn associate_address(
    allocation_id: Option<&str>,
    allow_reassociation: Option<bool>,
    instance_id: Option<&str>,
    network_interface_id: Option<&str>,
    private_ip_address: Option<&str>,
    public_ip: Option<&str>,
    region: Option<&str>,
) -> Result<AssociateAddressOutput, String> {
    unimplemented!("associate_address")
}

/// Disassociate Address
pub async fn disassociate_address(
    association_id: Option<&str>,
    public_ip: Option<&str>,
    region: Option<&str>,
) -> Result<DisassociateAddressOutput, String> {
    unimplemented!("disassociate_address")
}

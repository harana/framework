// Harana Actions - AWS VPC Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// create_vpc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVpcOutput {
    pub cidr_block: String,
    pub dhcp_options_id: Option<String>,
    pub instance_tenancy: String,
    pub ipv6_cidr_block: Option<String>,
    pub is_default: bool,
    pub owner_id: String,
    pub state: String,
    pub success: bool,
    pub vpc_id: String,
}

// delete_vpc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteVpcOutput {
    pub success: bool,
}

// describe_vpcs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeVpcsOutput {
    pub vpcs: Vec<Vpc>,
}

// modify_vpc_attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyVpcAttributeOutput {
    pub success: bool,
}

// create_subnet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubnetOutput {
    pub availability_zone: String,
    pub availability_zone_id: String,
    pub available_ip_address_count: i32,
    pub cidr_block: String,
    pub default_for_az: bool,
    pub map_public_ip_on_launch: bool,
    pub owner_id: String,
    pub state: String,
    pub subnet_arn: String,
    pub subnet_id: String,
    pub success: bool,
    pub vpc_id: String,
}

// delete_subnet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubnetOutput {
    pub success: bool,
}

// describe_subnets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeSubnetsOutput {
    pub subnets: Vec<Subnet>,
}

// modify_subnet_attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifySubnetAttributeOutput {
    pub success: bool,
}

// create_internet_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInternetGatewayOutput {
    pub attachments: Vec<InternetGatewayAttachment>,
    pub internet_gateway_id: String,
    pub owner_id: String,
    pub success: bool,
}

// delete_internet_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteInternetGatewayOutput {
    pub success: bool,
}

// attach_internet_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachInternetGatewayOutput {
    pub success: bool,
}

// detach_internet_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachInternetGatewayOutput {
    pub success: bool,
}

// describe_internet_gateways
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeInternetGatewaysOutput {
    pub internet_gateways: Vec<InternetGateway>,
}

// create_nat_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNatGatewayOutput {
    pub connectivity_type: String,
    pub create_time: DateTime<Utc>,
    pub nat_gateway_addresses: Vec<NatGatewayAddress>,
    pub nat_gateway_id: String,
    pub state: String,
    pub subnet_id: String,
    pub success: bool,
    pub vpc_id: String,
}

// delete_nat_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNatGatewayOutput {
    pub nat_gateway_id: String,
    pub success: bool,
}

// describe_nat_gateways
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeNatGatewaysOutput {
    pub nat_gateways: Vec<NatGateway>,
}

// create_route_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRouteTableOutput {
    pub associations: Vec<RouteTableAssociation>,
    pub owner_id: String,
    pub propagating_vgws: Vec<PropagatingVgw>,
    pub route_table_id: String,
    pub routes: Vec<Route>,
    pub success: bool,
    pub vpc_id: String,
}

// delete_route_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRouteTableOutput {
    pub success: bool,
}

// describe_route_tables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeRouteTablesOutput {
    pub route_tables: Vec<RouteTable>,
}

// create_route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRouteOutput {
    pub success: bool,
}

// delete_route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRouteOutput {
    pub success: bool,
}

// replace_route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceRouteOutput {
    pub success: bool,
}

// associate_route_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociateRouteTableOutput {
    pub association_id: String,
    pub association_state: RouteTableAssociationState,
    pub success: bool,
}

// disassociate_route_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisassociateRouteTableOutput {
    pub success: bool,
}

// allocate_address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateAddressOutput {
    pub allocation_id: String,
    pub carrier_ip: Option<String>,
    pub customer_owned_ip: Option<String>,
    pub customer_owned_ipv4_pool: Option<String>,
    pub domain: String,
    pub network_border_group: Option<String>,
    pub public_ip: String,
    pub public_ipv4_pool: Option<String>,
    pub success: bool,
}

// release_address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAddressOutput {
    pub success: bool,
}

// describe_addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeAddressesOutput {
    pub addresses: Vec<Address>,
}

// associate_address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociateAddressOutput {
    pub association_id: String,
    pub success: bool,
}

// disassociate_address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisassociateAddressOutput {
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vpc {
    pub cidr_block: String,
    pub dhcp_options_id: Option<String>,
    pub state: String,
    pub vpc_id: String,
    pub owner_id: Option<String>,
    pub instance_tenancy: String,
    pub ipv6_cidr_block_association_set: Option<Vec<VpcIpv6CidrBlockAssociation>>,
    pub cidr_block_association_set: Option<Vec<VpcCidrBlockAssociation>>,
    pub is_default: bool,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcIpv6CidrBlockAssociation {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub ipv6_cidr_block_state: Option<VpcCidrBlockState>,
    pub network_border_group: Option<String>,
    pub ipv6_pool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcCidrBlockAssociation {
    pub association_id: String,
    pub cidr_block: String,
    pub cidr_block_state: Option<VpcCidrBlockState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcCidrBlockState {
    pub state: String,
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub availability_zone: Option<String>,
    pub availability_zone_id: Option<String>,
    pub available_ip_address_count: i32,
    pub cidr_block: String,
    pub default_for_az: bool,
    pub enable_lni_at_device_index: Option<i32>,
    pub map_customer_owned_ip_on_launch: bool,
    pub map_public_ip_on_launch: bool,
    pub owner_id: Option<String>,
    pub state: String,
    pub subnet_arn: Option<String>,
    pub subnet_id: String,
    pub vpc_id: String,
    pub tags: Option<Vec<Tag>>,
    pub ipv6_cidr_block_association_set: Option<Vec<SubnetIpv6CidrBlockAssociation>>,
    pub outpost_arn: Option<String>,
    pub enable_dns64: bool,
    pub ipv6_native: bool,
    pub private_dns_name_options_on_launch: Option<PrivateDnsNameOptionsOnLaunch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetIpv6CidrBlockAssociation {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub ipv6_cidr_block_state: Option<SubnetCidrBlockState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetCidrBlockState {
    pub state: String,
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateDnsNameOptionsOnLaunch {
    pub hostname_type: Option<String>,
    pub enable_resource_name_dns_a_record: Option<bool>,
    pub enable_resource_name_dns_aaaa_record: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetGateway {
    pub attachments: Option<Vec<InternetGatewayAttachment>>,
    pub internet_gateway_id: String,
    pub owner_id: Option<String>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetGatewayAttachment {
    pub state: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatGateway {
    pub create_time: Option<DateTime<Utc>>,
    pub delete_time: Option<DateTime<Utc>>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub nat_gateway_addresses: Option<Vec<NatGatewayAddress>>,
    pub nat_gateway_id: String,
    pub provisioned_bandwidth: Option<ProvisionedBandwidth>,
    pub state: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub tags: Option<Vec<Tag>>,
    pub connectivity_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatGatewayAddress {
    pub allocation_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub private_ip: Option<String>,
    pub public_ip: Option<String>,
    pub association_id: Option<String>,
    pub is_primary: Option<bool>,
    pub failure_message: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedBandwidth {
    pub provisioned: Option<String>,
    pub provision_time: Option<DateTime<Utc>>,
    pub requested: Option<String>,
    pub request_time: Option<DateTime<Utc>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTable {
    pub associations: Option<Vec<RouteTableAssociation>>,
    pub propagating_vgws: Option<Vec<PropagatingVgw>>,
    pub route_table_id: String,
    pub routes: Option<Vec<Route>>,
    pub tags: Option<Vec<Tag>>,
    pub vpc_id: String,
    pub owner_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTableAssociation {
    pub main: bool,
    pub route_table_association_id: Option<String>,
    pub route_table_id: Option<String>,
    pub subnet_id: Option<String>,
    pub gateway_id: Option<String>,
    pub association_state: Option<RouteTableAssociationState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTableAssociationState {
    pub state: String,
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropagatingVgw {
    pub gateway_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub destination_cidr_block: Option<String>,
    pub destination_ipv6_cidr_block: Option<String>,
    pub destination_prefix_list_id: Option<String>,
    pub egress_only_internet_gateway_id: Option<String>,
    pub gateway_id: Option<String>,
    pub instance_id: Option<String>,
    pub instance_owner_id: Option<String>,
    pub nat_gateway_id: Option<String>,
    pub transit_gateway_id: Option<String>,
    pub local_gateway_id: Option<String>,
    pub carrier_gateway_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub origin: Option<String>,
    pub state: Option<String>,
    pub vpc_peering_connection_id: Option<String>,
    pub core_network_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub instance_id: Option<String>,
    pub public_ip: Option<String>,
    pub allocation_id: Option<String>,
    pub association_id: Option<String>,
    pub domain: Option<String>,
    pub network_interface_id: Option<String>,
    pub network_interface_owner_id: Option<String>,
    pub private_ip_address: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub public_ipv4_pool: Option<String>,
    pub network_border_group: Option<String>,
    pub customer_owned_ip: Option<String>,
    pub customer_owned_ipv4_pool: Option<String>,
    pub carrier_ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcFilter {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcTags {
    pub tags: HashMap<String, String>,
}

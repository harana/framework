// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcInput {
    pub cidr_block: String,
    #[serde(default)]
    pub enable_dns_hostnames: bool,
    #[serde(default)]
    pub enable_dns_support: bool,
    pub instance_tenancy: String,
    pub ipv6_cidr_block: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcOutput {
    pub cidr_block: String,
    pub dhcp_options_id: String,
    pub instance_tenancy: String,
    pub ipv6_cidr_block: String,
    pub is_default: bool,
    pub owner_id: String,
    pub state: String,
    pub success: bool,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcInput {
    pub region: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVpcsInput {
    pub filters: Vec<String>,
    pub region: String,
    pub vpc_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVpcsOutput {
    pub vpcs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyVpcAttributeInput {
    pub enable_dns_hostnames: bool,
    pub enable_dns_support: bool,
    pub region: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyVpcAttributeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSubnetInput {
    pub availability_zone: String,
    pub availability_zone_id: String,
    pub cidr_block: String,
    pub ipv6_cidr_block: String,
    #[serde(default)]
    pub map_public_ip_on_launch: bool,
    pub region: String,
    pub tags: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSubnetOutput {
    pub availability_zone: String,
    pub availability_zone_id: String,
    pub available_ip_address_count: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSubnetInput {
    pub region: String,
    pub subnet_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSubnetOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSubnetsInput {
    pub filters: Vec<String>,
    pub region: String,
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSubnetsOutput {
    pub subnets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifySubnetAttributeInput {
    pub assign_ipv6_address_on_creation: bool,
    pub map_public_ip_on_launch: bool,
    pub region: String,
    pub subnet_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifySubnetAttributeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInternetGatewayInput {
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInternetGatewayOutput {
    pub attachments: Vec<String>,
    pub internet_gateway_id: String,
    pub owner_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInternetGatewayInput {
    pub internet_gateway_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInternetGatewayOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachInternetGatewayInput {
    pub internet_gateway_id: String,
    pub region: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachInternetGatewayOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachInternetGatewayInput {
    pub internet_gateway_id: String,
    pub region: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachInternetGatewayOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInternetGatewaysInput {
    pub filters: Vec<String>,
    pub internet_gateway_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInternetGatewaysOutput {
    pub internet_gateways: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNatGatewayInput {
    pub allocation_id: String,
    pub connectivity_type: String,
    pub region: String,
    pub subnet_id: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNatGatewayOutput {
    pub connectivity_type: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub nat_gateway_addresses: Vec<String>,
    pub nat_gateway_id: String,
    pub state: String,
    pub subnet_id: String,
    pub success: bool,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNatGatewayInput {
    pub nat_gateway_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNatGatewayOutput {
    pub nat_gateway_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNatGatewaysInput {
    pub filters: Vec<String>,
    pub nat_gateway_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNatGatewaysOutput {
    pub nat_gateways: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRouteTableInput {
    pub region: String,
    pub tags: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRouteTableOutput {
    pub associations: Vec<String>,
    pub owner_id: String,
    pub propagating_vgws: Vec<String>,
    pub route_table_id: String,
    pub routes: Vec<String>,
    pub success: bool,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRouteTableInput {
    pub region: String,
    pub route_table_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRouteTableOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRouteTablesInput {
    pub filters: Vec<String>,
    pub region: String,
    pub route_table_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRouteTablesOutput {
    pub route_tables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRouteInput {
    pub carrier_gateway_id: String,
    pub destination_cidr_block: String,
    pub destination_ipv6_cidr_block: String,
    pub egress_only_internet_gateway_id: String,
    pub gateway_id: String,
    pub instance_id: String,
    pub nat_gateway_id: String,
    pub network_interface_id: String,
    pub region: String,
    pub route_table_id: String,
    pub transit_gateway_id: String,
    pub vpc_endpoint_id: String,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRouteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRouteInput {
    pub destination_cidr_block: String,
    pub destination_ipv6_cidr_block: String,
    pub region: String,
    pub route_table_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRouteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplaceRouteInput {
    pub carrier_gateway_id: String,
    pub destination_cidr_block: String,
    pub destination_ipv6_cidr_block: String,
    pub gateway_id: String,
    pub instance_id: String,
    pub nat_gateway_id: String,
    pub network_interface_id: String,
    pub region: String,
    pub route_table_id: String,
    pub transit_gateway_id: String,
    pub vpc_endpoint_id: String,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplaceRouteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateRouteTableInput {
    pub gateway_id: String,
    pub region: String,
    pub route_table_id: String,
    pub subnet_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateRouteTableOutput {
    pub association_id: String,
    pub association_state: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisassociateRouteTableInput {
    pub association_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisassociateRouteTableOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSecurityGroupInput {
    pub description: String,
    pub group_name: String,
    pub region: String,
    pub tags: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSecurityGroupOutput {
    pub group_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSecurityGroupInput {
    pub group_id: String,
    pub group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSecurityGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSecurityGroupsInput {
    pub filters: Vec<String>,
    pub group_ids: Vec<String>,
    pub group_names: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSecurityGroupsOutput {
    pub security_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorizeSecurityGroupIngressInput {
    pub cidr_ip: String,
    pub from_port: i64,
    pub group_id: String,
    pub ip_permissions: Vec<String>,
    pub ip_protocol: String,
    pub region: String,
    pub source_security_group_name: String,
    pub source_security_group_owner_id: String,
    pub to_port: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorizeSecurityGroupIngressOutput {
    pub security_group_rules: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorizeSecurityGroupEgressInput {
    pub cidr_ip: String,
    pub from_port: i64,
    pub group_id: String,
    pub ip_permissions: Vec<String>,
    pub ip_protocol: String,
    pub region: String,
    pub to_port: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorizeSecurityGroupEgressOutput {
    pub security_group_rules: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeSecurityGroupIngressInput {
    pub cidr_ip: String,
    pub from_port: i64,
    pub group_id: String,
    pub ip_permissions: Vec<String>,
    pub ip_protocol: String,
    pub region: String,
    pub security_group_rule_ids: Vec<String>,
    pub source_security_group_name: String,
    pub source_security_group_owner_id: String,
    pub to_port: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeSecurityGroupIngressOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeSecurityGroupEgressInput {
    pub cidr_ip: String,
    pub from_port: i64,
    pub group_id: String,
    pub ip_permissions: Vec<String>,
    pub ip_protocol: String,
    pub region: String,
    pub security_group_rule_ids: Vec<String>,
    pub to_port: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeSecurityGroupEgressOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkAclInput {
    pub region: String,
    pub tags: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkAclOutput {
    pub associations: Vec<String>,
    pub entries: Vec<String>,
    pub is_default: bool,
    pub network_acl_id: String,
    pub owner_id: String,
    pub success: bool,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNetworkAclInput {
    pub network_acl_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNetworkAclOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNetworkAclsInput {
    pub filters: Vec<String>,
    pub network_acl_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNetworkAclsOutput {
    pub network_acls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkAclEntryInput {
    pub cidr_block: String,
    pub egress: bool,
    pub icmp_code: i64,
    pub icmp_type: i64,
    pub ipv6_cidr_block: String,
    pub network_acl_id: String,
    pub port_range_from: i64,
    pub port_range_to: i64,
    pub protocol: String,
    pub region: String,
    pub rule_method: String,
    pub rule_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkAclEntryOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNetworkAclEntryInput {
    pub egress: bool,
    pub network_acl_id: String,
    pub region: String,
    pub rule_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNetworkAclEntryOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplaceNetworkAclAssociationInput {
    pub association_id: String,
    pub network_acl_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplaceNetworkAclAssociationOutput {
    pub new_association_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllocateAddressInput {
    pub domain: String,
    pub network_border_group: String,
    pub public_ipv4_pool: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllocateAddressOutput {
    pub allocation_id: String,
    pub domain: String,
    pub network_border_group: String,
    pub public_ip: String,
    pub public_ipv4_pool: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseAddressInput {
    pub allocation_id: String,
    pub public_ip: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseAddressOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateAddressInput {
    pub allocation_id: String,
    #[serde(default)]
    pub allow_reassociation: bool,
    pub instance_id: String,
    pub network_interface_id: String,
    pub private_ip_address: String,
    pub public_ip: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateAddressOutput {
    pub association_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisassociateAddressInput {
    pub association_id: String,
    pub public_ip: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisassociateAddressOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAddressesInput {
    pub allocation_ids: Vec<String>,
    pub filters: Vec<String>,
    pub public_ips: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAddressesOutput {
    pub addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcPeeringConnectionInput {
    pub peer_owner_id: String,
    pub peer_region: String,
    pub peer_vpc_id: String,
    pub region: String,
    pub tags: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcPeeringConnectionOutput {
    pub accepter_vpc_info: String,
    pub expiration_time: chrono::DateTime<chrono::Utc>,
    pub requester_vpc_info: String,
    pub status: String,
    pub success: bool,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcceptVpcPeeringConnectionInput {
    pub region: String,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcceptVpcPeeringConnectionOutput {
    pub accepter_vpc_info: String,
    pub requester_vpc_info: String,
    pub status: String,
    pub success: bool,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcPeeringConnectionInput {
    pub region: String,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcPeeringConnectionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RejectVpcPeeringConnectionInput {
    pub region: String,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RejectVpcPeeringConnectionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVpcPeeringConnectionsInput {
    pub filters: Vec<String>,
    pub region: String,
    pub vpc_peering_connection_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVpcPeeringConnectionsOutput {
    pub vpc_peering_connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcEndpointInput {
    pub policy_document: String,
    #[serde(default)]
    pub private_dns_enabled: bool,
    pub region: String,
    pub route_table_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub service_name: String,
    pub subnet_ids: Vec<String>,
    pub tags: String,
    pub vpc_endpoint_type: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcEndpointOutput {
    pub creation_timestamp: chrono::DateTime<chrono::Utc>,
    pub dns_entries: Vec<String>,
    pub groups: Vec<String>,
    pub network_interface_ids: Vec<String>,
    pub owner_id: String,
    pub policy_document: String,
    pub private_dns_enabled: bool,
    pub requester_managed: bool,
    pub route_table_ids: Vec<String>,
    pub service_name: String,
    pub state: String,
    pub subnet_ids: Vec<String>,
    pub success: bool,
    pub vpc_endpoint_id: String,
    pub vpc_endpoint_type: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcEndpointsInput {
    pub region: String,
    pub vpc_endpoint_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcEndpointsOutput {
    pub unsuccessful: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVpcEndpointsInput {
    pub filters: Vec<String>,
    pub region: String,
    pub vpc_endpoint_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVpcEndpointsOutput {
    pub vpc_endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyVpcEndpointInput {
    pub add_route_table_ids: Vec<String>,
    pub add_security_group_ids: Vec<String>,
    pub add_subnet_ids: Vec<String>,
    pub policy_document: String,
    pub private_dns_enabled: bool,
    pub region: String,
    pub remove_route_table_ids: Vec<String>,
    pub remove_security_group_ids: Vec<String>,
    pub remove_subnet_ids: Vec<String>,
    #[serde(default)]
    pub reset_policy: bool,
    pub vpc_endpoint_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyVpcEndpointOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDhcpOptionsInput {
    pub dhcp_configurations: Vec<String>,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDhcpOptionsOutput {
    pub dhcp_configurations: Vec<String>,
    pub dhcp_options_id: String,
    pub owner_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDhcpOptionsInput {
    pub dhcp_options_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDhcpOptionsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDhcpOptionsInput {
    pub dhcp_options_ids: Vec<String>,
    pub filters: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDhcpOptionsOutput {
    pub dhcp_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateDhcpOptionsInput {
    pub dhcp_options_id: String,
    pub region: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateDhcpOptionsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEgressOnlyInternetGatewayInput {
    pub region: String,
    pub tags: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEgressOnlyInternetGatewayOutput {
    pub attachments: Vec<String>,
    pub egress_only_internet_gateway_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEgressOnlyInternetGatewayInput {
    pub egress_only_internet_gateway_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEgressOnlyInternetGatewayOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeEgressOnlyInternetGatewaysInput {
    pub egress_only_internet_gateway_ids: Vec<String>,
    pub filters: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeEgressOnlyInternetGatewaysOutput {
    pub egress_only_internet_gateways: Vec<String>,
}

#[async_trait]
pub trait VpcAction {
    async fn create_vpc(&self, input: CreateVpcInput) -> Result<CreateVpcOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc(&self, input: DeleteVpcInput) -> Result<DeleteVpcOutput, Box<dyn std::error::Error>>;
    async fn describe_vpcs(&self, input: DescribeVpcsInput) -> Result<DescribeVpcsOutput, Box<dyn std::error::Error>>;
    async fn modify_vpc_attribute(&self, input: ModifyVpcAttributeInput) -> Result<ModifyVpcAttributeOutput, Box<dyn std::error::Error>>;
    async fn create_subnet(&self, input: CreateSubnetInput) -> Result<CreateSubnetOutput, Box<dyn std::error::Error>>;
    async fn delete_subnet(&self, input: DeleteSubnetInput) -> Result<DeleteSubnetOutput, Box<dyn std::error::Error>>;
    async fn describe_subnets(&self, input: DescribeSubnetsInput) -> Result<DescribeSubnetsOutput, Box<dyn std::error::Error>>;
    async fn modify_subnet_attribute(&self, input: ModifySubnetAttributeInput) -> Result<ModifySubnetAttributeOutput, Box<dyn std::error::Error>>;
    async fn create_internet_gateway(&self, input: CreateInternetGatewayInput) -> Result<CreateInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn delete_internet_gateway(&self, input: DeleteInternetGatewayInput) -> Result<DeleteInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn attach_internet_gateway(&self, input: AttachInternetGatewayInput) -> Result<AttachInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn detach_internet_gateway(&self, input: DetachInternetGatewayInput) -> Result<DetachInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn describe_internet_gateways(&self, input: DescribeInternetGatewaysInput) -> Result<DescribeInternetGatewaysOutput, Box<dyn std::error::Error>>;
    async fn create_nat_gateway(&self, input: CreateNatGatewayInput) -> Result<CreateNatGatewayOutput, Box<dyn std::error::Error>>;
    async fn delete_nat_gateway(&self, input: DeleteNatGatewayInput) -> Result<DeleteNatGatewayOutput, Box<dyn std::error::Error>>;
    async fn describe_nat_gateways(&self, input: DescribeNatGatewaysInput) -> Result<DescribeNatGatewaysOutput, Box<dyn std::error::Error>>;
    async fn create_route_table(&self, input: CreateRouteTableInput) -> Result<CreateRouteTableOutput, Box<dyn std::error::Error>>;
    async fn delete_route_table(&self, input: DeleteRouteTableInput) -> Result<DeleteRouteTableOutput, Box<dyn std::error::Error>>;
    async fn describe_route_tables(&self, input: DescribeRouteTablesInput) -> Result<DescribeRouteTablesOutput, Box<dyn std::error::Error>>;
    async fn create_route(&self, input: CreateRouteInput) -> Result<CreateRouteOutput, Box<dyn std::error::Error>>;
    async fn delete_route(&self, input: DeleteRouteInput) -> Result<DeleteRouteOutput, Box<dyn std::error::Error>>;
    async fn replace_route(&self, input: ReplaceRouteInput) -> Result<ReplaceRouteOutput, Box<dyn std::error::Error>>;
    async fn associate_route_table(&self, input: AssociateRouteTableInput) -> Result<AssociateRouteTableOutput, Box<dyn std::error::Error>>;
    async fn disassociate_route_table(&self, input: DisassociateRouteTableInput) -> Result<DisassociateRouteTableOutput, Box<dyn std::error::Error>>;
    async fn create_security_group(&self, input: CreateSecurityGroupInput) -> Result<CreateSecurityGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_security_group(&self, input: DeleteSecurityGroupInput) -> Result<DeleteSecurityGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_security_groups(&self, input: DescribeSecurityGroupsInput) -> Result<DescribeSecurityGroupsOutput, Box<dyn std::error::Error>>;
    async fn authorize_security_group_ingress(&self, input: AuthorizeSecurityGroupIngressInput) -> Result<AuthorizeSecurityGroupIngressOutput, Box<dyn std::error::Error>>;
    async fn authorize_security_group_egress(&self, input: AuthorizeSecurityGroupEgressInput) -> Result<AuthorizeSecurityGroupEgressOutput, Box<dyn std::error::Error>>;
    async fn revoke_security_group_ingress(&self, input: RevokeSecurityGroupIngressInput) -> Result<RevokeSecurityGroupIngressOutput, Box<dyn std::error::Error>>;
    async fn revoke_security_group_egress(&self, input: RevokeSecurityGroupEgressInput) -> Result<RevokeSecurityGroupEgressOutput, Box<dyn std::error::Error>>;
    async fn create_network_acl(&self, input: CreateNetworkAclInput) -> Result<CreateNetworkAclOutput, Box<dyn std::error::Error>>;
    async fn delete_network_acl(&self, input: DeleteNetworkAclInput) -> Result<DeleteNetworkAclOutput, Box<dyn std::error::Error>>;
    async fn describe_network_acls(&self, input: DescribeNetworkAclsInput) -> Result<DescribeNetworkAclsOutput, Box<dyn std::error::Error>>;
    async fn create_network_acl_entry(&self, input: CreateNetworkAclEntryInput) -> Result<CreateNetworkAclEntryOutput, Box<dyn std::error::Error>>;
    async fn delete_network_acl_entry(&self, input: DeleteNetworkAclEntryInput) -> Result<DeleteNetworkAclEntryOutput, Box<dyn std::error::Error>>;
    async fn replace_network_acl_association(&self, input: ReplaceNetworkAclAssociationInput) -> Result<ReplaceNetworkAclAssociationOutput, Box<dyn std::error::Error>>;
    async fn allocate_address(&self, input: AllocateAddressInput) -> Result<AllocateAddressOutput, Box<dyn std::error::Error>>;
    async fn release_address(&self, input: ReleaseAddressInput) -> Result<ReleaseAddressOutput, Box<dyn std::error::Error>>;
    async fn associate_address(&self, input: AssociateAddressInput) -> Result<AssociateAddressOutput, Box<dyn std::error::Error>>;
    async fn disassociate_address(&self, input: DisassociateAddressInput) -> Result<DisassociateAddressOutput, Box<dyn std::error::Error>>;
    async fn describe_addresses(&self, input: DescribeAddressesInput) -> Result<DescribeAddressesOutput, Box<dyn std::error::Error>>;
    async fn create_vpc_peering_connection(&self, input: CreateVpcPeeringConnectionInput) -> Result<CreateVpcPeeringConnectionOutput, Box<dyn std::error::Error>>;
    async fn accept_vpc_peering_connection(&self, input: AcceptVpcPeeringConnectionInput) -> Result<AcceptVpcPeeringConnectionOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc_peering_connection(&self, input: DeleteVpcPeeringConnectionInput) -> Result<DeleteVpcPeeringConnectionOutput, Box<dyn std::error::Error>>;
    async fn reject_vpc_peering_connection(&self, input: RejectVpcPeeringConnectionInput) -> Result<RejectVpcPeeringConnectionOutput, Box<dyn std::error::Error>>;
    async fn describe_vpc_peering_connections(&self, input: DescribeVpcPeeringConnectionsInput) -> Result<DescribeVpcPeeringConnectionsOutput, Box<dyn std::error::Error>>;
    async fn create_vpc_endpoint(&self, input: CreateVpcEndpointInput) -> Result<CreateVpcEndpointOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc_endpoints(&self, input: DeleteVpcEndpointsInput) -> Result<DeleteVpcEndpointsOutput, Box<dyn std::error::Error>>;
    async fn describe_vpc_endpoints(&self, input: DescribeVpcEndpointsInput) -> Result<DescribeVpcEndpointsOutput, Box<dyn std::error::Error>>;
    async fn modify_vpc_endpoint(&self, input: ModifyVpcEndpointInput) -> Result<ModifyVpcEndpointOutput, Box<dyn std::error::Error>>;
    async fn create_dhcp_options(&self, input: CreateDhcpOptionsInput) -> Result<CreateDhcpOptionsOutput, Box<dyn std::error::Error>>;
    async fn delete_dhcp_options(&self, input: DeleteDhcpOptionsInput) -> Result<DeleteDhcpOptionsOutput, Box<dyn std::error::Error>>;
    async fn describe_dhcp_options(&self, input: DescribeDhcpOptionsInput) -> Result<DescribeDhcpOptionsOutput, Box<dyn std::error::Error>>;
    async fn associate_dhcp_options(&self, input: AssociateDhcpOptionsInput) -> Result<AssociateDhcpOptionsOutput, Box<dyn std::error::Error>>;
    async fn create_egress_only_internet_gateway(&self, input: CreateEgressOnlyInternetGatewayInput) -> Result<CreateEgressOnlyInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn delete_egress_only_internet_gateway(&self, input: DeleteEgressOnlyInternetGatewayInput) -> Result<DeleteEgressOnlyInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn describe_egress_only_internet_gateways(&self, input: DescribeEgressOnlyInternetGatewaysInput) -> Result<DescribeEgressOnlyInternetGatewaysOutput, Box<dyn std::error::Error>>;
}

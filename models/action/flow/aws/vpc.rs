// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInternetGatewayOutput {
    pub attachments: Vec<String>,
    pub internet_gateway_id: String,
    pub owner_id: String,
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
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateRouteTableOutput {
    pub association_id: String,
    pub association_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkAclOutput {
    pub associations: Vec<String>,
    pub entries: Vec<String>,
    pub is_default: bool,
    pub network_acl_id: String,
    pub owner_id: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllocateAddressOutput {
    pub allocation_id: String,
    pub domain: String,
    pub network_border_group: String,
    pub public_ip: String,
    pub public_ipv4_pool: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcPeeringConnectionOutput {
    pub accepter_vpc_info: String,
    pub expiration_time: chrono::DateTime<chrono::Utc>,
    pub requester_vpc_info: String,
    pub status: String,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcceptVpcPeeringConnectionOutput {
    pub accepter_vpc_info: String,
    pub requester_vpc_info: String,
    pub status: String,
    pub vpc_peering_connection_id: String,
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
    pub vpc_endpoint_id: String,
    pub vpc_endpoint_type: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDhcpOptionsOutput {
    pub dhcp_configurations: Vec<String>,
    pub dhcp_options_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEgressOnlyInternetGatewayOutput {
    pub attachments: Vec<String>,
    pub egress_only_internet_gateway_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpc {
    pub account_id: String,
    pub cidr_block: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub enable_dns_hostnames: bool,
    #[serde(default)]
    pub enable_dns_support: bool,
    pub instance_tenancy: String,
    pub ipv6_cidr_block: String,
    #[serde(default)]
    pub is_default: bool,
    pub owner_id: String,
    pub region: String,
    pub state: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcSubnet {
    pub availability_zone: String,
    pub availability_zone_id: String,
    pub available_ip_address_count: i64,
    pub cidr_block: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub default_for_az: bool,
    pub ipv6_cidr_block: String,
    #[serde(default)]
    pub map_public_ip_on_launch: bool,
    pub owner_id: String,
    pub state: String,
    pub subnet_arn: String,
    pub subnet_id: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcInternetGateway {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub internet_gateway_id: String,
    pub owner_id: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcInternetGatewayAttachment {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub internet_gateway_id: String,
    pub state: String,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcNatGateway {
    pub account_id: String,
    pub allocation_id: String,
    pub connectivity_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub nat_gateway_id: String,
    pub public_ip: String,
    pub private_ip: String,
    pub state: String,
    pub subnet_id: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcRouteTable {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub owner_id: String,
    pub route_table_id: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcRoute {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub destination_cidr_block: String,
    pub destination_ipv6_cidr_block: String,
    pub gateway_id: String,
    pub instance_id: String,
    pub nat_gateway_id: String,
    pub network_interface_id: String,
    pub origin: String,
    pub route_table_id: String,
    pub state: String,
    pub transit_gateway_id: String,
    pub vpc_peering_connection_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcNetworkAcl {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_default: bool,
    pub network_acl_id: String,
    pub owner_id: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcPeeringConnection {
    pub accepter_vpc_id: String,
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub requester_vpc_id: String,
    pub status: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_peering_connection_id: String,
}

#[async_trait]
pub trait VpcAction {
    async fn create_vpc(&self, cidr_block: String, enable_dns_hostnames: bool, enable_dns_support: bool, instance_tenancy: String, ipv6_cidr_block: String, region: String, tags: String) -> Result<CreateVpcOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc(&self, region: String, vpc_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_vpcs(&self, filters: Vec<String>, region: String, vpc_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn modify_vpc_attribute(&self, enable_dns_hostnames: bool, enable_dns_support: bool, region: String, vpc_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_subnet(&self, availability_zone: String, availability_zone_id: String, cidr_block: String, ipv6_cidr_block: String, map_public_ip_on_launch: bool, region: String, tags: String, vpc_id: String) -> Result<CreateSubnetOutput, Box<dyn std::error::Error>>;
    async fn delete_subnet(&self, region: String, subnet_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_subnets(&self, filters: Vec<String>, region: String, subnet_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn modify_subnet_attribute(&self, assign_ipv6_address_on_creation: bool, map_public_ip_on_launch: bool, region: String, subnet_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_internet_gateway(&self, region: String, tags: String) -> Result<CreateInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn delete_internet_gateway(&self, internet_gateway_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn attach_internet_gateway(&self, internet_gateway_id: String, region: String, vpc_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn detach_internet_gateway(&self, internet_gateway_id: String, region: String, vpc_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_internet_gateways(&self, filters: Vec<String>, internet_gateway_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_nat_gateway(&self, allocation_id: String, connectivity_type: String, region: String, subnet_id: String, tags: String) -> Result<CreateNatGatewayOutput, Box<dyn std::error::Error>>;
    async fn delete_nat_gateway(&self, nat_gateway_id: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_nat_gateways(&self, filters: Vec<String>, nat_gateway_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_route_table(&self, region: String, tags: String, vpc_id: String) -> Result<CreateRouteTableOutput, Box<dyn std::error::Error>>;
    async fn delete_route_table(&self, region: String, route_table_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_route_tables(&self, filters: Vec<String>, region: String, route_table_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_route(&self, carrier_gateway_id: String, destination_cidr_block: String, destination_ipv6_cidr_block: String, egress_only_internet_gateway_id: String, gateway_id: String, instance_id: String, nat_gateway_id: String, network_interface_id: String, region: String, route_table_id: String, transit_gateway_id: String, vpc_endpoint_id: String, vpc_peering_connection_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_route(&self, destination_cidr_block: String, destination_ipv6_cidr_block: String, region: String, route_table_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn replace_route(&self, carrier_gateway_id: String, destination_cidr_block: String, destination_ipv6_cidr_block: String, gateway_id: String, instance_id: String, nat_gateway_id: String, network_interface_id: String, region: String, route_table_id: String, transit_gateway_id: String, vpc_endpoint_id: String, vpc_peering_connection_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn associate_route_table(&self, gateway_id: String, region: String, route_table_id: String, subnet_id: String) -> Result<AssociateRouteTableOutput, Box<dyn std::error::Error>>;
    async fn disassociate_route_table(&self, association_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_security_group(&self, description: String, group_name: String, region: String, tags: String, vpc_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_security_group(&self, group_id: String, group_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_security_groups(&self, filters: Vec<String>, group_ids: Vec<String>, group_names: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn authorize_security_group_ingress(&self, cidr_ip: String, from_port: i64, group_id: String, ip_permissions: Vec<String>, ip_protocol: String, region: String, source_security_group_name: String, source_security_group_owner_id: String, to_port: i64) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn authorize_security_group_egress(&self, cidr_ip: String, from_port: i64, group_id: String, ip_permissions: Vec<String>, ip_protocol: String, region: String, to_port: i64) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn revoke_security_group_ingress(&self, cidr_ip: String, from_port: i64, group_id: String, ip_permissions: Vec<String>, ip_protocol: String, region: String, security_group_rule_ids: Vec<String>, source_security_group_name: String, source_security_group_owner_id: String, to_port: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn revoke_security_group_egress(&self, cidr_ip: String, from_port: i64, group_id: String, ip_permissions: Vec<String>, ip_protocol: String, region: String, security_group_rule_ids: Vec<String>, to_port: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_network_acl(&self, region: String, tags: String, vpc_id: String) -> Result<CreateNetworkAclOutput, Box<dyn std::error::Error>>;
    async fn delete_network_acl(&self, network_acl_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_network_acls(&self, filters: Vec<String>, network_acl_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_network_acl_entry(&self, cidr_block: String, egress: bool, icmp_code: i64, icmp_type: i64, ipv6_cidr_block: String, network_acl_id: String, port_range_from: i64, port_range_to: i64, protocol: String, region: String, rule_method: String, rule_number: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_network_acl_entry(&self, egress: bool, network_acl_id: String, region: String, rule_number: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn replace_network_acl_association(&self, association_id: String, network_acl_id: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn allocate_address(&self, domain: String, network_border_group: String, public_ipv4_pool: String, region: String, tags: String) -> Result<AllocateAddressOutput, Box<dyn std::error::Error>>;
    async fn release_address(&self, allocation_id: String, public_ip: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn associate_address(&self, allocation_id: String, allow_reassociation: bool, instance_id: String, network_interface_id: String, private_ip_address: String, public_ip: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn disassociate_address(&self, association_id: String, public_ip: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_addresses(&self, allocation_ids: Vec<String>, filters: Vec<String>, public_ips: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_vpc_peering_connection(&self, peer_owner_id: String, peer_region: String, peer_vpc_id: String, region: String, tags: String, vpc_id: String) -> Result<CreateVpcPeeringConnectionOutput, Box<dyn std::error::Error>>;
    async fn accept_vpc_peering_connection(&self, region: String, vpc_peering_connection_id: String) -> Result<AcceptVpcPeeringConnectionOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc_peering_connection(&self, region: String, vpc_peering_connection_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn reject_vpc_peering_connection(&self, region: String, vpc_peering_connection_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_vpc_peering_connections(&self, filters: Vec<String>, region: String, vpc_peering_connection_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_vpc_endpoint(&self, policy_document: String, private_dns_enabled: bool, region: String, route_table_ids: Vec<String>, security_group_ids: Vec<String>, service_name: String, subnet_ids: Vec<String>, tags: String, vpc_endpoint_type: String, vpc_id: String) -> Result<CreateVpcEndpointOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc_endpoints(&self, region: String, vpc_endpoint_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_vpc_endpoints(&self, filters: Vec<String>, region: String, vpc_endpoint_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn modify_vpc_endpoint(&self, add_route_table_ids: Vec<String>, add_security_group_ids: Vec<String>, add_subnet_ids: Vec<String>, policy_document: String, private_dns_enabled: bool, region: String, remove_route_table_ids: Vec<String>, remove_security_group_ids: Vec<String>, remove_subnet_ids: Vec<String>, reset_policy: bool, vpc_endpoint_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_dhcp_options(&self, dhcp_configurations: Vec<String>, region: String, tags: String) -> Result<CreateDhcpOptionsOutput, Box<dyn std::error::Error>>;
    async fn delete_dhcp_options(&self, dhcp_options_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_dhcp_options(&self, dhcp_options_ids: Vec<String>, filters: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn associate_dhcp_options(&self, dhcp_options_id: String, region: String, vpc_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_egress_only_internet_gateway(&self, region: String, tags: String, vpc_id: String) -> Result<CreateEgressOnlyInternetGatewayOutput, Box<dyn std::error::Error>>;
    async fn delete_egress_only_internet_gateway(&self, egress_only_internet_gateway_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_egress_only_internet_gateways(&self, egress_only_internet_gateway_ids: Vec<String>, filters: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

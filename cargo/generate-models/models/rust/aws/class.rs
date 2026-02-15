// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// vpc_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcTags {
    pub type: std::collections::HashMap<String, String>,
    pub key: String,
    pub value: String,
}

impl VpcTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcFilter {
    pub name: String,
    pub values: Vec<String>,
}

impl VpcFilter {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Vpc {
    pub cidr_block: String,
    pub cidr_block_association_set: Vec<String>,
    pub dhcp_options_id: String,
    pub instance_tenancy: String,
    pub ipv6_cidr_block_association_set: Vec<String>,
    pub is_default: bool,
    pub owner_id: String,
    pub state: String,
    pub tags: Vec<String>,
    pub vpc_id: String,
}

impl Vpc {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_cidr_block_association
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcCidrBlockAssociation {
    pub association_id: String,
    pub cidr_block: String,
    pub cidr_block_state: String,
}

impl VpcCidrBlockAssociation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_cidr_block_state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcCidrBlockState {
    pub state: String,
    pub status_message: String,
}

impl VpcCidrBlockState {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_ipv6cidr_block_association
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcIpv6cidrBlockAssociation {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub ipv6_cidr_block_state: String,
    pub ipv6_pool: String,
    pub network_border_group: String,
}

impl VpcIpv6cidrBlockAssociation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// subnet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Subnet {
    pub assign_ipv6_address_on_creation: bool,
    pub availability_zone: String,
    pub availability_zone_id: String,
    pub available_ip_address_count: i64,
    pub cidr_block: String,
    pub default_for_az: bool,
    pub enable_dns64: bool,
    pub ipv6_cidr_block_association_set: Vec<String>,
    pub ipv6_native: bool,
    pub map_customer_owned_ip_on_launch: bool,
    pub map_public_ip_on_launch: bool,
    pub outpost_arn: String,
    pub owner_id: String,
    pub private_dns_name_options_on_launch: String,
    pub state: String,
    pub subnet_arn: String,
    pub subnet_id: String,
    pub tags: Vec<String>,
    pub vpc_id: String,
}

impl Subnet {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// subnet_ipv6cidr_block_association
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubnetIpv6cidrBlockAssociation {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub ipv6_cidr_block_state: String,
}

impl SubnetIpv6cidrBlockAssociation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// subnet_cidr_block_state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubnetCidrBlockState {
    pub state: String,
    pub status_message: String,
}

impl SubnetCidrBlockState {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// private_dns_name_options_on_launch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivateDnsNameOptionsOnLaunch {
    pub enable_resource_name_dns_aaaa_record: bool,
    pub enable_resource_name_dns_a_record: bool,
    pub hostname_type: String,
}

impl PrivateDnsNameOptionsOnLaunch {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// internet_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InternetGateway {
    pub attachments: Vec<String>,
    pub internet_gateway_id: String,
    pub owner_id: String,
    pub tags: Vec<String>,
}

impl InternetGateway {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// internet_gateway_attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InternetGatewayAttachment {
    pub state: String,
    pub vpc_id: String,
}

impl InternetGatewayAttachment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// nat_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NatGateway {
    pub connectivity_type: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub delete_time: chrono::DateTime<chrono::Utc>,
    pub failure_code: String,
    pub failure_message: String,
    pub nat_gateway_addresses: Vec<String>,
    pub nat_gateway_id: String,
    pub provisioned_bandwidth: String,
    pub state: String,
    pub subnet_id: String,
    pub tags: Vec<String>,
    pub vpc_id: String,
}

impl NatGateway {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// nat_gateway_address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NatGatewayAddress {
    pub allocation_id: String,
    pub network_interface_id: String,
    pub private_ip: String,
    pub public_ip: String,
}

impl NatGatewayAddress {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// provisioned_bandwidth
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProvisionedBandwidth {
    pub provision_time: chrono::DateTime<chrono::Utc>,
    pub provisioned: String,
    pub request_time: chrono::DateTime<chrono::Utc>,
    pub requested: String,
    pub status: String,
}

impl ProvisionedBandwidth {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteTable {
    pub associations: Vec<String>,
    pub owner_id: String,
    pub propagating_vgws: Vec<String>,
    pub route_table_id: String,
    pub routes: Vec<String>,
    pub tags: Vec<String>,
    pub vpc_id: String,
}

impl RouteTable {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_table_association
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteTableAssociation {
    pub association_state: String,
    pub gateway_id: String,
    pub main: bool,
    pub route_table_association_id: String,
    pub route_table_id: String,
    pub subnet_id: String,
}

impl RouteTableAssociation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_table_association_state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteTableAssociationState {
    pub state: String,
    pub status_message: String,
}

impl RouteTableAssociationState {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// propagating_vgw
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PropagatingVgw {
    pub gateway_id: String,
}

impl PropagatingVgw {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Route {
    pub carrier_gateway_id: String,
    pub core_network_arn: String,
    pub destination_cidr_block: String,
    pub destination_ipv6_cidr_block: String,
    pub destination_prefix_list_id: String,
    pub egress_only_internet_gateway_id: String,
    pub gateway_id: String,
    pub instance_id: String,
    pub instance_owner_id: String,
    pub local_gateway_id: String,
    pub nat_gateway_id: String,
    pub network_interface_id: String,
    pub origin: String,
    pub state: String,
    pub transit_gateway_id: String,
    pub vpc_peering_connection_id: String,
}

impl Route {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// security_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecurityGroup {
    pub description: String,
    pub group_id: String,
    pub group_name: String,
    pub ip_permissions: Vec<String>,
    pub ip_permissions_egress: Vec<String>,
    pub owner_id: String,
    pub tags: Vec<String>,
    pub vpc_id: String,
}

impl SecurityGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ip_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IpPermission {
    pub from_port: i64,
    pub ip_protocol: String,
    pub ip_ranges: Vec<String>,
    pub ipv6_ranges: Vec<String>,
    pub prefix_list_ids: Vec<String>,
    pub to_port: i64,
    pub user_id_group_pairs: Vec<String>,
}

impl IpPermission {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ip_range
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IpRange {
    pub cidr_ip: String,
    pub description: String,
}

impl IpRange {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ipv6range
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Ipv6range {
    pub cidr_ipv6: String,
    pub description: String,
}

impl Ipv6range {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// prefix_list_id
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrefixListId {
    pub description: String,
    pub prefix_list_id: String,
}

impl PrefixListId {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_id_group_pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserIdGroupPair {
    pub description: String,
    pub group_id: String,
    pub group_name: String,
    pub peering_status: String,
    pub user_id: String,
    pub vpc_id: String,
    pub vpc_peering_connection_id: String,
}

impl UserIdGroupPair {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// security_group_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecurityGroupRule {
    pub cidr_ipv4: String,
    pub cidr_ipv6: String,
    pub description: String,
    pub from_port: i64,
    pub group_id: String,
    pub group_owner_id: String,
    pub ip_protocol: String,
    pub is_egress: bool,
    pub prefix_list_id: String,
    pub referenced_group_info: String,
    pub security_group_rule_id: String,
    pub tags: Vec<String>,
    pub to_port: i64,
}

impl SecurityGroupRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// referenced_security_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReferencedSecurityGroup {
    pub group_id: String,
    pub peering_status: String,
    pub user_id: String,
    pub vpc_id: String,
    pub vpc_peering_connection_id: String,
}

impl ReferencedSecurityGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// network_acl
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NetworkAcl {
    pub associations: Vec<String>,
    pub entries: Vec<String>,
    pub is_default: bool,
    pub network_acl_id: String,
    pub owner_id: String,
    pub tags: Vec<String>,
    pub vpc_id: String,
}

impl NetworkAcl {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// network_acl_association
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NetworkAclAssociation {
    pub network_acl_association_id: String,
    pub network_acl_id: String,
    pub subnet_id: String,
}

impl NetworkAclAssociation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// network_acl_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NetworkAclEntry {
    pub cidr_block: String,
    pub egress: bool,
    pub icmp_type_code: String,
    pub ipv6_cidr_block: String,
    pub port_range: String,
    pub protocol: String,
    pub rule_method: String,
    pub rule_number: i64,
}

impl NetworkAclEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// icmp_type_code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IcmpTypeCode {
    pub code: i64,
    pub type: i64,
}

impl IcmpTypeCode {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// port_range
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PortRange {
    pub from: i64,
    pub to: i64,
}

impl PortRange {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// elastic_ip
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ElasticIp {
    pub allocation_id: String,
    pub association_id: String,
    pub carrier_ip: String,
    pub customer_owned_ip: String,
    pub customer_owned_ipv4_pool: String,
    pub domain: String,
    pub instance_id: String,
    pub network_border_group: String,
    pub network_interface_id: String,
    pub network_interface_owner_id: String,
    pub private_ip_address: String,
    pub public_ip: String,
    pub public_ipv4_pool: String,
    pub tags: Vec<String>,
}

impl ElasticIp {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_peering_connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcPeeringConnection {
    pub accepter_vpc_info: String,
    pub expiration_time: chrono::DateTime<chrono::Utc>,
    pub requester_vpc_info: String,
    pub status: String,
    pub tags: Vec<String>,
    pub vpc_peering_connection_id: String,
}

impl VpcPeeringConnection {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcInfo {
    pub cidr_block: String,
    pub cidr_block_set: Vec<String>,
    pub ipv6_cidr_block_set: Vec<String>,
    pub owner_id: String,
    pub peering_options: String,
    pub region: String,
    pub vpc_id: String,
}

impl VpcInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cidr_block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CidrBlock {
    pub cidr_block: String,
}

impl CidrBlock {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ipv6cidr_block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Ipv6cidrBlock {
    pub ipv6_cidr_block: String,
}

impl Ipv6cidrBlock {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_peering_connection_options_description
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcPeeringConnectionOptionsDescription {
    pub allow_dns_resolution_from_remote_vpc: bool,
    pub allow_egress_from_local_classic_link_to_remote_vpc: bool,
    pub allow_egress_from_local_vpc_to_remote_classic_link: bool,
}

impl VpcPeeringConnectionOptionsDescription {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// peering_connection_status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PeeringConnectionStatus {
    pub code: String,
    pub message: String,
}

impl PeeringConnectionStatus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// vpc_endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcEndpoint {
    pub creation_timestamp: chrono::DateTime<chrono::Utc>,
    pub dns_entries: Vec<String>,
    pub dns_options: String,
    pub groups: Vec<String>,
    pub ip_address_type: String,
    pub last_error: String,
    pub network_interface_ids: Vec<String>,
    pub owner_id: String,
    pub policy_document: String,
    pub private_dns_enabled: bool,
    pub requester_managed: bool,
    pub route_table_ids: Vec<String>,
    pub service_name: String,
    pub state: String,
    pub subnet_ids: Vec<String>,
    pub tags: Vec<String>,
    pub vpc_endpoint_id: String,
    pub vpc_endpoint_type: String,
    pub vpc_id: String,
}

impl VpcEndpoint {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// dns_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DnsEntry {
    pub dns_name: String,
    pub hosted_zone_id: String,
}

impl DnsEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// dns_options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DnsOptions {
    pub dns_record_ip_type: String,
    pub private_dns_only_for_inbound_resolver_endpoint: bool,
}

impl DnsOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// security_group_identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecurityGroupIdentifier {
    pub group_id: String,
    pub group_name: String,
}

impl SecurityGroupIdentifier {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// last_error
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LastError {
    pub code: String,
    pub message: String,
}

impl LastError {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// unsuccessful_item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsuccessfulItem {
    pub error: String,
    pub resource_id: String,
}

impl UnsuccessfulItem {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// unsuccessful_item_error
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsuccessfulItemError {
    pub code: String,
    pub message: String,
}

impl UnsuccessfulItemError {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// dhcp_options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DhcpOptions {
    pub dhcp_configurations: Vec<String>,
    pub dhcp_options_id: String,
    pub owner_id: String,
    pub tags: Vec<String>,
}

impl DhcpOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// dhcp_configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DhcpConfiguration {
    pub key: String,
    pub values: Vec<String>,
}

impl DhcpConfiguration {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// attribute_value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttributeValue {
    pub value: String,
}

impl AttributeValue {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// egress_only_internet_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EgressOnlyInternetGateway {
    pub attachments: Vec<String>,
    pub egress_only_internet_gateway_id: String,
    pub tags: Vec<String>,
}

impl EgressOnlyInternetGateway {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sts_credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StsCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
}

impl StsCredentials {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// assumed_role_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumedRoleUser {
    pub assumed_role_id: String,
    pub arn: String,
}

impl AssumedRoleUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// federated_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FederatedUser {
    pub federated_user_id: String,
    pub arn: String,
}

impl FederatedUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tags {
    pub key: String,
    pub value: String,
}

impl Tags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sts_policy_arn
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StsPolicyArn {
    pub arn: String,
}

impl StsPolicyArn {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// s3object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3object {
    pub bucket: String,
    pub key: String,
    pub etag: String,
    pub size: i64,
    pub content_type: String,
    pub storage_class: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub version_id: String,
}

impl S3object {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// s3bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3bucket {
    pub name: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}

impl S3bucket {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// s3tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3tags {
    pub items: Vec<String>,
}

impl S3tags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// s3tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3tag {
    pub key: String,
    pub value: String,
}

impl S3tag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// object_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ObjectMetadata {
    pub content_type: String,
    pub content_encoding: String,
    pub content_language: String,
    pub content_disposition: String,
    pub cache_control: String,
    pub custom: String,
}

impl ObjectMetadata {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deleted_object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletedObject {
    pub key: String,
    pub version_id: String,
    pub delete_marker: bool,
    pub delete_marker_version_id: String,
}

impl DeletedObject {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// s3error
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3error {
    pub key: String,
    pub version_id: String,
    pub code: String,
    pub message: String,
}

impl S3error {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// encryption_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptionRule {
    pub sse_algorithm: String,
    pub kms_master_key_id: String,
    pub bucket_key_enabled: bool,
}

impl EncryptionRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cors_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CorsRule {
    pub allowed_headers: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub expose_headers: Vec<String>,
    pub max_age_seconds: i64,
}

impl CorsRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// lifecycle_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LifecycleRule {
    pub id: String,
    pub status: String,
    pub prefix: String,
    pub expiration_days: i64,
    pub noncurrent_version_expiration_days: i64,
    pub transitions: Vec<String>,
}

impl LifecycleRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// lifecycle_transition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LifecycleTransition {
    pub days: i64,
    pub storage_class: String,
}

impl LifecycleTransition {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// completed_part
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompletedPart {
    pub etag: String,
    pub part_number: i64,
}

impl CompletedPart {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// iam_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamUser {
    pub user_id: String,
    pub user_name: String,
    pub arn: String,
    pub path: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub password_last_used: chrono::DateTime<chrono::Utc>,
    pub permissions_boundary: String,
    pub tags: Vec<String>,
}

impl IamUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// iam_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamTags {
    pub items: Vec<String>,
}

impl IamTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// iam_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamTag {
    pub key: String,
    pub value: String,
}

impl IamTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// permissions_boundary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PermissionsBoundary {
    pub permissions_boundary_type: String,
    pub permissions_boundary_arn: String,
}

impl PermissionsBoundary {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// iam_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamGroup {
    pub group_id: String,
    pub group_name: String,
    pub arn: String,
    pub path: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
}

impl IamGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// iam_role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamRole {
    pub role_id: String,
    pub role_name: String,
    pub arn: String,
    pub path: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub max_session_duration: i64,
    pub assume_role_policy_document: String,
}

impl IamRole {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// iam_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamPolicy {
    pub policy_id: String,
    pub policy_name: String,
    pub arn: String,
    pub path: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub update_date: chrono::DateTime<chrono::Utc>,
    pub attachment_count: i64,
    pub is_attachable: bool,
}

impl IamPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// access_key_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessKeyMetadata {
    pub access_key_id: String,
    pub user_name: String,
    pub status: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
}

impl AccessKeyMetadata {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// account_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountSummary {
    pub users: i64,
    pub groups: i64,
    pub roles: i64,
    pub policies: i64,
    pub users_quota: i64,
    pub groups_quota: i64,
    pub roles_quota: i64,
    pub policies_quota: i64,
    pub server_certificates: i64,
    pub mfa_devices: i64,
    pub account_mfa_enabled: bool,
}

impl AccountSummary {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// event_bridge_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBridgeRule {
    pub name: String,
    pub arn: String,
    pub event_bus_name: String,
    pub event_pattern: String,
    pub schedule_expression: String,
    pub state: String,
    pub description: String,
    pub role_arn: String,
}

impl EventBridgeRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// event_bridge_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBridgeEntry {
    pub source: String,
    pub detail_type: String,
    pub detail: String,
    pub resources: Vec<String>,
    pub event_bus_name: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub trace_header: String,
}

impl EventBridgeEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// put_events_result_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutEventsResultEntry {
    pub event_id: String,
    pub error_code: String,
    pub error_message: String,
}

impl PutEventsResultEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// event_bridge_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBridgeTags {
    pub items: Vec<String>,
}

impl EventBridgeTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// event_bridge_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBridgeTag {
    pub key: String,
    pub value: String,
}

impl EventBridgeTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// event_bridge_target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBridgeTarget {
    pub id: String,
    pub arn: String,
    pub role_arn: String,
    pub input: String,
    pub input_path: String,
    pub input_transformer: String,
}

impl EventBridgeTarget {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// input_transformer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InputTransformer {
    pub input_paths_map: String,
    pub input_template: String,
}

impl InputTransformer {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// target_failure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TargetFailure {
    pub target_id: String,
    pub error_code: String,
    pub error_message: String,
}

impl TargetFailure {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// event_bus
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBus {
    pub name: String,
    pub arn: String,
    pub policy: String,
}

impl EventBus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// rule_condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RuleCondition {
    pub type: String,
    pub key: String,
    pub value: String,
}

impl RuleCondition {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// event_destination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventDestination {
    pub arn: String,
    pub filter_arns: Vec<String>,
}

impl EventDestination {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ecr_repository
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrRepository {
    pub repository_name: String,
    pub repository_uri: String,
    pub repository_arn: String,
    pub registry_id: String,
    pub image_tag_mutability: String,
    pub scan_on_push: bool,
    pub encryption_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl EcrRepository {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ecr_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrTags {
    pub items: Vec<String>,
}

impl EcrTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ecr_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrTag {
    pub key: String,
    pub value: String,
}

impl EcrTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// image_filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageFilter {
    pub tag_status: String,
}

impl ImageFilter {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// image_id
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageId {
    pub image_digest: String,
    pub image_tag: String,
}

impl ImageId {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ecr_image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrImage {
    pub registry_id: String,
    pub repository_name: String,
    pub image_id: String,
    pub image_manifest: String,
    pub image_manifest_media_type: String,
}

impl EcrImage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ecr_image_detail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrImageDetail {
    pub registry_id: String,
    pub repository_name: String,
    pub image_digest: String,
    pub image_tags: Vec<String>,
    pub image_size_in_bytes: i64,
    pub image_pushed_at: chrono::DateTime<chrono::Utc>,
    pub image_scan_status: String,
}

impl EcrImageDetail {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// image_failure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageFailure {
    pub image_id: String,
    pub failure_code: String,
    pub failure_reason: String,
}

impl ImageFailure {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// image_scan_status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageScanStatus {
    pub status: String,
    pub description: String,
}

impl ImageScanStatus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// image_scan_finding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageScanFinding {
    pub name: String,
    pub description: String,
    pub uri: String,
    pub severity: String,
    pub attributes: Vec<String>,
}

impl ImageScanFinding {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// scan_finding_attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScanFindingAttribute {
    pub key: String,
    pub value: String,
}

impl ScanFindingAttribute {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ses_email
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesEmail {
    pub message_id: String,
    pub from_address: String,
    pub to_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub bcc_addresses: Vec<String>,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
    pub configuration_set_name: String,
}

impl SesEmail {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ses_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesTags {
    pub items: Vec<String>,
}

impl SesTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ses_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesTag {
    pub name: String,
    pub value: String,
}

impl SesTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// default_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DefaultTags {
    pub items: Vec<String>,
}

impl DefaultTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// bulk_email_destination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkEmailDestination {
    pub destination: String,
    pub replacement_tags: Vec<String>,
    pub replacement_template_data: String,
}

impl BulkEmailDestination {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ses_destination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesDestination {
    pub to_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub bcc_addresses: Vec<String>,
}

impl SesDestination {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// bulk_email_status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkEmailStatus {
    pub status: String,
    pub error: String,
    pub message_id: String,
}

impl BulkEmailStatus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// verification_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationAttributes {
    pub entries: String,
}

impl VerificationAttributes {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// verification_attribute_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationAttributeEntry {
    pub verification_status: String,
    pub verification_token: String,
}

impl VerificationAttributeEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ses_template
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesTemplate {
    pub name: String,
    pub created_timestamp: chrono::DateTime<chrono::Utc>,
}

impl SesTemplate {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// send_data_point
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendDataPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub delivery_attempts: i64,
    pub bounces: i64,
    pub complaints: i64,
    pub rejects: i64,
}

impl SendDataPoint {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ses_configuration_set
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesConfigurationSet {
    pub name: String,
}

impl SesConfigurationSet {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// identity_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdentityPolicies {
    pub policies: String,
}

impl IdentityPolicies {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// dkim_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DkimAttributes {
    pub entries: String,
}

impl DkimAttributes {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// dkim_attribute_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DkimAttributeEntry {
    pub dkim_enabled: bool,
    pub dkim_verification_status: String,
    pub dkim_tokens: Vec<String>,
}

impl DkimAttributeEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sqs_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqsMessage {
    pub message_id: String,
    pub queue_url: String,
    pub message_body: String,
    pub message_attributes: String,
    pub receipt_handle: String,
    pub md5_of_body: String,
}

impl SqsMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// queue_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueueAttributes {
    pub delay_seconds: String,
    pub maximum_message_size: String,
    pub message_retention_period: String,
    pub policy: String,
    pub receive_message_wait_time_seconds: String,
    pub visibility_timeout: String,
    pub redrive_policy: String,
    pub kms_master_key_id: String,
    pub kms_data_key_reuse_period_seconds: String,
    pub fifo_queue: String,
    pub content_based_deduplication: String,
}

impl QueueAttributes {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sqs_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqsTags {
    pub tags: Vec<String>,
}

impl SqsTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// sqs_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqsTag {
    pub key: String,
    pub value: String,
}

impl SqsTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// message_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageAttributes {
    pub attributes: Vec<String>,
}

impl MessageAttributes {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// message_attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageAttribute {
    pub name: String,
    pub data_type: String,
    pub string_value: String,
    pub binary_value: String,
}

impl MessageAttribute {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// send_message_batch_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageBatchEntry {
    pub id: String,
    pub message_body: String,
    pub delay_seconds: i64,
    pub message_attributes: String,
    pub message_deduplication_id: String,
    pub message_group_id: String,
}

impl SendMessageBatchEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// send_message_batch_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageBatchResult {
    pub id: String,
    pub message_id: String,
    pub md5_of_message_body: String,
    pub md5_of_message_attributes: String,
    pub sequence_number: String,
}

impl SendMessageBatchResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// delete_message_batch_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageBatchEntry {
    pub id: String,
    pub receipt_handle: String,
}

impl DeleteMessageBatchEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// delete_message_batch_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageBatchResult {
    pub id: String,
}

impl DeleteMessageBatchResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// batch_result_error_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchResultErrorEntry {
    pub id: String,
    pub sender_fault: bool,
    pub code: String,
    pub message: String,
}

impl BatchResultErrorEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// change_message_visibility_batch_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityBatchEntry {
    pub id: String,
    pub receipt_handle: String,
    pub visibility_timeout: i64,
}

impl ChangeMessageVisibilityBatchEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// change_message_visibility_batch_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityBatchResult {
    pub id: String,
}

impl ChangeMessageVisibilityBatchResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloud_front_distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontDistribution {
    pub distribution_id: String,
    pub domain_name: String,
    pub arn: String,
    pub status: String,
    pub enabled: bool,
    pub aliases: Vec<String>,
    pub origins: Vec<String>,
    pub price_class: String,
    pub last_modified_time: chrono::DateTime<chrono::Utc>,
}

impl CloudFrontDistribution {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloud_front_origin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontOrigin {
    pub id: String,
    pub domain_name: String,
    pub origin_path: String,
    pub connection_attempts: i64,
    pub connection_timeout: i64,
}

impl CloudFrontOrigin {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cache_behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheBehavior {
    pub path_pattern: String,
    pub target_origin_id: String,
    pub viewer_protocol_policy: String,
    pub allowed_methods: Vec<String>,
    pub cached_methods: Vec<String>,
    pub cache_policy_id: String,
    pub origin_request_policy_id: String,
    pub compress: bool,
    pub smooth_streaming: bool,
}

impl CacheBehavior {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cache_key_parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheKeyParameters {
    pub headers_config: String,
    pub cookies_config: String,
    pub query_strings_config: String,
    pub enable_accept_encoding_gzip: bool,
    pub enable_accept_encoding_brotli: bool,
}

impl CacheKeyParameters {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// headers_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HeadersConfig {
    pub header_behavior: String,
    pub headers: Vec<String>,
}

impl HeadersConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cookies_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CookiesConfig {
    pub cookie_behavior: String,
    pub cookies: Vec<String>,
}

impl CookiesConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// query_strings_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryStringsConfig {
    pub query_string_behavior: String,
    pub query_strings: Vec<String>,
}

impl QueryStringsConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cors_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CorsConfig {
    pub access_control_allow_origins: Vec<String>,
    pub access_control_allow_headers: Vec<String>,
    pub access_control_allow_methods: Vec<String>,
    pub access_control_allow_credentials: bool,
    pub access_control_max_age_sec: i64,
    pub origin_override: bool,
}

impl CorsConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// security_headers_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecurityHeadersConfig {
    pub content_security_policy: String,
    pub content_type_options: String,
    pub frame_options: String,
    pub referrer_policy: String,
    pub strict_transport_security: String,
    pub xss_protection: String,
}

impl SecurityHeadersConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// custom_headers_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomHeadersConfig {
    pub items: Vec<String>,
}

impl CustomHeadersConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// custom_header
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomHeader {
    pub header: String,
    pub value: String,
    pub override: bool,
}

impl CustomHeader {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// custom_error_response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomErrorResponse {
    pub error_code: i64,
    pub response_page_path: String,
    pub response_code: String,
    pub error_caching_min_ttl: i64,
}

impl CustomErrorResponse {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloud_front_invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontInvalidation {
    pub id: String,
    pub status: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
}

impl CloudFrontInvalidation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloud_front_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontTags {
    pub items: Vec<String>,
}

impl CloudFrontTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloud_front_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontTag {
    pub key: String,
    pub value: String,
}

impl CloudFrontTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// distribution_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DistributionConfig {
    pub caller_reference: String,
    pub aliases: Vec<String>,
    pub default_root_object: String,
    pub origins: Vec<String>,
    pub default_cache_behavior: String,
    pub cache_behaviors: Vec<String>,
    pub custom_error_responses: Vec<String>,
    pub comment: String,
    pub logging: bool,
    pub price_class: String,
    pub enabled: bool,
    pub viewer_certificate: String,
    pub restrictions: String,
    pub web_acl_id: String,
}

impl DistributionConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cache_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CachePolicy {
    pub cache_policy_id: String,
    pub name: String,
    pub default_ttl: i64,
    pub max_ttl: i64,
    pub min_ttl: i64,
    pub comment: String,
}

impl CachePolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecret {
    pub arn: String,
    pub name: String,
    pub description: String,
    pub secret_value: String,
    pub version_id: String,
    pub kms_key_id: String,
    pub rotation_enabled: bool,
    pub last_changed_date: chrono::DateTime<chrono::Utc>,
    pub last_rotated_date: chrono::DateTime<chrono::Utc>,
}

impl AwsSecret {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// secrets_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretsTags {
    pub items: Vec<String>,
}

impl SecretsTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// secrets_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretsTag {
    pub key: String,
    pub value: String,
}

impl SecretsTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// secret_filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretFilter {
    pub key: String,
    pub values: Vec<String>,
}

impl SecretFilter {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// secret_list_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretListEntry {
    pub arn: String,
    pub name: String,
    pub description: String,
    pub kms_key_id: String,
    pub rotation_enabled: bool,
    pub last_accessed_date: chrono::DateTime<chrono::Utc>,
    pub last_changed_date: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
}

impl SecretListEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// rotation_rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotationRules {
    pub automatically_after_days: i64,
    pub duration: String,
    pub schedule_expression: String,
}

impl RotationRules {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// kms_key_ids
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KmsKeyIds {
    pub regions: String,
}

impl KmsKeyIds {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// replication_status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplicationStatus {
    pub region: String,
    pub kms_key_id: String,
    pub status: String,
    pub status_message: String,
    pub last_accessed_date: chrono::DateTime<chrono::Utc>,
}

impl ReplicationStatus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Certificate {
    pub certificate_arn: String,
    pub domain_name: String,
    pub subject_alternative_names: Vec<String>,
    pub domain_validation_options: Vec<String>,
    pub serial: String,
    pub subject: String,
    pub issuer: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub imported_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub revoked_at: chrono::DateTime<chrono::Utc>,
    pub revocation_reason: String,
    pub not_before: chrono::DateTime<chrono::Utc>,
    pub not_after: chrono::DateTime<chrono::Utc>,
    pub key_algorithm: String,
    pub signature_algorithm: String,
    pub in_use_by: Vec<String>,
    pub failure_reason: String,
    pub type: String,
    pub renewal_summary: String,
    pub key_usages: Vec<String>,
    pub extended_key_usages: Vec<String>,
    pub certificate_authority_arn: String,
    pub renewal_eligibility: String,
    pub options: String,
}

impl Certificate {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// certificate_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CertificateSummary {
    pub certificate_arn: String,
    pub domain_name: String,
    pub subject_alternative_names: Vec<String>,
    pub has_additional_subject_alternative_names: bool,
    pub status: String,
    pub type: String,
    pub key_algorithm: String,
    pub key_usages: Vec<String>,
    pub extended_key_usages: Vec<String>,
    pub in_use: bool,
    pub exported: bool,
    pub renewal_eligibility: String,
    pub not_before: chrono::DateTime<chrono::Utc>,
    pub not_after: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub imported_at: chrono::DateTime<chrono::Utc>,
    pub revoked_at: chrono::DateTime<chrono::Utc>,
}

impl CertificateSummary {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// domain_validation_option
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DomainValidationOption {
    pub domain_name: String,
    pub validation_domain: String,
    pub validation_status: String,
    pub resource_record: String,
    pub validation_method: String,
    pub validation_emails: Vec<String>,
}

impl DomainValidationOption {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// resource_record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResourceRecord {
    pub name: String,
    pub type: String,
    pub value: String,
}

impl ResourceRecord {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// renewal_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenewalSummary {
    pub renewal_status: String,
    pub domain_validation_options: Vec<String>,
    pub renewal_status_reason: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl RenewalSummary {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// key_usage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyUsage {
    pub name: String,
}

impl KeyUsage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// extended_key_usage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtendedKeyUsage {
    pub name: String,
    pub oid: String,
}

impl ExtendedKeyUsage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// certificate_options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CertificateOptions {
    pub certificate_transparency_logging_preference: String,
}

impl CertificateOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// certificate_filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CertificateFilters {
    pub extended_key_usage: Vec<String>,
    pub key_usage: Vec<String>,
    pub key_types: Vec<String>,
}

impl CertificateFilters {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// expiry_events_configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExpiryEventsConfiguration {
    pub days_before_expiry: i64,
}

impl ExpiryEventsConfiguration {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// account_configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountConfiguration {
    pub expiry_events: String,
}

impl AccountConfiguration {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// acm_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcmTags {
    pub tags: String,
}

impl AcmTags {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


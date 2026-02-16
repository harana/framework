// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcTags {
    pub type: std::collections::HashMap<String, String>,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcFilter {
    pub name: String,
    pub values: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcCidrBlockAssociation {
    pub association_id: String,
    pub cidr_block: String,
    pub cidr_block_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcCidrBlockState {
    pub state: String,
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcIpv6CidrBlockAssociation {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub ipv6_cidr_block_state: String,
    pub ipv6_pool: String,
    pub network_border_group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubnetIpv6CidrBlockAssociation {
    pub association_id: String,
    pub ipv6_cidr_block: String,
    pub ipv6_cidr_block_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubnetCidrBlockState {
    pub state: String,
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrivateDnsNameOptionsOnLaunch {
    pub enable_resource_name_dns_aaaa_record: bool,
    pub enable_resource_name_dns_a_record: bool,
    pub hostname_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InternetGateway {
    pub attachments: Vec<String>,
    pub internet_gateway_id: String,
    pub owner_id: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InternetGatewayAttachment {
    pub state: String,
    pub vpc_id: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NatGatewayAddress {
    pub allocation_id: String,
    pub network_interface_id: String,
    pub private_ip: String,
    pub public_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProvisionedBandwidth {
    pub provision_time: chrono::DateTime<chrono::Utc>,
    pub provisioned: String,
    pub request_time: chrono::DateTime<chrono::Utc>,
    pub requested: String,
    pub status: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteTableAssociationState {
    pub state: String,
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PropagatingVgw {
    pub gateway_id: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IpRange {
    pub cidr_ip: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Ipv6Range {
    pub cidr_ipv6: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PrefixListId {
    pub description: String,
    pub prefix_list_id: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReferencedSecurityGroup {
    pub group_id: String,
    pub peering_status: String,
    pub user_id: String,
    pub vpc_id: String,
    pub vpc_peering_connection_id: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NetworkAclAssociation {
    pub network_acl_association_id: String,
    pub network_acl_id: String,
    pub subnet_id: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IcmpTypeCode {
    pub code: i64,
    pub type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PortRange {
    pub from: i64,
    pub to: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CidrBlock {
    pub cidr_block: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Ipv6CidrBlock {
    pub ipv6_cidr_block: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VpcPeeringConnectionOptionsDescription {
    pub allow_dns_resolution_from_remote_vpc: bool,
    pub allow_egress_from_local_classic_link_to_remote_vpc: bool,
    pub allow_egress_from_local_vpc_to_remote_classic_link: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PeeringConnectionStatus {
    pub code: String,
    pub message: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DnsEntry {
    pub dns_name: String,
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DnsOptions {
    pub dns_record_ip_type: String,
    pub private_dns_only_for_inbound_resolver_endpoint: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecurityGroupIdentifier {
    pub group_id: String,
    pub group_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LastError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsuccessfulItem {
    pub error: String,
    pub resource_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsuccessfulItemError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DhcpOptions {
    pub dhcp_configurations: Vec<String>,
    pub dhcp_options_id: String,
    pub owner_id: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DhcpConfiguration {
    pub key: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttributeValue {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EgressOnlyInternetGateway {
    pub attachments: Vec<String>,
    pub egress_only_internet_gateway_id: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StsCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumedRoleUser {
    pub assumed_role_id: String,
    pub arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FederatedUser {
    pub federated_user_id: String,
    pub arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tags {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StsPolicyArn {
    pub arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3Object {
    pub bucket: String,
    pub key: String,
    pub etag: String,
    pub size: i64,
    pub content_type: String,
    pub storage_schema: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3Bucket {
    pub name: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3Tags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ObjectMetadata {
    pub content_type: String,
    pub content_encoding: String,
    pub content_language: String,
    pub content_disposition: String,
    pub cache_control: String,
    pub custom: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletedObject {
    pub key: String,
    pub version_id: String,
    pub delete_marker: bool,
    pub delete_marker_version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct S3Error {
    pub key: String,
    pub version_id: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptionRule {
    pub sse_algorithm: String,
    pub kms_master_key_id: String,
    pub bucket_key_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CorsRule {
    pub allowed_headers: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub expose_headers: Vec<String>,
    pub max_age_seconds: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LifecycleTransition {
    pub days: i64,
    pub storage_schema: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompletedPart {
    pub etag: String,
    pub part_number: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamTags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PermissionsBoundary {
    pub permissions_boundary_type: String,
    pub permissions_boundary_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IamGroup {
    pub group_id: String,
    pub group_name: String,
    pub arn: String,
    pub path: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessKeyMetadata {
    pub access_key_id: String,
    pub user_name: String,
    pub status: String,
    pub create_date: chrono::DateTime<chrono::Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutEventsResultEntry {
    pub event_id: String,
    pub error_code: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBridgeTags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBridgeTag {
    pub key: String,
    pub value: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InputTransformer {
    pub input_paths_map: std::collections::HashMap<String, String>,
    pub input_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TargetFailure {
    pub target_id: String,
    pub error_code: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventBus {
    pub name: String,
    pub arn: String,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RuleCondition {
    pub type: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventDestination {
    pub arn: String,
    pub filter_arns: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrTags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageFilter {
    pub tag_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageId {
    pub image_digest: String,
    pub image_tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EcrImage {
    pub registry_id: String,
    pub repository_name: String,
    pub image_id: String,
    pub image_manifest: String,
    pub image_manifest_media_type: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageFailure {
    pub image_id: String,
    pub failure_code: String,
    pub failure_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageScanStatus {
    pub status: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageScanFinding {
    pub name: String,
    pub description: String,
    pub uri: String,
    pub severity: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScanFindingAttribute {
    pub key: String,
    pub value: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesTags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesTag {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DefaultTags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkEmailDestination {
    pub destination: String,
    pub replacement_tags: Vec<String>,
    pub replacement_template_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesDestination {
    pub to_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub bcc_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkEmailStatus {
    pub status: String,
    pub error: String,
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationAttributes {
    pub entries: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationAttributeEntry {
    pub verification_status: String,
    pub verification_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesTemplate {
    pub name: String,
    pub created_timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendDataPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub delivery_attempts: i64,
    pub bounces: i64,
    pub complaints: i64,
    pub rejects: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SesConfigurationSet {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdentityPolicies {
    pub policies: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DkimAttributes {
    pub entries: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DkimAttributeEntry {
    pub dkim_enabled: bool,
    pub dkim_verification_status: String,
    pub dkim_tokens: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqsTags {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqsTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageAttributes {
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageAttribute {
    pub name: String,
    pub data_type: String,
    pub string_value: String,
    pub binary_value: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageBatchResult {
    pub id: String,
    pub message_id: String,
    pub md5_of_message_body: String,
    pub md5_of_message_attributes: String,
    pub sequence_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageBatchEntry {
    pub id: String,
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMessageBatchResult {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchResultErrorEntry {
    pub id: String,
    pub sender_fault: bool,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityBatchEntry {
    pub id: String,
    pub receipt_handle: String,
    pub visibility_timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeMessageVisibilityBatchResult {
    pub id: String,
}

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
    pub price_schema: String,
    pub last_modified_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontOrigin {
    pub id: String,
    pub domain_name: String,
    pub origin_path: String,
    pub connection_attempts: i64,
    pub connection_timeout: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheKeyParameters {
    pub headers_config: String,
    pub cookies_config: String,
    pub query_strings_config: String,
    pub enable_accept_encoding_gzip: bool,
    pub enable_accept_encoding_brotli: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HeadersConfig {
    pub header_behavior: String,
    pub headers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CookiesConfig {
    pub cookie_behavior: String,
    pub cookies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryStringsConfig {
    pub query_string_behavior: String,
    pub query_strings: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomHeadersConfig {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomHeader {
    pub header: String,
    pub value: String,
    pub override: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomErrorResponse {
    pub error_code: i64,
    pub response_page_path: String,
    pub response_code: String,
    pub error_caching_min_ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontInvalidation {
    pub id: String,
    pub status: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontTags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudFrontTag {
    pub key: String,
    pub value: String,
}

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
    pub price_schema: String,
    pub enabled: bool,
    pub viewer_certificate: String,
    pub restrictions: String,
    pub web_acl_id: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretsTags {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretsTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretFilter {
    pub key: String,
    pub values: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotationRules {
    pub automatically_after_days: i64,
    pub duration: String,
    pub schedule_expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KmsKeyIds {
    pub regions: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplicationStatus {
    pub region: String,
    pub kms_key_id: String,
    pub status: String,
    pub status_message: String,
    pub last_accessed_date: chrono::DateTime<chrono::Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResourceRecord {
    pub name: String,
    pub type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenewalSummary {
    pub renewal_status: String,
    pub domain_validation_options: Vec<String>,
    pub renewal_status_reason: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyUsage {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtendedKeyUsage {
    pub name: String,
    pub oid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CertificateOptions {
    pub certificate_transparency_logging_preference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CertificateFilters {
    pub extended_key_usage: Vec<String>,
    pub key_usage: Vec<String>,
    pub key_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExpiryEventsConfiguration {
    pub days_before_expiry: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountConfiguration {
    pub expiry_events: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcmTags {
    pub tags: std::collections::HashMap<String, String>,
}


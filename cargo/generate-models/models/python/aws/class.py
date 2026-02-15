# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class VpcTags(BaseModel):
    """
    vpc_tags
    
    ID fields: id
    """

    type: dict
    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcFilter(BaseModel):
    """
    vpc_filter
    
    ID fields: id
    """

    name: str
    values: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class Vpc(BaseModel):
    """
    vpc
    
    ID fields: id
    """

    cidr_block: str
    cidr_block_association_set: List[str]
    dhcp_options_id: str
    instance_tenancy: str
    ipv6_cidr_block_association_set: List[str]
    is_default: bool
    owner_id: str
    state: str
    tags: List[str]
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcCidrBlockAssociation(BaseModel):
    """
    vpc_cidr_block_association
    
    ID fields: id
    """

    association_id: str
    cidr_block: str
    cidr_block_state: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcCidrBlockState(BaseModel):
    """
    vpc_cidr_block_state
    
    ID fields: id
    """

    state: str
    status_message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcIpv6cidrBlockAssociation(BaseModel):
    """
    vpc_ipv6cidr_block_association
    
    ID fields: id
    """

    association_id: str
    ipv6_cidr_block: str
    ipv6_cidr_block_state: str
    ipv6_pool: str
    network_border_group: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Tag(BaseModel):
    """
    tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Subnet(BaseModel):
    """
    subnet
    
    ID fields: id
    """

    assign_ipv6_address_on_creation: bool
    availability_zone: str
    availability_zone_id: str
    available_ip_address_count: int
    cidr_block: str
    default_for_az: bool
    enable_dns64: bool
    ipv6_cidr_block_association_set: List[str]
    ipv6_native: bool
    map_customer_owned_ip_on_launch: bool
    map_public_ip_on_launch: bool
    outpost_arn: str
    owner_id: str
    private_dns_name_options_on_launch: str
    state: str
    subnet_arn: str
    subnet_id: str
    tags: List[str]
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SubnetIpv6cidrBlockAssociation(BaseModel):
    """
    subnet_ipv6cidr_block_association
    
    ID fields: id
    """

    association_id: str
    ipv6_cidr_block: str
    ipv6_cidr_block_state: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SubnetCidrBlockState(BaseModel):
    """
    subnet_cidr_block_state
    
    ID fields: id
    """

    state: str
    status_message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PrivateDnsNameOptionsOnLaunch(BaseModel):
    """
    private_dns_name_options_on_launch
    
    ID fields: id
    """

    enable_resource_name_dns_aaaa_record: bool
    enable_resource_name_dns_a_record: bool
    hostname_type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class InternetGateway(BaseModel):
    """
    internet_gateway
    
    ID fields: id
    """

    attachments: List[str]
    internet_gateway_id: str
    owner_id: str
    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class InternetGatewayAttachment(BaseModel):
    """
    internet_gateway_attachment
    
    ID fields: id
    """

    state: str
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class NatGateway(BaseModel):
    """
    nat_gateway
    
    ID fields: id
    """

    connectivity_type: str
    create_time: datetime
    delete_time: datetime
    failure_code: str
    failure_message: str
    nat_gateway_addresses: List[str]
    nat_gateway_id: str
    provisioned_bandwidth: str
    state: str
    subnet_id: str
    tags: List[str]
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class NatGatewayAddress(BaseModel):
    """
    nat_gateway_address
    
    ID fields: id
    """

    allocation_id: str
    network_interface_id: str
    private_ip: str
    public_ip: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ProvisionedBandwidth(BaseModel):
    """
    provisioned_bandwidth
    
    ID fields: id
    """

    provision_time: datetime
    provisioned: str
    request_time: datetime
    requested: str
    status: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteTable(BaseModel):
    """
    route_table
    
    ID fields: id
    """

    associations: List[str]
    owner_id: str
    propagating_vgws: List[str]
    route_table_id: str
    routes: List[str]
    tags: List[str]
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteTableAssociation(BaseModel):
    """
    route_table_association
    
    ID fields: id
    """

    association_state: str
    gateway_id: str
    main: bool
    route_table_association_id: str
    route_table_id: str
    subnet_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RouteTableAssociationState(BaseModel):
    """
    route_table_association_state
    
    ID fields: id
    """

    state: str
    status_message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PropagatingVgw(BaseModel):
    """
    propagating_vgw
    
    ID fields: id
    """

    gateway_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Route(BaseModel):
    """
    route
    
    ID fields: id
    """

    carrier_gateway_id: str
    core_network_arn: str
    destination_cidr_block: str
    destination_ipv6_cidr_block: str
    destination_prefix_list_id: str
    egress_only_internet_gateway_id: str
    gateway_id: str
    instance_id: str
    instance_owner_id: str
    local_gateway_id: str
    nat_gateway_id: str
    network_interface_id: str
    origin: str
    state: str
    transit_gateway_id: str
    vpc_peering_connection_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityGroup(BaseModel):
    """
    security_group
    
    ID fields: id
    """

    description: str
    group_id: str
    group_name: str
    ip_permissions: List[str]
    ip_permissions_egress: List[str]
    owner_id: str
    tags: List[str]
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class IpPermission(BaseModel):
    """
    ip_permission
    
    ID fields: id
    """

    from_port: int
    ip_protocol: str
    ip_ranges: List[str]
    ipv6_ranges: List[str]
    prefix_list_ids: List[str]
    to_port: int
    user_id_group_pairs: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class IpRange(BaseModel):
    """
    ip_range
    
    ID fields: id
    """

    cidr_ip: str
    description: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Ipv6range(BaseModel):
    """
    ipv6range
    
    ID fields: id
    """

    cidr_ipv6: str
    description: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PrefixListId(BaseModel):
    """
    prefix_list_id
    
    ID fields: id
    """

    description: str
    prefix_list_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class UserIdGroupPair(BaseModel):
    """
    user_id_group_pair
    
    ID fields: id
    """

    description: str
    group_id: str
    group_name: str
    peering_status: str
    user_id: str
    vpc_id: str
    vpc_peering_connection_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityGroupRule(BaseModel):
    """
    security_group_rule
    
    ID fields: id
    """

    cidr_ipv4: str
    cidr_ipv6: str
    description: str
    from_port: int
    group_id: str
    group_owner_id: str
    ip_protocol: str
    is_egress: bool
    prefix_list_id: str
    referenced_group_info: str
    security_group_rule_id: str
    tags: List[str]
    to_port: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ReferencedSecurityGroup(BaseModel):
    """
    referenced_security_group
    
    ID fields: id
    """

    group_id: str
    peering_status: str
    user_id: str
    vpc_id: str
    vpc_peering_connection_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class NetworkAcl(BaseModel):
    """
    network_acl
    
    ID fields: id
    """

    associations: List[str]
    entries: List[str]
    is_default: bool
    network_acl_id: str
    owner_id: str
    tags: List[str]
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class NetworkAclAssociation(BaseModel):
    """
    network_acl_association
    
    ID fields: id
    """

    network_acl_association_id: str
    network_acl_id: str
    subnet_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class NetworkAclEntry(BaseModel):
    """
    network_acl_entry
    
    ID fields: id
    """

    cidr_block: str
    egress: bool
    icmp_type_code: str
    ipv6_cidr_block: str
    port_range: str
    protocol: str
    rule_method: str
    rule_number: int
    class Config:
        from_attributes = True
        populate_by_name = True


class IcmpTypeCode(BaseModel):
    """
    icmp_type_code
    
    ID fields: id
    """

    code: int
    type: int
    class Config:
        from_attributes = True
        populate_by_name = True


class PortRange(BaseModel):
    """
    port_range
    
    ID fields: id
    """

    from: int
    to: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ElasticIp(BaseModel):
    """
    elastic_ip
    
    ID fields: id
    """

    allocation_id: str
    association_id: str
    carrier_ip: str
    customer_owned_ip: str
    customer_owned_ipv4_pool: str
    domain: str
    instance_id: str
    network_border_group: str
    network_interface_id: str
    network_interface_owner_id: str
    private_ip_address: str
    public_ip: str
    public_ipv4_pool: str
    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcPeeringConnection(BaseModel):
    """
    vpc_peering_connection
    
    ID fields: id
    """

    accepter_vpc_info: str
    expiration_time: datetime
    requester_vpc_info: str
    status: str
    tags: List[str]
    vpc_peering_connection_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcInfo(BaseModel):
    """
    vpc_info
    
    ID fields: id
    """

    cidr_block: str
    cidr_block_set: List[str]
    ipv6_cidr_block_set: List[str]
    owner_id: str
    peering_options: str
    region: str
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CidrBlock(BaseModel):
    """
    cidr_block
    
    ID fields: id
    """

    cidr_block: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Ipv6cidrBlock(BaseModel):
    """
    ipv6cidr_block
    
    ID fields: id
    """

    ipv6_cidr_block: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcPeeringConnectionOptionsDescription(BaseModel):
    """
    vpc_peering_connection_options_description
    
    ID fields: id
    """

    allow_dns_resolution_from_remote_vpc: bool
    allow_egress_from_local_classic_link_to_remote_vpc: bool
    allow_egress_from_local_vpc_to_remote_classic_link: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class PeeringConnectionStatus(BaseModel):
    """
    peering_connection_status
    
    ID fields: id
    """

    code: str
    message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VpcEndpoint(BaseModel):
    """
    vpc_endpoint
    
    ID fields: id
    """

    creation_timestamp: datetime
    dns_entries: List[str]
    dns_options: str
    groups: List[str]
    ip_address_type: str
    last_error: str
    network_interface_ids: List[str]
    owner_id: str
    policy_document: str
    private_dns_enabled: bool
    requester_managed: bool
    route_table_ids: List[str]
    service_name: str
    state: str
    subnet_ids: List[str]
    tags: List[str]
    vpc_endpoint_id: str
    vpc_endpoint_type: str
    vpc_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DnsEntry(BaseModel):
    """
    dns_entry
    
    ID fields: id
    """

    dns_name: str
    hosted_zone_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DnsOptions(BaseModel):
    """
    dns_options
    
    ID fields: id
    """

    dns_record_ip_type: str
    private_dns_only_for_inbound_resolver_endpoint: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityGroupIdentifier(BaseModel):
    """
    security_group_identifier
    
    ID fields: id
    """

    group_id: str
    group_name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class LastError(BaseModel):
    """
    last_error
    
    ID fields: id
    """

    code: str
    message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class UnsuccessfulItem(BaseModel):
    """
    unsuccessful_item
    
    ID fields: id
    """

    error: str
    resource_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class UnsuccessfulItemError(BaseModel):
    """
    unsuccessful_item_error
    
    ID fields: id
    """

    code: str
    message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DhcpOptions(BaseModel):
    """
    dhcp_options
    
    ID fields: id
    """

    dhcp_configurations: List[str]
    dhcp_options_id: str
    owner_id: str
    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class DhcpConfiguration(BaseModel):
    """
    dhcp_configuration
    
    ID fields: id
    """

    key: str
    values: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class AttributeValue(BaseModel):
    """
    attribute_value
    
    ID fields: id
    """

    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EgressOnlyInternetGateway(BaseModel):
    """
    egress_only_internet_gateway
    
    ID fields: id
    """

    attachments: List[str]
    egress_only_internet_gateway_id: str
    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class StsCredentials(BaseModel):
    """
    sts_credentials
    
    ID fields: id
    """

    access_key_id: str
    secret_access_key: str
    session_token: str
    expiration: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class AssumedRoleUser(BaseModel):
    """
    assumed_role_user
    
    ID fields: id
    """

    assumed_role_id: str
    arn: str
    class Config:
        from_attributes = True
        populate_by_name = True


class FederatedUser(BaseModel):
    """
    federated_user
    
    ID fields: id
    """

    federated_user_id: str
    arn: str
    class Config:
        from_attributes = True
        populate_by_name = True


class Tags(BaseModel):
    """
    tags
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class StsPolicyArn(BaseModel):
    """
    sts_policy_arn
    
    ID fields: id
    """

    arn: str
    class Config:
        from_attributes = True
        populate_by_name = True


class S3object(BaseModel):
    """
    s3object
    
    ID fields: id
    """

    bucket: str
    key: str
    etag: str
    size: int
    content_type: str
    storage_class: str
    last_modified: datetime
    metadata: str
    version_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class S3bucket(BaseModel):
    """
    s3bucket
    
    ID fields: id
    """

    name: str
    creation_date: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class S3tags(BaseModel):
    """
    s3tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class S3tag(BaseModel):
    """
    s3tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ObjectMetadata(BaseModel):
    """
    object_metadata
    
    ID fields: id
    """

    content_type: str
    content_encoding: str
    content_language: str
    content_disposition: str
    cache_control: str
    custom: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DeletedObject(BaseModel):
    """
    deleted_object
    
    ID fields: id
    """

    key: str
    version_id: str
    delete_marker: bool
    delete_marker_version_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class S3error(BaseModel):
    """
    s3error
    
    ID fields: id
    """

    key: str
    version_id: str
    code: str
    message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EncryptionRule(BaseModel):
    """
    encryption_rule
    
    ID fields: id
    """

    sse_algorithm: str
    kms_master_key_id: str
    bucket_key_enabled: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class CorsRule(BaseModel):
    """
    cors_rule
    
    ID fields: id
    """

    allowed_headers: List[str]
    allowed_methods: List[str]
    allowed_origins: List[str]
    expose_headers: List[str]
    max_age_seconds: int
    class Config:
        from_attributes = True
        populate_by_name = True


class LifecycleRule(BaseModel):
    """
    lifecycle_rule
    
    ID fields: id
    """

    id: str
    status: str
    prefix: str
    expiration_days: int
    noncurrent_version_expiration_days: int
    transitions: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class LifecycleTransition(BaseModel):
    """
    lifecycle_transition
    
    ID fields: id
    """

    days: int
    storage_class: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CompletedPart(BaseModel):
    """
    completed_part
    
    ID fields: id
    """

    etag: str
    part_number: int
    class Config:
        from_attributes = True
        populate_by_name = True


class IamUser(BaseModel):
    """
    iam_user
    
    ID fields: id
    """

    user_id: str
    user_name: str
    arn: str
    path: str
    create_date: datetime
    password_last_used: datetime
    permissions_boundary: str
    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class IamTags(BaseModel):
    """
    iam_tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class IamTag(BaseModel):
    """
    iam_tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PermissionsBoundary(BaseModel):
    """
    permissions_boundary
    
    ID fields: id
    """

    permissions_boundary_type: str
    permissions_boundary_arn: str
    class Config:
        from_attributes = True
        populate_by_name = True


class IamGroup(BaseModel):
    """
    iam_group
    
    ID fields: id
    """

    group_id: str
    group_name: str
    arn: str
    path: str
    create_date: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class IamRole(BaseModel):
    """
    iam_role
    
    ID fields: id
    """

    role_id: str
    role_name: str
    arn: str
    path: str
    create_date: datetime
    description: str
    max_session_duration: int
    assume_role_policy_document: str
    class Config:
        from_attributes = True
        populate_by_name = True


class IamPolicy(BaseModel):
    """
    iam_policy
    
    ID fields: id
    """

    policy_id: str
    policy_name: str
    arn: str
    path: str
    create_date: datetime
    update_date: datetime
    attachment_count: int
    is_attachable: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class AccessKeyMetadata(BaseModel):
    """
    access_key_metadata
    
    ID fields: id
    """

    access_key_id: str
    user_name: str
    status: str
    create_date: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class AccountSummary(BaseModel):
    """
    account_summary
    
    ID fields: id
    """

    users: int
    groups: int
    roles: int
    policies: int
    users_quota: int
    groups_quota: int
    roles_quota: int
    policies_quota: int
    server_certificates: int
    mfa_devices: int
    account_mfa_enabled: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeRule(BaseModel):
    """
    event_bridge_rule
    
    ID fields: id
    """

    name: str
    arn: str
    event_bus_name: str
    event_pattern: str
    schedule_expression: str
    state: str
    description: str
    role_arn: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeEntry(BaseModel):
    """
    event_bridge_entry
    
    ID fields: id
    """

    source: str
    detail_type: str
    detail: str
    resources: List[str]
    event_bus_name: str
    time: datetime
    trace_header: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PutEventsResultEntry(BaseModel):
    """
    put_events_result_entry
    
    ID fields: id
    """

    event_id: str
    error_code: str
    error_message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeTags(BaseModel):
    """
    event_bridge_tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeTag(BaseModel):
    """
    event_bridge_tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeTarget(BaseModel):
    """
    event_bridge_target
    
    ID fields: id
    """

    id: str
    arn: str
    role_arn: str
    input: str
    input_path: str
    input_transformer: str
    class Config:
        from_attributes = True
        populate_by_name = True


class InputTransformer(BaseModel):
    """
    input_transformer
    
    ID fields: id
    """

    input_paths_map: str
    input_template: str
    class Config:
        from_attributes = True
        populate_by_name = True


class TargetFailure(BaseModel):
    """
    target_failure
    
    ID fields: id
    """

    target_id: str
    error_code: str
    error_message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EventBus(BaseModel):
    """
    event_bus
    
    ID fields: id
    """

    name: str
    arn: str
    policy: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RuleCondition(BaseModel):
    """
    rule_condition
    
    ID fields: id
    """

    type: str
    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EventDestination(BaseModel):
    """
    event_destination
    
    ID fields: id
    """

    arn: str
    filter_arns: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class EcrRepository(BaseModel):
    """
    ecr_repository
    
    ID fields: id
    """

    repository_name: str
    repository_uri: str
    repository_arn: str
    registry_id: str
    image_tag_mutability: str
    scan_on_push: bool
    encryption_type: str
    created_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class EcrTags(BaseModel):
    """
    ecr_tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class EcrTag(BaseModel):
    """
    ecr_tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ImageFilter(BaseModel):
    """
    image_filter
    
    ID fields: id
    """

    tag_status: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ImageId(BaseModel):
    """
    image_id
    
    ID fields: id
    """

    image_digest: str
    image_tag: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EcrImage(BaseModel):
    """
    ecr_image
    
    ID fields: id
    """

    registry_id: str
    repository_name: str
    image_id: str
    image_manifest: str
    image_manifest_media_type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EcrImageDetail(BaseModel):
    """
    ecr_image_detail
    
    ID fields: id
    """

    registry_id: str
    repository_name: str
    image_digest: str
    image_tags: List[str]
    image_size_in_bytes: int
    image_pushed_at: datetime
    image_scan_status: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ImageFailure(BaseModel):
    """
    image_failure
    
    ID fields: id
    """

    image_id: str
    failure_code: str
    failure_reason: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ImageScanStatus(BaseModel):
    """
    image_scan_status
    
    ID fields: id
    """

    status: str
    description: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ImageScanFinding(BaseModel):
    """
    image_scan_finding
    
    ID fields: id
    """

    name: str
    description: str
    uri: str
    severity: str
    attributes: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class ScanFindingAttribute(BaseModel):
    """
    scan_finding_attribute
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SesEmail(BaseModel):
    """
    ses_email
    
    ID fields: id
    """

    message_id: str
    from_address: str
    to_addresses: List[str]
    cc_addresses: List[str]
    bcc_addresses: List[str]
    subject: str
    html_body: str
    text_body: str
    configuration_set_name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SesTags(BaseModel):
    """
    ses_tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SesTag(BaseModel):
    """
    ses_tag
    
    ID fields: id
    """

    name: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DefaultTags(BaseModel):
    """
    default_tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class BulkEmailDestination(BaseModel):
    """
    bulk_email_destination
    
    ID fields: id
    """

    destination: str
    replacement_tags: List[str]
    replacement_template_data: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SesDestination(BaseModel):
    """
    ses_destination
    
    ID fields: id
    """

    to_addresses: List[str]
    cc_addresses: List[str]
    bcc_addresses: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class BulkEmailStatus(BaseModel):
    """
    bulk_email_status
    
    ID fields: id
    """

    status: str
    error: str
    message_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VerificationAttributes(BaseModel):
    """
    verification_attributes
    
    ID fields: id
    """

    entries: str
    class Config:
        from_attributes = True
        populate_by_name = True


class VerificationAttributeEntry(BaseModel):
    """
    verification_attribute_entry
    
    ID fields: id
    """

    verification_status: str
    verification_token: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SesTemplate(BaseModel):
    """
    ses_template
    
    ID fields: id
    """

    name: str
    created_timestamp: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class SendDataPoint(BaseModel):
    """
    send_data_point
    
    ID fields: id
    """

    timestamp: datetime
    delivery_attempts: int
    bounces: int
    complaints: int
    rejects: int
    class Config:
        from_attributes = True
        populate_by_name = True


class SesConfigurationSet(BaseModel):
    """
    ses_configuration_set
    
    ID fields: id
    """

    name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class IdentityPolicies(BaseModel):
    """
    identity_policies
    
    ID fields: id
    """

    policies: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DkimAttributes(BaseModel):
    """
    dkim_attributes
    
    ID fields: id
    """

    entries: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DkimAttributeEntry(BaseModel):
    """
    dkim_attribute_entry
    
    ID fields: id
    """

    dkim_enabled: bool
    dkim_verification_status: str
    dkim_tokens: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SqsMessage(BaseModel):
    """
    sqs_message
    
    ID fields: id
    """

    message_id: str
    queue_url: str
    message_body: str
    message_attributes: str
    receipt_handle: str
    md5_of_body: str
    class Config:
        from_attributes = True
        populate_by_name = True


class QueueAttributes(BaseModel):
    """
    queue_attributes
    
    ID fields: id
    """

    delay_seconds: str
    maximum_message_size: str
    message_retention_period: str
    policy: str
    receive_message_wait_time_seconds: str
    visibility_timeout: str
    redrive_policy: str
    kms_master_key_id: str
    kms_data_key_reuse_period_seconds: str
    fifo_queue: str
    content_based_deduplication: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SqsTags(BaseModel):
    """
    sqs_tags
    
    ID fields: id
    """

    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SqsTag(BaseModel):
    """
    sqs_tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class MessageAttributes(BaseModel):
    """
    message_attributes
    
    ID fields: id
    """

    attributes: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class MessageAttribute(BaseModel):
    """
    message_attribute
    
    ID fields: id
    """

    name: str
    data_type: str
    string_value: str
    binary_value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SendMessageBatchEntry(BaseModel):
    """
    send_message_batch_entry
    
    ID fields: id
    """

    id: str
    message_body: str
    delay_seconds: int
    message_attributes: str
    message_deduplication_id: str
    message_group_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SendMessageBatchResult(BaseModel):
    """
    send_message_batch_result
    
    ID fields: id
    """

    id: str
    message_id: str
    md5_of_message_body: str
    md5_of_message_attributes: str
    sequence_number: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessageBatchEntry(BaseModel):
    """
    delete_message_batch_entry
    
    ID fields: id
    """

    id: str
    receipt_handle: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessageBatchResult(BaseModel):
    """
    delete_message_batch_result
    
    ID fields: id
    """

    id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class BatchResultErrorEntry(BaseModel):
    """
    batch_result_error_entry
    
    ID fields: id
    """

    id: str
    sender_fault: bool
    code: str
    message: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeMessageVisibilityBatchEntry(BaseModel):
    """
    change_message_visibility_batch_entry
    
    ID fields: id
    """

    id: str
    receipt_handle: str
    visibility_timeout: int
    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeMessageVisibilityBatchResult(BaseModel):
    """
    change_message_visibility_batch_result
    
    ID fields: id
    """

    id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontDistribution(BaseModel):
    """
    cloud_front_distribution
    
    ID fields: id
    """

    distribution_id: str
    domain_name: str
    arn: str
    status: str
    enabled: bool
    aliases: List[str]
    origins: List[str]
    price_class: str
    last_modified_time: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontOrigin(BaseModel):
    """
    cloud_front_origin
    
    ID fields: id
    """

    id: str
    domain_name: str
    origin_path: str
    connection_attempts: int
    connection_timeout: int
    class Config:
        from_attributes = True
        populate_by_name = True


class CacheBehavior(BaseModel):
    """
    cache_behavior
    
    ID fields: id
    """

    path_pattern: str
    target_origin_id: str
    viewer_protocol_policy: str
    allowed_methods: List[str]
    cached_methods: List[str]
    cache_policy_id: str
    origin_request_policy_id: str
    compress: bool
    smooth_streaming: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class CacheKeyParameters(BaseModel):
    """
    cache_key_parameters
    
    ID fields: id
    """

    headers_config: str
    cookies_config: str
    query_strings_config: str
    enable_accept_encoding_gzip: bool
    enable_accept_encoding_brotli: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class HeadersConfig(BaseModel):
    """
    headers_config
    
    ID fields: id
    """

    header_behavior: str
    headers: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class CookiesConfig(BaseModel):
    """
    cookies_config
    
    ID fields: id
    """

    cookie_behavior: str
    cookies: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class QueryStringsConfig(BaseModel):
    """
    query_strings_config
    
    ID fields: id
    """

    query_string_behavior: str
    query_strings: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class CorsConfig(BaseModel):
    """
    cors_config
    
    ID fields: id
    """

    access_control_allow_origins: List[str]
    access_control_allow_headers: List[str]
    access_control_allow_methods: List[str]
    access_control_allow_credentials: bool
    access_control_max_age_sec: int
    origin_override: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityHeadersConfig(BaseModel):
    """
    security_headers_config
    
    ID fields: id
    """

    content_security_policy: str
    content_type_options: str
    frame_options: str
    referrer_policy: str
    strict_transport_security: str
    xss_protection: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CustomHeadersConfig(BaseModel):
    """
    custom_headers_config
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class CustomHeader(BaseModel):
    """
    custom_header
    
    ID fields: id
    """

    header: str
    value: str
    override: bool
    class Config:
        from_attributes = True
        populate_by_name = True


class CustomErrorResponse(BaseModel):
    """
    custom_error_response
    
    ID fields: id
    """

    error_code: int
    response_page_path: str
    response_code: str
    error_caching_min_ttl: int
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontInvalidation(BaseModel):
    """
    cloud_front_invalidation
    
    ID fields: id
    """

    id: str
    status: str
    create_time: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontTags(BaseModel):
    """
    cloud_front_tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontTag(BaseModel):
    """
    cloud_front_tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class DistributionConfig(BaseModel):
    """
    distribution_config
    
    ID fields: id
    """

    caller_reference: str
    aliases: List[str]
    default_root_object: str
    origins: List[str]
    default_cache_behavior: str
    cache_behaviors: List[str]
    custom_error_responses: List[str]
    comment: str
    logging: bool
    price_class: str
    enabled: bool
    viewer_certificate: str
    restrictions: str
    web_acl_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CachePolicy(BaseModel):
    """
    cache_policy
    
    ID fields: id
    """

    cache_policy_id: str
    name: str
    default_ttl: int
    max_ttl: int
    min_ttl: int
    comment: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSecret(BaseModel):
    """
    aws_secret
    
    ID fields: id
    """

    arn: str
    name: str
    description: str
    secret_value: str
    version_id: str
    kms_key_id: str
    rotation_enabled: bool
    last_changed_date: datetime
    last_rotated_date: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class SecretsTags(BaseModel):
    """
    secrets_tags
    
    ID fields: id
    """

    items: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SecretsTag(BaseModel):
    """
    secrets_tag
    
    ID fields: id
    """

    key: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class SecretFilter(BaseModel):
    """
    secret_filter
    
    ID fields: id
    """

    key: str
    values: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class SecretListEntry(BaseModel):
    """
    secret_list_entry
    
    ID fields: id
    """

    arn: str
    name: str
    description: str
    kms_key_id: str
    rotation_enabled: bool
    last_accessed_date: datetime
    last_changed_date: datetime
    tags: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class RotationRules(BaseModel):
    """
    rotation_rules
    
    ID fields: id
    """

    automatically_after_days: int
    duration: str
    schedule_expression: str
    class Config:
        from_attributes = True
        populate_by_name = True


class KmsKeyIds(BaseModel):
    """
    kms_key_ids
    
    ID fields: id
    """

    regions: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ReplicationStatus(BaseModel):
    """
    replication_status
    
    ID fields: id
    """

    region: str
    kms_key_id: str
    status: str
    status_message: str
    last_accessed_date: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class Certificate(BaseModel):
    """
    certificate
    
    ID fields: id
    """

    certificate_arn: str
    domain_name: str
    subject_alternative_names: List[str]
    domain_validation_options: List[str]
    serial: str
    subject: str
    issuer: str
    created_at: datetime
    issued_at: datetime
    imported_at: datetime
    status: str
    revoked_at: datetime
    revocation_reason: str
    not_before: datetime
    not_after: datetime
    key_algorithm: str
    signature_algorithm: str
    in_use_by: List[str]
    failure_reason: str
    type: str
    renewal_summary: str
    key_usages: List[str]
    extended_key_usages: List[str]
    certificate_authority_arn: str
    renewal_eligibility: str
    options: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CertificateSummary(BaseModel):
    """
    certificate_summary
    
    ID fields: id
    """

    certificate_arn: str
    domain_name: str
    subject_alternative_names: List[str]
    has_additional_subject_alternative_names: bool
    status: str
    type: str
    key_algorithm: str
    key_usages: List[str]
    extended_key_usages: List[str]
    in_use: bool
    exported: bool
    renewal_eligibility: str
    not_before: datetime
    not_after: datetime
    created_at: datetime
    issued_at: datetime
    imported_at: datetime
    revoked_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class DomainValidationOption(BaseModel):
    """
    domain_validation_option
    
    ID fields: id
    """

    domain_name: str
    validation_domain: str
    validation_status: str
    resource_record: str
    validation_method: str
    validation_emails: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class ResourceRecord(BaseModel):
    """
    resource_record
    
    ID fields: id
    """

    name: str
    type: str
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class RenewalSummary(BaseModel):
    """
    renewal_summary
    
    ID fields: id
    """

    renewal_status: str
    domain_validation_options: List[str]
    renewal_status_reason: str
    updated_at: datetime
    class Config:
        from_attributes = True
        populate_by_name = True


class KeyUsage(BaseModel):
    """
    key_usage
    
    ID fields: id
    """

    name: str
    class Config:
        from_attributes = True
        populate_by_name = True


class ExtendedKeyUsage(BaseModel):
    """
    extended_key_usage
    
    ID fields: id
    """

    name: str
    oid: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CertificateOptions(BaseModel):
    """
    certificate_options
    
    ID fields: id
    """

    certificate_transparency_logging_preference: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CertificateFilters(BaseModel):
    """
    certificate_filters
    
    ID fields: id
    """

    extended_key_usage: List[str]
    key_usage: List[str]
    key_types: List[str]
    class Config:
        from_attributes = True
        populate_by_name = True


class ExpiryEventsConfiguration(BaseModel):
    """
    expiry_events_configuration
    
    ID fields: id
    """

    days_before_expiry: int
    class Config:
        from_attributes = True
        populate_by_name = True


class AccountConfiguration(BaseModel):
    """
    account_configuration
    
    ID fields: id
    """

    expiry_events: str
    class Config:
        from_attributes = True
        populate_by_name = True


class AcmTags(BaseModel):
    """
    acm_tags
    
    ID fields: id
    """

    tags: str
    class Config:
        from_attributes = True
        populate_by_name = True



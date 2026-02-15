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

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcFilter(BaseModel):
    """
    vpc_filter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Vpc(BaseModel):
    """
    vpc
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcCidrBlockAssociation(BaseModel):
    """
    vpc_cidr_block_association
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcCidrBlockState(BaseModel):
    """
    vpc_cidr_block_state
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcIpv6CidrBlockAssociation(BaseModel):
    """
    vpc_ipv6_cidr_block_association
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Tag(BaseModel):
    """
    tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Subnet(BaseModel):
    """
    subnet
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SubnetIpv6CidrBlockAssociation(BaseModel):
    """
    subnet_ipv6_cidr_block_association
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SubnetCidrBlockState(BaseModel):
    """
    subnet_cidr_block_state
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PrivateDnsNameOptionsOnLaunch(BaseModel):
    """
    private_dns_name_options_on_launch
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class InternetGateway(BaseModel):
    """
    internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class InternetGatewayAttachment(BaseModel):
    """
    internet_gateway_attachment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NatGateway(BaseModel):
    """
    nat_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NatGatewayAddress(BaseModel):
    """
    nat_gateway_address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ProvisionedBandwidth(BaseModel):
    """
    provisioned_bandwidth
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RouteTable(BaseModel):
    """
    route_table
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RouteTableAssociation(BaseModel):
    """
    route_table_association
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RouteTableAssociationState(BaseModel):
    """
    route_table_association_state
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PropagatingVgw(BaseModel):
    """
    propagating_vgw
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Route(BaseModel):
    """
    route
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityGroup(BaseModel):
    """
    security_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IpPermission(BaseModel):
    """
    ip_permission
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IpRange(BaseModel):
    """
    ip_range
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Ipv6Range(BaseModel):
    """
    ipv6_range
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PrefixListId(BaseModel):
    """
    prefix_list_id
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UserIdGroupPair(BaseModel):
    """
    user_id_group_pair
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityGroupRule(BaseModel):
    """
    security_group_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReferencedSecurityGroup(BaseModel):
    """
    referenced_security_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NetworkAcl(BaseModel):
    """
    network_acl
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NetworkAclAssociation(BaseModel):
    """
    network_acl_association
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class NetworkAclEntry(BaseModel):
    """
    network_acl_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IcmpTypeCode(BaseModel):
    """
    icmp_type_code
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PortRange(BaseModel):
    """
    port_range
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ElasticIp(BaseModel):
    """
    elastic_ip
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcPeeringConnection(BaseModel):
    """
    vpc_peering_connection
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcInfo(BaseModel):
    """
    vpc_info
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CidrBlock(BaseModel):
    """
    cidr_block
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Ipv6CidrBlock(BaseModel):
    """
    ipv6_cidr_block
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcPeeringConnectionOptionsDescription(BaseModel):
    """
    vpc_peering_connection_options_description
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PeeringConnectionStatus(BaseModel):
    """
    peering_connection_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VpcEndpoint(BaseModel):
    """
    vpc_endpoint
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DnsEntry(BaseModel):
    """
    dns_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DnsOptions(BaseModel):
    """
    dns_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityGroupIdentifier(BaseModel):
    """
    security_group_identifier
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LastError(BaseModel):
    """
    last_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UnsuccessfulItem(BaseModel):
    """
    unsuccessful_item
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UnsuccessfulItemError(BaseModel):
    """
    unsuccessful_item_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DhcpOptions(BaseModel):
    """
    dhcp_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DhcpConfiguration(BaseModel):
    """
    dhcp_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttributeValue(BaseModel):
    """
    attribute_value
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EgressOnlyInternetGateway(BaseModel):
    """
    egress_only_internet_gateway
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StsCredentials(BaseModel):
    """
    sts_credentials
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssumedRoleUser(BaseModel):
    """
    assumed_role_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class FederatedUser(BaseModel):
    """
    federated_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Tags(BaseModel):
    """
    tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StsPolicyArn(BaseModel):
    """
    sts_policy_arn
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class S3Object(BaseModel):
    """
    s3_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class S3Bucket(BaseModel):
    """
    s3_bucket
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class S3Tags(BaseModel):
    """
    s3_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class S3Tag(BaseModel):
    """
    s3_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ObjectMetadata(BaseModel):
    """
    object_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeletedObject(BaseModel):
    """
    deleted_object
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class S3Error(BaseModel):
    """
    s3_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EncryptionRule(BaseModel):
    """
    encryption_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CorsRule(BaseModel):
    """
    cors_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LifecycleRule(BaseModel):
    """
    lifecycle_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class LifecycleTransition(BaseModel):
    """
    lifecycle_transition
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CompletedPart(BaseModel):
    """
    completed_part
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IamUser(BaseModel):
    """
    iam_user
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IamTags(BaseModel):
    """
    iam_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IamTag(BaseModel):
    """
    iam_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PermissionsBoundary(BaseModel):
    """
    permissions_boundary
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IamGroup(BaseModel):
    """
    iam_group
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IamRole(BaseModel):
    """
    iam_role
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IamPolicy(BaseModel):
    """
    iam_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AccessKeyMetadata(BaseModel):
    """
    access_key_metadata
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AccountSummary(BaseModel):
    """
    account_summary
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeRule(BaseModel):
    """
    event_bridge_rule
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeEntry(BaseModel):
    """
    event_bridge_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PutEventsResultEntry(BaseModel):
    """
    put_events_result_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeTags(BaseModel):
    """
    event_bridge_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeTag(BaseModel):
    """
    event_bridge_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventBridgeTarget(BaseModel):
    """
    event_bridge_target
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class InputTransformer(BaseModel):
    """
    input_transformer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TargetFailure(BaseModel):
    """
    target_failure
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventBus(BaseModel):
    """
    event_bus
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RuleCondition(BaseModel):
    """
    rule_condition
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EventDestination(BaseModel):
    """
    event_destination
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EcrRepository(BaseModel):
    """
    ecr_repository
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EcrTags(BaseModel):
    """
    ecr_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EcrTag(BaseModel):
    """
    ecr_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImageFilter(BaseModel):
    """
    image_filter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImageId(BaseModel):
    """
    image_id
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EcrImage(BaseModel):
    """
    ecr_image
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EcrImageDetail(BaseModel):
    """
    ecr_image_detail
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImageFailure(BaseModel):
    """
    image_failure
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImageScanStatus(BaseModel):
    """
    image_scan_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ImageScanFinding(BaseModel):
    """
    image_scan_finding
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ScanFindingAttribute(BaseModel):
    """
    scan_finding_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SesEmail(BaseModel):
    """
    ses_email
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SesTags(BaseModel):
    """
    ses_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SesTag(BaseModel):
    """
    ses_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DefaultTags(BaseModel):
    """
    default_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BulkEmailDestination(BaseModel):
    """
    bulk_email_destination
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SesDestination(BaseModel):
    """
    ses_destination
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BulkEmailStatus(BaseModel):
    """
    bulk_email_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerificationAttributes(BaseModel):
    """
    verification_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerificationAttributeEntry(BaseModel):
    """
    verification_attribute_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SesTemplate(BaseModel):
    """
    ses_template
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendDataPoint(BaseModel):
    """
    send_data_point
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SesConfigurationSet(BaseModel):
    """
    ses_configuration_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class IdentityPolicies(BaseModel):
    """
    identity_policies
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DkimAttributes(BaseModel):
    """
    dkim_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DkimAttributeEntry(BaseModel):
    """
    dkim_attribute_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqsMessage(BaseModel):
    """
    sqs_message
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueueAttributes(BaseModel):
    """
    queue_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqsTags(BaseModel):
    """
    sqs_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SqsTag(BaseModel):
    """
    sqs_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MessageAttributes(BaseModel):
    """
    message_attributes
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MessageAttribute(BaseModel):
    """
    message_attribute
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendMessageBatchEntry(BaseModel):
    """
    send_message_batch_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendMessageBatchResult(BaseModel):
    """
    send_message_batch_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessageBatchEntry(BaseModel):
    """
    delete_message_batch_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteMessageBatchResult(BaseModel):
    """
    delete_message_batch_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class BatchResultErrorEntry(BaseModel):
    """
    batch_result_error_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeMessageVisibilityBatchEntry(BaseModel):
    """
    change_message_visibility_batch_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeMessageVisibilityBatchResult(BaseModel):
    """
    change_message_visibility_batch_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontDistribution(BaseModel):
    """
    cloud_front_distribution
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontOrigin(BaseModel):
    """
    cloud_front_origin
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CacheBehavior(BaseModel):
    """
    cache_behavior
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CacheKeyParameters(BaseModel):
    """
    cache_key_parameters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class HeadersConfig(BaseModel):
    """
    headers_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CookiesConfig(BaseModel):
    """
    cookies_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class QueryStringsConfig(BaseModel):
    """
    query_strings_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CorsConfig(BaseModel):
    """
    cors_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecurityHeadersConfig(BaseModel):
    """
    security_headers_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CustomHeadersConfig(BaseModel):
    """
    custom_headers_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CustomHeader(BaseModel):
    """
    custom_header
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CustomErrorResponse(BaseModel):
    """
    custom_error_response
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontInvalidation(BaseModel):
    """
    cloud_front_invalidation
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontTags(BaseModel):
    """
    cloud_front_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudFrontTag(BaseModel):
    """
    cloud_front_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DistributionConfig(BaseModel):
    """
    distribution_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CachePolicy(BaseModel):
    """
    cache_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AwsSecret(BaseModel):
    """
    aws_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecretsTags(BaseModel):
    """
    secrets_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecretsTag(BaseModel):
    """
    secrets_tag
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecretFilter(BaseModel):
    """
    secret_filter
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SecretListEntry(BaseModel):
    """
    secret_list_entry
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RotationRules(BaseModel):
    """
    rotation_rules
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class KmsKeyIds(BaseModel):
    """
    kms_key_ids
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReplicationStatus(BaseModel):
    """
    replication_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Certificate(BaseModel):
    """
    certificate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CertificateSummary(BaseModel):
    """
    certificate_summary
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DomainValidationOption(BaseModel):
    """
    domain_validation_option
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ResourceRecord(BaseModel):
    """
    resource_record
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RenewalSummary(BaseModel):
    """
    renewal_summary
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class KeyUsage(BaseModel):
    """
    key_usage
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExtendedKeyUsage(BaseModel):
    """
    extended_key_usage
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CertificateOptions(BaseModel):
    """
    certificate_options
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CertificateFilters(BaseModel):
    """
    certificate_filters
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ExpiryEventsConfiguration(BaseModel):
    """
    expiry_events_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AccountConfiguration(BaseModel):
    """
    account_configuration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AcmTags(BaseModel):
    """
    acm_tags
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True



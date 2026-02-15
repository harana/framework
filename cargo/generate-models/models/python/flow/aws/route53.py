# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreateHostedZone(BaseModel):
    """
    create_hosted_zone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteHostedZone(BaseModel):
    """
    delete_hosted_zone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetHostedZone(BaseModel):
    """
    get_hosted_zone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListHostedZones(BaseModel):
    """
    list_hosted_zones
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListHostedZonesByName(BaseModel):
    """
    list_hosted_zones_by_name
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListHostedZonesByVpc(BaseModel):
    """
    list_hosted_zones_by_vpc
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateHostedZoneComment(BaseModel):
    """
    update_hosted_zone_comment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetHostedZoneCount(BaseModel):
    """
    get_hosted_zone_count
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeResourceRecordSets(BaseModel):
    """
    change_resource_record_sets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListResourceRecordSets(BaseModel):
    """
    list_resource_record_sets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetChange(BaseModel):
    """
    get_change
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateHealthCheck(BaseModel):
    """
    create_health_check
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteHealthCheck(BaseModel):
    """
    delete_health_check
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetHealthCheck(BaseModel):
    """
    get_health_check
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListHealthChecks(BaseModel):
    """
    list_health_checks
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateHealthCheck(BaseModel):
    """
    update_health_check
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetHealthCheckStatus(BaseModel):
    """
    get_health_check_status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetHealthCheckLastFailureReason(BaseModel):
    """
    get_health_check_last_failure_reason
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetHealthCheckCount(BaseModel):
    """
    get_health_check_count
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateReusableDelegationSet(BaseModel):
    """
    create_reusable_delegation_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteReusableDelegationSet(BaseModel):
    """
    delete_reusable_delegation_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetReusableDelegationSet(BaseModel):
    """
    get_reusable_delegation_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListReusableDelegationSets(BaseModel):
    """
    list_reusable_delegation_sets
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTrafficPolicy(BaseModel):
    """
    create_traffic_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTrafficPolicy(BaseModel):
    """
    delete_traffic_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetTrafficPolicy(BaseModel):
    """
    get_traffic_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTrafficPolicies(BaseModel):
    """
    list_traffic_policies
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTrafficPolicyVersion(BaseModel):
    """
    create_traffic_policy_version
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTrafficPolicyVersions(BaseModel):
    """
    list_traffic_policy_versions
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateTrafficPolicyComment(BaseModel):
    """
    update_traffic_policy_comment
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateTrafficPolicyInstance(BaseModel):
    """
    create_traffic_policy_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteTrafficPolicyInstance(BaseModel):
    """
    delete_traffic_policy_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetTrafficPolicyInstance(BaseModel):
    """
    get_traffic_policy_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTrafficPolicyInstances(BaseModel):
    """
    list_traffic_policy_instances
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTrafficPolicyInstancesByHostedZone(BaseModel):
    """
    list_traffic_policy_instances_by_hosted_zone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTrafficPolicyInstancesByPolicy(BaseModel):
    """
    list_traffic_policy_instances_by_policy
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateTrafficPolicyInstance(BaseModel):
    """
    update_traffic_policy_instance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetTrafficPolicyInstanceCount(BaseModel):
    """
    get_traffic_policy_instance_count
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateQueryLoggingConfig(BaseModel):
    """
    create_query_logging_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteQueryLoggingConfig(BaseModel):
    """
    delete_query_logging_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetQueryLoggingConfig(BaseModel):
    """
    get_query_logging_config
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListQueryLoggingConfigs(BaseModel):
    """
    list_query_logging_configs
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AssociateVpcWithHostedZone(BaseModel):
    """
    associate_vpc_with_hosted_zone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DisassociateVpcFromHostedZone(BaseModel):
    """
    disassociate_vpc_from_hosted_zone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateVpcAssociationAuthorization(BaseModel):
    """
    create_vpc_association_authorization
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteVpcAssociationAuthorization(BaseModel):
    """
    delete_vpc_association_authorization
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListVpcAssociationAuthorizations(BaseModel):
    """
    list_vpc_association_authorizations
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class EnableHostedZoneDnssec(BaseModel):
    """
    enable_hosted_zone_dnssec
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DisableHostedZoneDnssec(BaseModel):
    """
    disable_hosted_zone_dnssec
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDnssec(BaseModel):
    """
    get_dnssec
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateKeySigningKey(BaseModel):
    """
    create_key_signing_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteKeySigningKey(BaseModel):
    """
    delete_key_signing_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ActivateKeySigningKey(BaseModel):
    """
    activate_key_signing_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeactivateKeySigningKey(BaseModel):
    """
    deactivate_key_signing_key
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class TestDnsAnswer(BaseModel):
    """
    test_dns_answer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetGeoLocation(BaseModel):
    """
    get_geo_location
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListGeoLocations(BaseModel):
    """
    list_geo_locations
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ChangeTagsForResource(BaseModel):
    """
    change_tags_for_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTagsForResource(BaseModel):
    """
    list_tags_for_resource
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListTagsForResources(BaseModel):
    """
    list_tags_for_resources
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetAccountLimit(BaseModel):
    """
    get_account_limit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetHostedZoneLimit(BaseModel):
    """
    get_hosted_zone_limit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetReusableDelegationSetLimit(BaseModel):
    """
    get_reusable_delegation_set_limit
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCheckerIpRanges(BaseModel):
    """
    get_checker_ip_ranges
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True



// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateHostedZoneInput {
    pub caller_reference: String,
    pub delegation_set_id: String,
    pub hosted_zone_config: String,
    pub name: String,
    pub vpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateHostedZoneOutput {
    pub change_info: String,
    pub delegation_set: String,
    pub hosted_zone: String,
    pub location: String,
    pub success: bool,
    pub vpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteHostedZoneInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteHostedZoneOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHostedZoneInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHostedZoneOutput {
    pub delegation_set: String,
    pub hosted_zone: String,
    pub vpcs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHostedZonesInput {
    pub delegation_set_id: String,
    pub hosted_zone_type: String,
    pub marker: String,
    pub max_items: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHostedZonesOutput {
    pub hosted_zones: Vec<String>,
    pub is_truncated: bool,
    pub marker: String,
    pub max_items: i64,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHostedZonesByNameInput {
    pub dns_name: String,
    pub hosted_zone_id: String,
    pub max_items: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHostedZonesByNameOutput {
    pub dns_name: String,
    pub hosted_zone_id: String,
    pub hosted_zones: Vec<String>,
    pub is_truncated: bool,
    pub max_items: i64,
    pub next_dns_name: String,
    pub next_hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHostedZonesByVpcInput {
    pub max_items: i64,
    pub next_token: String,
    pub vpc_id: String,
    pub vpc_region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHostedZonesByVpcOutput {
    pub hosted_zone_summaries: Vec<String>,
    pub max_items: i64,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateHostedZoneCommentInput {
    pub comment: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateHostedZoneCommentOutput {
    pub hosted_zone: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHostedZoneCountOutput {
    pub hosted_zone_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeResourceRecordSetsInput {
    pub change_batch: String,
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeResourceRecordSetsOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListResourceRecordSetsInput {
    pub hosted_zone_id: String,
    pub max_items: i64,
    pub start_record_identifier: String,
    pub start_record_name: String,
    pub start_record_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListResourceRecordSetsOutput {
    pub is_truncated: bool,
    pub max_items: i64,
    pub next_record_identifier: String,
    pub next_record_name: String,
    pub next_record_type: String,
    pub resource_record_sets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetChangeInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetChangeOutput {
    pub change_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateHealthCheckInput {
    pub caller_reference: String,
    pub health_check_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateHealthCheckOutput {
    pub health_check: String,
    pub location: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteHealthCheckInput {
    pub health_check_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteHealthCheckOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHealthCheckInput {
    pub health_check_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHealthCheckOutput {
    pub health_check: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHealthChecksInput {
    pub marker: String,
    pub max_items: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListHealthChecksOutput {
    pub health_checks: Vec<String>,
    pub is_truncated: bool,
    pub marker: String,
    pub max_items: i64,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateHealthCheckInput {
    pub alarm_identifier: String,
    pub child_health_checks: Vec<String>,
    pub disabled: bool,
    pub enable_sni: bool,
    pub failure_threshold: i64,
    pub fully_qualified_domain_name: String,
    pub health_check_id: String,
    pub health_check_version: i64,
    pub health_threshold: i64,
    pub insufficient_data_health_status: String,
    pub inverted: bool,
    pub ip_address: String,
    pub port: i64,
    pub regions: Vec<String>,
    pub reset_elements: String,
    pub resource_path: String,
    pub search_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateHealthCheckOutput {
    pub health_check: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHealthCheckStatusInput {
    pub health_check_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHealthCheckStatusOutput {
    pub health_check_observations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHealthCheckLastFailureReasonInput {
    pub health_check_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHealthCheckLastFailureReasonOutput {
    pub health_check_observations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHealthCheckCountOutput {
    pub health_check_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReusableDelegationSetInput {
    pub caller_reference: String,
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReusableDelegationSetOutput {
    pub delegation_set: String,
    pub location: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteReusableDelegationSetInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteReusableDelegationSetOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetReusableDelegationSetInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetReusableDelegationSetOutput {
    pub delegation_set: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListReusableDelegationSetsInput {
    pub marker: String,
    pub max_items: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListReusableDelegationSetsOutput {
    pub delegation_sets: Vec<String>,
    pub is_truncated: bool,
    pub marker: String,
    pub max_items: i64,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTrafficPolicyInput {
    pub comment: String,
    pub document: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTrafficPolicyOutput {
    pub location: String,
    pub success: bool,
    pub traffic_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTrafficPolicyInput {
    pub id: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTrafficPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTrafficPolicyInput {
    pub id: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTrafficPolicyOutput {
    pub traffic_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPoliciesInput {
    pub max_items: i64,
    pub traffic_policy_id_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPoliciesOutput {
    pub is_truncated: bool,
    pub max_items: i64,
    pub traffic_policy_id_marker: String,
    pub traffic_policy_summaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTrafficPolicyVersionInput {
    pub comment: String,
    pub document: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTrafficPolicyVersionOutput {
    pub location: String,
    pub success: bool,
    pub traffic_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyVersionsInput {
    pub id: String,
    pub max_items: i64,
    pub traffic_policy_version_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyVersionsOutput {
    pub is_truncated: bool,
    pub max_items: i64,
    pub traffic_policies: Vec<String>,
    pub traffic_policy_version_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTrafficPolicyCommentInput {
    pub comment: String,
    pub id: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTrafficPolicyCommentOutput {
    pub success: bool,
    pub traffic_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTrafficPolicyInstanceInput {
    pub hosted_zone_id: String,
    pub name: String,
    pub traffic_policy_id: String,
    pub traffic_policy_version: i64,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTrafficPolicyInstanceOutput {
    pub location: String,
    pub success: bool,
    pub traffic_policy_instance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTrafficPolicyInstanceInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTrafficPolicyInstanceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTrafficPolicyInstanceInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTrafficPolicyInstanceOutput {
    pub traffic_policy_instance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyInstancesInput {
    pub hosted_zone_id_marker: String,
    pub max_items: i64,
    pub traffic_policy_instance_name_marker: String,
    pub traffic_policy_instance_type_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyInstancesOutput {
    pub hosted_zone_id_marker: String,
    pub is_truncated: bool,
    pub max_items: i64,
    pub traffic_policy_instance_name_marker: String,
    pub traffic_policy_instance_type_marker: String,
    pub traffic_policy_instances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyInstancesByHostedZoneInput {
    pub hosted_zone_id: String,
    pub max_items: i64,
    pub traffic_policy_instance_name_marker: String,
    pub traffic_policy_instance_type_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyInstancesByHostedZoneOutput {
    pub is_truncated: bool,
    pub max_items: i64,
    pub traffic_policy_instance_name_marker: String,
    pub traffic_policy_instance_type_marker: String,
    pub traffic_policy_instances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyInstancesByPolicyInput {
    pub hosted_zone_id_marker: String,
    pub max_items: i64,
    pub traffic_policy_id: String,
    pub traffic_policy_instance_name_marker: String,
    pub traffic_policy_instance_type_marker: String,
    pub traffic_policy_version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTrafficPolicyInstancesByPolicyOutput {
    pub hosted_zone_id_marker: String,
    pub is_truncated: bool,
    pub max_items: i64,
    pub traffic_policy_instance_name_marker: String,
    pub traffic_policy_instance_type_marker: String,
    pub traffic_policy_instances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTrafficPolicyInstanceInput {
    pub id: String,
    pub traffic_policy_id: String,
    pub traffic_policy_version: i64,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTrafficPolicyInstanceOutput {
    pub success: bool,
    pub traffic_policy_instance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTrafficPolicyInstanceCountOutput {
    pub traffic_policy_instance_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateQueryLoggingConfigInput {
    pub cloud_watch_logs_log_group_arn: String,
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateQueryLoggingConfigOutput {
    pub location: String,
    pub query_logging_config: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteQueryLoggingConfigInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteQueryLoggingConfigOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueryLoggingConfigInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetQueryLoggingConfigOutput {
    pub query_logging_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueryLoggingConfigsInput {
    pub hosted_zone_id: String,
    pub max_results: i64,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueryLoggingConfigsOutput {
    pub next_token: String,
    pub query_logging_configs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateVpcWithHostedZoneInput {
    pub comment: String,
    pub hosted_zone_id: String,
    pub vpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssociateVpcWithHostedZoneOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisassociateVpcFromHostedZoneInput {
    pub comment: String,
    pub hosted_zone_id: String,
    pub vpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisassociateVpcFromHostedZoneOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcAssociationAuthorizationInput {
    pub hosted_zone_id: String,
    pub vpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcAssociationAuthorizationOutput {
    pub hosted_zone_id: String,
    pub success: bool,
    pub vpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcAssociationAuthorizationInput {
    pub hosted_zone_id: String,
    pub vpc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVpcAssociationAuthorizationOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVpcAssociationAuthorizationsInput {
    pub hosted_zone_id: String,
    pub max_results: i64,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVpcAssociationAuthorizationsOutput {
    pub hosted_zone_id: String,
    pub next_token: String,
    pub vpcs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableHostedZoneDnssecInput {
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableHostedZoneDnssecOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableHostedZoneDnssecInput {
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableHostedZoneDnssecOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDnssecInput {
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDnssecOutput {
    pub key_signing_keys: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateKeySigningKeyInput {
    pub caller_reference: String,
    pub hosted_zone_id: String,
    pub key_management_service_arn: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateKeySigningKeyOutput {
    pub change_info: String,
    pub key_signing_key: String,
    pub location: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteKeySigningKeyInput {
    pub hosted_zone_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteKeySigningKeyOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActivateKeySigningKeyInput {
    pub hosted_zone_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActivateKeySigningKeyOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeactivateKeySigningKeyInput {
    pub hosted_zone_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeactivateKeySigningKeyOutput {
    pub change_info: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestDnsAnswerInput {
    pub edns0_client_subnet_ip: String,
    pub edns0_client_subnet_mask: String,
    pub hosted_zone_id: String,
    pub record_name: String,
    pub record_type: String,
    pub resolver_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestDnsAnswerOutput {
    pub nameserver: String,
    pub protocol: String,
    pub record_data: Vec<String>,
    pub record_name: String,
    pub record_type: String,
    pub response_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetGeoLocationInput {
    pub continent_code: String,
    pub country_code: String,
    pub subdivision_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetGeoLocationOutput {
    pub geo_location_details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListGeoLocationsInput {
    pub max_items: i64,
    pub start_continent_code: String,
    pub start_country_code: String,
    pub start_subdivision_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListGeoLocationsOutput {
    pub geo_location_details_list: Vec<String>,
    pub is_truncated: bool,
    pub max_items: i64,
    pub next_continent_code: String,
    pub next_country_code: String,
    pub next_subdivision_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeTagsForResourceInput {
    pub add_tags: Vec<String>,
    pub remove_tag_keys: Vec<String>,
    pub resource_id: String,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangeTagsForResourceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourceInput {
    pub resource_id: String,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourceOutput {
    pub resource_tag_set: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourcesInput {
    pub resource_ids: Vec<String>,
    pub resource_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourcesOutput {
    pub resource_tag_sets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountLimitInput {
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountLimitOutput {
    pub count: i64,
    pub limit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHostedZoneLimitInput {
    pub hosted_zone_id: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHostedZoneLimitOutput {
    pub count: i64,
    pub limit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetReusableDelegationSetLimitInput {
    pub delegation_set_id: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetReusableDelegationSetLimitOutput {
    pub count: i64,
    pub limit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCheckerIpRangesOutput {
    pub checker_ip_ranges: Vec<String>,
}

#[async_trait]
pub trait Route53Action {
    async fn create_hosted_zone(&self, input: CreateHostedZoneInput) -> Result<CreateHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn delete_hosted_zone(&self, input: DeleteHostedZoneInput) -> Result<DeleteHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn get_hosted_zone(&self, input: GetHostedZoneInput) -> Result<GetHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn list_hosted_zones(&self, input: ListHostedZonesInput) -> Result<ListHostedZonesOutput, Box<dyn std::error::Error>>;
    async fn list_hosted_zones_by_name(&self, input: ListHostedZonesByNameInput) -> Result<ListHostedZonesByNameOutput, Box<dyn std::error::Error>>;
    async fn list_hosted_zones_by_vpc(&self, input: ListHostedZonesByVpcInput) -> Result<ListHostedZonesByVpcOutput, Box<dyn std::error::Error>>;
    async fn update_hosted_zone_comment(&self, input: UpdateHostedZoneCommentInput) -> Result<UpdateHostedZoneCommentOutput, Box<dyn std::error::Error>>;
    async fn get_hosted_zone_count(&self) -> Result<GetHostedZoneCountOutput, Box<dyn std::error::Error>>;
    async fn change_resource_record_sets(&self, input: ChangeResourceRecordSetsInput) -> Result<ChangeResourceRecordSetsOutput, Box<dyn std::error::Error>>;
    async fn list_resource_record_sets(&self, input: ListResourceRecordSetsInput) -> Result<ListResourceRecordSetsOutput, Box<dyn std::error::Error>>;
    async fn get_change(&self, input: GetChangeInput) -> Result<GetChangeOutput, Box<dyn std::error::Error>>;
    async fn create_health_check(&self, input: CreateHealthCheckInput) -> Result<CreateHealthCheckOutput, Box<dyn std::error::Error>>;
    async fn delete_health_check(&self, input: DeleteHealthCheckInput) -> Result<DeleteHealthCheckOutput, Box<dyn std::error::Error>>;
    async fn get_health_check(&self, input: GetHealthCheckInput) -> Result<GetHealthCheckOutput, Box<dyn std::error::Error>>;
    async fn list_health_checks(&self, input: ListHealthChecksInput) -> Result<ListHealthChecksOutput, Box<dyn std::error::Error>>;
    async fn update_health_check(&self, input: UpdateHealthCheckInput) -> Result<UpdateHealthCheckOutput, Box<dyn std::error::Error>>;
    async fn get_health_check_status(&self, input: GetHealthCheckStatusInput) -> Result<GetHealthCheckStatusOutput, Box<dyn std::error::Error>>;
    async fn get_health_check_last_failure_reason(&self, input: GetHealthCheckLastFailureReasonInput) -> Result<GetHealthCheckLastFailureReasonOutput, Box<dyn std::error::Error>>;
    async fn get_health_check_count(&self) -> Result<GetHealthCheckCountOutput, Box<dyn std::error::Error>>;
    async fn create_reusable_delegation_set(&self, input: CreateReusableDelegationSetInput) -> Result<CreateReusableDelegationSetOutput, Box<dyn std::error::Error>>;
    async fn delete_reusable_delegation_set(&self, input: DeleteReusableDelegationSetInput) -> Result<DeleteReusableDelegationSetOutput, Box<dyn std::error::Error>>;
    async fn get_reusable_delegation_set(&self, input: GetReusableDelegationSetInput) -> Result<GetReusableDelegationSetOutput, Box<dyn std::error::Error>>;
    async fn list_reusable_delegation_sets(&self, input: ListReusableDelegationSetsInput) -> Result<ListReusableDelegationSetsOutput, Box<dyn std::error::Error>>;
    async fn create_traffic_policy(&self, input: CreateTrafficPolicyInput) -> Result<CreateTrafficPolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_traffic_policy(&self, input: DeleteTrafficPolicyInput) -> Result<DeleteTrafficPolicyOutput, Box<dyn std::error::Error>>;
    async fn get_traffic_policy(&self, input: GetTrafficPolicyInput) -> Result<GetTrafficPolicyOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policies(&self, input: ListTrafficPoliciesInput) -> Result<ListTrafficPoliciesOutput, Box<dyn std::error::Error>>;
    async fn create_traffic_policy_version(&self, input: CreateTrafficPolicyVersionInput) -> Result<CreateTrafficPolicyVersionOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_versions(&self, input: ListTrafficPolicyVersionsInput) -> Result<ListTrafficPolicyVersionsOutput, Box<dyn std::error::Error>>;
    async fn update_traffic_policy_comment(&self, input: UpdateTrafficPolicyCommentInput) -> Result<UpdateTrafficPolicyCommentOutput, Box<dyn std::error::Error>>;
    async fn create_traffic_policy_instance(&self, input: CreateTrafficPolicyInstanceInput) -> Result<CreateTrafficPolicyInstanceOutput, Box<dyn std::error::Error>>;
    async fn delete_traffic_policy_instance(&self, input: DeleteTrafficPolicyInstanceInput) -> Result<DeleteTrafficPolicyInstanceOutput, Box<dyn std::error::Error>>;
    async fn get_traffic_policy_instance(&self, input: GetTrafficPolicyInstanceInput) -> Result<GetTrafficPolicyInstanceOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_instances(&self, input: ListTrafficPolicyInstancesInput) -> Result<ListTrafficPolicyInstancesOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_instances_by_hosted_zone(&self, input: ListTrafficPolicyInstancesByHostedZoneInput) -> Result<ListTrafficPolicyInstancesByHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_instances_by_policy(&self, input: ListTrafficPolicyInstancesByPolicyInput) -> Result<ListTrafficPolicyInstancesByPolicyOutput, Box<dyn std::error::Error>>;
    async fn update_traffic_policy_instance(&self, input: UpdateTrafficPolicyInstanceInput) -> Result<UpdateTrafficPolicyInstanceOutput, Box<dyn std::error::Error>>;
    async fn get_traffic_policy_instance_count(&self) -> Result<GetTrafficPolicyInstanceCountOutput, Box<dyn std::error::Error>>;
    async fn create_query_logging_config(&self, input: CreateQueryLoggingConfigInput) -> Result<CreateQueryLoggingConfigOutput, Box<dyn std::error::Error>>;
    async fn delete_query_logging_config(&self, input: DeleteQueryLoggingConfigInput) -> Result<DeleteQueryLoggingConfigOutput, Box<dyn std::error::Error>>;
    async fn get_query_logging_config(&self, input: GetQueryLoggingConfigInput) -> Result<GetQueryLoggingConfigOutput, Box<dyn std::error::Error>>;
    async fn list_query_logging_configs(&self, input: ListQueryLoggingConfigsInput) -> Result<ListQueryLoggingConfigsOutput, Box<dyn std::error::Error>>;
    async fn associate_vpc_with_hosted_zone(&self, input: AssociateVpcWithHostedZoneInput) -> Result<AssociateVpcWithHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn disassociate_vpc_from_hosted_zone(&self, input: DisassociateVpcFromHostedZoneInput) -> Result<DisassociateVpcFromHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn create_vpc_association_authorization(&self, input: CreateVpcAssociationAuthorizationInput) -> Result<CreateVpcAssociationAuthorizationOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc_association_authorization(&self, input: DeleteVpcAssociationAuthorizationInput) -> Result<DeleteVpcAssociationAuthorizationOutput, Box<dyn std::error::Error>>;
    async fn list_vpc_association_authorizations(&self, input: ListVpcAssociationAuthorizationsInput) -> Result<ListVpcAssociationAuthorizationsOutput, Box<dyn std::error::Error>>;
    async fn enable_hosted_zone_dnssec(&self, input: EnableHostedZoneDnssecInput) -> Result<EnableHostedZoneDnssecOutput, Box<dyn std::error::Error>>;
    async fn disable_hosted_zone_dnssec(&self, input: DisableHostedZoneDnssecInput) -> Result<DisableHostedZoneDnssecOutput, Box<dyn std::error::Error>>;
    async fn get_dnssec(&self, input: GetDnssecInput) -> Result<GetDnssecOutput, Box<dyn std::error::Error>>;
    async fn create_key_signing_key(&self, input: CreateKeySigningKeyInput) -> Result<CreateKeySigningKeyOutput, Box<dyn std::error::Error>>;
    async fn delete_key_signing_key(&self, input: DeleteKeySigningKeyInput) -> Result<DeleteKeySigningKeyOutput, Box<dyn std::error::Error>>;
    async fn activate_key_signing_key(&self, input: ActivateKeySigningKeyInput) -> Result<ActivateKeySigningKeyOutput, Box<dyn std::error::Error>>;
    async fn deactivate_key_signing_key(&self, input: DeactivateKeySigningKeyInput) -> Result<DeactivateKeySigningKeyOutput, Box<dyn std::error::Error>>;
    async fn test_dns_answer(&self, input: TestDnsAnswerInput) -> Result<TestDnsAnswerOutput, Box<dyn std::error::Error>>;
    async fn get_geo_location(&self, input: GetGeoLocationInput) -> Result<GetGeoLocationOutput, Box<dyn std::error::Error>>;
    async fn list_geo_locations(&self, input: ListGeoLocationsInput) -> Result<ListGeoLocationsOutput, Box<dyn std::error::Error>>;
    async fn change_tags_for_resource(&self, input: ChangeTagsForResourceInput) -> Result<ChangeTagsForResourceOutput, Box<dyn std::error::Error>>;
    async fn list_tags_for_resource(&self, input: ListTagsForResourceInput) -> Result<ListTagsForResourceOutput, Box<dyn std::error::Error>>;
    async fn list_tags_for_resources(&self, input: ListTagsForResourcesInput) -> Result<ListTagsForResourcesOutput, Box<dyn std::error::Error>>;
    async fn get_account_limit(&self, input: GetAccountLimitInput) -> Result<GetAccountLimitOutput, Box<dyn std::error::Error>>;
    async fn get_hosted_zone_limit(&self, input: GetHostedZoneLimitInput) -> Result<GetHostedZoneLimitOutput, Box<dyn std::error::Error>>;
    async fn get_reusable_delegation_set_limit(&self, input: GetReusableDelegationSetLimitInput) -> Result<GetReusableDelegationSetLimitOutput, Box<dyn std::error::Error>>;
    async fn get_checker_ip_ranges(&self) -> Result<GetCheckerIpRangesOutput, Box<dyn std::error::Error>>;
}

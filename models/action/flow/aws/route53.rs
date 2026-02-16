// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateHostedZoneOutput {
    pub change_info: String,
    pub delegation_set: String,
    pub hosted_zone: String,
    pub location: String,
    pub vpc: String,
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
pub struct ListHostedZonesOutput {
    pub hosted_zones: Vec<String>,
    pub is_truncated: bool,
    pub marker: String,
    pub max_items: i64,
    pub next_marker: String,
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
pub struct ListHostedZonesByVpcOutput {
    pub hosted_zone_summaries: Vec<String>,
    pub max_items: i64,
    pub next_token: String,
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
pub struct CreateHealthCheckOutput {
    pub health_check: String,
    pub location: String,
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
pub struct CreateReusableDelegationSetOutput {
    pub delegation_set: String,
    pub location: String,
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
pub struct CreateTrafficPolicyOutput {
    pub location: String,
    pub traffic_policy: String,
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
pub struct CreateTrafficPolicyVersionOutput {
    pub location: String,
    pub traffic_policy: String,
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
pub struct CreateTrafficPolicyInstanceOutput {
    pub location: String,
    pub traffic_policy_instance: String,
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
pub struct ListTrafficPolicyInstancesByHostedZoneOutput {
    pub is_truncated: bool,
    pub max_items: i64,
    pub traffic_policy_instance_name_marker: String,
    pub traffic_policy_instance_type_marker: String,
    pub traffic_policy_instances: Vec<String>,
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
pub struct CreateQueryLoggingConfigOutput {
    pub location: String,
    pub query_logging_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListQueryLoggingConfigsOutput {
    pub next_token: String,
    pub query_logging_configs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVpcAssociationAuthorizationOutput {
    pub hosted_zone_id: String,
    pub vpc: String,
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
pub struct GetDnssecOutput {
    pub key_signing_keys: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateKeySigningKeyOutput {
    pub change_info: String,
    pub key_signing_key: String,
    pub location: String,
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
pub struct GetAccountLimitOutput {
    pub count: i64,
    pub limit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetHostedZoneLimitOutput {
    pub count: i64,
    pub limit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetReusableDelegationSetLimitOutput {
    pub count: i64,
    pub limit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRoute53HostedZone {
    pub account_id: String,
    pub caller_reference: String,
    pub comment: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub hosted_zone_id: String,
    #[serde(default)]
    pub is_private: bool,
    pub name: String,
    pub name_servers: String,
    pub record_set_count: i64,
    pub region: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRoute53RecordSet {
    pub alias_dns_name: String,
    #[serde(default)]
    pub alias_evaluate_target_health: bool,
    pub alias_hosted_zone_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub failover: String,
    pub health_check_id: String,
    pub hosted_zone_id: String,
    #[serde(default)]
    pub multi_value_answer: bool,
    pub name: String,
    pub region: String,
    pub resource_records: String,
    pub set_identifier: String,
    pub ttl: i64,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub weight: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRoute53HealthCheck {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub enable_sni: bool,
    pub failure_threshold: i64,
    pub fqdn: String,
    pub health_check_id: String,
    pub health_threshold: i64,
    pub insufficient_data_health_status: String,
    #[serde(default)]
    pub inverted: bool,
    pub ip_address: String,
    pub port: i64,
    pub region: String,
    pub request_interval: i64,
    pub resource_path: String,
    pub search_string: String,
    pub status: String,
    pub tags: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRoute53TrafficPolicy {
    pub account_id: String,
    pub comment: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub document: String,
    pub name: String,
    pub region: String,
    pub traffic_policy_id: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

#[async_trait]
pub trait Route53Action {
    async fn create_hosted_zone(&self, caller_reference: String, delegation_set_id: String, hosted_zone_config: String, name: String, vpc: String) -> Result<CreateHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn delete_hosted_zone(&self, id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_hosted_zone(&self, id: String) -> Result<GetHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn list_hosted_zones(&self, delegation_set_id: String, hosted_zone_type: String, marker: String, max_items: i64) -> Result<ListHostedZonesOutput, Box<dyn std::error::Error>>;
    async fn list_hosted_zones_by_name(&self, dns_name: String, hosted_zone_id: String, max_items: i64) -> Result<ListHostedZonesByNameOutput, Box<dyn std::error::Error>>;
    async fn list_hosted_zones_by_vpc(&self, max_items: i64, next_token: String, vpc_id: String, vpc_region: String) -> Result<ListHostedZonesByVpcOutput, Box<dyn std::error::Error>>;
    async fn update_hosted_zone_comment(&self, comment: String, id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_hosted_zone_count(&self) -> Result<i64, Box<dyn std::error::Error>>;
    async fn change_resource_record_sets(&self, change_batch: String, hosted_zone_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_resource_record_sets(&self, hosted_zone_id: String, max_items: i64, start_record_identifier: String, start_record_name: String, start_record_type: String) -> Result<ListResourceRecordSetsOutput, Box<dyn std::error::Error>>;
    async fn get_change(&self, id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_health_check(&self, caller_reference: String, health_check_config: String) -> Result<CreateHealthCheckOutput, Box<dyn std::error::Error>>;
    async fn delete_health_check(&self, health_check_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_health_check(&self, health_check_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_health_checks(&self, marker: String, max_items: i64) -> Result<ListHealthChecksOutput, Box<dyn std::error::Error>>;
    async fn update_health_check(&self, alarm_identifier: String, child_health_checks: Vec<String>, disabled: bool, enable_sni: bool, failure_threshold: i64, fully_qualified_domain_name: String, health_check_id: String, health_check_version: i64, health_threshold: i64, insufficient_data_health_status: String, inverted: bool, ip_address: String, port: i64, regions: Vec<String>, reset_elements: String, resource_path: String, search_string: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_health_check_status(&self, health_check_id: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn get_health_check_last_failure_reason(&self, health_check_id: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn get_health_check_count(&self) -> Result<i64, Box<dyn std::error::Error>>;
    async fn create_reusable_delegation_set(&self, caller_reference: String, hosted_zone_id: String) -> Result<CreateReusableDelegationSetOutput, Box<dyn std::error::Error>>;
    async fn delete_reusable_delegation_set(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_reusable_delegation_set(&self, id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_reusable_delegation_sets(&self, marker: String, max_items: i64) -> Result<ListReusableDelegationSetsOutput, Box<dyn std::error::Error>>;
    async fn create_traffic_policy(&self, comment: String, document: String, name: String) -> Result<CreateTrafficPolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_traffic_policy(&self, id: String, version: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_traffic_policy(&self, id: String, version: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_traffic_policies(&self, max_items: i64, traffic_policy_id_marker: String) -> Result<ListTrafficPoliciesOutput, Box<dyn std::error::Error>>;
    async fn create_traffic_policy_version(&self, comment: String, document: String, id: String) -> Result<CreateTrafficPolicyVersionOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_versions(&self, id: String, max_items: i64, traffic_policy_version_marker: String) -> Result<ListTrafficPolicyVersionsOutput, Box<dyn std::error::Error>>;
    async fn update_traffic_policy_comment(&self, comment: String, id: String, version: i64, traffic_policy: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_traffic_policy_instance(&self, hosted_zone_id: String, name: String, traffic_policy_id: String, traffic_policy_version: i64, ttl: i64) -> Result<CreateTrafficPolicyInstanceOutput, Box<dyn std::error::Error>>;
    async fn delete_traffic_policy_instance(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_traffic_policy_instance(&self, id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_instances(&self, hosted_zone_id_marker: String, max_items: i64, traffic_policy_instance_name_marker: String, traffic_policy_instance_type_marker: String) -> Result<ListTrafficPolicyInstancesOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_instances_by_hosted_zone(&self, hosted_zone_id: String, max_items: i64, traffic_policy_instance_name_marker: String, traffic_policy_instance_type_marker: String) -> Result<ListTrafficPolicyInstancesByHostedZoneOutput, Box<dyn std::error::Error>>;
    async fn list_traffic_policy_instances_by_policy(&self, hosted_zone_id_marker: String, max_items: i64, traffic_policy_id: String, traffic_policy_instance_name_marker: String, traffic_policy_instance_type_marker: String, traffic_policy_version: i64) -> Result<ListTrafficPolicyInstancesByPolicyOutput, Box<dyn std::error::Error>>;
    async fn update_traffic_policy_instance(&self, id: String, traffic_policy_id: String, traffic_policy_version: i64, ttl: i64, traffic_policy_instance: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_traffic_policy_instance_count(&self) -> Result<i64, Box<dyn std::error::Error>>;
    async fn create_query_logging_config(&self, cloud_watch_logs_log_group_arn: String, hosted_zone_id: String) -> Result<CreateQueryLoggingConfigOutput, Box<dyn std::error::Error>>;
    async fn delete_query_logging_config(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_query_logging_config(&self, id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_query_logging_configs(&self, hosted_zone_id: String, max_results: i64, next_token: String) -> Result<ListQueryLoggingConfigsOutput, Box<dyn std::error::Error>>;
    async fn associate_vpc_with_hosted_zone(&self, comment: String, hosted_zone_id: String, vpc: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn disassociate_vpc_from_hosted_zone(&self, comment: String, hosted_zone_id: String, vpc: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_vpc_association_authorization(&self, hosted_zone_id: String, vpc: String) -> Result<CreateVpcAssociationAuthorizationOutput, Box<dyn std::error::Error>>;
    async fn delete_vpc_association_authorization(&self, hosted_zone_id: String, vpc: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_vpc_association_authorizations(&self, hosted_zone_id: String, max_results: i64, next_token: String) -> Result<ListVpcAssociationAuthorizationsOutput, Box<dyn std::error::Error>>;
    async fn enable_hosted_zone_dnssec(&self, hosted_zone_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn disable_hosted_zone_dnssec(&self, hosted_zone_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_dnssec(&self, hosted_zone_id: String) -> Result<GetDnssecOutput, Box<dyn std::error::Error>>;
    async fn create_key_signing_key(&self, caller_reference: String, hosted_zone_id: String, key_management_service_arn: String, name: String, status: String) -> Result<CreateKeySigningKeyOutput, Box<dyn std::error::Error>>;
    async fn delete_key_signing_key(&self, hosted_zone_id: String, name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn activate_key_signing_key(&self, hosted_zone_id: String, name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn deactivate_key_signing_key(&self, hosted_zone_id: String, name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn test_dns_answer(&self, edns0_client_subnet_ip: String, edns0_client_subnet_mask: String, hosted_zone_id: String, record_name: String, record_type: String, resolver_ip: String) -> Result<TestDnsAnswerOutput, Box<dyn std::error::Error>>;
    async fn get_geo_location(&self, continent_code: String, country_code: String, subdivision_code: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_geo_locations(&self, max_items: i64, start_continent_code: String, start_country_code: String, start_subdivision_code: String) -> Result<ListGeoLocationsOutput, Box<dyn std::error::Error>>;
    async fn change_tags_for_resource(&self, add_tags: Vec<String>, remove_tag_keys: Vec<String>, resource_id: String, resource_type: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_tags_for_resource(&self, resource_id: String, resource_type: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_tags_for_resources(&self, resource_ids: Vec<String>, resource_type: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn get_account_limit(&self, type: String) -> Result<GetAccountLimitOutput, Box<dyn std::error::Error>>;
    async fn get_hosted_zone_limit(&self, hosted_zone_id: String, type: String) -> Result<GetHostedZoneLimitOutput, Box<dyn std::error::Error>>;
    async fn get_reusable_delegation_set_limit(&self, delegation_set_id: String, type: String) -> Result<GetReusableDelegationSetLimitOutput, Box<dyn std::error::Error>>;
    async fn get_checker_ip_ranges(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

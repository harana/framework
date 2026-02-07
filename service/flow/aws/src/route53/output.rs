//! Output types for AWS Route53 actions
//!
//! This module contains all the output structs and helper types used by the AWS Route53 actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ==================== Hosted Zone Output Types ====================

/// Output for create_hosted_zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHostedZoneOutput {
        pub change_info: ChangeInfo,
        pub delegation_set: Option<DelegationSet>,
        pub hosted_zone: HostedZone,
        pub location: String,
        pub success: bool,
        pub vpc: Option<VPC>,
}

/// Output for delete_hosted_zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteHostedZoneOutput {
        pub change_info: ChangeInfo,
        pub success: bool,
}

/// Output for get_hosted_zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHostedZoneOutput {
        pub delegation_set: Option<DelegationSet>,
        pub hosted_zone: HostedZone,
        pub vpcs: Option<Vec<VPC>>,
}

/// Output for list_hosted_zones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHostedZonesOutput {
        pub hosted_zones: Vec<HostedZone>,
        pub is_truncated: bool,
        pub marker: Option<String>,
        pub max_items: i32,
        pub next_marker: Option<String>,
}

/// Output for list_hosted_zones_by_name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHostedZonesByNameOutput {
        pub dns_name: Option<String>,
        pub hosted_zone_id: Option<String>,
        pub hosted_zones: Vec<HostedZone>,
        pub is_truncated: bool,
        pub max_items: i32,
        pub next_dns_name: Option<String>,
        pub next_hosted_zone_id: Option<String>,
}

/// Output for list_hosted_zones_by_vpc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHostedZonesByVpcOutput {
        pub hosted_zone_summaries: Vec<HostedZoneSummary>,
        pub max_items: i32,
        pub next_token: Option<String>,
}

/// Output for update_hosted_zone_comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHostedZoneCommentOutput {
        pub hosted_zone: HostedZone,
        pub success: bool,
}

/// Output for get_hosted_zone_count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHostedZoneCountOutput {
        pub hosted_zone_count: i64,
}

// ==================== Resource Record Set Output Types ====================

/// Output for change_resource_record_sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeResourceRecordSetsOutput {
        pub change_info: ChangeInfo,
        pub success: bool,
}

/// Output for list_resource_record_sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResourceRecordSetsOutput {
        pub is_truncated: bool,
        pub max_items: i32,
        pub next_record_identifier: Option<String>,
        pub next_record_name: Option<String>,
        pub next_record_type: Option<String>,
        pub resource_record_sets: Vec<ResourceRecordSet>,
}

/// Output for get_change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChangeOutput {
        pub change_info: ChangeInfo,
}

// ==================== Health Check Output Types ====================

/// Output for create_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHealthCheckOutput {
        pub health_check: HealthCheck,
        pub location: String,
        pub success: bool,
}

/// Output for delete_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteHealthCheckOutput {
        pub success: bool,
}

/// Output for get_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckOutput {
        pub health_check: HealthCheck,
}

/// Output for list_health_checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHealthChecksOutput {
        pub health_checks: Vec<HealthCheck>,
        pub is_truncated: bool,
        pub marker: Option<String>,
        pub max_items: i32,
        pub next_marker: Option<String>,
}

/// Output for update_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHealthCheckOutput {
        pub health_check: HealthCheck,
        pub success: bool,
}

/// Output for get_health_check_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckStatusOutput {
        pub health_check_observations: Vec<HealthCheckObservation>,
}

/// Output for get_health_check_last_failure_reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckLastFailureReasonOutput {
        pub health_check_observations: Vec<HealthCheckObservation>,
}

/// Output for get_health_check_count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckCountOutput {
        pub health_check_count: i64,
}

// ==================== Helper Structs ====================

/// Represents a Route53 hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZone {
        pub id: String,
        pub name: String,
        pub caller_reference: String,
        pub config: Option<HostedZoneConfig>,
        pub resource_record_set_count: Option<i64>,
        pub linked_service: Option<LinkedService>,
}

/// Configuration for a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZoneConfig {
        pub comment: Option<String>,
        pub private_zone: bool,
}

/// Summary of a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZoneSummary {
        pub hosted_zone_id: String,
        pub name: String,
        pub owner: Option<HostedZoneOwner>,
}

/// Owner information for a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZoneOwner {
        pub owning_account: Option<String>,
        pub owning_service: Option<String>,
}

/// Information about a linked service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedService {
        pub service_principal: Option<String>,
        pub description: Option<String>,
}

/// Delegation set for a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationSet {
        pub id: Option<String>,
        pub caller_reference: Option<String>,
        pub name_servers: Vec<String>,
}

/// VPC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPC {
        pub vpc_region: Option<String>,
        pub vpc_id: Option<String>,
}

/// Information about a change request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeInfo {
        pub id: String,
        pub status: String,
        pub submitted_at: DateTime<Utc>,
        pub comment: Option<String>,
}

/// Batch of changes to apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeBatch {
        pub comment: Option<String>,
        pub changes: Vec<Change>,
}

/// A single change to apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
        pub action: String,
        pub resource_record_set: ResourceRecordSet,
}

/// A resource record set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecordSet {
        pub name: String,
        pub record_type: String,
        pub set_identifier: Option<String>,
        pub weight: Option<i64>,
        pub region: Option<String>,
        pub geo_location: Option<GeoLocation>,
        pub failover: Option<String>,
        pub multi_value_answer: Option<bool>,
        pub ttl: Option<i64>,
        pub resource_records: Option<Vec<ResourceRecord>>,
        pub alias_target: Option<AliasTarget>,
        pub health_check_id: Option<String>,
        pub traffic_policy_instance_id: Option<String>,
        pub cidr_routing_config: Option<CidrRoutingConfig>,
}

/// A single resource record value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecord {
        pub value: String,
}

/// Alias target for alias record sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasTarget {
        pub hosted_zone_id: String,
        pub dns_name: String,
        pub evaluate_target_health: bool,
}

/// Geolocation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
        pub continent_code: Option<String>,
        pub country_code: Option<String>,
        pub subdivision_code: Option<String>,
}

/// CIDR routing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CidrRoutingConfig {
        pub collection_id: String,
        pub location_name: String,
}

/// A health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
        pub id: String,
        pub caller_reference: String,
        pub linked_service: Option<LinkedService>,
        pub health_check_config: HealthCheckConfig,
        pub health_check_version: i64,
        pub cloud_watch_alarm_configuration: Option<CloudWatchAlarmConfiguration>,
}

/// Configuration for a health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
        pub ip_address: Option<String>,
        pub port: Option<i32>,
        pub health_check_type: String,
        pub resource_path: Option<String>,
        pub fully_qualified_domain_name: Option<String>,
        pub search_string: Option<String>,
        pub request_interval: Option<i32>,
        pub failure_threshold: Option<i32>,
        pub measure_latency: Option<bool>,
        pub inverted: Option<bool>,
        pub disabled: Option<bool>,
        pub health_threshold: Option<i32>,
        pub child_health_checks: Option<Vec<String>>,
        pub enable_sni: Option<bool>,
        pub regions: Option<Vec<String>>,
        pub alarm_identifier: Option<AlarmIdentifier>,
        pub insufficient_data_health_status: Option<String>,
        pub routing_control_arn: Option<String>,
}

/// CloudWatch alarm identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmIdentifier {
        pub region: String,
        pub name: String,
}

/// CloudWatch alarm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudWatchAlarmConfiguration {
        pub evaluation_periods: i32,
        pub threshold: f64,
        pub comparison_operator: String,
        pub period: i32,
        pub metric_name: String,
        pub namespace: String,
        pub statistic: String,
        pub dimensions: Option<Vec<Dimension>>,
}

/// CloudWatch dimension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
        pub name: String,
        pub value: String,
}

/// Health check observation from a specific region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckObservation {
        pub region: Option<String>,
        pub ip_address: Option<String>,
        pub status_report: Option<StatusReport>,
}

/// Status report for a health check observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusReport {
        pub status: Option<String>,
        pub checked_time: Option<DateTime<Utc>>,
}

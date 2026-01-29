//! Output types for AWS Route53 actions
//!
//! This module contains all the output structs and helper types used by the AWS Route53 actions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ==================== Hosted Zone Output Types ====================

/// Output for create_hosted_zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHostedZoneOutput {
    /// Information about the change request
    pub change_info: ChangeInfo,
    /// Delegation set for the hosted zone
    pub delegation_set: Option<DelegationSet>,
    /// The hosted zone that was created
    pub hosted_zone: HostedZone,
    /// The unique URL representing the new hosted zone
    pub location: String,
    /// Whether the operation was successful
    pub success: bool,
    /// VPC associated with a private hosted zone
    pub vpc: Option<VPC>,
}

/// Output for delete_hosted_zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteHostedZoneOutput {
    /// Information about the change request
    pub change_info: ChangeInfo,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_hosted_zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHostedZoneOutput {
    /// Delegation set for the hosted zone
    pub delegation_set: Option<DelegationSet>,
    /// The hosted zone
    pub hosted_zone: HostedZone,
    /// VPCs associated with the private hosted zone
    pub vpcs: Option<Vec<VPC>>,
}

/// Output for list_hosted_zones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHostedZonesOutput {
    /// List of hosted zones
    pub hosted_zones: Vec<HostedZone>,
    /// Whether there are more hosted zones to list
    pub is_truncated: bool,
    /// The marker for the current page of results
    pub marker: Option<String>,
    /// Maximum number of items returned
    pub max_items: i32,
    /// Marker for the next page of results
    pub next_marker: Option<String>,
}

/// Output for list_hosted_zones_by_name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHostedZonesByNameOutput {
    /// DNS name for the first hosted zone
    pub dns_name: Option<String>,
    /// Hosted zone ID for the first hosted zone
    pub hosted_zone_id: Option<String>,
    /// List of hosted zones
    pub hosted_zones: Vec<HostedZone>,
    /// Whether there are more hosted zones to list
    pub is_truncated: bool,
    /// Maximum number of items returned
    pub max_items: i32,
    /// DNS name for next page
    pub next_dns_name: Option<String>,
    /// Hosted zone ID for next page
    pub next_hosted_zone_id: Option<String>,
}

/// Output for list_hosted_zones_by_vpc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHostedZonesByVpcOutput {
    /// List of hosted zone summaries
    pub hosted_zone_summaries: Vec<HostedZoneSummary>,
    /// Maximum number of items returned
    pub max_items: i32,
    /// Pagination token
    pub next_token: Option<String>,
}

/// Output for update_hosted_zone_comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHostedZoneCommentOutput {
    /// The updated hosted zone
    pub hosted_zone: HostedZone,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_hosted_zone_count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHostedZoneCountOutput {
    /// The total number of hosted zones
    pub hosted_zone_count: i64,
}

// ==================== Resource Record Set Output Types ====================

/// Output for change_resource_record_sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeResourceRecordSetsOutput {
    /// Information about the change request
    pub change_info: ChangeInfo,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for list_resource_record_sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResourceRecordSetsOutput {
    /// Whether there are more record sets to list
    pub is_truncated: bool,
    /// Maximum number of items returned
    pub max_items: i32,
    /// Next record identifier for pagination
    pub next_record_identifier: Option<String>,
    /// Next record name for pagination
    pub next_record_name: Option<String>,
    /// Next record type for pagination
    pub next_record_type: Option<String>,
    /// List of resource record sets
    pub resource_record_sets: Vec<ResourceRecordSet>,
}

/// Output for get_change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChangeOutput {
    /// Information about the change request
    pub change_info: ChangeInfo,
}

// ==================== Health Check Output Types ====================

/// Output for create_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHealthCheckOutput {
    /// The created health check
    pub health_check: HealthCheck,
    /// The unique URL representing the new health check
    pub location: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteHealthCheckOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckOutput {
    /// The health check
    pub health_check: HealthCheck,
}

/// Output for list_health_checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHealthChecksOutput {
    /// List of health checks
    pub health_checks: Vec<HealthCheck>,
    /// Whether there are more health checks to list
    pub is_truncated: bool,
    /// The marker for the current page of results
    pub marker: Option<String>,
    /// Maximum number of items returned
    pub max_items: i32,
    /// Marker for the next page of results
    pub next_marker: Option<String>,
}

/// Output for update_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHealthCheckOutput {
    /// The updated health check
    pub health_check: HealthCheck,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for get_health_check_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckStatusOutput {
    /// List of health check observations
    pub health_check_observations: Vec<HealthCheckObservation>,
}

/// Output for get_health_check_last_failure_reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckLastFailureReasonOutput {
    /// List of health check observations for the failure
    pub health_check_observations: Vec<HealthCheckObservation>,
}

/// Output for get_health_check_count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthCheckCountOutput {
    /// The total number of health checks
    pub health_check_count: i64,
}

// ==================== Helper Structs ====================

/// Represents a Route53 hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZone {
    /// The ID of the hosted zone
    pub id: String,
    /// The name of the domain
    pub name: String,
    /// The unique string that identifies the request to create the hosted zone
    pub caller_reference: String,
    /// Configuration of the hosted zone
    pub config: Option<HostedZoneConfig>,
    /// The number of resource record sets in the hosted zone
    pub resource_record_set_count: Option<i64>,
    /// If the hosted zone was created by another AWS service
    pub linked_service: Option<LinkedService>,
}

/// Configuration for a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZoneConfig {
    /// A comment for the hosted zone
    pub comment: Option<String>,
    /// Whether this is a private hosted zone
    pub private_zone: bool,
}

/// Summary of a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZoneSummary {
    /// The ID of the hosted zone
    pub hosted_zone_id: String,
    /// The name of the hosted zone
    pub name: String,
    /// The owner of the hosted zone
    pub owner: Option<HostedZoneOwner>,
}

/// Owner information for a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZoneOwner {
    /// The owning AWS account
    pub owning_account: Option<String>,
    /// The owning AWS service
    pub owning_service: Option<String>,
}

/// Information about a linked service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedService {
    /// The service principal
    pub service_principal: Option<String>,
    /// Description of the service
    pub description: Option<String>,
}

/// Delegation set for a hosted zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationSet {
    /// The ID of the delegation set
    pub id: Option<String>,
    /// The unique string that identifies the request
    pub caller_reference: Option<String>,
    /// The name servers for the delegation set
    pub name_servers: Vec<String>,
}

/// VPC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPC {
    /// The region of the VPC
    pub vpc_region: Option<String>,
    /// The ID of the VPC
    pub vpc_id: Option<String>,
}

/// Information about a change request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeInfo {
    /// The ID of the change request
    pub id: String,
    /// The status of the change (PENDING or INSYNC)
    pub status: String,
    /// The date and time the change was submitted
    pub submitted_at: DateTime<Utc>,
    /// A comment about the change
    pub comment: Option<String>,
}

/// Batch of changes to apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeBatch {
    /// A comment about the changes
    pub comment: Option<String>,
    /// List of changes to apply
    pub changes: Vec<Change>,
}

/// A single change to apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// The action to perform (CREATE, DELETE, or UPSERT)
    pub action: String,
    /// The resource record set to change
    pub resource_record_set: ResourceRecordSet,
}

/// A resource record set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecordSet {
    /// The name of the domain
    pub name: String,
    /// The DNS record type (A, AAAA, CNAME, MX, etc.)
    pub record_type: String,
    /// Identifier for weighted, latency, geolocation, or failover record sets
    pub set_identifier: Option<String>,
    /// Weight for weighted record sets
    pub weight: Option<i64>,
    /// Region for latency-based routing
    pub region: Option<String>,
    /// Geolocation configuration
    pub geo_location: Option<GeoLocation>,
    /// Failover configuration (PRIMARY or SECONDARY)
    pub failover: Option<String>,
    /// Whether this is a multivalue answer record set
    pub multi_value_answer: Option<bool>,
    /// Time to live in seconds
    pub ttl: Option<i64>,
    /// List of resource records
    pub resource_records: Option<Vec<ResourceRecord>>,
    /// Alias target for alias record sets
    pub alias_target: Option<AliasTarget>,
    /// Health check ID to associate
    pub health_check_id: Option<String>,
    /// Traffic policy instance ID
    pub traffic_policy_instance_id: Option<String>,
    /// CIDR routing configuration
    pub cidr_routing_config: Option<CidrRoutingConfig>,
}

/// A single resource record value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecord {
    /// The value of the resource record
    pub value: String,
}

/// Alias target for alias record sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasTarget {
    /// The hosted zone ID of the alias target
    pub hosted_zone_id: String,
    /// The DNS name of the alias target
    pub dns_name: String,
    /// Whether to evaluate target health
    pub evaluate_target_health: bool,
}

/// Geolocation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    /// The continent code
    pub continent_code: Option<String>,
    /// The country code
    pub country_code: Option<String>,
    /// The subdivision code (state/province)
    pub subdivision_code: Option<String>,
}

/// CIDR routing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CidrRoutingConfig {
    /// The CIDR collection ID
    pub collection_id: String,
    /// The CIDR location name
    pub location_name: String,
}

/// A health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// The ID of the health check
    pub id: String,
    /// The unique string that identifies the request
    pub caller_reference: String,
    /// If the health check was created by another AWS service
    pub linked_service: Option<LinkedService>,
    /// The health check configuration
    pub health_check_config: HealthCheckConfig,
    /// The version of the health check
    pub health_check_version: i64,
    /// CloudWatch alarm configuration if associated
    pub cloud_watch_alarm_configuration: Option<CloudWatchAlarmConfiguration>,
}

/// Configuration for a health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// The IP address of the endpoint to check
    pub ip_address: Option<String>,
    /// The port to use for the health check
    pub port: Option<i32>,
    /// The type of health check (HTTP, HTTPS, HTTP_STR_MATCH, etc.)
    pub health_check_type: String,
    /// The path for HTTP/HTTPS health checks
    pub resource_path: Option<String>,
    /// The fully qualified domain name
    pub fully_qualified_domain_name: Option<String>,
    /// The string to search for in the response body
    pub search_string: Option<String>,
    /// The request interval in seconds (10 or 30)
    pub request_interval: Option<i32>,
    /// The number of consecutive failures before marking unhealthy
    pub failure_threshold: Option<i32>,
    /// Whether to measure latency
    pub measure_latency: Option<bool>,
    /// Whether to invert the health check status
    pub inverted: Option<bool>,
    /// Whether the health check is disabled
    pub disabled: Option<bool>,
    /// For calculated health checks, the minimum number of healthy children
    pub health_threshold: Option<i32>,
    /// IDs of child health checks for calculated health checks
    pub child_health_checks: Option<Vec<String>>,
    /// Whether to send SNI to the endpoint
    pub enable_sni: Option<bool>,
    /// Regions from which to perform health checks
    pub regions: Option<Vec<String>>,
    /// CloudWatch alarm identifier
    pub alarm_identifier: Option<AlarmIdentifier>,
    /// What to report when CloudWatch has insufficient data
    pub insufficient_data_health_status: Option<String>,
    /// ARN of the routing control
    pub routing_control_arn: Option<String>,
}

/// CloudWatch alarm identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmIdentifier {
    /// The region of the CloudWatch alarm
    pub region: String,
    /// The name of the CloudWatch alarm
    pub name: String,
}

/// CloudWatch alarm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudWatchAlarmConfiguration {
    /// The number of evaluation periods
    pub evaluation_periods: i32,
    /// The threshold value
    pub threshold: f64,
    /// The comparison operator
    pub comparison_operator: String,
    /// The period in seconds
    pub period: i32,
    /// The metric name
    pub metric_name: String,
    /// The CloudWatch namespace
    pub namespace: String,
    /// The statistic
    pub statistic: String,
    /// The dimensions
    pub dimensions: Option<Vec<Dimension>>,
}

/// CloudWatch dimension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    /// The dimension name
    pub name: String,
    /// The dimension value
    pub value: String,
}

/// Health check observation from a specific region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckObservation {
    /// The region from which the observation was made
    pub region: Option<String>,
    /// The IP address of the health checker
    pub ip_address: Option<String>,
    /// The status report
    pub status_report: Option<StatusReport>,
}

/// Status report for a health check observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusReport {
    /// The status message
    pub status: Option<String>,
    /// The time the health check was performed
    pub checked_time: Option<DateTime<Utc>>,
}

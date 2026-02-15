// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_route53_hosted_zone
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRoute53HostedZone {
    pub account_id: String,
    pub caller_reference: Option<String>,
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub hosted_zone_id: String,
    #[serde(default)]
    pub is_private: bool,
    pub name: String,
    pub name_servers: Option<String>,
    pub record_set_count: i64,
    pub region: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsRoute53HostedZone {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_route53_record_set
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRoute53RecordSet {
    pub alias_dns_name: Option<String>,
    #[serde(default)]
    pub alias_evaluate_target_health: bool,
    pub alias_hosted_zone_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub failover: String,
    pub health_check_id: Option<String>,
    /// Reference: aws_route53_hosted_zone.id
    pub hosted_zone_id: String,
    #[serde(default)]
    pub multi_value_answer: bool,
    pub name: String,
    pub region: Option<String>,
    pub resource_records: Option<String>,
    pub set_identifier: Option<String>,
    pub ttl: Option<i64>,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub weight: Option<i64>,
}

impl AwsRoute53RecordSet {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_route53_health_check
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
    pub fqdn: Option<String>,
    pub health_check_id: String,
    pub health_threshold: Option<i64>,
    pub insufficient_data_health_status: String,
    #[serde(default)]
    pub inverted: bool,
    pub ip_address: Option<String>,
    pub port: Option<i64>,
    pub region: Option<String>,
    pub request_interval: i64,
    pub resource_path: Option<String>,
    pub search_string: Option<String>,
    pub status: String,
    pub tags: Option<String>,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsRoute53HealthCheck {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_route53_traffic_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRoute53TrafficPolicy {
    pub account_id: String,
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub document: String,
    pub name: String,
    pub region: Option<String>,
    pub traffic_policy_id: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

impl AwsRoute53TrafficPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


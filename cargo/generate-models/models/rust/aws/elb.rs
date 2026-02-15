// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_elb_load_balancer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbLoadBalancer {
    pub account_id: String,
    pub availability_zones: Option<String>,
    pub canonical_hosted_zone_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dns_name: Option<String>,
    pub ip_address_type: String,
    pub load_balancer_arn: String,
    pub name: String,
    pub region: Option<String>,
    pub scheme: String,
    pub security_groups: Option<String>,
    pub state: String,
    pub subnets: Option<String>,
    pub tags: Option<String>,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: Option<String>,
}

impl AwsElbLoadBalancer {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_elb_target_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbTargetGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub health_check_enabled: bool,
    pub health_check_interval_seconds: i64,
    pub health_check_path: Option<String>,
    pub health_check_port: Option<String>,
    pub health_check_protocol: String,
    pub health_check_timeout_seconds: Option<i64>,
    pub healthy_threshold_count: i64,
    pub ip_address_type: String,
    pub load_balancer_arns: Option<String>,
    pub name: String,
    pub port: Option<i64>,
    pub protocol: String,
    pub protocol_version: String,
    pub region: Option<String>,
    pub tags: Option<String>,
    pub target_group_arn: String,
    pub target_type: String,
    pub unhealthy_threshold_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: Option<String>,
}

impl AwsElbTargetGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_elb_listener
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbListener {
    pub certificates: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_actions: Option<String>,
    pub listener_arn: String,
    /// Reference: aws_elb_load_balancer.id
    pub load_balancer_id: String,
    pub port: i64,
    pub protocol: String,
    pub ssl_policy: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsElbListener {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_elb_listener_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbListenerRule {
    pub actions: Option<String>,
    pub conditions: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_default: bool,
    /// Reference: aws_elb_listener.id
    pub listener_id: String,
    pub priority: Option<i64>,
    pub rule_arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsElbListenerRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


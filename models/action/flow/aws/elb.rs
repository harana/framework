// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancersOutput {
    pub load_balancers: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSubnetsOutput {
    pub availability_zones: Vec<String>,
    pub ip_address_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTargetGroupsOutput {
    pub next_marker: String,
    pub target_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeListenersOutput {
    pub listeners: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRulesOutput {
    pub next_marker: String,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeListenerCertificatesOutput {
    pub certificates: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSslPoliciesOutput {
    pub next_marker: String,
    pub ssl_policies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAccountLimitsOutput {
    pub limits: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbLoadBalancer {
    pub account_id: String,
    pub availability_zones: String,
    pub canonical_hosted_zone_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dns_name: String,
    pub ip_address_type: String,
    pub load_balancer_arn: String,
    pub name: String,
    pub region: String,
    pub scheme: String,
    pub security_groups: String,
    pub state: String,
    pub subnets: String,
    pub tags: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbTargetGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub health_check_enabled: bool,
    pub health_check_interval_seconds: i64,
    pub health_check_path: String,
    pub health_check_port: String,
    pub health_check_protocol: String,
    pub health_check_timeout_seconds: i64,
    pub healthy_threshold_count: i64,
    pub ip_address_type: String,
    pub load_balancer_arns: String,
    pub name: String,
    pub port: i64,
    pub protocol: String,
    pub protocol_version: String,
    pub region: String,
    pub tags: String,
    pub target_group_arn: String,
    pub target_type: String,
    pub unhealthy_threshold_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbListener {
    pub certificates: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_actions: String,
    pub listener_arn: String,
    pub load_balancer_id: String,
    pub port: i64,
    pub protocol: String,
    pub ssl_policy: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsElbListenerRule {
    pub actions: String,
    pub conditions: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_default: bool,
    pub listener_id: String,
    pub priority: i64,
    pub rule_arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait ElbAction {
    async fn create_load_balancer(&self, customer_owned_ipv4_pool: String, ip_address_type: String, name: String, region: String, scheme: String, security_groups: Vec<String>, subnet_mappings: Vec<String>, subnets: Vec<String>, tags: String, type: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn delete_load_balancer(&self, load_balancer_arn: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_load_balancers(&self, load_balancer_arns: Vec<String>, marker: String, names: Vec<String>, page_size: i64, region: String) -> Result<DescribeLoadBalancersOutput, Box<dyn std::error::Error>>;
    async fn modify_load_balancer_attributes(&self, attributes: Vec<String>, load_balancer_arn: String, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_load_balancer_attributes(&self, load_balancer_arn: String, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn set_security_groups(&self, load_balancer_arn: String, region: String, security_groups: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn set_subnets(&self, ip_address_type: String, load_balancer_arn: String, region: String, subnet_mappings: Vec<String>, subnets: Vec<String>) -> Result<SetSubnetsOutput, Box<dyn std::error::Error>>;
    async fn set_ip_address_type(&self, ip_address_type: String, load_balancer_arn: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_target_group(&self, health_check_enabled: bool, health_check_interval_seconds: i64, health_check_path: String, health_check_port: String, health_check_protocol: String, health_check_timeout_seconds: i64, healthy_threshold_count: i64, ip_address_type: String, matcher: String, name: String, port: i64, protocol: String, protocol_version: String, region: String, tags: String, target_type: String, unhealthy_threshold_count: i64, vpc_id: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn delete_target_group(&self, region: String, target_group_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_target_groups(&self, load_balancer_arn: String, marker: String, names: Vec<String>, page_size: i64, region: String, target_group_arns: Vec<String>) -> Result<DescribeTargetGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_target_group(&self, health_check_enabled: bool, health_check_interval_seconds: i64, health_check_path: String, health_check_port: String, health_check_protocol: String, health_check_timeout_seconds: i64, healthy_threshold_count: i64, matcher: String, region: String, target_group_arn: String, unhealthy_threshold_count: i64) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn modify_target_group_attributes(&self, attributes: Vec<String>, region: String, target_group_arn: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_target_group_attributes(&self, region: String, target_group_arn: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn register_targets(&self, region: String, target_group_arn: String, targets: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn deregister_targets(&self, region: String, target_group_arn: String, targets: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_target_health(&self, region: String, target_group_arn: String, targets: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_listener(&self, alpn_policy: Vec<String>, certificates: Vec<String>, default_actions: Vec<String>, load_balancer_arn: String, port: i64, protocol: String, region: String, ssl_policy: String, tags: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn delete_listener(&self, listener_arn: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_listeners(&self, listener_arns: Vec<String>, load_balancer_arn: String, marker: String, page_size: i64, region: String) -> Result<DescribeListenersOutput, Box<dyn std::error::Error>>;
    async fn modify_listener(&self, alpn_policy: Vec<String>, certificates: Vec<String>, default_actions: Vec<String>, listener_arn: String, port: i64, protocol: String, region: String, ssl_policy: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_rule(&self, actions: Vec<String>, conditions: Vec<String>, listener_arn: String, priority: i64, region: String, tags: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn delete_rule(&self, region: String, rule_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_rules(&self, listener_arn: String, marker: String, page_size: i64, region: String, rule_arns: Vec<String>) -> Result<DescribeRulesOutput, Box<dyn std::error::Error>>;
    async fn modify_rule(&self, actions: Vec<String>, conditions: Vec<String>, region: String, rule_arn: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn set_rule_priorities(&self, region: String, rule_priorities: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn add_listener_certificates(&self, certificates: Vec<String>, listener_arn: String, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn remove_listener_certificates(&self, certificates: Vec<String>, listener_arn: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_listener_certificates(&self, listener_arn: String, marker: String, page_size: i64, region: String) -> Result<DescribeListenerCertificatesOutput, Box<dyn std::error::Error>>;
    async fn describe_ssl_policies(&self, load_balancer_type: String, marker: String, names: Vec<String>, page_size: i64, region: String) -> Result<DescribeSslPoliciesOutput, Box<dyn std::error::Error>>;
    async fn describe_account_limits(&self, marker: String, page_size: i64, region: String) -> Result<DescribeAccountLimitsOutput, Box<dyn std::error::Error>>;
    async fn add_tags(&self, region: String, resource_arns: Vec<String>, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_tags(&self, region: String, resource_arns: Vec<String>, tag_keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_tags(&self, region: String, resource_arns: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

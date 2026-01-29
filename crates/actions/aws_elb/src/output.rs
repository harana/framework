//! Output types for AWS ELB actions
//!
//! This module contains all the output structs and helper types used by the AWS ELB actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Output for create_load_balancer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoadBalancerOutput {
    pub load_balancers: Vec<LoadBalancer>,
    pub success: bool,
}

// delete_load_balancer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteLoadBalancerOutput {
    pub success: bool,
}

// describe_load_balancers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeLoadBalancersOutput {
    pub load_balancers: Vec<LoadBalancer>,
    pub next_marker: Option<String>,
}

// modify_load_balancer_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyLoadBalancerAttributesOutput {
    pub attributes: Vec<LoadBalancerAttribute>,
    pub success: bool,
}

// describe_load_balancer_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeLoadBalancerAttributesOutput {
    pub attributes: Vec<LoadBalancerAttribute>,
}

// set_security_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSecurityGroupsOutput {
    pub security_group_ids: Vec<String>,
    pub success: bool,
}

// set_subnets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSubnetsOutput {
    pub availability_zones: Vec<AvailabilityZone>,
    pub ip_address_type: String,
    pub success: bool,
}

// set_ip_address_type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIpAddressTypeOutput {
    pub ip_address_type: String,
    pub success: bool,
}

// create_target_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTargetGroupOutput {
    pub target_groups: Vec<TargetGroup>,
    pub success: bool,
}

// delete_target_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTargetGroupOutput {
    pub success: bool,
}

// describe_target_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeTargetGroupsOutput {
    pub next_marker: Option<String>,
    pub target_groups: Vec<TargetGroup>,
}

// modify_target_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyTargetGroupOutput {
    pub target_groups: Vec<TargetGroup>,
    pub success: bool,
}

// modify_target_group_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyTargetGroupAttributesOutput {
    pub attributes: Vec<TargetGroupAttribute>,
    pub success: bool,
}

// describe_target_group_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeTargetGroupAttributesOutput {
    pub attributes: Vec<TargetGroupAttribute>,
}

// register_targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterTargetsOutput {
    pub success: bool,
}

// deregister_targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeregisterTargetsOutput {
    pub success: bool,
}

// describe_target_health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeTargetHealthOutput {
    pub target_health_descriptions: Vec<TargetHealthDescription>,
}

// create_listener
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateListenerOutput {
    pub listeners: Vec<Listener>,
    pub success: bool,
}

// delete_listener
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteListenerOutput {
    pub success: bool,
}

// describe_listeners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeListenersOutput {
    pub listeners: Vec<Listener>,
    pub next_marker: Option<String>,
}

// modify_listener
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyListenerOutput {
    pub listeners: Vec<Listener>,
    pub success: bool,
}

// create_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRuleOutput {
    pub rules: Vec<Rule>,
    pub success: bool,
}

// delete_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRuleOutput {
    pub success: bool,
}

// describe_rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeRulesOutput {
    pub next_marker: Option<String>,
    pub rules: Vec<Rule>,
}

// modify_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRuleOutput {
    pub rules: Vec<Rule>,
    pub success: bool,
}

// set_rule_priorities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRulePrioritiesOutput {
    pub rules: Vec<Rule>,
    pub success: bool,
}

// add_listener_certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddListenerCertificatesOutput {
    pub certificates: Vec<Certificate>,
    pub success: bool,
}

// remove_listener_certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveListenerCertificatesOutput {
    pub success: bool,
}

// describe_listener_certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeListenerCertificatesOutput {
    pub certificates: Vec<Certificate>,
    pub next_marker: Option<String>,
}

// describe_ssl_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeSslPoliciesOutput {
    pub next_marker: Option<String>,
    pub ssl_policies: Vec<SslPolicy>,
}

// describe_account_limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeAccountLimitsOutput {
    pub limits: Vec<Limit>,
    pub next_marker: Option<String>,
}

// add_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTagsOutput {
    pub success: bool,
}

// remove_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTagsOutput {
    pub success: bool,
}

// describe_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeTagsOutput {
    pub tag_descriptions: Vec<TagDescription>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    pub load_balancer_arn: String,
    pub dns_name: String,
    pub canonical_hosted_zone_id: String,
    pub created_time: Option<DateTime<Utc>>,
    pub load_balancer_name: String,
    pub scheme: String,
    pub vpc_id: String,
    pub state: Option<LoadBalancerState>,
    pub load_balancer_type: String,
    pub availability_zones: Vec<AvailabilityZone>,
    pub security_groups: Option<Vec<String>>,
    pub ip_address_type: String,
    pub customer_owned_ipv4_pool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerState {
    pub code: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerAttribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityZone {
    pub zone_name: String,
    pub subnet_id: String,
    pub outpost_id: Option<String>,
    pub load_balancer_addresses: Option<Vec<LoadBalancerAddress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerAddress {
    pub ip_address: Option<String>,
    pub allocation_id: Option<String>,
    pub private_ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetMapping {
    pub subnet_id: String,
    pub allocation_id: Option<String>,
    pub private_ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetGroup {
    pub target_group_arn: String,
    pub target_group_name: String,
    pub protocol: Option<String>,
    pub port: Option<i32>,
    pub vpc_id: Option<String>,
    pub health_check_protocol: Option<String>,
    pub health_check_port: Option<String>,
    pub health_check_enabled: Option<bool>,
    pub health_check_interval_seconds: Option<i32>,
    pub health_check_timeout_seconds: Option<i32>,
    pub healthy_threshold_count: Option<i32>,
    pub unhealthy_threshold_count: Option<i32>,
    pub health_check_path: Option<String>,
    pub matcher: Option<Matcher>,
    pub load_balancer_arns: Option<Vec<String>>,
    pub target_type: String,
    pub protocol_version: Option<String>,
    pub ip_address_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetGroupAttribute {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matcher {
    pub http_code: Option<String>,
    pub grpc_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDescription {
    pub id: String,
    pub port: Option<i32>,
    pub availability_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetHealthDescription {
    pub target: TargetDescription,
    pub health_check_port: Option<String>,
    pub target_health: Option<TargetHealth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetHealth {
    pub state: String,
    pub reason: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    pub listener_arn: String,
    pub load_balancer_arn: String,
    pub port: Option<i32>,
    pub protocol: Option<String>,
    pub certificates: Option<Vec<Certificate>>,
    pub ssl_policy: Option<String>,
    pub default_actions: Vec<Action>,
    pub alpn_policy: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub certificate_arn: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: String,
    pub target_group_arn: Option<String>,
    pub authenticate_oidc_config: Option<Value>,
    pub authenticate_cognito_config: Option<Value>,
    pub order: Option<i32>,
    pub redirect_config: Option<Value>,
    pub fixed_response_config: Option<Value>,
    pub forward_config: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub rule_arn: String,
    pub priority: String,
    pub conditions: Vec<RuleCondition>,
    pub actions: Vec<Action>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: Option<String>,
    pub values: Option<Vec<String>>,
    pub host_header_config: Option<Value>,
    pub path_pattern_config: Option<Value>,
    pub http_header_config: Option<Value>,
    pub query_string_config: Option<Value>,
    pub http_request_method_config: Option<Value>,
    pub source_ip_config: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePriorityPair {
    pub rule_arn: String,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslPolicy {
    pub ssl_protocols: Vec<String>,
    pub ciphers: Vec<Cipher>,
    pub name: String,
    pub supported_load_balancer_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cipher {
    pub name: String,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub name: String,
    pub max: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDescription {
    pub resource_arn: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElbTags {
    pub tags: HashMap<String, String>,
}

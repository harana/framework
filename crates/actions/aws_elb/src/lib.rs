// Harana Actions - AWS ELB Module
// This module provides AWS Elastic Load Balancer actions.

#![warn(missing_docs)]

pub mod output;

use output::*;

/// Create Load Balancer
pub async fn create_load_balancer(
    name: &str,
    customer_owned_ipv4_pool: Option<&str>,
    ip_address_type: Option<&str>,
    region: Option<&str>,
    scheme: Option<&str>,
    security_groups: Option<Vec<String>>,
    subnet_mappings: Option<Vec<SubnetMapping>>,
    subnets: Option<Vec<String>>,
    tags: Option<ElbTags>,
    lb_type: Option<&str>,
) -> Result<CreateLoadBalancerOutput, String> {
    unimplemented!("create_load_balancer")
}

/// Delete Load Balancer
pub async fn delete_load_balancer(
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<DeleteLoadBalancerOutput, String> {
    unimplemented!("delete_load_balancer")
}

/// Describe Load Balancers
pub async fn describe_load_balancers(
    load_balancer_arns: Option<Vec<String>>,
    marker: Option<&str>,
    names: Option<Vec<String>>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeLoadBalancersOutput, String> {
    unimplemented!("describe_load_balancers")
}

/// Modify Load Balancer Attributes
pub async fn modify_load_balancer_attributes(
    attributes: Vec<LoadBalancerAttribute>,
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<ModifyLoadBalancerAttributesOutput, String> {
    unimplemented!("modify_load_balancer_attributes")
}

/// Describe Load Balancer Attributes
pub async fn describe_load_balancer_attributes(
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<DescribeLoadBalancerAttributesOutput, String> {
    unimplemented!("describe_load_balancer_attributes")
}

/// Set Security Groups
pub async fn set_security_groups(
    load_balancer_arn: &str,
    security_groups: Vec<String>,
    region: Option<&str>,
) -> Result<SetSecurityGroupsOutput, String> {
    unimplemented!("set_security_groups")
}

/// Set Subnets
pub async fn set_subnets(
    load_balancer_arn: &str,
    ip_address_type: Option<&str>,
    region: Option<&str>,
    subnet_mappings: Option<Vec<SubnetMapping>>,
    subnets: Option<Vec<String>>,
) -> Result<SetSubnetsOutput, String> {
    unimplemented!("set_subnets")
}

/// Set IP Address Type
pub async fn set_ip_address_type(
    ip_address_type: &str,
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<SetIpAddressTypeOutput, String> {
    unimplemented!("set_ip_address_type")
}

/// Create Target Group
pub async fn create_target_group(
    name: &str,
    health_check_enabled: Option<bool>,
    health_check_interval_seconds: Option<i32>,
    health_check_path: Option<&str>,
    health_check_port: Option<&str>,
    health_check_protocol: Option<&str>,
    health_check_timeout_seconds: Option<i32>,
    healthy_threshold_count: Option<i32>,
    ip_address_type: Option<&str>,
    matcher: Option<Matcher>,
    port: Option<i32>,
    protocol: Option<&str>,
    protocol_version: Option<&str>,
    region: Option<&str>,
    tags: Option<ElbTags>,
    target_type: Option<&str>,
    unhealthy_threshold_count: Option<i32>,
    vpc_id: Option<&str>,
) -> Result<CreateTargetGroupOutput, String> {
    unimplemented!("create_target_group")
}

/// Delete Target Group
pub async fn delete_target_group(
    target_group_arn: &str,
    region: Option<&str>,
) -> Result<DeleteTargetGroupOutput, String> {
    unimplemented!("delete_target_group")
}

/// Describe Target Groups
pub async fn describe_target_groups(
    load_balancer_arn: Option<&str>,
    marker: Option<&str>,
    names: Option<Vec<String>>,
    page_size: Option<i32>,
    region: Option<&str>,
    target_group_arns: Option<Vec<String>>,
) -> Result<DescribeTargetGroupsOutput, String> {
    unimplemented!("describe_target_groups")
}

/// Modify Target Group
pub async fn modify_target_group(
    target_group_arn: &str,
    health_check_enabled: Option<bool>,
    health_check_interval_seconds: Option<i32>,
    health_check_path: Option<&str>,
    health_check_port: Option<&str>,
    health_check_protocol: Option<&str>,
    health_check_timeout_seconds: Option<i32>,
    healthy_threshold_count: Option<i32>,
    matcher: Option<Matcher>,
    region: Option<&str>,
    unhealthy_threshold_count: Option<i32>,
) -> Result<ModifyTargetGroupOutput, String> {
    unimplemented!("modify_target_group")
}

/// Modify Target Group Attributes
pub async fn modify_target_group_attributes(
    attributes: Vec<TargetGroupAttribute>,
    target_group_arn: &str,
    region: Option<&str>,
) -> Result<ModifyTargetGroupAttributesOutput, String> {
    unimplemented!("modify_target_group_attributes")
}

/// Describe Target Group Attributes
pub async fn describe_target_group_attributes(
    target_group_arn: &str,
    region: Option<&str>,
) -> Result<DescribeTargetGroupAttributesOutput, String> {
    unimplemented!("describe_target_group_attributes")
}

/// Register Targets
pub async fn register_targets(
    target_group_arn: &str,
    targets: Vec<TargetDescription>,
    region: Option<&str>,
) -> Result<RegisterTargetsOutput, String> {
    unimplemented!("register_targets")
}

/// Deregister Targets
pub async fn deregister_targets(
    target_group_arn: &str,
    targets: Vec<TargetDescription>,
    region: Option<&str>,
) -> Result<DeregisterTargetsOutput, String> {
    unimplemented!("deregister_targets")
}

/// Describe Target Health
pub async fn describe_target_health(
    target_group_arn: &str,
    region: Option<&str>,
    targets: Option<Vec<TargetDescription>>,
) -> Result<DescribeTargetHealthOutput, String> {
    unimplemented!("describe_target_health")
}

/// Create Listener
pub async fn create_listener(
    default_actions: Vec<Action>,
    load_balancer_arn: &str,
    alpn_policy: Option<Vec<String>>,
    certificates: Option<Vec<Certificate>>,
    port: Option<i32>,
    protocol: Option<&str>,
    region: Option<&str>,
    ssl_policy: Option<&str>,
    tags: Option<ElbTags>,
) -> Result<CreateListenerOutput, String> {
    unimplemented!("create_listener")
}

/// Delete Listener
pub async fn delete_listener(
    listener_arn: &str,
    region: Option<&str>,
) -> Result<DeleteListenerOutput, String> {
    unimplemented!("delete_listener")
}

/// Describe Listeners
pub async fn describe_listeners(
    listener_arns: Option<Vec<String>>,
    load_balancer_arn: Option<&str>,
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeListenersOutput, String> {
    unimplemented!("describe_listeners")
}

/// Modify Listener
pub async fn modify_listener(
    listener_arn: &str,
    alpn_policy: Option<Vec<String>>,
    certificates: Option<Vec<Certificate>>,
    default_actions: Option<Vec<Action>>,
    port: Option<i32>,
    protocol: Option<&str>,
    region: Option<&str>,
    ssl_policy: Option<&str>,
) -> Result<ModifyListenerOutput, String> {
    unimplemented!("modify_listener")
}

/// Create Rule
pub async fn create_rule(
    actions: Vec<Action>,
    conditions: Vec<RuleCondition>,
    listener_arn: &str,
    priority: i32,
    region: Option<&str>,
    tags: Option<ElbTags>,
) -> Result<CreateRuleOutput, String> {
    unimplemented!("create_rule")
}

/// Delete Rule
pub async fn delete_rule(
    rule_arn: &str,
    region: Option<&str>,
) -> Result<DeleteRuleOutput, String> {
    unimplemented!("delete_rule")
}

/// Describe Rules
pub async fn describe_rules(
    listener_arn: Option<&str>,
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
    rule_arns: Option<Vec<String>>,
) -> Result<DescribeRulesOutput, String> {
    unimplemented!("describe_rules")
}

/// Modify Rule
pub async fn modify_rule(
    rule_arn: &str,
    actions: Option<Vec<Action>>,
    conditions: Option<Vec<RuleCondition>>,
    region: Option<&str>,
) -> Result<ModifyRuleOutput, String> {
    unimplemented!("modify_rule")
}

/// Set Rule Priorities
pub async fn set_rule_priorities(
    rule_priorities: Vec<RulePriorityPair>,
    region: Option<&str>,
) -> Result<SetRulePrioritiesOutput, String> {
    unimplemented!("set_rule_priorities")
}

/// Add Listener Certificates
pub async fn add_listener_certificates(
    certificates: Vec<Certificate>,
    listener_arn: &str,
    region: Option<&str>,
) -> Result<AddListenerCertificatesOutput, String> {
    unimplemented!("add_listener_certificates")
}

/// Remove Listener Certificates
pub async fn remove_listener_certificates(
    certificates: Vec<Certificate>,
    listener_arn: &str,
    region: Option<&str>,
) -> Result<RemoveListenerCertificatesOutput, String> {
    unimplemented!("remove_listener_certificates")
}

/// Describe Listener Certificates
pub async fn describe_listener_certificates(
    listener_arn: &str,
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeListenerCertificatesOutput, String> {
    unimplemented!("describe_listener_certificates")
}

/// Describe SSL Policies
pub async fn describe_ssl_policies(
    load_balancer_type: Option<&str>,
    marker: Option<&str>,
    names: Option<Vec<String>>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeSslPoliciesOutput, String> {
    unimplemented!("describe_ssl_policies")
}

/// Describe Account Limits
pub async fn describe_account_limits(
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeAccountLimitsOutput, String> {
    unimplemented!("describe_account_limits")
}

/// Add Tags
pub async fn add_tags(
    resource_arns: Vec<String>,
    tags: ElbTags,
    region: Option<&str>,
) -> Result<AddTagsOutput, String> {
    unimplemented!("add_tags")
}

/// Remove Tags
pub async fn remove_tags(
    resource_arns: Vec<String>,
    tag_keys: Vec<String>,
    region: Option<&str>,
) -> Result<RemoveTagsOutput, String> {
    unimplemented!("remove_tags")
}

/// Describe Tags
pub async fn describe_tags(
    resource_arns: Vec<String>,
    region: Option<&str>,
) -> Result<DescribeTagsOutput, String> {
    unimplemented!("describe_tags")
}

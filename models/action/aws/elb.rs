// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLoadBalancerInput {
    pub customer_owned_ipv4_pool: String,
    pub ip_address_type: String,
    pub name: String,
    pub region: String,
    pub scheme: String,
    pub security_groups: Vec<String>,
    pub subnet_mappings: Vec<String>,
    pub subnets: Vec<String>,
    pub tags: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLoadBalancerOutput {
    pub load_balancers: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLoadBalancerInput {
    pub load_balancer_arn: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLoadBalancerOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancersInput {
    pub load_balancer_arns: Vec<String>,
    pub marker: String,
    pub names: Vec<String>,
    pub page_size: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancersOutput {
    pub load_balancers: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyLoadBalancerAttributesInput {
    pub attributes: Vec<String>,
    pub load_balancer_arn: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyLoadBalancerAttributesOutput {
    pub attributes: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancerAttributesInput {
    pub load_balancer_arn: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLoadBalancerAttributesOutput {
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSecurityGroupsInput {
    pub load_balancer_arn: String,
    pub region: String,
    pub security_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSecurityGroupsOutput {
    pub security_group_ids: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSubnetsInput {
    pub ip_address_type: String,
    pub load_balancer_arn: String,
    pub region: String,
    pub subnet_mappings: Vec<String>,
    pub subnets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSubnetsOutput {
    pub availability_zones: Vec<String>,
    pub ip_address_type: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIpAddressTypeInput {
    pub ip_address_type: String,
    pub load_balancer_arn: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIpAddressTypeOutput {
    pub ip_address_type: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTargetGroupInput {
    #[serde(default)]
    pub health_check_enabled: bool,
    pub health_check_interval_seconds: i64,
    pub health_check_path: String,
    pub health_check_port: String,
    pub health_check_protocol: String,
    pub health_check_timeout_seconds: i64,
    pub healthy_threshold_count: i64,
    pub ip_address_type: String,
    pub matcher: String,
    pub name: String,
    pub port: i64,
    pub protocol: String,
    pub protocol_version: String,
    pub region: String,
    pub tags: String,
    pub target_type: String,
    pub unhealthy_threshold_count: i64,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTargetGroupOutput {
    pub target_groups: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTargetGroupInput {
    pub region: String,
    pub target_group_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTargetGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTargetGroupsInput {
    pub load_balancer_arn: String,
    pub marker: String,
    pub names: Vec<String>,
    pub page_size: i64,
    pub region: String,
    pub target_group_arns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTargetGroupsOutput {
    pub next_marker: String,
    pub target_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyTargetGroupInput {
    pub health_check_enabled: bool,
    pub health_check_interval_seconds: i64,
    pub health_check_path: String,
    pub health_check_port: String,
    pub health_check_protocol: String,
    pub health_check_timeout_seconds: i64,
    pub healthy_threshold_count: i64,
    pub matcher: String,
    pub region: String,
    pub target_group_arn: String,
    pub unhealthy_threshold_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyTargetGroupOutput {
    pub target_groups: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyTargetGroupAttributesInput {
    pub attributes: Vec<String>,
    pub region: String,
    pub target_group_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyTargetGroupAttributesOutput {
    pub attributes: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTargetGroupAttributesInput {
    pub region: String,
    pub target_group_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTargetGroupAttributesOutput {
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterTargetsInput {
    pub region: String,
    pub target_group_arn: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterTargetsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeregisterTargetsInput {
    pub region: String,
    pub target_group_arn: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeregisterTargetsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTargetHealthInput {
    pub region: String,
    pub target_group_arn: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTargetHealthOutput {
    pub target_health_descriptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateListenerInput {
    pub alpn_policy: Vec<String>,
    pub certificates: Vec<String>,
    pub default_actions: Vec<String>,
    pub load_balancer_arn: String,
    pub port: i64,
    pub protocol: String,
    pub region: String,
    pub ssl_policy: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateListenerOutput {
    pub listeners: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteListenerInput {
    pub listener_arn: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteListenerOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeListenersInput {
    pub listener_arns: Vec<String>,
    pub load_balancer_arn: String,
    pub marker: String,
    pub page_size: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeListenersOutput {
    pub listeners: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyListenerInput {
    pub alpn_policy: Vec<String>,
    pub certificates: Vec<String>,
    pub default_actions: Vec<String>,
    pub listener_arn: String,
    pub port: i64,
    pub protocol: String,
    pub region: String,
    pub ssl_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyListenerOutput {
    pub listeners: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRuleInput {
    pub actions: Vec<String>,
    pub conditions: Vec<String>,
    pub listener_arn: String,
    pub priority: i64,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRuleOutput {
    pub rules: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRuleInput {
    pub region: String,
    pub rule_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRuleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRulesInput {
    pub listener_arn: String,
    pub marker: String,
    pub page_size: i64,
    pub region: String,
    pub rule_arns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRulesOutput {
    pub next_marker: String,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyRuleInput {
    pub actions: Vec<String>,
    pub conditions: Vec<String>,
    pub region: String,
    pub rule_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyRuleOutput {
    pub rules: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetRulePrioritiesInput {
    pub region: String,
    pub rule_priorities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetRulePrioritiesOutput {
    pub rules: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddListenerCertificatesInput {
    pub certificates: Vec<String>,
    pub listener_arn: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddListenerCertificatesOutput {
    pub certificates: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveListenerCertificatesInput {
    pub certificates: Vec<String>,
    pub listener_arn: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveListenerCertificatesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeListenerCertificatesInput {
    pub listener_arn: String,
    pub marker: String,
    pub page_size: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeListenerCertificatesOutput {
    pub certificates: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSslPoliciesInput {
    pub load_balancer_type: String,
    pub marker: String,
    pub names: Vec<String>,
    pub page_size: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSslPoliciesOutput {
    pub next_marker: String,
    pub ssl_policies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAccountLimitsInput {
    pub marker: String,
    pub page_size: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAccountLimitsOutput {
    pub limits: Vec<String>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTagsInput {
    pub region: String,
    pub resource_arns: Vec<String>,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTagsInput {
    pub region: String,
    pub resource_arns: Vec<String>,
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTagsInput {
    pub region: String,
    pub resource_arns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTagsOutput {
    pub tag_descriptions: Vec<String>,
}

#[async_trait]
pub trait ElbAction {
    async fn create_load_balancer(&self, input: CreateLoadBalancerInput) -> Result<CreateLoadBalancerOutput, Box<dyn std::error::Error>>;
    async fn delete_load_balancer(&self, input: DeleteLoadBalancerInput) -> Result<DeleteLoadBalancerOutput, Box<dyn std::error::Error>>;
    async fn describe_load_balancers(&self, input: DescribeLoadBalancersInput) -> Result<DescribeLoadBalancersOutput, Box<dyn std::error::Error>>;
    async fn modify_load_balancer_attributes(&self, input: ModifyLoadBalancerAttributesInput) -> Result<ModifyLoadBalancerAttributesOutput, Box<dyn std::error::Error>>;
    async fn describe_load_balancer_attributes(&self, input: DescribeLoadBalancerAttributesInput) -> Result<DescribeLoadBalancerAttributesOutput, Box<dyn std::error::Error>>;
    async fn set_security_groups(&self, input: SetSecurityGroupsInput) -> Result<SetSecurityGroupsOutput, Box<dyn std::error::Error>>;
    async fn set_subnets(&self, input: SetSubnetsInput) -> Result<SetSubnetsOutput, Box<dyn std::error::Error>>;
    async fn set_ip_address_type(&self, input: SetIpAddressTypeInput) -> Result<SetIpAddressTypeOutput, Box<dyn std::error::Error>>;
    async fn create_target_group(&self, input: CreateTargetGroupInput) -> Result<CreateTargetGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_target_group(&self, input: DeleteTargetGroupInput) -> Result<DeleteTargetGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_target_groups(&self, input: DescribeTargetGroupsInput) -> Result<DescribeTargetGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_target_group(&self, input: ModifyTargetGroupInput) -> Result<ModifyTargetGroupOutput, Box<dyn std::error::Error>>;
    async fn modify_target_group_attributes(&self, input: ModifyTargetGroupAttributesInput) -> Result<ModifyTargetGroupAttributesOutput, Box<dyn std::error::Error>>;
    async fn describe_target_group_attributes(&self, input: DescribeTargetGroupAttributesInput) -> Result<DescribeTargetGroupAttributesOutput, Box<dyn std::error::Error>>;
    async fn register_targets(&self, input: RegisterTargetsInput) -> Result<RegisterTargetsOutput, Box<dyn std::error::Error>>;
    async fn deregister_targets(&self, input: DeregisterTargetsInput) -> Result<DeregisterTargetsOutput, Box<dyn std::error::Error>>;
    async fn describe_target_health(&self, input: DescribeTargetHealthInput) -> Result<DescribeTargetHealthOutput, Box<dyn std::error::Error>>;
    async fn create_listener(&self, input: CreateListenerInput) -> Result<CreateListenerOutput, Box<dyn std::error::Error>>;
    async fn delete_listener(&self, input: DeleteListenerInput) -> Result<DeleteListenerOutput, Box<dyn std::error::Error>>;
    async fn describe_listeners(&self, input: DescribeListenersInput) -> Result<DescribeListenersOutput, Box<dyn std::error::Error>>;
    async fn modify_listener(&self, input: ModifyListenerInput) -> Result<ModifyListenerOutput, Box<dyn std::error::Error>>;
    async fn create_rule(&self, input: CreateRuleInput) -> Result<CreateRuleOutput, Box<dyn std::error::Error>>;
    async fn delete_rule(&self, input: DeleteRuleInput) -> Result<DeleteRuleOutput, Box<dyn std::error::Error>>;
    async fn describe_rules(&self, input: DescribeRulesInput) -> Result<DescribeRulesOutput, Box<dyn std::error::Error>>;
    async fn modify_rule(&self, input: ModifyRuleInput) -> Result<ModifyRuleOutput, Box<dyn std::error::Error>>;
    async fn set_rule_priorities(&self, input: SetRulePrioritiesInput) -> Result<SetRulePrioritiesOutput, Box<dyn std::error::Error>>;
    async fn add_listener_certificates(&self, input: AddListenerCertificatesInput) -> Result<AddListenerCertificatesOutput, Box<dyn std::error::Error>>;
    async fn remove_listener_certificates(&self, input: RemoveListenerCertificatesInput) -> Result<RemoveListenerCertificatesOutput, Box<dyn std::error::Error>>;
    async fn describe_listener_certificates(&self, input: DescribeListenerCertificatesInput) -> Result<DescribeListenerCertificatesOutput, Box<dyn std::error::Error>>;
    async fn describe_ssl_policies(&self, input: DescribeSslPoliciesInput) -> Result<DescribeSslPoliciesOutput, Box<dyn std::error::Error>>;
    async fn describe_account_limits(&self, input: DescribeAccountLimitsInput) -> Result<DescribeAccountLimitsOutput, Box<dyn std::error::Error>>;
    async fn add_tags(&self, input: AddTagsInput) -> Result<AddTagsOutput, Box<dyn std::error::Error>>;
    async fn remove_tags(&self, input: RemoveTagsInput) -> Result<RemoveTagsOutput, Box<dyn std::error::Error>>;
    async fn describe_tags(&self, input: DescribeTagsInput) -> Result<DescribeTagsOutput, Box<dyn std::error::Error>>;
}

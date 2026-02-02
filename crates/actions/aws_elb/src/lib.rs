//! Harana Actions - AWS ELB Module
//!
//! This module provides AWS Elastic Load Balancer v2 (ALB/NLB) actions for managing
//! Application Load Balancers and Network Load Balancers.

/// Output types for AWS ELB actions
pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_elasticloadbalancingv2::{
    config::Region,
    types::{
        ActionTypeEnum, IpAddressType, LoadBalancerSchemeEnum, LoadBalancerTypeEnum,
        ProtocolEnum, TargetTypeEnum,
    },
    Client,
};
use output::*;

/// Creates an ELBv2 client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let elb_config = if let Some(region_str) = region {
        aws_sdk_elasticloadbalancingv2::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_elasticloadbalancingv2::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(elb_config))
}

/// Converts ElbTags to AWS SDK Tag format
fn tags_to_aws_tags(tags: &ElbTags) -> Vec<aws_sdk_elasticloadbalancingv2::types::Tag> {
    tags.tags
        .iter()
        .map(|(k, v)| {
            aws_sdk_elasticloadbalancingv2::types::Tag::builder()
                .key(k)
                .value(v)
                .build()
        })
        .collect()
}

/// Convert SubnetMapping to AWS SDK type
fn subnet_mapping_to_aws(
    mapping: &SubnetMapping,
) -> aws_sdk_elasticloadbalancingv2::types::SubnetMapping {
    let mut builder = aws_sdk_elasticloadbalancingv2::types::SubnetMapping::builder()
        .subnet_id(&mapping.subnet_id);

    if let Some(ref alloc_id) = mapping.allocation_id {
        builder = builder.allocation_id(alloc_id);
    }
    if let Some(ref private_ip) = mapping.private_ipv4_address {
        builder = builder.private_ipv4_address(private_ip);
    }
    if let Some(ref ipv6) = mapping.ipv6_address {
        builder = builder.ipv6_address(ipv6);
    }

    builder.build()
}

/// Convert AWS LoadBalancer to our type
fn convert_load_balancer(
    lb: &aws_sdk_elasticloadbalancingv2::types::LoadBalancer,
) -> LoadBalancer {
    LoadBalancer {
        load_balancer_arn: lb.load_balancer_arn().unwrap_or_default().to_string(),
        dns_name: lb.dns_name().unwrap_or_default().to_string(),
        canonical_hosted_zone_id: lb.canonical_hosted_zone_id().unwrap_or_default().to_string(),
        created_time: lb.created_time().map(|t| {
            chrono::DateTime::from_timestamp(t.secs(), t.subsec_nanos())
                .unwrap_or_default()
        }),
        load_balancer_name: lb.load_balancer_name().unwrap_or_default().to_string(),
        scheme: lb
            .scheme()
            .map(|s| s.as_str().to_string())
            .unwrap_or_default(),
        vpc_id: lb.vpc_id().unwrap_or_default().to_string(),
        state: lb.state().map(|s| LoadBalancerState {
            code: s.code().map(|c| c.as_str().to_string()).unwrap_or_default(),
            reason: s.reason().map(|r| r.to_string()),
        }),
        load_balancer_type: lb
            .r#type()
            .map(|t| t.as_str().to_string())
            .unwrap_or_default(),
        availability_zones: lb
            .availability_zones()
            .iter()
            .map(|az| AvailabilityZone {
                zone_name: az.zone_name().unwrap_or_default().to_string(),
                subnet_id: az.subnet_id().unwrap_or_default().to_string(),
                outpost_id: az.outpost_id().map(|s| s.to_string()),
                load_balancer_addresses: if az.load_balancer_addresses().is_empty() {
                    None
                } else {
                    Some(
                        az.load_balancer_addresses()
                            .iter()
                            .map(|addr| LoadBalancerAddress {
                                ip_address: addr.ip_address().map(|s| s.to_string()),
                                allocation_id: addr.allocation_id().map(|s| s.to_string()),
                                private_ipv4_address: addr.private_ipv4_address().map(|s| s.to_string()),
                                ipv6_address: addr.ipv6_address().map(|s| s.to_string()),
                            })
                            .collect(),
                    )
                },
            })
            .collect(),
        security_groups: if lb.security_groups().is_empty() {
            None
        } else {
            Some(lb.security_groups().iter().map(|s| s.to_string()).collect())
        },
        ip_address_type: lb
            .ip_address_type()
            .map(|t| t.as_str().to_string())
            .unwrap_or_default(),
        customer_owned_ipv4_pool: lb.customer_owned_ipv4_pool().map(|s| s.to_string()),
    }
}

/// Convert AWS TargetGroup to our type
fn convert_target_group(
    tg: &aws_sdk_elasticloadbalancingv2::types::TargetGroup,
) -> TargetGroup {
    TargetGroup {
        target_group_arn: tg.target_group_arn().unwrap_or_default().to_string(),
        target_group_name: tg.target_group_name().unwrap_or_default().to_string(),
        protocol: tg.protocol().map(|p| p.as_str().to_string()),
        port: tg.port(),
        vpc_id: tg.vpc_id().map(|s| s.to_string()),
        health_check_protocol: tg.health_check_protocol().map(|p| p.as_str().to_string()),
        health_check_port: tg.health_check_port().map(|s| s.to_string()),
        health_check_enabled: tg.health_check_enabled(),
        health_check_interval_seconds: tg.health_check_interval_seconds(),
        health_check_timeout_seconds: tg.health_check_timeout_seconds(),
        healthy_threshold_count: tg.healthy_threshold_count(),
        unhealthy_threshold_count: tg.unhealthy_threshold_count(),
        health_check_path: tg.health_check_path().map(|s| s.to_string()),
        matcher: tg.matcher().map(|m| Matcher {
            http_code: m.http_code().map(|s| s.to_string()),
            grpc_code: m.grpc_code().map(|s| s.to_string()),
        }),
        load_balancer_arns: if tg.load_balancer_arns().is_empty() {
            None
        } else {
            Some(tg.load_balancer_arns().iter().map(|s| s.to_string()).collect())
        },
        target_type: tg
            .target_type()
            .map(|t| t.as_str().to_string())
            .unwrap_or_default(),
        protocol_version: tg.protocol_version().map(|s| s.to_string()),
        ip_address_type: tg.ip_address_type().map(|t| t.as_str().to_string()),
    }
}

/// Convert AWS Listener to our type
fn convert_listener(
    listener: &aws_sdk_elasticloadbalancingv2::types::Listener,
) -> Listener {
    Listener {
        listener_arn: listener.listener_arn().unwrap_or_default().to_string(),
        load_balancer_arn: listener.load_balancer_arn().unwrap_or_default().to_string(),
        port: listener.port(),
        protocol: listener.protocol().map(|p| p.as_str().to_string()),
        certificates: if listener.certificates().is_empty() {
            None
        } else {
            Some(
                listener
                    .certificates()
                    .iter()
                    .map(|c| Certificate {
                        certificate_arn: c.certificate_arn().map(|s| s.to_string()),
                        is_default: c.is_default(),
                    })
                    .collect(),
            )
        },
        ssl_policy: listener.ssl_policy().map(|s| s.to_string()),
        default_actions: listener
            .default_actions()
            .iter()
            .map(convert_action)
            .collect(),
        alpn_policy: if listener.alpn_policy().is_empty() {
            None
        } else {
            Some(listener.alpn_policy().iter().map(|s| s.to_string()).collect())
        },
    }
}

/// Convert AWS Action to our type
fn convert_action(action: &aws_sdk_elasticloadbalancingv2::types::Action) -> Action {
    Action {
        action_type: action
            .r#type()
            .map(|t| t.as_str().to_string())
            .unwrap_or_default(),
        target_group_arn: action.target_group_arn().map(|s| s.to_string()),
        authenticate_oidc_config: action.authenticate_oidc_config().map(|c| {
            serde_json::json!({
                "issuer": c.issuer(),
                "authorization_endpoint": c.authorization_endpoint(),
                "token_endpoint": c.token_endpoint(),
                "user_info_endpoint": c.user_info_endpoint(),
                "client_id": c.client_id(),
            })
        }),
        authenticate_cognito_config: action.authenticate_cognito_config().map(|c| {
            serde_json::json!({
                "user_pool_arn": c.user_pool_arn(),
                "user_pool_client_id": c.user_pool_client_id(),
                "user_pool_domain": c.user_pool_domain(),
            })
        }),
        order: action.order(),
        redirect_config: action.redirect_config().map(|c| {
            serde_json::json!({
                "protocol": c.protocol(),
                "port": c.port(),
                "host": c.host(),
                "path": c.path(),
                "query": c.query(),
                "status_code": c.status_code().map(|s| s.as_str()),
            })
        }),
        fixed_response_config: action.fixed_response_config().map(|c| {
            serde_json::json!({
                "message_body": c.message_body(),
                "status_code": c.status_code(),
                "content_type": c.content_type(),
            })
        }),
        forward_config: action.forward_config().map(|c| {
            serde_json::json!({
                "target_groups": c.target_groups().iter().map(|tg| {
                    serde_json::json!({
                        "target_group_arn": tg.target_group_arn(),
                        "weight": tg.weight(),
                    })
                }).collect::<Vec<_>>(),
            })
        }),
    }
}

/// Convert AWS Rule to our type
fn convert_rule(rule: &aws_sdk_elasticloadbalancingv2::types::Rule) -> Rule {
    Rule {
        rule_arn: rule.rule_arn().unwrap_or_default().to_string(),
        priority: rule.priority().unwrap_or_default().to_string(),
        conditions: rule
            .conditions()
            .iter()
            .map(|c| RuleCondition {
                field: c.field().map(|s| s.to_string()),
                values: if c.values().is_empty() {
                    None
                } else {
                    Some(c.values().iter().map(|s| s.to_string()).collect())
                },
                host_header_config: c.host_header_config().map(|h| {
                    serde_json::json!({
                        "values": h.values(),
                    })
                }),
                path_pattern_config: c.path_pattern_config().map(|p| {
                    serde_json::json!({
                        "values": p.values(),
                    })
                }),
                http_header_config: c.http_header_config().map(|h| {
                    serde_json::json!({
                        "http_header_name": h.http_header_name(),
                        "values": h.values(),
                    })
                }),
                query_string_config: c.query_string_config().map(|q| {
                    serde_json::json!({
                        "values": q.values().iter().map(|v| {
                            serde_json::json!({
                                "key": v.key(),
                                "value": v.value(),
                            })
                        }).collect::<Vec<_>>(),
                    })
                }),
                http_request_method_config: c.http_request_method_config().map(|m| {
                    serde_json::json!({
                        "values": m.values(),
                    })
                }),
                source_ip_config: c.source_ip_config().map(|s| {
                    serde_json::json!({
                        "values": s.values(),
                    })
                }),
            })
            .collect(),
        actions: rule.actions().iter().map(convert_action).collect(),
        is_default: rule.is_default().unwrap_or(false),
    }
}

/// Convert our Action to AWS SDK type
fn action_to_aws(
    action: &Action,
) -> aws_sdk_elasticloadbalancingv2::types::Action {
    let action_type = ActionTypeEnum::from(action.action_type.as_str());

    let mut builder = aws_sdk_elasticloadbalancingv2::types::Action::builder()
        .r#type(action_type);

    if let Some(ref tg_arn) = action.target_group_arn {
        builder = builder.target_group_arn(tg_arn);
    }

    if let Some(order) = action.order {
        builder = builder.order(order);
    }

    if let Some(ref redirect_config) = action.redirect_config {
        let mut redirect_builder =
            aws_sdk_elasticloadbalancingv2::types::RedirectActionConfig::builder();

        if let Some(protocol) = redirect_config.get("protocol").and_then(|v| v.as_str()) {
            redirect_builder = redirect_builder.protocol(protocol);
        }
        if let Some(port) = redirect_config.get("port").and_then(|v| v.as_str()) {
            redirect_builder = redirect_builder.port(port);
        }
        if let Some(host) = redirect_config.get("host").and_then(|v| v.as_str()) {
            redirect_builder = redirect_builder.host(host);
        }
        if let Some(path) = redirect_config.get("path").and_then(|v| v.as_str()) {
            redirect_builder = redirect_builder.path(path);
        }
        if let Some(query) = redirect_config.get("query").and_then(|v| v.as_str()) {
            redirect_builder = redirect_builder.query(query);
        }
        if let Some(status_code) = redirect_config.get("status_code").and_then(|v| v.as_str()) {
            redirect_builder = redirect_builder.status_code(
                aws_sdk_elasticloadbalancingv2::types::RedirectActionStatusCodeEnum::from(
                    status_code,
                ),
            );
        }

        builder = builder.redirect_config(redirect_builder.build());
    }

    if let Some(ref fixed_response_config) = action.fixed_response_config {
        let mut fixed_builder =
            aws_sdk_elasticloadbalancingv2::types::FixedResponseActionConfig::builder();

        if let Some(message_body) = fixed_response_config.get("message_body").and_then(|v| v.as_str())
        {
            fixed_builder = fixed_builder.message_body(message_body);
        }
        if let Some(status_code) = fixed_response_config.get("status_code").and_then(|v| v.as_str())
        {
            fixed_builder = fixed_builder.status_code(status_code);
        }
        if let Some(content_type) = fixed_response_config.get("content_type").and_then(|v| v.as_str())
        {
            fixed_builder = fixed_builder.content_type(content_type);
        }

        builder = builder.fixed_response_config(fixed_builder.build());
    }

    if let Some(ref forward_config) = action.forward_config {
        let mut forward_builder =
            aws_sdk_elasticloadbalancingv2::types::ForwardActionConfig::builder();

        if let Some(target_groups) = forward_config.get("target_groups").and_then(|v| v.as_array()) {
            for tg in target_groups {
                let mut tg_builder =
                    aws_sdk_elasticloadbalancingv2::types::TargetGroupTuple::builder();
                if let Some(arn) = tg.get("target_group_arn").and_then(|v| v.as_str()) {
                    tg_builder = tg_builder.target_group_arn(arn);
                }
                if let Some(weight) = tg.get("weight").and_then(|v| v.as_i64()) {
                    tg_builder = tg_builder.weight(weight as i32);
                }
                forward_builder = forward_builder.target_groups(tg_builder.build());
            }
        }

        builder = builder.forward_config(forward_builder.build());
    }

    builder.build()
}

/// Convert our RuleCondition to AWS SDK type
fn rule_condition_to_aws(
    condition: &RuleCondition,
) -> aws_sdk_elasticloadbalancingv2::types::RuleCondition {
    let mut builder = aws_sdk_elasticloadbalancingv2::types::RuleCondition::builder();

    if let Some(ref field) = condition.field {
        builder = builder.field(field);
    }

    if let Some(ref values) = condition.values {
        builder = builder.set_values(Some(values.clone()));
    }

    if let Some(ref host_header_config) = condition.host_header_config {
        let mut hh_builder =
            aws_sdk_elasticloadbalancingv2::types::HostHeaderConditionConfig::builder();
        if let Some(values) = host_header_config.get("values").and_then(|v| v.as_array()) {
            let vals: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            hh_builder = hh_builder.set_values(Some(vals));
        }
        builder = builder.host_header_config(hh_builder.build());
    }

    if let Some(ref path_pattern_config) = condition.path_pattern_config {
        let mut pp_builder =
            aws_sdk_elasticloadbalancingv2::types::PathPatternConditionConfig::builder();
        if let Some(values) = path_pattern_config.get("values").and_then(|v| v.as_array()) {
            let vals: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            pp_builder = pp_builder.set_values(Some(vals));
        }
        builder = builder.path_pattern_config(pp_builder.build());
    }

    if let Some(ref http_header_config) = condition.http_header_config {
        let mut hh_builder =
            aws_sdk_elasticloadbalancingv2::types::HttpHeaderConditionConfig::builder();
        if let Some(name) = http_header_config
            .get("http_header_name")
            .and_then(|v| v.as_str())
        {
            hh_builder = hh_builder.http_header_name(name);
        }
        if let Some(values) = http_header_config.get("values").and_then(|v| v.as_array()) {
            let vals: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            hh_builder = hh_builder.set_values(Some(vals));
        }
        builder = builder.http_header_config(hh_builder.build());
    }

    if let Some(ref query_string_config) = condition.query_string_config {
        let mut qs_builder =
            aws_sdk_elasticloadbalancingv2::types::QueryStringConditionConfig::builder();
        if let Some(values) = query_string_config.get("values").and_then(|v| v.as_array()) {
            for val in values {
                let mut kv_builder =
                    aws_sdk_elasticloadbalancingv2::types::QueryStringKeyValuePair::builder();
                if let Some(key) = val.get("key").and_then(|v| v.as_str()) {
                    kv_builder = kv_builder.key(key);
                }
                if let Some(value) = val.get("value").and_then(|v| v.as_str()) {
                    kv_builder = kv_builder.value(value);
                }
                qs_builder = qs_builder.values(kv_builder.build());
            }
        }
        builder = builder.query_string_config(qs_builder.build());
    }

    if let Some(ref http_request_method_config) = condition.http_request_method_config {
        let mut hrm_builder =
            aws_sdk_elasticloadbalancingv2::types::HttpRequestMethodConditionConfig::builder();
        if let Some(values) = http_request_method_config
            .get("values")
            .and_then(|v| v.as_array())
        {
            let vals: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            hrm_builder = hrm_builder.set_values(Some(vals));
        }
        builder = builder.http_request_method_config(hrm_builder.build());
    }

    if let Some(ref source_ip_config) = condition.source_ip_config {
        let mut si_builder =
            aws_sdk_elasticloadbalancingv2::types::SourceIpConditionConfig::builder();
        if let Some(values) = source_ip_config.get("values").and_then(|v| v.as_array()) {
            let vals: Vec<String> = values
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            si_builder = si_builder.set_values(Some(vals));
        }
        builder = builder.source_ip_config(si_builder.build());
    }

    builder.build()
}

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
    let client = create_client(region).await?;

    let mut request = client.create_load_balancer().name(name);

    if let Some(pool) = customer_owned_ipv4_pool {
        request = request.customer_owned_ipv4_pool(pool);
    }

    if let Some(ip_type) = ip_address_type {
        request = request.ip_address_type(IpAddressType::from(ip_type));
    }

    if let Some(s) = scheme {
        request = request.scheme(LoadBalancerSchemeEnum::from(s));
    }

    if let Some(sgs) = security_groups {
        request = request.set_security_groups(Some(sgs));
    }

    if let Some(ref mappings) = subnet_mappings {
        let aws_mappings: Vec<_> = mappings.iter().map(subnet_mapping_to_aws).collect();
        request = request.set_subnet_mappings(Some(aws_mappings));
    }

    if let Some(subs) = subnets {
        request = request.set_subnets(Some(subs));
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    if let Some(t) = lb_type {
        request = request.r#type(LoadBalancerTypeEnum::from(t));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create load balancer: {}", e))?;

    let load_balancers: Vec<LoadBalancer> = response
        .load_balancers()
        .iter()
        .map(convert_load_balancer)
        .collect();

    Ok(CreateLoadBalancerOutput {
        load_balancers,
        success: true,
    })
}

/// Delete Load Balancer
pub async fn delete_load_balancer(
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<DeleteLoadBalancerOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_load_balancer()
        .load_balancer_arn(load_balancer_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to delete load balancer: {}", e))?;

    Ok(DeleteLoadBalancerOutput { success: true })
}

/// Describe Load Balancers
pub async fn describe_load_balancers(
    load_balancer_arns: Option<Vec<String>>,
    marker: Option<&str>,
    names: Option<Vec<String>>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeLoadBalancersOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_load_balancers();

    if let Some(arns) = load_balancer_arns {
        request = request.set_load_balancer_arns(Some(arns));
    }

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(n) = names {
        request = request.set_names(Some(n));
    }

    if let Some(size) = page_size {
        request = request.page_size(size);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe load balancers: {}", e))?;

    let load_balancers: Vec<LoadBalancer> = response
        .load_balancers()
        .iter()
        .map(convert_load_balancer)
        .collect();

    Ok(DescribeLoadBalancersOutput {
        load_balancers,
        next_marker: response.next_marker().map(|s| s.to_string()),
    })
}

/// Modify Load Balancer Attributes
pub async fn modify_load_balancer_attributes(
    attributes: Vec<LoadBalancerAttribute>,
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<ModifyLoadBalancerAttributesOutput, String> {
    let client = create_client(region).await?;

    let aws_attributes: Vec<_> = attributes
        .iter()
        .map(|attr| {
            aws_sdk_elasticloadbalancingv2::types::LoadBalancerAttribute::builder()
                .key(&attr.key)
                .value(&attr.value)
                .build()
        })
        .collect();

    let response = client
        .modify_load_balancer_attributes()
        .load_balancer_arn(load_balancer_arn)
        .set_attributes(Some(aws_attributes))
        .send()
        .await
        .map_err(|e| format!("Failed to modify load balancer attributes: {}", e))?;

    let result_attributes: Vec<LoadBalancerAttribute> = response
        .attributes()
        .iter()
        .map(|attr| LoadBalancerAttribute {
            key: attr.key().unwrap_or_default().to_string(),
            value: attr.value().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(ModifyLoadBalancerAttributesOutput {
        attributes: result_attributes,
        success: true,
    })
}

/// Describe Load Balancer Attributes
pub async fn describe_load_balancer_attributes(
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<DescribeLoadBalancerAttributesOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .describe_load_balancer_attributes()
        .load_balancer_arn(load_balancer_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to describe load balancer attributes: {}", e))?;

    let attributes: Vec<LoadBalancerAttribute> = response
        .attributes()
        .iter()
        .map(|attr| LoadBalancerAttribute {
            key: attr.key().unwrap_or_default().to_string(),
            value: attr.value().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(DescribeLoadBalancerAttributesOutput { attributes })
}

/// Set Security Groups
pub async fn set_security_groups(
    load_balancer_arn: &str,
    security_groups: Vec<String>,
    region: Option<&str>,
) -> Result<SetSecurityGroupsOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .set_security_groups()
        .load_balancer_arn(load_balancer_arn)
        .set_security_groups(Some(security_groups))
        .send()
        .await
        .map_err(|e| format!("Failed to set security groups: {}", e))?;

    let security_group_ids: Vec<String> = response
        .security_group_ids()
        .iter()
        .map(|s| s.to_string())
        .collect();

    Ok(SetSecurityGroupsOutput {
        security_group_ids,
        success: true,
    })
}

/// Set Subnets
pub async fn set_subnets(
    load_balancer_arn: &str,
    ip_address_type: Option<&str>,
    region: Option<&str>,
    subnet_mappings: Option<Vec<SubnetMapping>>,
    subnets: Option<Vec<String>>,
) -> Result<SetSubnetsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.set_subnets().load_balancer_arn(load_balancer_arn);

    if let Some(ip_type) = ip_address_type {
        request = request.ip_address_type(IpAddressType::from(ip_type));
    }

    if let Some(ref mappings) = subnet_mappings {
        let aws_mappings: Vec<_> = mappings.iter().map(subnet_mapping_to_aws).collect();
        request = request.set_subnet_mappings(Some(aws_mappings));
    }

    if let Some(subs) = subnets {
        request = request.set_subnets(Some(subs));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to set subnets: {}", e))?;

    let availability_zones: Vec<AvailabilityZone> = response
        .availability_zones()
        .iter()
        .map(|az| AvailabilityZone {
            zone_name: az.zone_name().unwrap_or_default().to_string(),
            subnet_id: az.subnet_id().unwrap_or_default().to_string(),
            outpost_id: az.outpost_id().map(|s| s.to_string()),
            load_balancer_addresses: if az.load_balancer_addresses().is_empty() {
                None
            } else {
                Some(
                    az.load_balancer_addresses()
                        .iter()
                        .map(|addr| LoadBalancerAddress {
                            ip_address: addr.ip_address().map(|s| s.to_string()),
                            allocation_id: addr.allocation_id().map(|s| s.to_string()),
                            private_ipv4_address: addr.private_ipv4_address().map(|s| s.to_string()),
                            ipv6_address: addr.ipv6_address().map(|s| s.to_string()),
                        })
                        .collect(),
                )
            },
        })
        .collect();

    Ok(SetSubnetsOutput {
        availability_zones,
        ip_address_type: response
            .ip_address_type()
            .map(|t| t.as_str().to_string())
            .unwrap_or_default(),
        success: true,
    })
}

/// Set IP Address Type
pub async fn set_ip_address_type(
    ip_address_type: &str,
    load_balancer_arn: &str,
    region: Option<&str>,
) -> Result<SetIpAddressTypeOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .set_ip_address_type()
        .ip_address_type(IpAddressType::from(ip_address_type))
        .load_balancer_arn(load_balancer_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to set IP address type: {}", e))?;

    Ok(SetIpAddressTypeOutput {
        ip_address_type: response
            .ip_address_type()
            .map(|t| t.as_str().to_string())
            .unwrap_or_default(),
        success: true,
    })
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
    let client = create_client(region).await?;

    let mut request = client.create_target_group().name(name);

    if let Some(enabled) = health_check_enabled {
        request = request.health_check_enabled(enabled);
    }

    if let Some(interval) = health_check_interval_seconds {
        request = request.health_check_interval_seconds(interval);
    }

    if let Some(path) = health_check_path {
        request = request.health_check_path(path);
    }

    if let Some(hc_port) = health_check_port {
        request = request.health_check_port(hc_port);
    }

    if let Some(hc_protocol) = health_check_protocol {
        request = request.health_check_protocol(ProtocolEnum::from(hc_protocol));
    }

    if let Some(timeout) = health_check_timeout_seconds {
        request = request.health_check_timeout_seconds(timeout);
    }

    if let Some(healthy_count) = healthy_threshold_count {
        request = request.healthy_threshold_count(healthy_count);
    }

    if let Some(ip_type) = ip_address_type {
        request = request.ip_address_type(
            aws_sdk_elasticloadbalancingv2::types::TargetGroupIpAddressTypeEnum::from(ip_type),
        );
    }

    if let Some(ref m) = matcher {
        let mut matcher_builder = aws_sdk_elasticloadbalancingv2::types::Matcher::builder();
        if let Some(ref http_code) = m.http_code {
            matcher_builder = matcher_builder.http_code(http_code);
        }
        if let Some(ref grpc_code) = m.grpc_code {
            matcher_builder = matcher_builder.grpc_code(grpc_code);
        }
        request = request.matcher(matcher_builder.build());
    }

    if let Some(p) = port {
        request = request.port(p);
    }

    if let Some(proto) = protocol {
        request = request.protocol(ProtocolEnum::from(proto));
    }

    if let Some(proto_version) = protocol_version {
        request = request.protocol_version(proto_version);
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    if let Some(tt) = target_type {
        request = request.target_type(TargetTypeEnum::from(tt));
    }

    if let Some(unhealthy_count) = unhealthy_threshold_count {
        request = request.unhealthy_threshold_count(unhealthy_count);
    }

    if let Some(vpc) = vpc_id {
        request = request.vpc_id(vpc);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create target group: {}", e))?;

    let target_groups: Vec<TargetGroup> = response
        .target_groups()
        .iter()
        .map(convert_target_group)
        .collect();

    Ok(CreateTargetGroupOutput {
        target_groups,
        success: true,
    })
}

/// Delete Target Group
pub async fn delete_target_group(
    target_group_arn: &str,
    region: Option<&str>,
) -> Result<DeleteTargetGroupOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_target_group()
        .target_group_arn(target_group_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to delete target group: {}", e))?;

    Ok(DeleteTargetGroupOutput { success: true })
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
    let client = create_client(region).await?;

    let mut request = client.describe_target_groups();

    if let Some(lb_arn) = load_balancer_arn {
        request = request.load_balancer_arn(lb_arn);
    }

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(n) = names {
        request = request.set_names(Some(n));
    }

    if let Some(size) = page_size {
        request = request.page_size(size);
    }

    if let Some(arns) = target_group_arns {
        request = request.set_target_group_arns(Some(arns));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe target groups: {}", e))?;

    let target_groups: Vec<TargetGroup> = response
        .target_groups()
        .iter()
        .map(convert_target_group)
        .collect();

    Ok(DescribeTargetGroupsOutput {
        next_marker: response.next_marker().map(|s| s.to_string()),
        target_groups,
    })
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
    let client = create_client(region).await?;

    let mut request = client
        .modify_target_group()
        .target_group_arn(target_group_arn);

    if let Some(enabled) = health_check_enabled {
        request = request.health_check_enabled(enabled);
    }

    if let Some(interval) = health_check_interval_seconds {
        request = request.health_check_interval_seconds(interval);
    }

    if let Some(path) = health_check_path {
        request = request.health_check_path(path);
    }

    if let Some(hc_port) = health_check_port {
        request = request.health_check_port(hc_port);
    }

    if let Some(hc_protocol) = health_check_protocol {
        request = request.health_check_protocol(ProtocolEnum::from(hc_protocol));
    }

    if let Some(timeout) = health_check_timeout_seconds {
        request = request.health_check_timeout_seconds(timeout);
    }

    if let Some(healthy_count) = healthy_threshold_count {
        request = request.healthy_threshold_count(healthy_count);
    }

    if let Some(ref m) = matcher {
        let mut matcher_builder = aws_sdk_elasticloadbalancingv2::types::Matcher::builder();
        if let Some(ref http_code) = m.http_code {
            matcher_builder = matcher_builder.http_code(http_code);
        }
        if let Some(ref grpc_code) = m.grpc_code {
            matcher_builder = matcher_builder.grpc_code(grpc_code);
        }
        request = request.matcher(matcher_builder.build());
    }

    if let Some(unhealthy_count) = unhealthy_threshold_count {
        request = request.unhealthy_threshold_count(unhealthy_count);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to modify target group: {}", e))?;

    let target_groups: Vec<TargetGroup> = response
        .target_groups()
        .iter()
        .map(convert_target_group)
        .collect();

    Ok(ModifyTargetGroupOutput {
        target_groups,
        success: true,
    })
}

/// Modify Target Group Attributes
pub async fn modify_target_group_attributes(
    attributes: Vec<TargetGroupAttribute>,
    target_group_arn: &str,
    region: Option<&str>,
) -> Result<ModifyTargetGroupAttributesOutput, String> {
    let client = create_client(region).await?;

    let aws_attributes: Vec<_> = attributes
        .iter()
        .map(|attr| {
            aws_sdk_elasticloadbalancingv2::types::TargetGroupAttribute::builder()
                .key(&attr.key)
                .value(&attr.value)
                .build()
        })
        .collect();

    let response = client
        .modify_target_group_attributes()
        .target_group_arn(target_group_arn)
        .set_attributes(Some(aws_attributes))
        .send()
        .await
        .map_err(|e| format!("Failed to modify target group attributes: {}", e))?;

    let result_attributes: Vec<TargetGroupAttribute> = response
        .attributes()
        .iter()
        .map(|attr| TargetGroupAttribute {
            key: attr.key().unwrap_or_default().to_string(),
            value: attr.value().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(ModifyTargetGroupAttributesOutput {
        attributes: result_attributes,
        success: true,
    })
}

/// Describe Target Group Attributes
pub async fn describe_target_group_attributes(
    target_group_arn: &str,
    region: Option<&str>,
) -> Result<DescribeTargetGroupAttributesOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .describe_target_group_attributes()
        .target_group_arn(target_group_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to describe target group attributes: {}", e))?;

    let attributes: Vec<TargetGroupAttribute> = response
        .attributes()
        .iter()
        .map(|attr| TargetGroupAttribute {
            key: attr.key().unwrap_or_default().to_string(),
            value: attr.value().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(DescribeTargetGroupAttributesOutput { attributes })
}

/// Register Targets
pub async fn register_targets(
    target_group_arn: &str,
    targets: Vec<TargetDescription>,
    region: Option<&str>,
) -> Result<RegisterTargetsOutput, String> {
    let client = create_client(region).await?;

    let aws_targets: Vec<_> = targets
        .iter()
        .map(|t| {
            let mut builder = aws_sdk_elasticloadbalancingv2::types::TargetDescription::builder()
                .id(&t.id);
            if let Some(port) = t.port {
                builder = builder.port(port);
            }
            if let Some(ref az) = t.availability_zone {
                builder = builder.availability_zone(az);
            }
            builder.build()
        })
        .collect();

    client
        .register_targets()
        .target_group_arn(target_group_arn)
        .set_targets(Some(aws_targets))
        .send()
        .await
        .map_err(|e| format!("Failed to register targets: {}", e))?;

    Ok(RegisterTargetsOutput { success: true })
}

/// Deregister Targets
pub async fn deregister_targets(
    target_group_arn: &str,
    targets: Vec<TargetDescription>,
    region: Option<&str>,
) -> Result<DeregisterTargetsOutput, String> {
    let client = create_client(region).await?;

    let aws_targets: Vec<_> = targets
        .iter()
        .map(|t| {
            let mut builder = aws_sdk_elasticloadbalancingv2::types::TargetDescription::builder()
                .id(&t.id);
            if let Some(port) = t.port {
                builder = builder.port(port);
            }
            if let Some(ref az) = t.availability_zone {
                builder = builder.availability_zone(az);
            }
            builder.build()
        })
        .collect();

    client
        .deregister_targets()
        .target_group_arn(target_group_arn)
        .set_targets(Some(aws_targets))
        .send()
        .await
        .map_err(|e| format!("Failed to deregister targets: {}", e))?;

    Ok(DeregisterTargetsOutput { success: true })
}

/// Describe Target Health
pub async fn describe_target_health(
    target_group_arn: &str,
    region: Option<&str>,
    targets: Option<Vec<TargetDescription>>,
) -> Result<DescribeTargetHealthOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .describe_target_health()
        .target_group_arn(target_group_arn);

    if let Some(ref t) = targets {
        let aws_targets: Vec<_> = t
            .iter()
            .map(|target| {
                let mut builder =
                    aws_sdk_elasticloadbalancingv2::types::TargetDescription::builder()
                        .id(&target.id);
                if let Some(port) = target.port {
                    builder = builder.port(port);
                }
                if let Some(ref az) = target.availability_zone {
                    builder = builder.availability_zone(az);
                }
                builder.build()
            })
            .collect();
        request = request.set_targets(Some(aws_targets));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe target health: {}", e))?;

    let target_health_descriptions: Vec<TargetHealthDescription> = response
        .target_health_descriptions()
        .iter()
        .map(|thd| TargetHealthDescription {
            target: TargetDescription {
                id: thd
                    .target()
                    .and_then(|t| t.id.clone())
                    .unwrap_or_default(),
                port: thd.target().and_then(|t| t.port()),
                availability_zone: thd
                    .target()
                    .and_then(|t| t.availability_zone().map(|s| s.to_string())),
            },
            health_check_port: thd.health_check_port().map(|s| s.to_string()),
            target_health: thd.target_health().map(|th| TargetHealth {
                state: th.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
                reason: th.reason().map(|r| r.as_str().to_string()),
                description: th.description().map(|s| s.to_string()),
            }),
        })
        .collect();

    Ok(DescribeTargetHealthOutput {
        target_health_descriptions,
    })
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
    let client = create_client(region).await?;

    let aws_actions: Vec<_> = default_actions.iter().map(action_to_aws).collect();

    let mut request = client
        .create_listener()
        .load_balancer_arn(load_balancer_arn)
        .set_default_actions(Some(aws_actions));

    if let Some(alpn) = alpn_policy {
        request = request.set_alpn_policy(Some(alpn));
    }

    if let Some(ref certs) = certificates {
        let aws_certs: Vec<_> = certs
            .iter()
            .map(|c| {
                let mut builder =
                    aws_sdk_elasticloadbalancingv2::types::Certificate::builder();
                if let Some(ref arn) = c.certificate_arn {
                    builder = builder.certificate_arn(arn);
                }
                if let Some(is_default) = c.is_default {
                    builder = builder.is_default(is_default);
                }
                builder.build()
            })
            .collect();
        request = request.set_certificates(Some(aws_certs));
    }

    if let Some(p) = port {
        request = request.port(p);
    }

    if let Some(proto) = protocol {
        request = request.protocol(ProtocolEnum::from(proto));
    }

    if let Some(ssl) = ssl_policy {
        request = request.ssl_policy(ssl);
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create listener: {}", e))?;

    let listeners: Vec<Listener> = response
        .listeners()
        .iter()
        .map(convert_listener)
        .collect();

    Ok(CreateListenerOutput {
        listeners,
        success: true,
    })
}

/// Delete Listener
pub async fn delete_listener(
    listener_arn: &str,
    region: Option<&str>,
) -> Result<DeleteListenerOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_listener()
        .listener_arn(listener_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to delete listener: {}", e))?;

    Ok(DeleteListenerOutput { success: true })
}

/// Describe Listeners
pub async fn describe_listeners(
    listener_arns: Option<Vec<String>>,
    load_balancer_arn: Option<&str>,
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeListenersOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_listeners();

    if let Some(arns) = listener_arns {
        request = request.set_listener_arns(Some(arns));
    }

    if let Some(lb_arn) = load_balancer_arn {
        request = request.load_balancer_arn(lb_arn);
    }

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(size) = page_size {
        request = request.page_size(size);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe listeners: {}", e))?;

    let listeners: Vec<Listener> = response
        .listeners()
        .iter()
        .map(convert_listener)
        .collect();

    Ok(DescribeListenersOutput {
        listeners,
        next_marker: response.next_marker().map(|s| s.to_string()),
    })
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
    let client = create_client(region).await?;

    let mut request = client.modify_listener().listener_arn(listener_arn);

    if let Some(alpn) = alpn_policy {
        request = request.set_alpn_policy(Some(alpn));
    }

    if let Some(ref certs) = certificates {
        let aws_certs: Vec<_> = certs
            .iter()
            .map(|c| {
                let mut builder =
                    aws_sdk_elasticloadbalancingv2::types::Certificate::builder();
                if let Some(ref arn) = c.certificate_arn {
                    builder = builder.certificate_arn(arn);
                }
                if let Some(is_default) = c.is_default {
                    builder = builder.is_default(is_default);
                }
                builder.build()
            })
            .collect();
        request = request.set_certificates(Some(aws_certs));
    }

    if let Some(ref actions) = default_actions {
        let aws_actions: Vec<_> = actions.iter().map(action_to_aws).collect();
        request = request.set_default_actions(Some(aws_actions));
    }

    if let Some(p) = port {
        request = request.port(p);
    }

    if let Some(proto) = protocol {
        request = request.protocol(ProtocolEnum::from(proto));
    }

    if let Some(ssl) = ssl_policy {
        request = request.ssl_policy(ssl);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to modify listener: {}", e))?;

    let listeners: Vec<Listener> = response
        .listeners()
        .iter()
        .map(convert_listener)
        .collect();

    Ok(ModifyListenerOutput {
        listeners,
        success: true,
    })
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
    let client = create_client(region).await?;

    let aws_actions: Vec<_> = actions.iter().map(action_to_aws).collect();

    let aws_conditions: Vec<_> = conditions.iter().map(rule_condition_to_aws).collect();

    let mut request = client
        .create_rule()
        .listener_arn(listener_arn)
        .priority(priority)
        .set_actions(Some(aws_actions))
        .set_conditions(Some(aws_conditions));

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create rule: {}", e))?;

    let rules: Vec<Rule> = response.rules().iter().map(convert_rule).collect();

    Ok(CreateRuleOutput {
        rules,
        success: true,
    })
}

/// Delete Rule
pub async fn delete_rule(
    rule_arn: &str,
    region: Option<&str>,
) -> Result<DeleteRuleOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_rule()
        .rule_arn(rule_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to delete rule: {}", e))?;

    Ok(DeleteRuleOutput { success: true })
}

/// Describe Rules
pub async fn describe_rules(
    listener_arn: Option<&str>,
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
    rule_arns: Option<Vec<String>>,
) -> Result<DescribeRulesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_rules();

    if let Some(arn) = listener_arn {
        request = request.listener_arn(arn);
    }

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(size) = page_size {
        request = request.page_size(size);
    }

    if let Some(arns) = rule_arns {
        request = request.set_rule_arns(Some(arns));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe rules: {}", e))?;

    let rules: Vec<Rule> = response.rules().iter().map(convert_rule).collect();

    Ok(DescribeRulesOutput {
        next_marker: response.next_marker().map(|s| s.to_string()),
        rules,
    })
}

/// Modify Rule
pub async fn modify_rule(
    rule_arn: &str,
    actions: Option<Vec<Action>>,
    conditions: Option<Vec<RuleCondition>>,
    region: Option<&str>,
) -> Result<ModifyRuleOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.modify_rule().rule_arn(rule_arn);

    if let Some(ref a) = actions {
        let aws_actions: Vec<_> = a.iter().map(action_to_aws).collect();
        request = request.set_actions(Some(aws_actions));
    }

    if let Some(ref c) = conditions {
        let aws_conditions: Vec<_> = c.iter().map(rule_condition_to_aws).collect();
        request = request.set_conditions(Some(aws_conditions));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to modify rule: {}", e))?;

    let rules: Vec<Rule> = response.rules().iter().map(convert_rule).collect();

    Ok(ModifyRuleOutput {
        rules,
        success: true,
    })
}

/// Set Rule Priorities
pub async fn set_rule_priorities(
    rule_priorities: Vec<RulePriorityPair>,
    region: Option<&str>,
) -> Result<SetRulePrioritiesOutput, String> {
    let client = create_client(region).await?;

    let aws_priorities: Vec<_> = rule_priorities
        .iter()
        .map(|rp| {
            aws_sdk_elasticloadbalancingv2::types::RulePriorityPair::builder()
                .rule_arn(&rp.rule_arn)
                .priority(rp.priority)
                .build()
        })
        .collect();

    let response = client
        .set_rule_priorities()
        .set_rule_priorities(Some(aws_priorities))
        .send()
        .await
        .map_err(|e| format!("Failed to set rule priorities: {}", e))?;

    let rules: Vec<Rule> = response.rules().iter().map(convert_rule).collect();

    Ok(SetRulePrioritiesOutput {
        rules,
        success: true,
    })
}

/// Add Listener Certificates
pub async fn add_listener_certificates(
    certificates: Vec<Certificate>,
    listener_arn: &str,
    region: Option<&str>,
) -> Result<AddListenerCertificatesOutput, String> {
    let client = create_client(region).await?;

    let aws_certs: Vec<_> = certificates
        .iter()
        .map(|c| {
            let mut builder = aws_sdk_elasticloadbalancingv2::types::Certificate::builder();
            if let Some(ref arn) = c.certificate_arn {
                builder = builder.certificate_arn(arn);
            }
            if let Some(is_default) = c.is_default {
                builder = builder.is_default(is_default);
            }
            builder.build()
        })
        .collect();

    let response = client
        .add_listener_certificates()
        .listener_arn(listener_arn)
        .set_certificates(Some(aws_certs))
        .send()
        .await
        .map_err(|e| format!("Failed to add listener certificates: {}", e))?;

    let result_certs: Vec<Certificate> = response
        .certificates()
        .iter()
        .map(|c| Certificate {
            certificate_arn: c.certificate_arn().map(|s| s.to_string()),
            is_default: c.is_default(),
        })
        .collect();

    Ok(AddListenerCertificatesOutput {
        certificates: result_certs,
        success: true,
    })
}

/// Remove Listener Certificates
pub async fn remove_listener_certificates(
    certificates: Vec<Certificate>,
    listener_arn: &str,
    region: Option<&str>,
) -> Result<RemoveListenerCertificatesOutput, String> {
    let client = create_client(region).await?;

    let aws_certs: Vec<_> = certificates
        .iter()
        .map(|c| {
            let mut builder = aws_sdk_elasticloadbalancingv2::types::Certificate::builder();
            if let Some(ref arn) = c.certificate_arn {
                builder = builder.certificate_arn(arn);
            }
            if let Some(is_default) = c.is_default {
                builder = builder.is_default(is_default);
            }
            builder.build()
        })
        .collect();

    client
        .remove_listener_certificates()
        .listener_arn(listener_arn)
        .set_certificates(Some(aws_certs))
        .send()
        .await
        .map_err(|e| format!("Failed to remove listener certificates: {}", e))?;

    Ok(RemoveListenerCertificatesOutput { success: true })
}

/// Describe Listener Certificates
pub async fn describe_listener_certificates(
    listener_arn: &str,
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeListenerCertificatesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .describe_listener_certificates()
        .listener_arn(listener_arn);

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(size) = page_size {
        request = request.page_size(size);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe listener certificates: {}", e))?;

    let certificates: Vec<Certificate> = response
        .certificates()
        .iter()
        .map(|c| Certificate {
            certificate_arn: c.certificate_arn().map(|s| s.to_string()),
            is_default: c.is_default(),
        })
        .collect();

    Ok(DescribeListenerCertificatesOutput {
        certificates,
        next_marker: response.next_marker().map(|s| s.to_string()),
    })
}

/// Describe SSL Policies
pub async fn describe_ssl_policies(
    load_balancer_type: Option<&str>,
    marker: Option<&str>,
    names: Option<Vec<String>>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeSslPoliciesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_ssl_policies();

    if let Some(lb_type) = load_balancer_type {
        request = request.load_balancer_type(LoadBalancerTypeEnum::from(lb_type));
    }

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(n) = names {
        request = request.set_names(Some(n));
    }

    if let Some(size) = page_size {
        request = request.page_size(size);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe SSL policies: {}", e))?;

    let ssl_policies: Vec<SslPolicy> = response
        .ssl_policies()
        .iter()
        .map(|p| SslPolicy {
            ssl_protocols: p.ssl_protocols().iter().map(|s| s.to_string()).collect(),
            ciphers: p
                .ciphers()
                .iter()
                .map(|c| Cipher {
                    name: c.name().unwrap_or_default().to_string(),
                    priority: c.priority().unwrap_or(0),
                })
                .collect(),
            name: p.name().unwrap_or_default().to_string(),
            supported_load_balancer_types: if p.supported_load_balancer_types().is_empty() {
                None
            } else {
                Some(
                    p.supported_load_balancer_types()
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                )
            },
        })
        .collect();

    Ok(DescribeSslPoliciesOutput {
        next_marker: response.next_marker().map(|s| s.to_string()),
        ssl_policies,
    })
}

/// Describe Account Limits
pub async fn describe_account_limits(
    marker: Option<&str>,
    page_size: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeAccountLimitsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_account_limits();

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(size) = page_size {
        request = request.page_size(size);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe account limits: {}", e))?;

    let limits: Vec<Limit> = response
        .limits()
        .iter()
        .map(|l| Limit {
            name: l.name().unwrap_or_default().to_string(),
            max: l.max().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(DescribeAccountLimitsOutput {
        limits,
        next_marker: response.next_marker().map(|s| s.to_string()),
    })
}

/// Add Tags
pub async fn add_tags(
    resource_arns: Vec<String>,
    tags: ElbTags,
    region: Option<&str>,
) -> Result<AddTagsOutput, String> {
    let client = create_client(region).await?;

    client
        .add_tags()
        .set_resource_arns(Some(resource_arns))
        .set_tags(Some(tags_to_aws_tags(&tags)))
        .send()
        .await
        .map_err(|e| format!("Failed to add tags: {}", e))?;

    Ok(AddTagsOutput { success: true })
}

/// Remove Tags
pub async fn remove_tags(
    resource_arns: Vec<String>,
    tag_keys: Vec<String>,
    region: Option<&str>,
) -> Result<RemoveTagsOutput, String> {
    let client = create_client(region).await?;

    client
        .remove_tags()
        .set_resource_arns(Some(resource_arns))
        .set_tag_keys(Some(tag_keys))
        .send()
        .await
        .map_err(|e| format!("Failed to remove tags: {}", e))?;

    Ok(RemoveTagsOutput { success: true })
}

/// Describe Tags
pub async fn describe_tags(
    resource_arns: Vec<String>,
    region: Option<&str>,
) -> Result<DescribeTagsOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .describe_tags()
        .set_resource_arns(Some(resource_arns))
        .send()
        .await
        .map_err(|e| format!("Failed to describe tags: {}", e))?;

    let tag_descriptions: Vec<TagDescription> = response
        .tag_descriptions()
        .iter()
        .map(|td| TagDescription {
            resource_arn: td.resource_arn().unwrap_or_default().to_string(),
            tags: td
                .tags()
                .iter()
                .map(|t| Tag {
                    key: t.key().unwrap_or_default().to_string(),
                    value: t.value().map(|s| s.to_string()),
                })
                .collect(),
        })
        .collect();

    Ok(DescribeTagsOutput { tag_descriptions })
}

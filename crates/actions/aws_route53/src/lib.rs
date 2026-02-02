//! Harana Actions - AWS Route53 Module
//!
//! This module provides AWS Route 53 actions for managing DNS zones,
//! resource record sets, and health checks.

/// Output types for AWS Route53 actions
pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_route53::{
    config::Region,
    types::{
        self as route53_types, AliasTarget as AwsAliasTarget, Change as AwsChange,
        ChangeBatch as AwsChangeBatch, ChangeAction, CidrRoutingConfig as AwsCidrRoutingConfig,
        GeoLocation as AwsGeoLocation, HealthCheckConfig as AwsHealthCheckConfig,
        HealthCheckType, HostedZoneConfig as AwsHostedZoneConfig,
        InsufficientDataHealthStatus, ResourceRecord as AwsResourceRecord,
        ResourceRecordSet as AwsResourceRecordSet, RrType, Vpc as AwsVpc, VpcRegion,
    },
    Client,
};
use chrono::{DateTime, Utc};
use output::*;

/// Creates a Route53 client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let route53_config = if let Some(region_str) = region {
        aws_sdk_route53::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_route53::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(route53_config))
}

/// Convert AWS SDK HostedZone to output type
fn convert_hosted_zone(hz: &route53_types::HostedZone) -> HostedZone {
    HostedZone {
        id: hz.id().to_string(),
        name: hz.name().to_string(),
        caller_reference: hz.caller_reference().to_string(),
        config: hz.config().map(|c| HostedZoneConfig {
            comment: c.comment().map(|s| s.to_string()),
            private_zone: c.private_zone(),
        }),
        resource_record_set_count: hz.resource_record_set_count(),
        linked_service: hz.linked_service().map(|ls| LinkedService {
            service_principal: ls.service_principal().map(|s| s.to_string()),
            description: ls.description().map(|s| s.to_string()),
        }),
    }
}

/// Convert AWS SDK DelegationSet to output type
fn convert_delegation_set(ds: &route53_types::DelegationSet) -> DelegationSet {
    DelegationSet {
        id: ds.id().map(|s| s.to_string()),
        caller_reference: ds.caller_reference().map(|s| s.to_string()),
        name_servers: ds.name_servers().iter().map(|s| s.to_string()).collect(),
    }
}

/// Convert AWS SDK VPC to output type
fn convert_vpc(vpc: &route53_types::Vpc) -> VPC {
    VPC {
        vpc_region: vpc.vpc_region().map(|r| r.as_str().to_string()),
        vpc_id: vpc.vpc_id().map(|s| s.to_string()),
    }
}

/// Convert AWS SDK ChangeInfo to output type
fn convert_change_info(ci: &route53_types::ChangeInfo) -> ChangeInfo {
    ChangeInfo {
        id: ci.id().to_string(),
        status: ci.status().as_str().to_string(),
        submitted_at: DateTime::<Utc>::from_timestamp(
            ci.submitted_at().secs(),
            ci.submitted_at().subsec_nanos(),
        )
        .unwrap_or_default(),
        comment: ci.comment().map(|s| s.to_string()),
    }
}

/// Convert AWS SDK ResourceRecordSet to output type
fn convert_resource_record_set(rrs: &route53_types::ResourceRecordSet) -> ResourceRecordSet {
    let resource_records = if rrs.resource_records().is_empty() {
        None
    } else {
        Some(
            rrs.resource_records()
                .iter()
                .map(|r| ResourceRecord {
                    value: r.value().to_string(),
                })
                .collect(),
        )
    };

    ResourceRecordSet {
        name: rrs.name().to_string(),
        record_type: rrs.r#type().as_str().to_string(),
        set_identifier: rrs.set_identifier().map(|s| s.to_string()),
        weight: rrs.weight(),
        region: rrs.region().map(|r| r.as_str().to_string()),
        geo_location: rrs.geo_location().map(|gl| GeoLocation {
            continent_code: gl.continent_code().map(|s| s.to_string()),
            country_code: gl.country_code().map(|s| s.to_string()),
            subdivision_code: gl.subdivision_code().map(|s| s.to_string()),
        }),
        failover: rrs.failover().map(|f| f.as_str().to_string()),
        multi_value_answer: rrs.multi_value_answer(),
        ttl: rrs.ttl(),
        resource_records,
        alias_target: rrs.alias_target().map(|at| AliasTarget {
            hosted_zone_id: at.hosted_zone_id().to_string(),
            dns_name: at.dns_name().to_string(),
            evaluate_target_health: at.evaluate_target_health(),
        }),
        health_check_id: rrs.health_check_id().map(|s| s.to_string()),
        traffic_policy_instance_id: rrs.traffic_policy_instance_id().map(|s| s.to_string()),
        cidr_routing_config: rrs.cidr_routing_config().map(|crc| CidrRoutingConfig {
            collection_id: crc.collection_id().to_string(),
            location_name: crc.location_name().to_string(),
        }),
    }
}

/// Convert AWS SDK HealthCheck to output type
fn convert_health_check(hc: &route53_types::HealthCheck) -> HealthCheck {
    let config = hc.health_check_config();

    let health_check_config = match config {
        Some(cfg) => HealthCheckConfig {
            ip_address: cfg.ip_address().map(|s| s.to_string()),
            port: cfg.port(),
            health_check_type: cfg.r#type().as_str().to_string(),
            resource_path: cfg.resource_path().map(|s| s.to_string()),
            fully_qualified_domain_name: cfg.fully_qualified_domain_name().map(|s| s.to_string()),
            search_string: cfg.search_string().map(|s| s.to_string()),
            request_interval: cfg.request_interval(),
            failure_threshold: cfg.failure_threshold(),
            measure_latency: cfg.measure_latency(),
            inverted: cfg.inverted(),
            disabled: cfg.disabled(),
            health_threshold: cfg.health_threshold(),
            child_health_checks: if cfg.child_health_checks().is_empty() {
                None
            } else {
                Some(cfg.child_health_checks().iter().map(|s| s.to_string()).collect())
            },
            enable_sni: cfg.enable_sni(),
            regions: if cfg.regions().is_empty() {
                None
            } else {
                Some(cfg.regions().iter().map(|r| r.as_str().to_string()).collect())
            },
            alarm_identifier: cfg.alarm_identifier().map(|ai| AlarmIdentifier {
                region: ai.region().as_str().to_string(),
                name: ai.name().to_string(),
            }),
            insufficient_data_health_status: cfg
                .insufficient_data_health_status()
                .map(|s| s.as_str().to_string()),
            routing_control_arn: cfg.routing_control_arn().map(|s| s.to_string()),
        },
        None => HealthCheckConfig {
            ip_address: None,
            port: None,
            health_check_type: String::new(),
            resource_path: None,
            fully_qualified_domain_name: None,
            search_string: None,
            request_interval: None,
            failure_threshold: None,
            measure_latency: None,
            inverted: None,
            disabled: None,
            health_threshold: None,
            child_health_checks: None,
            enable_sni: None,
            regions: None,
            alarm_identifier: None,
            insufficient_data_health_status: None,
            routing_control_arn: None,
        },
    };

    HealthCheck {
        id: hc.id().to_string(),
        caller_reference: hc.caller_reference().to_string(),
        linked_service: hc.linked_service().map(|ls| LinkedService {
            service_principal: ls.service_principal().map(|s| s.to_string()),
            description: ls.description().map(|s| s.to_string()),
        }),
        health_check_config,
        health_check_version: hc.health_check_version(),
        cloud_watch_alarm_configuration: hc.cloud_watch_alarm_configuration().map(|cwac| {
            CloudWatchAlarmConfiguration {
                evaluation_periods: cwac.evaluation_periods(),
                threshold: cwac.threshold(),
                comparison_operator: cwac.comparison_operator().as_str().to_string(),
                period: cwac.period(),
                metric_name: cwac.metric_name().to_string(),
                namespace: cwac.namespace().to_string(),
                statistic: cwac.statistic().as_str().to_string(),
                dimensions: if cwac.dimensions().is_empty() {
                    None
                } else {
                    Some(
                        cwac.dimensions()
                            .iter()
                            .map(|d| Dimension {
                                name: d.name().to_string(),
                                value: d.value().to_string(),
                            })
                            .collect(),
                    )
                },
            }
        }),
    }
}

/// Convert output VPC to AWS SDK VPC type
fn vpc_to_aws(vpc: &VPC) -> Result<AwsVpc, String> {
    let mut builder = AwsVpc::builder();

    if let Some(ref region) = vpc.vpc_region {
        builder = builder.vpc_region(VpcRegion::from(region.as_str()));
    }

    if let Some(ref id) = vpc.vpc_id {
        builder = builder.vpc_id(id);
    }

    Ok(builder.build())
}

/// Convert output HostedZoneConfig to AWS SDK HostedZoneConfig type
fn hosted_zone_config_to_aws(config: &HostedZoneConfig) -> AwsHostedZoneConfig {
    let mut builder = AwsHostedZoneConfig::builder();

    if let Some(ref comment) = config.comment {
        builder = builder.comment(comment);
    }

    builder = builder.private_zone(config.private_zone);

    builder.build()
}

/// Convert output ChangeBatch to AWS SDK ChangeBatch type
fn change_batch_to_aws(batch: &ChangeBatch) -> Result<AwsChangeBatch, String> {
    let mut builder = AwsChangeBatch::builder();

    if let Some(ref comment) = batch.comment {
        builder = builder.comment(comment);
    }

    for change in &batch.changes {
        let aws_change = change_to_aws(change)?;
        builder = builder.changes(aws_change);
    }

    builder
        .build()
        .map_err(|e| format!("Failed to build change batch: {}", e))
}

/// Convert output Change to AWS SDK Change type
fn change_to_aws(change: &Change) -> Result<AwsChange, String> {
    let action = match change.action.to_uppercase().as_str() {
        "CREATE" => ChangeAction::Create,
        "DELETE" => ChangeAction::Delete,
        "UPSERT" => ChangeAction::Upsert,
        _ => return Err(format!("Invalid change action: {}", change.action)),
    };

    let rrs = resource_record_set_to_aws(&change.resource_record_set)?;

    AwsChange::builder()
        .action(action)
        .resource_record_set(rrs)
        .build()
        .map_err(|e| format!("Failed to build change: {}", e))
}

/// Convert output ResourceRecordSet to AWS SDK ResourceRecordSet type
fn resource_record_set_to_aws(rrs: &ResourceRecordSet) -> Result<AwsResourceRecordSet, String> {
    let record_type = RrType::from(rrs.record_type.as_str());

    let mut builder = AwsResourceRecordSet::builder()
        .name(&rrs.name)
        .r#type(record_type);

    if let Some(ref set_id) = rrs.set_identifier {
        builder = builder.set_identifier(set_id);
    }

    if let Some(weight) = rrs.weight {
        builder = builder.weight(weight);
    }

    if let Some(ref region) = rrs.region {
        builder = builder.region(route53_types::ResourceRecordSetRegion::from(region.as_str()));
    }

    if let Some(ref geo) = rrs.geo_location {
        let mut geo_builder = AwsGeoLocation::builder();
        if let Some(ref cc) = geo.continent_code {
            geo_builder = geo_builder.continent_code(cc);
        }
        if let Some(ref country) = geo.country_code {
            geo_builder = geo_builder.country_code(country);
        }
        if let Some(ref sub) = geo.subdivision_code {
            geo_builder = geo_builder.subdivision_code(sub);
        }
        builder = builder.geo_location(geo_builder.build());
    }

    if let Some(ref failover) = rrs.failover {
        builder =
            builder.failover(route53_types::ResourceRecordSetFailover::from(failover.as_str()));
    }

    if let Some(mva) = rrs.multi_value_answer {
        builder = builder.multi_value_answer(mva);
    }

    if let Some(ttl) = rrs.ttl {
        builder = builder.ttl(ttl);
    }

    if let Some(ref records) = rrs.resource_records {
        for record in records {
            let aws_record = AwsResourceRecord::builder()
                .value(&record.value)
                .build()
                .map_err(|e| format!("Failed to build resource record: {}", e))?;
            builder = builder.resource_records(aws_record);
        }
    }

    if let Some(ref alias) = rrs.alias_target {
        let aws_alias = AwsAliasTarget::builder()
            .hosted_zone_id(&alias.hosted_zone_id)
            .dns_name(&alias.dns_name)
            .evaluate_target_health(alias.evaluate_target_health)
            .build()
            .map_err(|e| format!("Failed to build alias target: {}", e))?;
        builder = builder.alias_target(aws_alias);
    }

    if let Some(ref hc_id) = rrs.health_check_id {
        builder = builder.health_check_id(hc_id);
    }

    if let Some(ref cidr) = rrs.cidr_routing_config {
        let aws_cidr = AwsCidrRoutingConfig::builder()
            .collection_id(&cidr.collection_id)
            .location_name(&cidr.location_name)
            .build()
            .map_err(|e| format!("Failed to build CIDR routing config: {}", e))?;
        builder = builder.cidr_routing_config(aws_cidr);
    }

    builder
        .build()
        .map_err(|e| format!("Failed to build resource record set: {}", e))
}

/// Convert output HealthCheckConfig to AWS SDK HealthCheckConfig type
fn health_check_config_to_aws(config: &HealthCheckConfig) -> Result<AwsHealthCheckConfig, String> {
    let hc_type = HealthCheckType::from(config.health_check_type.as_str());

    let mut builder = AwsHealthCheckConfig::builder().r#type(hc_type);

    if let Some(ref ip) = config.ip_address {
        builder = builder.ip_address(ip);
    }

    if let Some(port) = config.port {
        builder = builder.port(port);
    }

    if let Some(ref path) = config.resource_path {
        builder = builder.resource_path(path);
    }

    if let Some(ref fqdn) = config.fully_qualified_domain_name {
        builder = builder.fully_qualified_domain_name(fqdn);
    }

    if let Some(ref search) = config.search_string {
        builder = builder.search_string(search);
    }

    if let Some(interval) = config.request_interval {
        builder = builder.request_interval(interval);
    }

    if let Some(threshold) = config.failure_threshold {
        builder = builder.failure_threshold(threshold);
    }

    if let Some(measure) = config.measure_latency {
        builder = builder.measure_latency(measure);
    }

    if let Some(inv) = config.inverted {
        builder = builder.inverted(inv);
    }

    if let Some(disabled) = config.disabled {
        builder = builder.disabled(disabled);
    }

    if let Some(health_threshold) = config.health_threshold {
        builder = builder.health_threshold(health_threshold);
    }

    if let Some(ref children) = config.child_health_checks {
        builder = builder.set_child_health_checks(Some(children.clone()));
    }

    if let Some(sni) = config.enable_sni {
        builder = builder.enable_sni(sni);
    }

    if let Some(ref regions) = config.regions {
        let aws_regions: Vec<route53_types::HealthCheckRegion> = regions
            .iter()
            .map(|r| route53_types::HealthCheckRegion::from(r.as_str()))
            .collect();
        builder = builder.set_regions(Some(aws_regions));
    }

    if let Some(ref status) = config.insufficient_data_health_status {
        builder =
            builder.insufficient_data_health_status(InsufficientDataHealthStatus::from(status.as_str()));
    }

    if let Some(ref arn) = config.routing_control_arn {
        builder = builder.routing_control_arn(arn);
    }

    builder
        .build()
        .map_err(|e| format!("Failed to build health check config: {}", e))
}

// ==================== Hosted Zone Operations ====================

/// Create Hosted Zone
///
/// Creates a new public or private hosted zone.
///
pub async fn create_hosted_zone(
    name: &str,
    caller_reference: &str,
    delegation_set_id: Option<&str>,
    hosted_zone_config: Option<HostedZoneConfig>,
    vpc: Option<VPC>,
    region: Option<&str>,
) -> Result<CreateHostedZoneOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .create_hosted_zone()
        .name(name)
        .caller_reference(caller_reference);

    if let Some(ds_id) = delegation_set_id {
        request = request.delegation_set_id(ds_id);
    }

    if let Some(ref config) = hosted_zone_config {
        request = request.hosted_zone_config(hosted_zone_config_to_aws(config));
    }

    if let Some(ref v) = vpc {
        request = request.vpc(vpc_to_aws(v)?);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create hosted zone: {}", e))?;

    let change_info = response
        .change_info()
        .ok_or("No change info in response")?;
    let hosted_zone = response
        .hosted_zone()
        .ok_or("No hosted zone in response")?;

    Ok(CreateHostedZoneOutput {
        change_info: convert_change_info(change_info),
        delegation_set: response.delegation_set().map(convert_delegation_set),
        hosted_zone: convert_hosted_zone(hosted_zone),
        location: response.location().to_string(),
        success: true,
        vpc: response.vpc().map(convert_vpc),
    })
}

/// Delete Hosted Zone
///
/// Deletes a hosted zone.
///
pub async fn delete_hosted_zone(id: &str, region: Option<&str>) -> Result<DeleteHostedZoneOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .delete_hosted_zone()
        .id(id)
        .send()
        .await
        .map_err(|e| format!("Failed to delete hosted zone: {}", e))?;

    let change_info = response
        .change_info()
        .ok_or("No change info in response")?;

    Ok(DeleteHostedZoneOutput {
        change_info: convert_change_info(change_info),
        success: true,
    })
}

/// Get Hosted Zone
///
/// Gets information about a specified hosted zone.
///
pub async fn get_hosted_zone(id: &str, region: Option<&str>) -> Result<GetHostedZoneOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_hosted_zone()
        .id(id)
        .send()
        .await
        .map_err(|e| format!("Failed to get hosted zone: {}", e))?;

    let hosted_zone = response
        .hosted_zone()
        .ok_or("No hosted zone in response")?;

    Ok(GetHostedZoneOutput {
        delegation_set: response.delegation_set().map(convert_delegation_set),
        hosted_zone: convert_hosted_zone(hosted_zone),
        vpcs: if response.vpcs().is_empty() {
            None
        } else {
            Some(response.vpcs().iter().map(convert_vpc).collect())
        },
    })
}

/// List Hosted Zones
///
/// Retrieves a list of the public and private hosted zones.
///
pub async fn list_hosted_zones(
    delegation_set_id: Option<&str>,
    hosted_zone_type: Option<&str>,
    marker: Option<&str>,
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListHostedZonesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_hosted_zones();

    if let Some(ds_id) = delegation_set_id {
        request = request.delegation_set_id(ds_id);
    }

    if let Some(hz_type) = hosted_zone_type {
        request = request.hosted_zone_type(route53_types::HostedZoneType::from(hz_type));
    }

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list hosted zones: {}", e))?;

    Ok(ListHostedZonesOutput {
        hosted_zones: response
            .hosted_zones()
            .iter()
            .map(convert_hosted_zone)
            .collect(),
        is_truncated: response.is_truncated(),
        marker: {
            let m = response.marker();
            if m.is_empty() { None } else { Some(m.to_string()) }
        },
        max_items: response.max_items(),
        next_marker: response.next_marker().map(|s| s.to_string()),
    })
}

/// List Hosted Zones By Name
///
/// Retrieves a list of hosted zones in lexicographic order.
///
pub async fn list_hosted_zones_by_name(
    dns_name: Option<&str>,
    hosted_zone_id: Option<&str>,
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListHostedZonesByNameOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_hosted_zones_by_name();

    if let Some(name) = dns_name {
        request = request.dns_name(name);
    }

    if let Some(hz_id) = hosted_zone_id {
        request = request.hosted_zone_id(hz_id);
    }

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list hosted zones by name: {}", e))?;

    Ok(ListHostedZonesByNameOutput {
        dns_name: response.dns_name().map(|s| s.to_string()),
        hosted_zone_id: response.hosted_zone_id().map(|s| s.to_string()),
        hosted_zones: response
            .hosted_zones()
            .iter()
            .map(convert_hosted_zone)
            .collect(),
        is_truncated: response.is_truncated(),
        max_items: response.max_items(),
        next_dns_name: response.next_dns_name().map(|s| s.to_string()),
        next_hosted_zone_id: response.next_hosted_zone_id().map(|s| s.to_string()),
    })
}

/// List Hosted Zones By VPC
///
/// Lists all the private hosted zones that a VPC is associated with.
///
pub async fn list_hosted_zones_by_vpc(
    vpc_id: &str,
    vpc_region: &str,
    max_items: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListHostedZonesByVpcOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .list_hosted_zones_by_vpc()
        .vpc_id(vpc_id)
        .vpc_region(VpcRegion::from(vpc_region));

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list hosted zones by VPC: {}", e))?;

    Ok(ListHostedZonesByVpcOutput {
        hosted_zone_summaries: response
            .hosted_zone_summaries()
            .iter()
            .map(|hzs| HostedZoneSummary {
                hosted_zone_id: hzs.hosted_zone_id().to_string(),
                name: hzs.name().to_string(),
                owner: hzs.owner().map(|o| HostedZoneOwner {
                    owning_account: o.owning_account().map(|s| s.to_string()),
                    owning_service: o.owning_service().map(|s| s.to_string()),
                }),
            })
            .collect(),
        max_items: response.max_items(),
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Update Hosted Zone Comment
///
/// Updates the comment for a specified hosted zone.
///
pub async fn update_hosted_zone_comment(
    id: &str,
    comment: Option<&str>,
    region: Option<&str>,
) -> Result<UpdateHostedZoneCommentOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.update_hosted_zone_comment().id(id);

    if let Some(c) = comment {
        request = request.comment(c);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to update hosted zone comment: {}", e))?;

    let hosted_zone = response
        .hosted_zone()
        .ok_or("No hosted zone in response")?;

    Ok(UpdateHostedZoneCommentOutput {
        hosted_zone: convert_hosted_zone(hosted_zone),
        success: true,
    })
}

/// Get Hosted Zone Count
///
/// Retrieves the number of hosted zones that are associated with the current AWS account.
///
pub async fn get_hosted_zone_count(region: Option<&str>) -> Result<GetHostedZoneCountOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_hosted_zone_count()
        .send()
        .await
        .map_err(|e| format!("Failed to get hosted zone count: {}", e))?;

    Ok(GetHostedZoneCountOutput {
        hosted_zone_count: response.hosted_zone_count(),
    })
}

// ==================== Resource Record Set Operations ====================

/// Change Resource Record Sets
///
/// Creates, changes, or deletes a resource record set.
///
pub async fn change_resource_record_sets(
    hosted_zone_id: &str,
    change_batch: ChangeBatch,
    region: Option<&str>,
) -> Result<ChangeResourceRecordSetsOutput, String> {
    let client = create_client(region).await?;

    let aws_change_batch = change_batch_to_aws(&change_batch)?;

    let response = client
        .change_resource_record_sets()
        .hosted_zone_id(hosted_zone_id)
        .change_batch(aws_change_batch)
        .send()
        .await
        .map_err(|e| format!("Failed to change resource record sets: {}", e))?;

    let change_info = response
        .change_info()
        .ok_or("No change info in response")?;

    Ok(ChangeResourceRecordSetsOutput {
        change_info: convert_change_info(change_info),
        success: true,
    })
}

/// List Resource Record Sets
///
/// Lists the resource record sets in a specified hosted zone.
///
pub async fn list_resource_record_sets(
    hosted_zone_id: &str,
    max_items: Option<i32>,
    start_record_identifier: Option<&str>,
    start_record_name: Option<&str>,
    start_record_type: Option<&str>,
    region: Option<&str>,
) -> Result<ListResourceRecordSetsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .list_resource_record_sets()
        .hosted_zone_id(hosted_zone_id);

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    if let Some(id) = start_record_identifier {
        request = request.start_record_identifier(id);
    }

    if let Some(name) = start_record_name {
        request = request.start_record_name(name);
    }

    if let Some(rtype) = start_record_type {
        request = request.start_record_type(RrType::from(rtype));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list resource record sets: {}", e))?;

    Ok(ListResourceRecordSetsOutput {
        is_truncated: response.is_truncated(),
        max_items: response.max_items(),
        next_record_identifier: response.next_record_identifier().map(|s| s.to_string()),
        next_record_name: response.next_record_name().map(|s| s.to_string()),
        next_record_type: response.next_record_type().map(|t| t.as_str().to_string()),
        resource_record_sets: response
            .resource_record_sets()
            .iter()
            .map(convert_resource_record_set)
            .collect(),
    })
}

/// Get Change
///
/// Returns the current status of a change batch request.
///
pub async fn get_change(id: &str, region: Option<&str>) -> Result<GetChangeOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_change()
        .id(id)
        .send()
        .await
        .map_err(|e| format!("Failed to get change: {}", e))?;

    let change_info = response
        .change_info()
        .ok_or("No change info in response")?;

    Ok(GetChangeOutput {
        change_info: convert_change_info(change_info),
    })
}

// ==================== Health Check Operations ====================

/// Create Health Check
///
/// Creates a new health check.
///
pub async fn create_health_check(
    caller_reference: &str,
    health_check_config: HealthCheckConfig,
    region: Option<&str>,
) -> Result<CreateHealthCheckOutput, String> {
    let client = create_client(region).await?;

    let aws_config = health_check_config_to_aws(&health_check_config)?;

    let response = client
        .create_health_check()
        .caller_reference(caller_reference)
        .health_check_config(aws_config)
        .send()
        .await
        .map_err(|e| format!("Failed to create health check: {}", e))?;

    let health_check = response
        .health_check()
        .ok_or("No health check in response")?;

    Ok(CreateHealthCheckOutput {
        health_check: convert_health_check(health_check),
        location: response.location().to_string(),
        success: true,
    })
}

/// Delete Health Check
///
/// Deletes a health check.
///
pub async fn delete_health_check(
    health_check_id: &str,
    region: Option<&str>,
) -> Result<DeleteHealthCheckOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_health_check()
        .health_check_id(health_check_id)
        .send()
        .await
        .map_err(|e| format!("Failed to delete health check: {}", e))?;

    Ok(DeleteHealthCheckOutput { success: true })
}

/// Get Health Check
///
/// Gets information about a specified health check.
///
pub async fn get_health_check(
    health_check_id: &str,
    region: Option<&str>,
) -> Result<GetHealthCheckOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_health_check()
        .health_check_id(health_check_id)
        .send()
        .await
        .map_err(|e| format!("Failed to get health check: {}", e))?;

    let health_check = response
        .health_check()
        .ok_or("No health check in response")?;

    Ok(GetHealthCheckOutput {
        health_check: convert_health_check(health_check),
    })
}

/// List Health Checks
///
/// Retrieve a list of the health checks.
///
pub async fn list_health_checks(
    marker: Option<&str>,
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListHealthChecksOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_health_checks();

    if let Some(m) = marker {
        request = request.marker(m);
    }

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list health checks: {}", e))?;

    Ok(ListHealthChecksOutput {
        health_checks: response
            .health_checks()
            .iter()
            .map(convert_health_check)
            .collect(),
        is_truncated: response.is_truncated(),
        marker: {
            let m = response.marker();
            if m.is_empty() { None } else { Some(m.to_string()) }
        },
        max_items: response.max_items(),
        next_marker: response.next_marker().map(|s| s.to_string()),
    })
}

/// Update Health Check
///
/// Updates an existing health check.
///
#[allow(clippy::too_many_arguments)]
pub async fn update_health_check(
    health_check_id: &str,
    alarm_identifier: Option<AlarmIdentifier>,
    child_health_checks: Option<Vec<String>>,
    disabled: Option<bool>,
    enable_sni: Option<bool>,
    failure_threshold: Option<i32>,
    fully_qualified_domain_name: Option<&str>,
    health_check_version: Option<i64>,
    health_threshold: Option<i32>,
    insufficient_data_health_status: Option<&str>,
    inverted: Option<bool>,
    ip_address: Option<&str>,
    port: Option<i32>,
    regions: Option<Vec<String>>,
    reset_elements: Option<Vec<String>>,
    resource_path: Option<&str>,
    search_string: Option<&str>,
    region: Option<&str>,
) -> Result<UpdateHealthCheckOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .update_health_check()
        .health_check_id(health_check_id);

    if let Some(ref ai) = alarm_identifier {
        let aws_ai = route53_types::AlarmIdentifier::builder()
            .region(route53_types::CloudWatchRegion::from(ai.region.as_str()))
            .name(&ai.name)
            .build()
            .map_err(|e| format!("Failed to build alarm identifier: {}", e))?;
        request = request.alarm_identifier(aws_ai);
    }

    if let Some(children) = child_health_checks {
        request = request.set_child_health_checks(Some(children));
    }

    if let Some(d) = disabled {
        request = request.disabled(d);
    }

    if let Some(sni) = enable_sni {
        request = request.enable_sni(sni);
    }

    if let Some(ft) = failure_threshold {
        request = request.failure_threshold(ft);
    }

    if let Some(fqdn) = fully_qualified_domain_name {
        request = request.fully_qualified_domain_name(fqdn);
    }

    if let Some(version) = health_check_version {
        request = request.health_check_version(version);
    }

    if let Some(ht) = health_threshold {
        request = request.health_threshold(ht);
    }

    if let Some(status) = insufficient_data_health_status {
        request =
            request.insufficient_data_health_status(InsufficientDataHealthStatus::from(status));
    }

    if let Some(inv) = inverted {
        request = request.inverted(inv);
    }

    if let Some(ip) = ip_address {
        request = request.ip_address(ip);
    }

    if let Some(p) = port {
        request = request.port(p);
    }

    if let Some(r) = regions {
        let aws_regions: Vec<route53_types::HealthCheckRegion> = r
            .iter()
            .map(|reg| route53_types::HealthCheckRegion::from(reg.as_str()))
            .collect();
        request = request.set_regions(Some(aws_regions));
    }

    if let Some(elements) = reset_elements {
        let aws_elements: Vec<route53_types::ResettableElementName> = elements
            .iter()
            .map(|e| route53_types::ResettableElementName::from(e.as_str()))
            .collect();
        request = request.set_reset_elements(Some(aws_elements));
    }

    if let Some(path) = resource_path {
        request = request.resource_path(path);
    }

    if let Some(search) = search_string {
        request = request.search_string(search);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to update health check: {}", e))?;

    let health_check = response
        .health_check()
        .ok_or("No health check in response")?;

    Ok(UpdateHealthCheckOutput {
        health_check: convert_health_check(health_check),
        success: true,
    })
}

/// Get Health Check Status
///
/// Gets the status of the specified health check.
///
pub async fn get_health_check_status(
    health_check_id: &str,
    region: Option<&str>,
) -> Result<GetHealthCheckStatusOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_health_check_status()
        .health_check_id(health_check_id)
        .send()
        .await
        .map_err(|e| format!("Failed to get health check status: {}", e))?;

    Ok(GetHealthCheckStatusOutput {
        health_check_observations: response
            .health_check_observations()
            .iter()
            .map(|obs| HealthCheckObservation {
                region: obs.region().map(|r| r.as_str().to_string()),
                ip_address: obs.ip_address().map(|s| s.to_string()),
                status_report: obs.status_report().map(|sr| StatusReport {
                    status: sr.status().map(|s| s.to_string()),
                    checked_time: sr.checked_time().map(|t| {
                        DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_default()
                    }),
                }),
            })
            .collect(),
    })
}

/// Get Health Check Last Failure Reason
///
/// Gets the reason for the last failure of a specified health check.
///
pub async fn get_health_check_last_failure_reason(
    health_check_id: &str,
    region: Option<&str>,
) -> Result<GetHealthCheckLastFailureReasonOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_health_check_last_failure_reason()
        .health_check_id(health_check_id)
        .send()
        .await
        .map_err(|e| format!("Failed to get health check last failure reason: {}", e))?;

    Ok(GetHealthCheckLastFailureReasonOutput {
        health_check_observations: response
            .health_check_observations()
            .iter()
            .map(|obs| HealthCheckObservation {
                region: obs.region().map(|r| r.as_str().to_string()),
                ip_address: obs.ip_address().map(|s| s.to_string()),
                status_report: obs.status_report().map(|sr| StatusReport {
                    status: sr.status().map(|s| s.to_string()),
                    checked_time: sr.checked_time().map(|t| {
                        DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_default()
                    }),
                }),
            })
            .collect(),
    })
}

/// Get Health Check Count
///
/// Retrieves the number of health checks that are associated with the current AWS account.
///
pub async fn get_health_check_count(region: Option<&str>) -> Result<GetHealthCheckCountOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_health_check_count()
        .send()
        .await
        .map_err(|e| format!("Failed to get health check count: {}", e))?;

    Ok(GetHealthCheckCountOutput {
        health_check_count: response.health_check_count(),
    })
}

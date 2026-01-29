//! Harana Actions - AWS ECS Module
//!
//! This module provides AWS ECS (Elastic Container Service) actions.

#![warn(missing_docs)]

pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_ecs::{
    config::Region,
    types::{
        AssignPublicIp, Compatibility, ContainerDefinition as AwsContainerDefinition,
        KeyValuePair as AwsKeyValuePair, LaunchType, LogConfiguration as AwsLogConfiguration,
        LogDriver, NetworkConfiguration as AwsNetworkConfiguration, PortMapping as AwsPortMapping,
        Tag, TransportProtocol, AwsVpcConfiguration as AwsAwsVpcConfiguration,
        LoadBalancer as AwsLoadBalancer,
    },
    Client,
};
use output::*;
use std::collections::HashMap;

/// Creates an ECS client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let ecs_config = if let Some(region_str) = region {
        aws_sdk_ecs::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_ecs::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(ecs_config))
}

/// Converts EcsTags to AWS SDK Tag format
fn tags_to_aws_tags(tags: &EcsTags) -> Vec<Tag> {
    tags.tags
        .iter()
        .map(|(k, v)| Tag::builder().key(k).value(v).build())
        .collect()
}

/// Converts AWS SDK Cluster to our Cluster type
fn convert_cluster(cluster: &aws_sdk_ecs::types::Cluster) -> Cluster {
    let tags_map: Option<HashMap<String, String>> = {
        let tags = cluster.tags();
        if tags.is_empty() {
            None
        } else {
            Some(
                tags.iter()
                    .filter_map(|t| {
                        let key = t.key()?.to_string();
                        let value = t.value()?.to_string();
                        Some((key, value))
                    })
                    .collect(),
            )
        }
    };

    Cluster {
        cluster_arn: cluster.cluster_arn().map(|s| s.to_string()),
        cluster_name: cluster.cluster_name().map(|s| s.to_string()),
        status: cluster.status().map(|s| s.to_string()),
        registered_container_instances_count: cluster.registered_container_instances_count(),
        running_tasks_count: cluster.running_tasks_count(),
        pending_tasks_count: cluster.pending_tasks_count(),
        active_services_count: cluster.active_services_count(),
        capacity_providers: cluster
            .capacity_providers()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        tags: tags_map,
    }
}

/// Converts AWS SDK Failure to our Failure type
fn convert_failure(failure: &aws_sdk_ecs::types::Failure) -> Failure {
    Failure {
        arn: failure.arn().map(|s| s.to_string()),
        reason: failure.reason().map(|s| s.to_string()),
        detail: failure.detail().map(|s| s.to_string()),
    }
}

/// Converts AWS SDK ContainerDefinition to our ContainerDefinition type
fn convert_container_definition(
    def: &aws_sdk_ecs::types::ContainerDefinition,
) -> ContainerDefinition {
    ContainerDefinition {
        name: def.name().map(|s| s.to_string()),
        image: def.image().map(|s| s.to_string()),
        cpu: def.cpu(),
        memory: def.memory(),
        memory_reservation: def.memory_reservation(),
        essential: def.essential().unwrap_or(true),
        port_mappings: def
            .port_mappings()
            .iter()
            .map(|pm| PortMapping {
                container_port: pm.container_port(),
                host_port: pm.host_port(),
                protocol: pm.protocol().map(|p| p.as_str().to_string()),
            })
            .collect(),
        environment: def
            .environment()
            .iter()
            .map(|kv| KeyValuePair {
                name: kv.name().map(|s| s.to_string()),
                value: kv.value().map(|s| s.to_string()),
            })
            .collect(),
        command: def.command().iter().map(|s| s.to_string()).collect(),
        entry_point: def.entry_point().iter().map(|s| s.to_string()).collect(),
        log_configuration: def.log_configuration().map(|lc| LogConfiguration {
            log_driver: lc.log_driver().as_str().to_string(),
            options: {
                let opts = lc.options();
                if opts.is_none() {
                    None
                } else {
                    opts.map(|o| {
                        o.iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect()
                    })
                }
            },
        }),
    }
}

/// Converts AWS SDK TaskDefinition to our TaskDefinition type
fn convert_task_definition(td: &aws_sdk_ecs::types::TaskDefinition) -> TaskDefinition {
    TaskDefinition {
        task_definition_arn: td.task_definition_arn().map(|s| s.to_string()),
        family: td.family().map(|s| s.to_string()),
        revision: td.revision(),
        status: td.status().map(|s| s.as_str().to_string()),
        container_definitions: td
            .container_definitions()
            .iter()
            .map(convert_container_definition)
            .collect(),
        requires_compatibilities: td
            .requires_compatibilities()
            .iter()
            .map(|c| c.as_str().to_string())
            .collect(),
        cpu: td.cpu().map(|s| s.to_string()),
        memory: td.memory().map(|s| s.to_string()),
        network_mode: td.network_mode().map(|m| m.as_str().to_string()),
        execution_role_arn: td.execution_role_arn().map(|s| s.to_string()),
        task_role_arn: td.task_role_arn().map(|s| s.to_string()),
    }
}

/// Converts AWS SDK Deployment to our Deployment type
fn convert_deployment(dep: &aws_sdk_ecs::types::Deployment) -> Deployment {
    Deployment {
        id: dep.id().map(|s| s.to_string()),
        status: dep.status().map(|s| s.to_string()),
        task_definition: dep.task_definition().map(|s| s.to_string()),
        desired_count: dep.desired_count(),
        running_count: dep.running_count(),
        pending_count: dep.pending_count(),
        created_at: dep.created_at().map(|t| t.to_string()),
        updated_at: dep.updated_at().map(|t| t.to_string()),
        launch_type: dep.launch_type().map(|l| l.as_str().to_string()),
    }
}

/// Converts AWS SDK LoadBalancer to our LoadBalancer type
fn convert_load_balancer(lb: &aws_sdk_ecs::types::LoadBalancer) -> LoadBalancer {
    LoadBalancer {
        target_group_arn: lb.target_group_arn().map(|s| s.to_string()),
        load_balancer_name: lb.load_balancer_name().map(|s| s.to_string()),
        container_name: lb.container_name().map(|s| s.to_string()),
        container_port: lb.container_port(),
    }
}

/// Converts AWS SDK NetworkConfiguration to our NetworkConfiguration type
fn convert_network_configuration(
    nc: &aws_sdk_ecs::types::NetworkConfiguration,
) -> NetworkConfiguration {
    NetworkConfiguration {
        awsvpc_configuration: nc.awsvpc_configuration().map(|vpc| AwsVpcConfiguration {
            subnets: vpc.subnets().iter().map(|s| s.to_string()).collect(),
            security_groups: vpc
                .security_groups()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            assign_public_ip: vpc.assign_public_ip().map(|a| a.as_str().to_string()),
        }),
    }
}

/// Converts AWS SDK Service to our Service type
fn convert_service(svc: &aws_sdk_ecs::types::Service) -> Service {
    let tags_map: Option<HashMap<String, String>> = {
        let tags = svc.tags();
        if tags.is_empty() {
            None
        } else {
            Some(
                tags.iter()
                    .filter_map(|t| {
                        let key = t.key()?.to_string();
                        let value = t.value()?.to_string();
                        Some((key, value))
                    })
                    .collect(),
            )
        }
    };

    Service {
        service_arn: svc.service_arn().map(|s| s.to_string()),
        service_name: svc.service_name().map(|s| s.to_string()),
        cluster_arn: svc.cluster_arn().map(|s| s.to_string()),
        status: svc.status().map(|s| s.to_string()),
        desired_count: svc.desired_count(),
        running_count: svc.running_count(),
        pending_count: svc.pending_count(),
        launch_type: svc.launch_type().map(|l| l.as_str().to_string()),
        task_definition: svc.task_definition().map(|s| s.to_string()),
        deployments: svc.deployments().iter().map(convert_deployment).collect(),
        load_balancers: svc
            .load_balancers()
            .iter()
            .map(convert_load_balancer)
            .collect(),
        network_configuration: svc.network_configuration().map(convert_network_configuration),
        created_at: svc.created_at().map(|t| t.to_string()),
        tags: tags_map,
    }
}

/// Converts AWS SDK Container to our Container type
fn convert_container(container: &aws_sdk_ecs::types::Container) -> Container {
    Container {
        container_arn: container.container_arn().map(|s| s.to_string()),
        name: container.name().map(|s| s.to_string()),
        image: container.image().map(|s| s.to_string()),
        last_status: container.last_status().map(|s| s.to_string()),
        exit_code: container.exit_code(),
        reason: container.reason().map(|s| s.to_string()),
        network_bindings: container
            .network_bindings()
            .iter()
            .map(|nb| NetworkBinding {
                bind_ip: nb.bind_ip().map(|s| s.to_string()),
                container_port: nb.container_port(),
                host_port: nb.host_port(),
                protocol: nb.protocol().map(|p| p.as_str().to_string()),
            })
            .collect(),
        network_interfaces: container
            .network_interfaces()
            .iter()
            .map(|ni| NetworkInterface {
                attachment_id: ni.attachment_id().map(|s| s.to_string()),
                private_ipv4_address: ni.private_ipv4_address().map(|s| s.to_string()),
                ipv6_address: ni.ipv6_address().map(|s| s.to_string()),
            })
            .collect(),
    }
}

/// Converts AWS SDK Task to our Task type
fn convert_task(task: &aws_sdk_ecs::types::Task) -> Task {
    let tags_map: Option<HashMap<String, String>> = {
        let tags = task.tags();
        if tags.is_empty() {
            None
        } else {
            Some(
                tags.iter()
                    .filter_map(|t| {
                        let key = t.key()?.to_string();
                        let value = t.value()?.to_string();
                        Some((key, value))
                    })
                    .collect(),
            )
        }
    };

    Task {
        task_arn: task.task_arn().map(|s| s.to_string()),
        task_definition_arn: task.task_definition_arn().map(|s| s.to_string()),
        cluster_arn: task.cluster_arn().map(|s| s.to_string()),
        container_instance_arn: task.container_instance_arn().map(|s| s.to_string()),
        last_status: task.last_status().map(|s| s.to_string()),
        desired_status: task.desired_status().map(|s| s.to_string()),
        cpu: task.cpu().map(|s| s.to_string()),
        memory: task.memory().map(|s| s.to_string()),
        containers: task.containers().iter().map(convert_container).collect(),
        started_at: task.started_at().map(|t| t.to_string()),
        stopped_at: task.stopped_at().map(|t| t.to_string()),
        stopped_reason: task.stopped_reason().map(|s| s.to_string()),
        launch_type: task.launch_type().map(|l| l.as_str().to_string()),
        group: task.group().map(|s| s.to_string()),
        tags: tags_map,
    }
}

/// Converts AWS SDK Resource to our Resource type
fn convert_resource(res: &aws_sdk_ecs::types::Resource) -> Resource {
    Resource {
        name: res.name().map(|s| s.to_string()),
        r#type: res.r#type().map(|s| s.to_string()),
        integer_value: res.integer_value(),
        string_set_value: res
            .string_set_value()
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

/// Converts AWS SDK ContainerInstance to our ContainerInstance type
fn convert_container_instance(ci: &aws_sdk_ecs::types::ContainerInstance) -> ContainerInstance {
    let tags_map: Option<HashMap<String, String>> = {
        let tags = ci.tags();
        if tags.is_empty() {
            None
        } else {
            Some(
                tags.iter()
                    .filter_map(|t| {
                        let key = t.key()?.to_string();
                        let value = t.value()?.to_string();
                        Some((key, value))
                    })
                    .collect(),
            )
        }
    };

    ContainerInstance {
        container_instance_arn: ci.container_instance_arn().map(|s| s.to_string()),
        ec2_instance_id: ci.ec2_instance_id().map(|s| s.to_string()),
        status: ci.status().map(|s| s.to_string()),
        status_reason: ci.status_reason().map(|s| s.to_string()),
        running_tasks_count: ci.running_tasks_count(),
        pending_tasks_count: ci.pending_tasks_count(),
        agent_connected: ci.agent_connected(),
        registered_at: ci.registered_at().map(|t| t.to_string()),
        remaining_resources: ci
            .remaining_resources()
            .iter()
            .map(convert_resource)
            .collect(),
        registered_resources: ci
            .registered_resources()
            .iter()
            .map(convert_resource)
            .collect(),
        tags: tags_map,
    }
}

/// Converts ContainerDefinitionInput to AWS SDK ContainerDefinition
fn container_definition_input_to_aws(input: &ContainerDefinitionInput) -> AwsContainerDefinition {
    let mut builder = AwsContainerDefinition::builder()
        .name(&input.name)
        .image(&input.image);

    if let Some(cpu) = input.cpu {
        builder = builder.cpu(cpu);
    }

    if let Some(memory) = input.memory {
        builder = builder.memory(memory);
    }

    if let Some(memory_reservation) = input.memory_reservation {
        builder = builder.memory_reservation(memory_reservation);
    }

    builder = builder.essential(input.essential.unwrap_or(true));

    if let Some(ref port_mappings) = input.port_mappings {
        for pm in port_mappings {
            let mut pm_builder = AwsPortMapping::builder().container_port(pm.container_port);
            if let Some(host_port) = pm.host_port {
                pm_builder = pm_builder.host_port(host_port);
            }
            if let Some(ref protocol) = pm.protocol {
                pm_builder = pm_builder.protocol(TransportProtocol::from(protocol.as_str()));
            }
            builder = builder.port_mappings(pm_builder.build());
        }
    }

    if let Some(ref environment) = input.environment {
        for env in environment {
            builder = builder.environment(
                AwsKeyValuePair::builder()
                    .name(&env.name)
                    .value(&env.value)
                    .build(),
            );
        }
    }

    if let Some(ref command) = input.command {
        for cmd in command {
            builder = builder.command(cmd);
        }
    }

    if let Some(ref entry_point) = input.entry_point {
        for ep in entry_point {
            builder = builder.entry_point(ep);
        }
    }

    if let Some(ref log_config) = input.log_configuration {
        let mut lc_builder =
            AwsLogConfiguration::builder().log_driver(LogDriver::from(log_config.log_driver.as_str()));
        if let Some(ref options) = log_config.options {
            for (k, v) in options {
                lc_builder = lc_builder.options(k, v);
            }
        }
        builder = builder.log_configuration(lc_builder.build().unwrap());
    }

    builder.build()
}

/// Converts NetworkConfigurationInput to AWS SDK NetworkConfiguration
fn network_configuration_input_to_aws(
    input: &NetworkConfigurationInput,
) -> AwsNetworkConfiguration {
    let mut vpc_builder = AwsAwsVpcConfiguration::builder();

    for subnet in &input.awsvpc_configuration.subnets {
        vpc_builder = vpc_builder.subnets(subnet);
    }

    if let Some(ref security_groups) = input.awsvpc_configuration.security_groups {
        for sg in security_groups {
            vpc_builder = vpc_builder.security_groups(sg);
        }
    }

    if let Some(ref assign_public_ip) = input.awsvpc_configuration.assign_public_ip {
        vpc_builder =
            vpc_builder.assign_public_ip(AssignPublicIp::from(assign_public_ip.as_str()));
    }

    AwsNetworkConfiguration::builder()
        .awsvpc_configuration(vpc_builder.build().unwrap())
        .build()
}

/// Converts LoadBalancerInput to AWS SDK LoadBalancer
fn load_balancer_input_to_aws(input: &LoadBalancerInput) -> AwsLoadBalancer {
    let mut builder = AwsLoadBalancer::builder()
        .container_name(&input.container_name)
        .container_port(input.container_port);

    if let Some(ref target_group_arn) = input.target_group_arn {
        builder = builder.target_group_arn(target_group_arn);
    }

    if let Some(ref load_balancer_name) = input.load_balancer_name {
        builder = builder.load_balancer_name(load_balancer_name);
    }

    builder.build()
}

// ========== Cluster Operations ==========

/// Create an ECS Cluster
pub async fn create_cluster(
    cluster_name: &str,
    capacity_providers: Option<Vec<String>>,
    region: Option<&str>,
    tags: Option<EcsTags>,
) -> Result<CreateClusterOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_cluster().cluster_name(cluster_name);

    if let Some(providers) = capacity_providers {
        request = request.set_capacity_providers(Some(providers));
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create cluster: {}", e))?;

    let cluster = response
        .cluster()
        .map(convert_cluster)
        .ok_or("No cluster returned")?;

    Ok(CreateClusterOutput {
        cluster,
        success: true,
    })
}

/// Delete an ECS Cluster
pub async fn delete_cluster(
    cluster: &str,
    region: Option<&str>,
) -> Result<DeleteClusterOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .delete_cluster()
        .cluster(cluster)
        .send()
        .await
        .map_err(|e| format!("Failed to delete cluster: {}", e))?;

    let cluster_result = response
        .cluster()
        .map(convert_cluster)
        .ok_or("No cluster returned")?;

    Ok(DeleteClusterOutput {
        cluster: cluster_result,
        success: true,
    })
}

/// Describe ECS Clusters
pub async fn describe_clusters(
    clusters: Vec<String>,
    include: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeClustersOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .describe_clusters()
        .set_clusters(Some(clusters));

    if let Some(inc) = include {
        request = request.set_include(Some(
            inc.iter()
                .map(|s| aws_sdk_ecs::types::ClusterField::from(s.as_str()))
                .collect(),
        ));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe clusters: {}", e))?;

    let clusters_result: Vec<Cluster> = response
        .clusters()
        .iter()
        .map(convert_cluster)
        .collect();

    let failures: Vec<Failure> = response
        .failures()
        .iter()
        .map(convert_failure)
        .collect();

    Ok(DescribeClustersOutput {
        clusters: clusters_result,
        failures,
    })
}

/// List ECS Clusters
pub async fn list_clusters(
    max_results: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListClustersOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_clusters();

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list clusters: {}", e))?;

    Ok(ListClustersOutput {
        cluster_arns: response
            .cluster_arns()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

// ========== Task Definition Operations ==========

/// Register a Task Definition
pub async fn register_task_definition(
    family: &str,
    container_definitions: Vec<ContainerDefinitionInput>,
    cpu: Option<&str>,
    execution_role_arn: Option<&str>,
    memory: Option<&str>,
    network_mode: Option<&str>,
    region: Option<&str>,
    requires_compatibilities: Option<Vec<String>>,
    tags: Option<EcsTags>,
    task_role_arn: Option<&str>,
) -> Result<RegisterTaskDefinitionOutput, String> {
    let client = create_client(region).await?;

    let aws_container_defs: Vec<AwsContainerDefinition> = container_definitions
        .iter()
        .map(container_definition_input_to_aws)
        .collect();

    let mut request = client
        .register_task_definition()
        .family(family)
        .set_container_definitions(Some(aws_container_defs));

    if let Some(c) = cpu {
        request = request.cpu(c);
    }

    if let Some(m) = memory {
        request = request.memory(m);
    }

    if let Some(exec_role) = execution_role_arn {
        request = request.execution_role_arn(exec_role);
    }

    if let Some(task_role) = task_role_arn {
        request = request.task_role_arn(task_role);
    }

    if let Some(nm) = network_mode {
        request = request.network_mode(aws_sdk_ecs::types::NetworkMode::from(nm));
    }

    if let Some(compatibilities) = requires_compatibilities {
        request = request.set_requires_compatibilities(Some(
            compatibilities
                .iter()
                .map(|s| Compatibility::from(s.as_str()))
                .collect(),
        ));
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to register task definition: {}", e))?;

    let task_definition = response
        .task_definition()
        .map(convert_task_definition)
        .ok_or("No task definition returned")?;

    Ok(RegisterTaskDefinitionOutput {
        task_definition,
        success: true,
    })
}

/// Deregister a Task Definition
pub async fn deregister_task_definition(
    task_definition: &str,
    region: Option<&str>,
) -> Result<DeregisterTaskDefinitionOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .deregister_task_definition()
        .task_definition(task_definition)
        .send()
        .await
        .map_err(|e| format!("Failed to deregister task definition: {}", e))?;

    let td = response
        .task_definition()
        .map(convert_task_definition)
        .ok_or("No task definition returned")?;

    Ok(DeregisterTaskDefinitionOutput {
        task_definition: td,
        success: true,
    })
}

/// Describe a Task Definition
pub async fn describe_task_definition(
    task_definition: &str,
    include: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeTaskDefinitionOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .describe_task_definition()
        .task_definition(task_definition);

    if let Some(inc) = include {
        request = request.set_include(Some(
            inc.iter()
                .map(|s| aws_sdk_ecs::types::TaskDefinitionField::from(s.as_str()))
                .collect(),
        ));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe task definition: {}", e))?;

    let td = response
        .task_definition()
        .map(convert_task_definition)
        .ok_or("No task definition returned")?;

    Ok(DescribeTaskDefinitionOutput {
        task_definition: td,
    })
}

/// List Task Definitions
pub async fn list_task_definitions(
    family_prefix: Option<&str>,
    max_results: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
    sort: Option<&str>,
    status: Option<&str>,
) -> Result<ListTaskDefinitionsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_task_definitions();

    if let Some(prefix) = family_prefix {
        request = request.family_prefix(prefix);
    }

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    if let Some(s) = sort {
        request = request.sort(aws_sdk_ecs::types::SortOrder::from(s));
    }

    if let Some(st) = status {
        request = request.status(aws_sdk_ecs::types::TaskDefinitionStatus::from(st));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list task definitions: {}", e))?;

    Ok(ListTaskDefinitionsOutput {
        task_definition_arns: response
            .task_definition_arns()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

// ========== Service Operations ==========

/// Create an ECS Service
pub async fn create_service(
    cluster: &str,
    service_name: &str,
    task_definition: &str,
    deployment_configuration: Option<(i32, i32)>,
    desired_count: Option<i32>,
    launch_type: Option<&str>,
    load_balancers: Option<Vec<LoadBalancerInput>>,
    network_configuration: Option<NetworkConfigurationInput>,
    region: Option<&str>,
    tags: Option<EcsTags>,
) -> Result<CreateServiceOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .create_service()
        .cluster(cluster)
        .service_name(service_name)
        .task_definition(task_definition);

    if let Some(count) = desired_count {
        request = request.desired_count(count);
    }

    if let Some(lt) = launch_type {
        request = request.launch_type(LaunchType::from(lt));
    }

    if let Some(ref nc) = network_configuration {
        request = request.network_configuration(network_configuration_input_to_aws(nc));
    }

    if let Some(ref lbs) = load_balancers {
        let aws_lbs: Vec<AwsLoadBalancer> = lbs.iter().map(load_balancer_input_to_aws).collect();
        request = request.set_load_balancers(Some(aws_lbs));
    }

    if let Some((min_healthy, max_percent)) = deployment_configuration {
        request = request.deployment_configuration(
            aws_sdk_ecs::types::DeploymentConfiguration::builder()
                .minimum_healthy_percent(min_healthy)
                .maximum_percent(max_percent)
                .build(),
        );
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create service: {}", e))?;

    let service = response
        .service()
        .map(convert_service)
        .ok_or("No service returned")?;

    Ok(CreateServiceOutput {
        service,
        success: true,
    })
}

/// Update an ECS Service
pub async fn update_service(
    cluster: &str,
    service: &str,
    desired_count: Option<i32>,
    force_new_deployment: Option<bool>,
    network_configuration: Option<NetworkConfigurationInput>,
    region: Option<&str>,
    task_definition: Option<&str>,
) -> Result<UpdateServiceOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.update_service().cluster(cluster).service(service);

    if let Some(count) = desired_count {
        request = request.desired_count(count);
    }

    if let Some(td) = task_definition {
        request = request.task_definition(td);
    }

    if let Some(force) = force_new_deployment {
        request = request.force_new_deployment(force);
    }

    if let Some(ref nc) = network_configuration {
        request = request.network_configuration(network_configuration_input_to_aws(nc));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to update service: {}", e))?;

    let service_result = response
        .service()
        .map(convert_service)
        .ok_or("No service returned")?;

    Ok(UpdateServiceOutput {
        service: service_result,
        success: true,
    })
}

/// Delete an ECS Service
pub async fn delete_service(
    cluster: &str,
    service: &str,
    force: Option<bool>,
    region: Option<&str>,
) -> Result<DeleteServiceOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.delete_service().cluster(cluster).service(service);

    if let Some(f) = force {
        request = request.force(f);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to delete service: {}", e))?;

    let service_result = response
        .service()
        .map(convert_service)
        .ok_or("No service returned")?;

    Ok(DeleteServiceOutput {
        service: service_result,
        success: true,
    })
}

/// Describe ECS Services
pub async fn describe_services(
    cluster: &str,
    services: Vec<String>,
    include: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeServicesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .describe_services()
        .cluster(cluster)
        .set_services(Some(services));

    if let Some(inc) = include {
        request = request.set_include(Some(
            inc.iter()
                .map(|s| aws_sdk_ecs::types::ServiceField::from(s.as_str()))
                .collect(),
        ));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe services: {}", e))?;

    let services_result: Vec<Service> = response.services().iter().map(convert_service).collect();

    let failures: Vec<Failure> = response.failures().iter().map(convert_failure).collect();

    Ok(DescribeServicesOutput {
        services: services_result,
        failures,
    })
}

/// List ECS Services
pub async fn list_services(
    cluster: &str,
    launch_type: Option<&str>,
    max_results: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
    scheduling_strategy: Option<&str>,
) -> Result<ListServicesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_services().cluster(cluster);

    if let Some(lt) = launch_type {
        request = request.launch_type(LaunchType::from(lt));
    }

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    if let Some(strategy) = scheduling_strategy {
        request =
            request.scheduling_strategy(aws_sdk_ecs::types::SchedulingStrategy::from(strategy));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list services: {}", e))?;

    Ok(ListServicesOutput {
        service_arns: response
            .service_arns()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

// ========== Task Operations ==========

/// Run an ECS Task
pub async fn run_task(
    cluster: &str,
    task_definition: &str,
    count: Option<i32>,
    group: Option<&str>,
    launch_type: Option<&str>,
    network_configuration: Option<NetworkConfigurationInput>,
    region: Option<&str>,
    started_by: Option<&str>,
    tags: Option<EcsTags>,
) -> Result<RunTaskOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .run_task()
        .cluster(cluster)
        .task_definition(task_definition);

    if let Some(c) = count {
        request = request.count(c);
    }

    if let Some(g) = group {
        request = request.group(g);
    }

    if let Some(lt) = launch_type {
        request = request.launch_type(LaunchType::from(lt));
    }

    if let Some(ref nc) = network_configuration {
        request = request.network_configuration(network_configuration_input_to_aws(nc));
    }

    if let Some(sb) = started_by {
        request = request.started_by(sb);
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to run task: {}", e))?;

    let tasks: Vec<Task> = response.tasks().iter().map(convert_task).collect();

    let failures: Vec<Failure> = response.failures().iter().map(convert_failure).collect();

    Ok(RunTaskOutput {
        tasks,
        failures,
        success: true,
    })
}

/// Stop an ECS Task
pub async fn stop_task(
    cluster: &str,
    task: &str,
    reason: Option<&str>,
    region: Option<&str>,
) -> Result<StopTaskOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.stop_task().cluster(cluster).task(task);

    if let Some(r) = reason {
        request = request.reason(r);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to stop task: {}", e))?;

    let task_result = response
        .task()
        .map(convert_task)
        .ok_or("No task returned")?;

    Ok(StopTaskOutput {
        task: task_result,
        success: true,
    })
}

/// Describe ECS Tasks
pub async fn describe_tasks(
    cluster: &str,
    tasks: Vec<String>,
    include: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeTasksOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .describe_tasks()
        .cluster(cluster)
        .set_tasks(Some(tasks));

    if let Some(inc) = include {
        request = request.set_include(Some(
            inc.iter()
                .map(|s| aws_sdk_ecs::types::TaskField::from(s.as_str()))
                .collect(),
        ));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe tasks: {}", e))?;

    let tasks_result: Vec<Task> = response.tasks().iter().map(convert_task).collect();

    let failures: Vec<Failure> = response.failures().iter().map(convert_failure).collect();

    Ok(DescribeTasksOutput {
        tasks: tasks_result,
        failures,
    })
}

/// List ECS Tasks
pub async fn list_tasks(
    cluster: &str,
    container_instance: Option<&str>,
    desired_status: Option<&str>,
    family: Option<&str>,
    launch_type: Option<&str>,
    max_results: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
    service_name: Option<&str>,
    started_by: Option<&str>,
) -> Result<ListTasksOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_tasks().cluster(cluster);

    if let Some(ci) = container_instance {
        request = request.container_instance(ci);
    }

    if let Some(ds) = desired_status {
        request = request.desired_status(aws_sdk_ecs::types::DesiredStatus::from(ds));
    }

    if let Some(f) = family {
        request = request.family(f);
    }

    if let Some(lt) = launch_type {
        request = request.launch_type(LaunchType::from(lt));
    }

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    if let Some(svc) = service_name {
        request = request.service_name(svc);
    }

    if let Some(sb) = started_by {
        request = request.started_by(sb);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list tasks: {}", e))?;

    Ok(ListTasksOutput {
        task_arns: response.task_arns().iter().map(|s| s.to_string()).collect(),
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

// ========== Container Instance Operations ==========

/// List ECS Container Instances
pub async fn list_container_instances(
    cluster: &str,
    filter: Option<&str>,
    max_results: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
    status: Option<&str>,
) -> Result<ListContainerInstancesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_container_instances().cluster(cluster);

    if let Some(f) = filter {
        request = request.filter(f);
    }

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    if let Some(st) = status {
        request = request.status(aws_sdk_ecs::types::ContainerInstanceStatus::from(st));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list container instances: {}", e))?;

    Ok(ListContainerInstancesOutput {
        container_instance_arns: response
            .container_instance_arns()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Describe ECS Container Instances
pub async fn describe_container_instances(
    cluster: &str,
    container_instances: Vec<String>,
    include: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeContainerInstancesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .describe_container_instances()
        .cluster(cluster)
        .set_container_instances(Some(container_instances));

    if let Some(inc) = include {
        request = request.set_include(Some(
            inc.iter()
                .map(|s| aws_sdk_ecs::types::ContainerInstanceField::from(s.as_str()))
                .collect(),
        ));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe container instances: {}", e))?;

    let container_instances_result: Vec<ContainerInstance> = response
        .container_instances()
        .iter()
        .map(convert_container_instance)
        .collect();

    let failures: Vec<Failure> = response.failures().iter().map(convert_failure).collect();

    Ok(DescribeContainerInstancesOutput {
        container_instances: container_instances_result,
        failures,
    })
}

/// Update Container Instances State
pub async fn update_container_instances_state(
    cluster: &str,
    container_instances: Vec<String>,
    status: &str,
    region: Option<&str>,
) -> Result<UpdateContainerInstancesStateOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .update_container_instances_state()
        .cluster(cluster)
        .set_container_instances(Some(container_instances))
        .status(aws_sdk_ecs::types::ContainerInstanceStatus::from(status))
        .send()
        .await
        .map_err(|e| format!("Failed to update container instances state: {}", e))?;

    let container_instances_result: Vec<ContainerInstance> = response
        .container_instances()
        .iter()
        .map(convert_container_instance)
        .collect();

    let failures: Vec<Failure> = response.failures().iter().map(convert_failure).collect();

    Ok(UpdateContainerInstancesStateOutput {
        container_instances: container_instances_result,
        failures,
        success: true,
    })
}

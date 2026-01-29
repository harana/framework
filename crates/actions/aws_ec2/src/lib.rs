//! Harana Actions - AWS EC2 Module
//!
//! This module provides AWS EC2 compute actions.

#![warn(missing_docs)]

pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_ec2::{
    config::Region,
    types::{
        Filter, InstanceType, RunInstancesMonitoringEnabled,
        Tag, TagSpecification, ResourceType,
    },
    Client,
};
use base64::Engine;
use output::*;

/// Creates an EC2 client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    
    let ec2_config = if let Some(region_str) = region {
        aws_sdk_ec2::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_ec2::config::Builder::from(&config).build()
    };
    
    Ok(Client::from_conf(ec2_config))
}

/// Converts Ec2Tags to AWS SDK Tag format
fn tags_to_aws_tags(tags: &Ec2Tags) -> Vec<Tag> {
    tags.tags
        .iter()
        .map(|(k, v)| {
            Tag::builder()
                .key(k)
                .value(v)
                .build()
        })
        .collect()
}

/// Converts Ec2Filter to AWS SDK Filter format
fn filters_to_aws_filters(filters: &[Ec2Filter]) -> Vec<Filter> {
    filters
        .iter()
        .map(|f| {
            Filter::builder()
                .name(&f.name)
                .set_values(Some(f.values.clone()))
                .build()
        })
        .collect()
}

/// Converts AWS SDK Instance to our Instance type
fn convert_instance(instance: &aws_sdk_ec2::types::Instance) -> Instance {
    let tags_map: Option<std::collections::HashMap<String, String>> = {
        let tags = instance.tags();
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
                    .collect()
            )
        }
    };
    
    Instance {
        instance_id: instance.instance_id().unwrap_or_default().to_string(),
        instance_type: instance
            .instance_type()
            .map(|t| t.as_str().to_string())
            .unwrap_or_default(),
        image_id: instance.image_id().unwrap_or_default().to_string(),
        state: instance
            .state()
            .map(|s| InstanceState {
                code: s.code().unwrap_or(0),
                name: s.name().map(|n| n.as_str().to_string()).unwrap_or_default(),
            })
            .unwrap_or(InstanceState {
                code: 0,
                name: "unknown".to_string(),
            }),
        private_ip_address: instance.private_ip_address().map(|s| s.to_string()),
        public_ip_address: instance.public_ip_address().map(|s| s.to_string()),
        private_dns_name: instance.private_dns_name().map(|s| s.to_string()),
        public_dns_name: instance.public_dns_name().map(|s| s.to_string()),
        subnet_id: instance.subnet_id().map(|s| s.to_string()),
        vpc_id: instance.vpc_id().map(|s| s.to_string()),
        key_name: instance.key_name().map(|s| s.to_string()),
        launch_time: instance
            .launch_time()
            .map(|t| t.to_string())
            .unwrap_or_default(),
        availability_zone: instance
            .placement()
            .and_then(|p| p.availability_zone())
            .map(|s| s.to_string()),
        tags: tags_map,
    }
}

/// Converts AWS SDK InstanceStateChange to our InstanceStateChange type
fn convert_instance_state_change(
    change: &aws_sdk_ec2::types::InstanceStateChange,
) -> InstanceStateChange {
    InstanceStateChange {
        instance_id: change.instance_id().unwrap_or_default().to_string(),
        current_state: change
            .current_state()
            .map(|s| InstanceState {
                code: s.code().unwrap_or(0),
                name: s.name().map(|n| n.as_str().to_string()).unwrap_or_default(),
            })
            .unwrap_or(InstanceState {
                code: 0,
                name: "unknown".to_string(),
            }),
        previous_state: change
            .previous_state()
            .map(|s| InstanceState {
                code: s.code().unwrap_or(0),
                name: s.name().map(|n| n.as_str().to_string()).unwrap_or_default(),
            })
            .unwrap_or(InstanceState {
                code: 0,
                name: "unknown".to_string(),
            }),
    }
}

/// Converts IpPermission to AWS SDK IpPermission format
fn ip_permission_to_aws(perm: &IpPermission) -> aws_sdk_ec2::types::IpPermission {
    let mut builder = aws_sdk_ec2::types::IpPermission::builder()
        .ip_protocol(&perm.ip_protocol);
    
    if let Some(from_port) = perm.from_port {
        builder = builder.from_port(from_port);
    }
    if let Some(to_port) = perm.to_port {
        builder = builder.to_port(to_port);
    }
    
    for ip_range in &perm.ip_ranges {
        let mut range_builder = aws_sdk_ec2::types::IpRange::builder()
            .cidr_ip(&ip_range.cidr_ip);
        if let Some(desc) = &ip_range.description {
            range_builder = range_builder.description(desc);
        }
        builder = builder.ip_ranges(range_builder.build());
    }
    
    for group_pair in &perm.user_id_group_pairs {
        let mut pair_builder = aws_sdk_ec2::types::UserIdGroupPair::builder()
            .group_id(&group_pair.group_id);
        if let Some(user_id) = &group_pair.user_id {
            pair_builder = pair_builder.user_id(user_id);
        }
        if let Some(desc) = &group_pair.description {
            pair_builder = pair_builder.description(desc);
        }
        builder = builder.user_id_group_pairs(pair_builder.build());
    }
    
    builder.build()
}

/// Converts AWS SDK IpPermission to our IpPermission type
fn convert_ip_permission(perm: &aws_sdk_ec2::types::IpPermission) -> IpPermission {
    IpPermission {
        ip_protocol: perm.ip_protocol().unwrap_or_default().to_string(),
        from_port: perm.from_port(),
        to_port: perm.to_port(),
        ip_ranges: perm
            .ip_ranges()
            .iter()
            .map(|r| IpRange {
                cidr_ip: r.cidr_ip().unwrap_or_default().to_string(),
                description: r.description().map(|s| s.to_string()),
            })
            .collect(),
        user_id_group_pairs: perm
            .user_id_group_pairs()
            .iter()
            .map(|p| UserIdGroupPair {
                group_id: p.group_id().unwrap_or_default().to_string(),
                user_id: p.user_id().map(|s| s.to_string()),
                description: p.description().map(|s| s.to_string()),
            })
            .collect(),
    }
}

/// Run EC2 Instance
pub async fn run_instances(
    image_id: &str,
    instance_type: &str,
    key_name: Option<&str>,
    max_count: Option<i32>,
    min_count: Option<i32>,
    monitoring: Option<bool>,
    region: Option<&str>,
    security_group_ids: Option<Vec<String>>,
    subnet_id: Option<&str>,
    tags: Option<Ec2Tags>,
    user_data: Option<&str>,
) -> Result<RunInstancesOutput, String> {
    let client = create_client(region).await?;
    
    let instance_type_enum = InstanceType::from(instance_type);
    
    let mut request = client
        .run_instances()
        .image_id(image_id)
        .instance_type(instance_type_enum)
        .max_count(max_count.unwrap_or(1))
        .min_count(min_count.unwrap_or(1));
    
    if let Some(key) = key_name {
        request = request.key_name(key);
    }
    
    if let Some(monitor) = monitoring {
        request = request.monitoring(
            RunInstancesMonitoringEnabled::builder()
                .enabled(monitor)
                .build(),
        );
    }
    
    if let Some(sg_ids) = security_group_ids {
        for sg_id in sg_ids {
            request = request.security_group_ids(sg_id);
        }
    }
    
    if let Some(subnet) = subnet_id {
        request = request.subnet_id(subnet);
    }
    
    if let Some(ref tags_data) = tags {
        let tag_spec = TagSpecification::builder()
            .resource_type(ResourceType::Instance)
            .set_tags(Some(tags_to_aws_tags(tags_data)))
            .build();
        request = request.tag_specifications(tag_spec);
    }
    
    if let Some(data) = user_data {
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        request = request.user_data(encoded);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to run instances: {}", e))?;
    
    let instances: Vec<Instance> = response
        .instances()
        .iter()
        .map(convert_instance)
        .collect();
    
    Ok(RunInstancesOutput {
        instances,
        owner_id: response.owner_id().unwrap_or_default().to_string(),
        reservation_id: response.reservation_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Start EC2 Instances
pub async fn start_instances(
    instance_ids: Vec<String>,
    region: Option<&str>,
) -> Result<StartInstancesOutput, String> {
    let client = create_client(region).await?;
    
    let response = client
        .start_instances()
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .map_err(|e| format!("Failed to start instances: {}", e))?;
    
    let starting_instances: Vec<InstanceStateChange> = response
        .starting_instances()
        .iter()
        .map(convert_instance_state_change)
        .collect();
    
    Ok(StartInstancesOutput {
        starting_instances,
        success: true,
    })
}

/// Stop EC2 Instances
pub async fn stop_instances(
    instance_ids: Vec<String>,
    force: Option<bool>,
    hibernate: Option<bool>,
    region: Option<&str>,
) -> Result<StopInstancesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .stop_instances()
        .set_instance_ids(Some(instance_ids));
    
    if let Some(f) = force {
        request = request.force(f);
    }
    
    if let Some(h) = hibernate {
        request = request.hibernate(h);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to stop instances: {}", e))?;
    
    let stopping_instances: Vec<InstanceStateChange> = response
        .stopping_instances()
        .iter()
        .map(convert_instance_state_change)
        .collect();
    
    Ok(StopInstancesOutput {
        stopping_instances,
        success: true,
    })
}

/// Reboot EC2 Instances
pub async fn reboot_instances(
    instance_ids: Vec<String>,
    region: Option<&str>,
) -> Result<RebootInstancesOutput, String> {
    let client = create_client(region).await?;
    
    client
        .reboot_instances()
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .map_err(|e| format!("Failed to reboot instances: {}", e))?;
    
    Ok(RebootInstancesOutput { success: true })
}

/// Terminate EC2 Instances
pub async fn terminate_instances(
    instance_ids: Vec<String>,
    region: Option<&str>,
) -> Result<TerminateInstancesOutput, String> {
    let client = create_client(region).await?;
    
    let response = client
        .terminate_instances()
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .map_err(|e| format!("Failed to terminate instances: {}", e))?;
    
    let terminating_instances: Vec<InstanceStateChange> = response
        .terminating_instances()
        .iter()
        .map(convert_instance_state_change)
        .collect();
    
    Ok(TerminateInstancesOutput {
        success: true,
        terminating_instances,
    })
}

/// Describe EC2 Instances
pub async fn describe_instances(
    filters: Option<Vec<Ec2Filter>>,
    instance_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeInstancesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.describe_instances();
    
    if let Some(ref f) = filters {
        request = request.set_filters(Some(filters_to_aws_filters(f)));
    }
    
    if let Some(ids) = instance_ids {
        request = request.set_instance_ids(Some(ids));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe instances: {}", e))?;
    
    let reservations: Vec<Reservation> = response
        .reservations()
        .iter()
        .map(|r| Reservation {
            reservation_id: r.reservation_id().unwrap_or_default().to_string(),
            owner_id: r.owner_id().unwrap_or_default().to_string(),
            instances: r.instances().iter().map(convert_instance).collect(),
        })
        .collect();
    
    Ok(DescribeInstancesOutput { reservations })
}

/// Describe Instance Status
pub async fn describe_instance_status(
    filters: Option<Vec<Ec2Filter>>,
    include_all_instances: Option<bool>,
    instance_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeInstanceStatusOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.describe_instance_status();
    
    if let Some(ref f) = filters {
        request = request.set_filters(Some(filters_to_aws_filters(f)));
    }
    
    if let Some(include_all) = include_all_instances {
        request = request.include_all_instances(include_all);
    }
    
    if let Some(ids) = instance_ids {
        request = request.set_instance_ids(Some(ids));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe instance status: {}", e))?;
    
    let instance_statuses: Vec<InstanceStatus> = response
        .instance_statuses()
        .iter()
        .map(|s| InstanceStatus {
            instance_id: s.instance_id().unwrap_or_default().to_string(),
            instance_state: s
                .instance_state()
                .map(|state| InstanceState {
                    code: state.code().unwrap_or(0),
                    name: state.name().map(|n| n.as_str().to_string()).unwrap_or_default(),
                })
                .unwrap_or(InstanceState {
                    code: 0,
                    name: "unknown".to_string(),
                }),
            availability_zone: s.availability_zone().unwrap_or_default().to_string(),
            system_status: s
                .system_status()
                .map(|ss| StatusSummary {
                    status: ss.status().map(|st| st.as_str().to_string()).unwrap_or_default(),
                    details: ss
                        .details()
                        .iter()
                        .map(|d| StatusDetail {
                            name: d.name().map(|n| n.as_str().to_string()).unwrap_or_default(),
                            status: d.status().map(|s| s.as_str().to_string()).unwrap_or_default(),
                        })
                        .collect(),
                })
                .unwrap_or(StatusSummary {
                    status: "unknown".to_string(),
                    details: vec![],
                }),
            instance_status: s
                .instance_status()
                .map(|is| StatusSummary {
                    status: is.status().map(|st| st.as_str().to_string()).unwrap_or_default(),
                    details: is
                        .details()
                        .iter()
                        .map(|d| StatusDetail {
                            name: d.name().map(|n| n.as_str().to_string()).unwrap_or_default(),
                            status: d.status().map(|s| s.as_str().to_string()).unwrap_or_default(),
                        })
                        .collect(),
                })
                .unwrap_or(StatusSummary {
                    status: "unknown".to_string(),
                    details: vec![],
                }),
        })
        .collect();
    
    Ok(DescribeInstanceStatusOutput {
        instance_statuses,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Create Image
pub async fn create_image(
    instance_id: &str,
    name: &str,
    description: Option<&str>,
    no_reboot: Option<bool>,
    region: Option<&str>,
    tags: Option<Ec2Tags>,
) -> Result<CreateImageOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .create_image()
        .instance_id(instance_id)
        .name(name);
    
    if let Some(desc) = description {
        request = request.description(desc);
    }
    
    if let Some(nr) = no_reboot {
        request = request.no_reboot(nr);
    }
    
    if let Some(ref tags_data) = tags {
        let tag_spec = TagSpecification::builder()
            .resource_type(ResourceType::Image)
            .set_tags(Some(tags_to_aws_tags(tags_data)))
            .build();
        request = request.tag_specifications(tag_spec);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create image: {}", e))?;
    
    Ok(CreateImageOutput {
        image_id: response.image_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Describe Images
pub async fn describe_images(
    filters: Option<Vec<Ec2Filter>>,
    image_ids: Option<Vec<String>>,
    owners: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeImagesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.describe_images();
    
    if let Some(ref f) = filters {
        request = request.set_filters(Some(filters_to_aws_filters(f)));
    }
    
    if let Some(ids) = image_ids {
        request = request.set_image_ids(Some(ids));
    }
    
    if let Some(o) = owners {
        request = request.set_owners(Some(o));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe images: {}", e))?;
    
    let images: Vec<Ec2Image> = response
        .images()
        .iter()
        .map(|img| Ec2Image {
            image_id: img.image_id().unwrap_or_default().to_string(),
            name: img.name().map(|s| s.to_string()),
            description: img.description().map(|s| s.to_string()),
            state: img.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
            owner_id: img.owner_id().unwrap_or_default().to_string(),
            creation_date: img.creation_date().map(|s| s.to_string()),
            architecture: img
                .architecture()
                .map(|a| a.as_str().to_string())
                .unwrap_or_default(),
            platform: img.platform().map(|p| p.as_str().to_string()),
            root_device_type: img
                .root_device_type()
                .map(|t| t.as_str().to_string())
                .unwrap_or_default(),
            virtualization_type: img
                .virtualization_type()
                .map(|t| t.as_str().to_string())
                .unwrap_or_default(),
        })
        .collect();
    
    Ok(DescribeImagesOutput { images })
}

/// Create Security Group
pub async fn create_security_group(
    description: &str,
    group_name: &str,
    region: Option<&str>,
    tags: Option<Ec2Tags>,
    vpc_id: Option<&str>,
) -> Result<CreateSecurityGroupOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .create_security_group()
        .description(description)
        .group_name(group_name);
    
    if let Some(vpc) = vpc_id {
        request = request.vpc_id(vpc);
    }
    
    if let Some(ref tags_data) = tags {
        let tag_spec = TagSpecification::builder()
            .resource_type(ResourceType::SecurityGroup)
            .set_tags(Some(tags_to_aws_tags(tags_data)))
            .build();
        request = request.tag_specifications(tag_spec);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create security group: {}", e))?;
    
    Ok(CreateSecurityGroupOutput {
        group_id: response.group_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete Security Group
pub async fn delete_security_group(
    group_id: Option<&str>,
    group_name: Option<&str>,
    region: Option<&str>,
) -> Result<DeleteSecurityGroupOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.delete_security_group();
    
    if let Some(id) = group_id {
        request = request.group_id(id);
    }
    
    if let Some(name) = group_name {
        request = request.group_name(name);
    }
    
    request
        .send()
        .await
        .map_err(|e| format!("Failed to delete security group: {}", e))?;
    
    Ok(DeleteSecurityGroupOutput { success: true })
}

/// Describe Security Groups
pub async fn describe_security_groups(
    filters: Option<Vec<Ec2Filter>>,
    group_ids: Option<Vec<String>>,
    group_names: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeSecurityGroupsOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.describe_security_groups();
    
    if let Some(ref f) = filters {
        request = request.set_filters(Some(filters_to_aws_filters(f)));
    }
    
    if let Some(ids) = group_ids {
        request = request.set_group_ids(Some(ids));
    }
    
    if let Some(names) = group_names {
        request = request.set_group_names(Some(names));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe security groups: {}", e))?;
    
    let security_groups: Vec<SecurityGroup> = response
        .security_groups()
        .iter()
        .map(|sg| {
            let tags_map: Option<std::collections::HashMap<String, String>> = {
                let tags = sg.tags();
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
                            .collect()
                    )
                }
            };
            
            SecurityGroup {
                group_id: sg.group_id().unwrap_or_default().to_string(),
                group_name: sg.group_name().unwrap_or_default().to_string(),
                description: sg.description().unwrap_or_default().to_string(),
                vpc_id: sg.vpc_id().map(|s| s.to_string()),
                owner_id: sg.owner_id().unwrap_or_default().to_string(),
                ip_permissions: sg
                    .ip_permissions()
                    .iter()
                    .map(convert_ip_permission)
                    .collect(),
                ip_permissions_egress: sg
                    .ip_permissions_egress()
                    .iter()
                    .map(convert_ip_permission)
                    .collect(),
                tags: tags_map,
            }
        })
        .collect();
    
    Ok(DescribeSecurityGroupsOutput {
        security_groups,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Authorize Security Group Ingress
pub async fn authorize_security_group_ingress(
    group_id: &str,
    ip_permissions: Vec<IpPermission>,
    region: Option<&str>,
) -> Result<AuthorizeSecurityGroupIngressOutput, String> {
    let client = create_client(region).await?;
    
    let aws_permissions: Vec<aws_sdk_ec2::types::IpPermission> = ip_permissions
        .iter()
        .map(ip_permission_to_aws)
        .collect();
    
    client
        .authorize_security_group_ingress()
        .group_id(group_id)
        .set_ip_permissions(Some(aws_permissions))
        .send()
        .await
        .map_err(|e| format!("Failed to authorize security group ingress: {}", e))?;
    
    Ok(AuthorizeSecurityGroupIngressOutput { success: true })
}

/// Revoke Security Group Ingress
pub async fn revoke_security_group_ingress(
    group_id: &str,
    ip_permissions: Vec<IpPermission>,
    region: Option<&str>,
) -> Result<RevokeSecurityGroupIngressOutput, String> {
    let client = create_client(region).await?;
    
    let aws_permissions: Vec<aws_sdk_ec2::types::IpPermission> = ip_permissions
        .iter()
        .map(ip_permission_to_aws)
        .collect();
    
    client
        .revoke_security_group_ingress()
        .group_id(group_id)
        .set_ip_permissions(Some(aws_permissions))
        .send()
        .await
        .map_err(|e| format!("Failed to revoke security group ingress: {}", e))?;
    
    Ok(RevokeSecurityGroupIngressOutput { success: true })
}

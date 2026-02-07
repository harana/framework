// Harana Actions - AWS VPC Module
//
// This module provides AWS Virtual Private Cloud (VPC) actions for managing
// VPCs, subnets, internet gateways, NAT gateways, route tables, and elastic IPs.

pub mod output;


use aws_config::BehaviorVersion;
use aws_sdk_ec2::{
    config::Region,
    types::{
        AttributeBooleanValue, Filter, Tag as Ec2Tag, TagSpecification, ResourceType, Tenancy,
    },
    Client,
};
use chrono::{DateTime, Utc};
use output::*;

/// Creates an EC2 client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let ec2_config = if let Some(region_str) = region {
        aws_sdk_ec2::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_ec2::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(ec2_config))
}

/// Converts VpcTags to AWS TagSpecification
fn tags_to_tag_specification(tags: Option<VpcTags>, resource_type: ResourceType) -> Option<TagSpecification> {
    tags.map(|t| {
        let ec2_tags: Vec<Ec2Tag> = t
            .tags
            .iter()
            .map(|(k, v)| {
                Ec2Tag::builder()
                    .key(k)
                    .value(v)
                    .build()
            })
            .collect();

        TagSpecification::builder()
            .resource_type(resource_type)
            .set_tags(Some(ec2_tags))
            .build()
    })
}

/// Converts VpcFilter to AWS Filter
fn convert_filters(filters: Option<Vec<VpcFilter>>) -> Option<Vec<Filter>> {
    filters.map(|f| {
        f.iter()
            .map(|filter| {
                Filter::builder()
                    .name(&filter.name)
                    .set_values(Some(filter.values.clone()))
                    .build()
            })
            .collect()
    })
}

/// Converts AWS tags to output Tag format
fn convert_tags(tags: Option<&[aws_sdk_ec2::types::Tag]>) -> Option<Vec<Tag>> {
    tags.map(|t| {
        t.iter()
            .map(|tag| Tag {
                key: tag.key().unwrap_or_default().to_string(),
                value: tag.value().map(|v| v.to_string()),
            })
            .collect()
    })
}

/// Create VPC
///
/// Creates a VPC with the specified IPv4 CIDR block.
///
pub async fn create_vpc(
    cidr_block: &str,
    enable_dns_hostnames: Option<bool>,
    enable_dns_support: Option<bool>,
    instance_tenancy: Option<&str>,
    ipv6_cidr_block: Option<&str>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateVpcOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_vpc().cidr_block(cidr_block);

    if let Some(tenancy) = instance_tenancy {
        let tenancy_value = match tenancy.to_lowercase().as_str() {
            "default" => Tenancy::Default,
            "dedicated" => Tenancy::Dedicated,
            "host" => Tenancy::Host,
            _ => Tenancy::Default,
        };
        request = request.instance_tenancy(tenancy_value);
    }

    if let Some(ipv6) = ipv6_cidr_block {
        request = request.ipv6_cidr_block(ipv6);
    }

    if let Some(tag_spec) = tags_to_tag_specification(tags, ResourceType::Vpc) {
        request = request.tag_specifications(tag_spec);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create VPC: {}", e))?;

    let vpc = response.vpc().ok_or("No VPC in response")?;

    // If DNS settings are specified, we need to modify the VPC attributes
    if enable_dns_hostnames.is_some() || enable_dns_support.is_some() {
        let vpc_id = vpc.vpc_id().unwrap_or_default();
        
        if let Some(dns_hostnames) = enable_dns_hostnames {
            client
                .modify_vpc_attribute()
                .vpc_id(vpc_id)
                .enable_dns_hostnames(AttributeBooleanValue::builder().value(dns_hostnames).build())
                .send()
                .await
                .map_err(|e| format!("Failed to modify DNS hostnames: {}", e))?;
        }

        if let Some(dns_support) = enable_dns_support {
            client
                .modify_vpc_attribute()
                .vpc_id(vpc_id)
                .enable_dns_support(AttributeBooleanValue::builder().value(dns_support).build())
                .send()
                .await
                .map_err(|e| format!("Failed to modify DNS support: {}", e))?;
        }
    }

    Ok(CreateVpcOutput {
        vpc_id: vpc.vpc_id().unwrap_or_default().to_string(),
        cidr_block: vpc.cidr_block().unwrap_or_default().to_string(),
        dhcp_options_id: vpc.dhcp_options_id().map(|s| s.to_string()),
        instance_tenancy: vpc
            .instance_tenancy()
            .map(|t| t.as_str().to_string())
            .unwrap_or_else(|| "default".to_string()),
        ipv6_cidr_block: vpc
            .ipv6_cidr_block_association_set()
            .first()
            .and_then(|a| a.ipv6_cidr_block())
            .map(|s| s.to_string()),
        is_default: vpc.is_default().unwrap_or(false),
        owner_id: vpc.owner_id().unwrap_or_default().to_string(),
        state: vpc
            .state()
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| "pending".to_string()),
        success: true,
    })
}

/// Delete VPC
///
/// Deletes the specified VPC.
///
pub async fn delete_vpc(vpc_id: &str, region: Option<&str>) -> Result<DeleteVpcOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_vpc()
        .vpc_id(vpc_id)
        .send()
        .await
        .map_err(|e| format!("Failed to delete VPC: {}", e))?;

    Ok(DeleteVpcOutput { success: true })
}

/// Describe VPCs
///
/// Describes one or more of your VPCs.
///
pub async fn describe_vpcs(
    filters: Option<Vec<VpcFilter>>,
    region: Option<&str>,
    vpc_ids: Option<Vec<String>>,
) -> Result<DescribeVpcsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_vpcs();

    if let Some(ids) = vpc_ids {
        request = request.set_vpc_ids(Some(ids));
    }

    if let Some(f) = convert_filters(filters) {
        request = request.set_filters(Some(f));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe VPCs: {}", e))?;

    let vpcs: Vec<Vpc> = response
        .vpcs()
        .iter()
        .map(|v| {
            let ipv6_associations: Option<Vec<VpcIpv6CidrBlockAssociation>> = if v
                .ipv6_cidr_block_association_set()
                .is_empty()
            {
                None
            } else {
                Some(
                    v.ipv6_cidr_block_association_set()
                        .iter()
                        .map(|a| VpcIpv6CidrBlockAssociation {
                            association_id: a.association_id().unwrap_or_default().to_string(),
                            ipv6_cidr_block: a.ipv6_cidr_block().unwrap_or_default().to_string(),
                            ipv6_cidr_block_state: a.ipv6_cidr_block_state().map(|s| VpcCidrBlockState {
                                state: s.state().map(|st| st.as_str().to_string()).unwrap_or_default(),
                                status_message: s.status_message().map(|m| m.to_string()),
                            }),
                            network_border_group: a.network_border_group().map(|n| n.to_string()),
                            ipv6_pool: a.ipv6_pool().map(|p| p.to_string()),
                        })
                        .collect(),
                )
            };

            let cidr_associations: Option<Vec<VpcCidrBlockAssociation>> = if v
                .cidr_block_association_set()
                .is_empty()
            {
                None
            } else {
                Some(
                    v.cidr_block_association_set()
                        .iter()
                        .map(|a| VpcCidrBlockAssociation {
                            association_id: a.association_id().unwrap_or_default().to_string(),
                            cidr_block: a.cidr_block().unwrap_or_default().to_string(),
                            cidr_block_state: a.cidr_block_state().map(|s| VpcCidrBlockState {
                                state: s.state().map(|st| st.as_str().to_string()).unwrap_or_default(),
                                status_message: s.status_message().map(|m| m.to_string()),
                            }),
                        })
                        .collect(),
                )
            };

            Vpc {
                vpc_id: v.vpc_id().unwrap_or_default().to_string(),
                cidr_block: v.cidr_block().unwrap_or_default().to_string(),
                dhcp_options_id: v.dhcp_options_id().map(|s| s.to_string()),
                state: v
                    .state()
                    .map(|s| s.as_str().to_string())
                    .unwrap_or_default(),
                owner_id: v.owner_id().map(|s| s.to_string()),
                instance_tenancy: v
                    .instance_tenancy()
                    .map(|t| t.as_str().to_string())
                    .unwrap_or_else(|| "default".to_string()),
                ipv6_cidr_block_association_set: ipv6_associations,
                cidr_block_association_set: cidr_associations,
                is_default: v.is_default().unwrap_or(false),
                tags: convert_tags(Some(v.tags())),
            }
        })
        .collect();

    Ok(DescribeVpcsOutput { vpcs })
}

/// Modify VPC Attribute
///
/// Modifies the specified attribute of the specified VPC.
///
pub async fn modify_vpc_attribute(
    vpc_id: &str,
    enable_dns_hostnames: Option<bool>,
    enable_dns_support: Option<bool>,
    region: Option<&str>,
) -> Result<ModifyVpcAttributeOutput, String> {
    let client = create_client(region).await?;

    if let Some(dns_hostnames) = enable_dns_hostnames {
        client
            .modify_vpc_attribute()
            .vpc_id(vpc_id)
            .enable_dns_hostnames(AttributeBooleanValue::builder().value(dns_hostnames).build())
            .send()
            .await
            .map_err(|e| format!("Failed to modify DNS hostnames: {}", e))?;
    }

    if let Some(dns_support) = enable_dns_support {
        client
            .modify_vpc_attribute()
            .vpc_id(vpc_id)
            .enable_dns_support(AttributeBooleanValue::builder().value(dns_support).build())
            .send()
            .await
            .map_err(|e| format!("Failed to modify DNS support: {}", e))?;
    }

    Ok(ModifyVpcAttributeOutput { success: true })
}

/// Create Subnet
///
/// Creates a subnet in an existing VPC.
///
pub async fn create_subnet(
    cidr_block: &str,
    vpc_id: &str,
    availability_zone: Option<&str>,
    availability_zone_id: Option<&str>,
    ipv6_cidr_block: Option<&str>,
    map_public_ip_on_launch: Option<bool>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateSubnetOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_subnet().vpc_id(vpc_id).cidr_block(cidr_block);

    if let Some(az) = availability_zone {
        request = request.availability_zone(az);
    }

    if let Some(az_id) = availability_zone_id {
        request = request.availability_zone_id(az_id);
    }

    if let Some(ipv6) = ipv6_cidr_block {
        request = request.ipv6_cidr_block(ipv6);
    }

    if let Some(tag_spec) = tags_to_tag_specification(tags, ResourceType::Subnet) {
        request = request.tag_specifications(tag_spec);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create subnet: {}", e))?;

    let subnet = response.subnet().ok_or("No subnet in response")?;
    let subnet_id = subnet.subnet_id().unwrap_or_default().to_string();

    // If map_public_ip_on_launch is specified, modify the subnet attribute
    if let Some(map_public) = map_public_ip_on_launch {
        client
            .modify_subnet_attribute()
            .subnet_id(&subnet_id)
            .map_public_ip_on_launch(AttributeBooleanValue::builder().value(map_public).build())
            .send()
            .await
            .map_err(|e| format!("Failed to modify subnet attribute: {}", e))?;
    }

    Ok(CreateSubnetOutput {
        subnet_id,
        vpc_id: subnet.vpc_id().unwrap_or_default().to_string(),
        cidr_block: subnet.cidr_block().unwrap_or_default().to_string(),
        availability_zone: subnet.availability_zone().unwrap_or_default().to_string(),
        availability_zone_id: subnet.availability_zone_id().unwrap_or_default().to_string(),
        available_ip_address_count: subnet.available_ip_address_count().unwrap_or(0),
        default_for_az: subnet.default_for_az().unwrap_or(false),
        map_public_ip_on_launch: subnet.map_public_ip_on_launch().unwrap_or(false),
        owner_id: subnet.owner_id().unwrap_or_default().to_string(),
        state: subnet
            .state()
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| "pending".to_string()),
        subnet_arn: subnet.subnet_arn().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete Subnet
///
/// Deletes the specified subnet.
///
pub async fn delete_subnet(subnet_id: &str, region: Option<&str>) -> Result<DeleteSubnetOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_subnet()
        .subnet_id(subnet_id)
        .send()
        .await
        .map_err(|e| format!("Failed to delete subnet: {}", e))?;

    Ok(DeleteSubnetOutput { success: true })
}

/// Describe Subnets
///
/// Describes one or more of your subnets.
///
pub async fn describe_subnets(
    filters: Option<Vec<VpcFilter>>,
    region: Option<&str>,
    subnet_ids: Option<Vec<String>>,
) -> Result<DescribeSubnetsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_subnets();

    if let Some(ids) = subnet_ids {
        request = request.set_subnet_ids(Some(ids));
    }

    if let Some(f) = convert_filters(filters) {
        request = request.set_filters(Some(f));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe subnets: {}", e))?;

    let subnets: Vec<Subnet> = response
        .subnets()
        .iter()
        .map(|s| {
            let ipv6_associations: Option<Vec<SubnetIpv6CidrBlockAssociation>> = if s
                .ipv6_cidr_block_association_set()
                .is_empty()
            {
                None
            } else {
                Some(
                    s.ipv6_cidr_block_association_set()
                        .iter()
                        .map(|a| SubnetIpv6CidrBlockAssociation {
                            association_id: a.association_id().unwrap_or_default().to_string(),
                            ipv6_cidr_block: a.ipv6_cidr_block().unwrap_or_default().to_string(),
                            ipv6_cidr_block_state: a.ipv6_cidr_block_state().map(|st| SubnetCidrBlockState {
                                state: st.state().map(|state| state.as_str().to_string()).unwrap_or_default(),
                                status_message: st.status_message().map(|m| m.to_string()),
                            }),
                        })
                        .collect(),
                )
            };

            Subnet {
                subnet_id: s.subnet_id().unwrap_or_default().to_string(),
                vpc_id: s.vpc_id().unwrap_or_default().to_string(),
                cidr_block: s.cidr_block().unwrap_or_default().to_string(),
                availability_zone: s.availability_zone().map(|az| az.to_string()),
                availability_zone_id: s.availability_zone_id().map(|az| az.to_string()),
                available_ip_address_count: s.available_ip_address_count().unwrap_or(0),
                default_for_az: s.default_for_az().unwrap_or(false),
                enable_lni_at_device_index: s.enable_lni_at_device_index(),
                map_customer_owned_ip_on_launch: s.map_customer_owned_ip_on_launch().unwrap_or(false),
                map_public_ip_on_launch: s.map_public_ip_on_launch().unwrap_or(false),
                owner_id: s.owner_id().map(|o| o.to_string()),
                state: s.state().map(|st| st.as_str().to_string()).unwrap_or_default(),
                subnet_arn: s.subnet_arn().map(|a| a.to_string()),
                tags: convert_tags(Some(s.tags())),
                ipv6_cidr_block_association_set: ipv6_associations,
                outpost_arn: s.outpost_arn().map(|a| a.to_string()),
                enable_dns64: s.enable_dns64().unwrap_or(false),
                ipv6_native: s.ipv6_native().unwrap_or(false),
                private_dns_name_options_on_launch: s.private_dns_name_options_on_launch().map(|p| {
                    PrivateDnsNameOptionsOnLaunch {
                        hostname_type: p.hostname_type().map(|h| h.as_str().to_string()),
                        enable_resource_name_dns_a_record: p.enable_resource_name_dns_a_record(),
                        enable_resource_name_dns_aaaa_record: p.enable_resource_name_dns_aaaa_record(),
                    }
                }),
            }
        })
        .collect();

    Ok(DescribeSubnetsOutput { subnets })
}

/// Modify Subnet Attribute
///
/// Modifies a subnet attribute.
///
pub async fn modify_subnet_attribute(
    subnet_id: &str,
    assign_ipv6_address_on_creation: Option<bool>,
    map_public_ip_on_launch: Option<bool>,
    region: Option<&str>,
) -> Result<ModifySubnetAttributeOutput, String> {
    let client = create_client(region).await?;

    if let Some(assign_ipv6) = assign_ipv6_address_on_creation {
        client
            .modify_subnet_attribute()
            .subnet_id(subnet_id)
            .assign_ipv6_address_on_creation(AttributeBooleanValue::builder().value(assign_ipv6).build())
            .send()
            .await
            .map_err(|e| format!("Failed to modify assign IPv6 address on creation: {}", e))?;
    }

    if let Some(map_public) = map_public_ip_on_launch {
        client
            .modify_subnet_attribute()
            .subnet_id(subnet_id)
            .map_public_ip_on_launch(AttributeBooleanValue::builder().value(map_public).build())
            .send()
            .await
            .map_err(|e| format!("Failed to modify map public IP on launch: {}", e))?;
    }

    Ok(ModifySubnetAttributeOutput { success: true })
}

/// Create Internet Gateway
///
/// Creates an internet gateway for use with a VPC.
///
pub async fn create_internet_gateway(
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateInternetGatewayOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_internet_gateway();

    if let Some(tag_spec) = tags_to_tag_specification(tags, ResourceType::InternetGateway) {
        request = request.tag_specifications(tag_spec);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create internet gateway: {}", e))?;

    let igw = response.internet_gateway().ok_or("No internet gateway in response")?;

    let attachments: Vec<InternetGatewayAttachment> = igw
        .attachments()
        .iter()
        .map(|a| InternetGatewayAttachment {
            state: a.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
            vpc_id: a.vpc_id().unwrap_or_default().to_string(),
        })
        .collect();

    Ok(CreateInternetGatewayOutput {
        internet_gateway_id: igw.internet_gateway_id().unwrap_or_default().to_string(),
        attachments,
        owner_id: igw.owner_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete Internet Gateway
///
/// Deletes the specified internet gateway.
///
pub async fn delete_internet_gateway(
    internet_gateway_id: &str,
    region: Option<&str>,
) -> Result<DeleteInternetGatewayOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_internet_gateway()
        .internet_gateway_id(internet_gateway_id)
        .send()
        .await
        .map_err(|e| format!("Failed to delete internet gateway: {}", e))?;

    Ok(DeleteInternetGatewayOutput { success: true })
}

/// Attach Internet Gateway
///
/// Attaches an internet gateway to a VPC.
///
pub async fn attach_internet_gateway(
    internet_gateway_id: &str,
    vpc_id: &str,
    region: Option<&str>,
) -> Result<AttachInternetGatewayOutput, String> {
    let client = create_client(region).await?;

    client
        .attach_internet_gateway()
        .internet_gateway_id(internet_gateway_id)
        .vpc_id(vpc_id)
        .send()
        .await
        .map_err(|e| format!("Failed to attach internet gateway: {}", e))?;

    Ok(AttachInternetGatewayOutput { success: true })
}

/// Detach Internet Gateway
///
/// Detaches an internet gateway from a VPC.
///
pub async fn detach_internet_gateway(
    internet_gateway_id: &str,
    vpc_id: &str,
    region: Option<&str>,
) -> Result<DetachInternetGatewayOutput, String> {
    let client = create_client(region).await?;

    client
        .detach_internet_gateway()
        .internet_gateway_id(internet_gateway_id)
        .vpc_id(vpc_id)
        .send()
        .await
        .map_err(|e| format!("Failed to detach internet gateway: {}", e))?;

    Ok(DetachInternetGatewayOutput { success: true })
}

/// Describe Internet Gateways
///
/// Describes one or more of your internet gateways.
///
pub async fn describe_internet_gateways(
    filters: Option<Vec<VpcFilter>>,
    internet_gateway_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeInternetGatewaysOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_internet_gateways();

    if let Some(ids) = internet_gateway_ids {
        request = request.set_internet_gateway_ids(Some(ids));
    }

    if let Some(f) = convert_filters(filters) {
        request = request.set_filters(Some(f));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe internet gateways: {}", e))?;

    let internet_gateways: Vec<InternetGateway> = response
        .internet_gateways()
        .iter()
        .map(|igw| {
            let attachments: Option<Vec<InternetGatewayAttachment>> = if igw.attachments().is_empty() {
                None
            } else {
                Some(
                    igw.attachments()
                        .iter()
                        .map(|a| InternetGatewayAttachment {
                            state: a.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
                            vpc_id: a.vpc_id().unwrap_or_default().to_string(),
                        })
                        .collect(),
                )
            };

            InternetGateway {
                internet_gateway_id: igw.internet_gateway_id().unwrap_or_default().to_string(),
                attachments,
                owner_id: igw.owner_id().map(|o| o.to_string()),
                tags: convert_tags(Some(igw.tags())),
            }
        })
        .collect();

    Ok(DescribeInternetGatewaysOutput { internet_gateways })
}

/// Create NAT Gateway
///
/// Creates a NAT gateway in the specified subnet.
///
pub async fn create_nat_gateway(
    subnet_id: &str,
    allocation_id: Option<&str>,
    connectivity_type: Option<&str>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateNatGatewayOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_nat_gateway().subnet_id(subnet_id);

    if let Some(alloc_id) = allocation_id {
        request = request.allocation_id(alloc_id);
    }

    if let Some(conn_type) = connectivity_type {
        let conn_type_value = match conn_type.to_lowercase().as_str() {
            "private" => aws_sdk_ec2::types::ConnectivityType::Private,
            _ => aws_sdk_ec2::types::ConnectivityType::Public,
        };
        request = request.connectivity_type(conn_type_value);
    }

    if let Some(tag_spec) = tags_to_tag_specification(tags, ResourceType::Natgateway) {
        request = request.tag_specifications(tag_spec);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create NAT gateway: {}", e))?;

    let nat = response.nat_gateway().ok_or("No NAT gateway in response")?;

    let nat_gateway_addresses: Vec<NatGatewayAddress> = nat
        .nat_gateway_addresses()
        .iter()
        .map(|a| NatGatewayAddress {
            allocation_id: a.allocation_id().map(|s| s.to_string()),
            network_interface_id: a.network_interface_id().map(|s| s.to_string()),
            private_ip: a.private_ip().map(|s| s.to_string()),
            public_ip: a.public_ip().map(|s| s.to_string()),
            association_id: a.association_id().map(|s| s.to_string()),
            is_primary: a.is_primary(),
            failure_message: a.failure_message().map(|s| s.to_string()),
            status: a.status().map(|s| s.as_str().to_string()),
        })
        .collect();

    let create_time: DateTime<Utc> = nat
        .create_time()
        .map(|t| {
            DateTime::from_timestamp(t.secs(), t.subsec_nanos())
                .unwrap_or_else(Utc::now)
        })
        .unwrap_or_else(Utc::now);

    Ok(CreateNatGatewayOutput {
        nat_gateway_id: nat.nat_gateway_id().unwrap_or_default().to_string(),
        subnet_id: nat.subnet_id().unwrap_or_default().to_string(),
        vpc_id: nat.vpc_id().unwrap_or_default().to_string(),
        connectivity_type: nat
            .connectivity_type()
            .map(|c| c.as_str().to_string())
            .unwrap_or_else(|| "public".to_string()),
        create_time,
        nat_gateway_addresses,
        state: nat
            .state()
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| "pending".to_string()),
        success: true,
    })
}

/// Delete NAT Gateway
///
/// Deletes the specified NAT gateway.
///
pub async fn delete_nat_gateway(
    nat_gateway_id: &str,
    region: Option<&str>,
) -> Result<DeleteNatGatewayOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .delete_nat_gateway()
        .nat_gateway_id(nat_gateway_id)
        .send()
        .await
        .map_err(|e| format!("Failed to delete NAT gateway: {}", e))?;

    Ok(DeleteNatGatewayOutput {
        nat_gateway_id: response.nat_gateway_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Describe NAT Gateways
///
/// Describes one or more of your NAT gateways.
///
pub async fn describe_nat_gateways(
    filters: Option<Vec<VpcFilter>>,
    nat_gateway_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeNatGatewaysOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_nat_gateways();

    if let Some(ids) = nat_gateway_ids {
        request = request.set_nat_gateway_ids(Some(ids));
    }

    if let Some(f) = convert_filters(filters) {
        request = request.set_filter(Some(f));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe NAT gateways: {}", e))?;

    let nat_gateways: Vec<NatGateway> = response
        .nat_gateways()
        .iter()
        .map(|nat| {
            let nat_gateway_addresses: Option<Vec<NatGatewayAddress>> = if nat
                .nat_gateway_addresses()
                .is_empty()
            {
                None
            } else {
                Some(
                    nat.nat_gateway_addresses()
                        .iter()
                        .map(|a| NatGatewayAddress {
                            allocation_id: a.allocation_id().map(|s| s.to_string()),
                            network_interface_id: a.network_interface_id().map(|s| s.to_string()),
                            private_ip: a.private_ip().map(|s| s.to_string()),
                            public_ip: a.public_ip().map(|s| s.to_string()),
                            association_id: a.association_id().map(|s| s.to_string()),
                            is_primary: a.is_primary(),
                            failure_message: a.failure_message().map(|s| s.to_string()),
                            status: a.status().map(|s| s.as_str().to_string()),
                        })
                        .collect(),
                )
            };

            let create_time: Option<DateTime<Utc>> = nat.create_time().map(|t| {
                DateTime::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_else(Utc::now)
            });

            let delete_time: Option<DateTime<Utc>> = nat.delete_time().map(|t| {
                DateTime::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_else(Utc::now)
            });

            let provisioned_bandwidth: Option<ProvisionedBandwidth> =
                nat.provisioned_bandwidth().map(|p| ProvisionedBandwidth {
                    provisioned: p.provisioned().map(|s| s.to_string()),
                    provision_time: p.provision_time().map(|t| {
                        DateTime::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_else(Utc::now)
                    }),
                    requested: p.requested().map(|s| s.to_string()),
                    request_time: p.request_time().map(|t| {
                        DateTime::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_else(Utc::now)
                    }),
                    status: p.status().map(|s| s.to_string()),
                });

            NatGateway {
                nat_gateway_id: nat.nat_gateway_id().unwrap_or_default().to_string(),
                subnet_id: nat.subnet_id().unwrap_or_default().to_string(),
                vpc_id: nat.vpc_id().unwrap_or_default().to_string(),
                state: nat
                    .state()
                    .map(|s| s.as_str().to_string())
                    .unwrap_or_default(),
                connectivity_type: nat.connectivity_type().map(|c| c.as_str().to_string()),
                create_time,
                delete_time,
                failure_code: nat.failure_code().map(|s| s.to_string()),
                failure_message: nat.failure_message().map(|s| s.to_string()),
                nat_gateway_addresses,
                provisioned_bandwidth,
                tags: convert_tags(Some(nat.tags())),
            }
        })
        .collect();

    Ok(DescribeNatGatewaysOutput { nat_gateways })
}

/// Create Route Table
///
/// Creates a route table for the specified VPC.
///
pub async fn create_route_table(
    vpc_id: &str,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<CreateRouteTableOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_route_table().vpc_id(vpc_id);

    if let Some(tag_spec) = tags_to_tag_specification(tags, ResourceType::RouteTable) {
        request = request.tag_specifications(tag_spec);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create route table: {}", e))?;

    let rt = response.route_table().ok_or("No route table in response")?;

    let associations: Vec<RouteTableAssociation> = rt
        .associations()
        .iter()
        .map(|a| RouteTableAssociation {
            main: a.main().unwrap_or(false),
            route_table_association_id: a.route_table_association_id().map(|s| s.to_string()),
            route_table_id: a.route_table_id().map(|s| s.to_string()),
            subnet_id: a.subnet_id().map(|s| s.to_string()),
            gateway_id: a.gateway_id().map(|s| s.to_string()),
            association_state: a.association_state().map(|s| RouteTableAssociationState {
                state: s.state().map(|st| st.as_str().to_string()).unwrap_or_default(),
                status_message: s.status_message().map(|m| m.to_string()),
            }),
        })
        .collect();

    let propagating_vgws: Vec<PropagatingVgw> = rt
        .propagating_vgws()
        .iter()
        .map(|v| PropagatingVgw {
            gateway_id: v.gateway_id().unwrap_or_default().to_string(),
        })
        .collect();

    let routes: Vec<Route> = rt
        .routes()
        .iter()
        .map(|r| Route {
            destination_cidr_block: r.destination_cidr_block().map(|s| s.to_string()),
            destination_ipv6_cidr_block: r.destination_ipv6_cidr_block().map(|s| s.to_string()),
            destination_prefix_list_id: r.destination_prefix_list_id().map(|s| s.to_string()),
            egress_only_internet_gateway_id: r.egress_only_internet_gateway_id().map(|s| s.to_string()),
            gateway_id: r.gateway_id().map(|s| s.to_string()),
            instance_id: r.instance_id().map(|s| s.to_string()),
            instance_owner_id: r.instance_owner_id().map(|s| s.to_string()),
            nat_gateway_id: r.nat_gateway_id().map(|s| s.to_string()),
            transit_gateway_id: r.transit_gateway_id().map(|s| s.to_string()),
            local_gateway_id: r.local_gateway_id().map(|s| s.to_string()),
            carrier_gateway_id: r.carrier_gateway_id().map(|s| s.to_string()),
            network_interface_id: r.network_interface_id().map(|s| s.to_string()),
            origin: r.origin().map(|o| o.as_str().to_string()),
            state: r.state().map(|s| s.as_str().to_string()),
            vpc_peering_connection_id: r.vpc_peering_connection_id().map(|s| s.to_string()),
            core_network_arn: r.core_network_arn().map(|s| s.to_string()),
        })
        .collect();

    Ok(CreateRouteTableOutput {
        route_table_id: rt.route_table_id().unwrap_or_default().to_string(),
        vpc_id: rt.vpc_id().unwrap_or_default().to_string(),
        owner_id: rt.owner_id().unwrap_or_default().to_string(),
        associations,
        propagating_vgws,
        routes,
        success: true,
    })
}

/// Delete Route Table
///
/// Deletes the specified route table.
///
pub async fn delete_route_table(
    route_table_id: &str,
    region: Option<&str>,
) -> Result<DeleteRouteTableOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_route_table()
        .route_table_id(route_table_id)
        .send()
        .await
        .map_err(|e| format!("Failed to delete route table: {}", e))?;

    Ok(DeleteRouteTableOutput { success: true })
}

/// Describe Route Tables
///
/// Describes one or more of your route tables.
///
pub async fn describe_route_tables(
    filters: Option<Vec<VpcFilter>>,
    region: Option<&str>,
    route_table_ids: Option<Vec<String>>,
) -> Result<DescribeRouteTablesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_route_tables();

    if let Some(ids) = route_table_ids {
        request = request.set_route_table_ids(Some(ids));
    }

    if let Some(f) = convert_filters(filters) {
        request = request.set_filters(Some(f));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe route tables: {}", e))?;

    let route_tables: Vec<RouteTable> = response
        .route_tables()
        .iter()
        .map(|rt| {
            let associations: Option<Vec<RouteTableAssociation>> = if rt.associations().is_empty() {
                None
            } else {
                Some(
                    rt.associations()
                        .iter()
                        .map(|a| RouteTableAssociation {
                            main: a.main().unwrap_or(false),
                            route_table_association_id: a.route_table_association_id().map(|s| s.to_string()),
                            route_table_id: a.route_table_id().map(|s| s.to_string()),
                            subnet_id: a.subnet_id().map(|s| s.to_string()),
                            gateway_id: a.gateway_id().map(|s| s.to_string()),
                            association_state: a.association_state().map(|s| RouteTableAssociationState {
                                state: s.state().map(|st| st.as_str().to_string()).unwrap_or_default(),
                                status_message: s.status_message().map(|m| m.to_string()),
                            }),
                        })
                        .collect(),
                )
            };

            let propagating_vgws: Option<Vec<PropagatingVgw>> = if rt.propagating_vgws().is_empty() {
                None
            } else {
                Some(
                    rt.propagating_vgws()
                        .iter()
                        .map(|v| PropagatingVgw {
                            gateway_id: v.gateway_id().unwrap_or_default().to_string(),
                        })
                        .collect(),
                )
            };

            let routes: Option<Vec<Route>> = if rt.routes().is_empty() {
                None
            } else {
                Some(
                    rt.routes()
                        .iter()
                        .map(|r| Route {
                            destination_cidr_block: r.destination_cidr_block().map(|s| s.to_string()),
                            destination_ipv6_cidr_block: r.destination_ipv6_cidr_block().map(|s| s.to_string()),
                            destination_prefix_list_id: r.destination_prefix_list_id().map(|s| s.to_string()),
                            egress_only_internet_gateway_id: r
                                .egress_only_internet_gateway_id()
                                .map(|s| s.to_string()),
                            gateway_id: r.gateway_id().map(|s| s.to_string()),
                            instance_id: r.instance_id().map(|s| s.to_string()),
                            instance_owner_id: r.instance_owner_id().map(|s| s.to_string()),
                            nat_gateway_id: r.nat_gateway_id().map(|s| s.to_string()),
                            transit_gateway_id: r.transit_gateway_id().map(|s| s.to_string()),
                            local_gateway_id: r.local_gateway_id().map(|s| s.to_string()),
                            carrier_gateway_id: r.carrier_gateway_id().map(|s| s.to_string()),
                            network_interface_id: r.network_interface_id().map(|s| s.to_string()),
                            origin: r.origin().map(|o| o.as_str().to_string()),
                            state: r.state().map(|s| s.as_str().to_string()),
                            vpc_peering_connection_id: r.vpc_peering_connection_id().map(|s| s.to_string()),
                            core_network_arn: r.core_network_arn().map(|s| s.to_string()),
                        })
                        .collect(),
                )
            };

            RouteTable {
                route_table_id: rt.route_table_id().unwrap_or_default().to_string(),
                vpc_id: rt.vpc_id().unwrap_or_default().to_string(),
                owner_id: rt.owner_id().map(|o| o.to_string()),
                associations,
                propagating_vgws,
                routes,
                tags: convert_tags(Some(rt.tags())),
            }
        })
        .collect();

    Ok(DescribeRouteTablesOutput { route_tables })
}

/// Create Route
///
/// Creates a route in a route table within a VPC.
///
pub async fn create_route(
    route_table_id: &str,
    destination_cidr_block: Option<&str>,
    destination_ipv6_cidr_block: Option<&str>,
    destination_prefix_list_id: Option<&str>,
    egress_only_internet_gateway_id: Option<&str>,
    gateway_id: Option<&str>,
    instance_id: Option<&str>,
    local_gateway_id: Option<&str>,
    nat_gateway_id: Option<&str>,
    network_interface_id: Option<&str>,
    region: Option<&str>,
    transit_gateway_id: Option<&str>,
    vpc_peering_connection_id: Option<&str>,
    carrier_gateway_id: Option<&str>,
    core_network_arn: Option<&str>,
) -> Result<CreateRouteOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_route().route_table_id(route_table_id);

    if let Some(cidr) = destination_cidr_block {
        request = request.destination_cidr_block(cidr);
    }
    if let Some(ipv6_cidr) = destination_ipv6_cidr_block {
        request = request.destination_ipv6_cidr_block(ipv6_cidr);
    }
    if let Some(prefix_list) = destination_prefix_list_id {
        request = request.destination_prefix_list_id(prefix_list);
    }
    if let Some(egress_igw) = egress_only_internet_gateway_id {
        request = request.egress_only_internet_gateway_id(egress_igw);
    }
    if let Some(gw) = gateway_id {
        request = request.gateway_id(gw);
    }
    if let Some(inst) = instance_id {
        request = request.instance_id(inst);
    }
    if let Some(local_gw) = local_gateway_id {
        request = request.local_gateway_id(local_gw);
    }
    if let Some(nat_gw) = nat_gateway_id {
        request = request.nat_gateway_id(nat_gw);
    }
    if let Some(eni) = network_interface_id {
        request = request.network_interface_id(eni);
    }
    if let Some(tgw) = transit_gateway_id {
        request = request.transit_gateway_id(tgw);
    }
    if let Some(pcx) = vpc_peering_connection_id {
        request = request.vpc_peering_connection_id(pcx);
    }
    if let Some(cgw) = carrier_gateway_id {
        request = request.carrier_gateway_id(cgw);
    }
    if let Some(arn) = core_network_arn {
        request = request.core_network_arn(arn);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create route: {}", e))?;

    Ok(CreateRouteOutput {
        success: response.r#return().unwrap_or(false),
    })
}

/// Delete Route
///
/// Deletes the specified route from the specified route table.
///
pub async fn delete_route(
    route_table_id: &str,
    destination_cidr_block: Option<&str>,
    destination_ipv6_cidr_block: Option<&str>,
    destination_prefix_list_id: Option<&str>,
    region: Option<&str>,
) -> Result<DeleteRouteOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.delete_route().route_table_id(route_table_id);

    if let Some(cidr) = destination_cidr_block {
        request = request.destination_cidr_block(cidr);
    }
    if let Some(ipv6_cidr) = destination_ipv6_cidr_block {
        request = request.destination_ipv6_cidr_block(ipv6_cidr);
    }
    if let Some(prefix_list) = destination_prefix_list_id {
        request = request.destination_prefix_list_id(prefix_list);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to delete route: {}", e))?;

    Ok(DeleteRouteOutput { success: true })
}

/// Replace Route
///
/// Replaces an existing route within a route table in a VPC.
///
pub async fn replace_route(
    route_table_id: &str,
    destination_cidr_block: Option<&str>,
    destination_ipv6_cidr_block: Option<&str>,
    destination_prefix_list_id: Option<&str>,
    egress_only_internet_gateway_id: Option<&str>,
    gateway_id: Option<&str>,
    instance_id: Option<&str>,
    local_gateway_id: Option<&str>,
    nat_gateway_id: Option<&str>,
    network_interface_id: Option<&str>,
    region: Option<&str>,
    transit_gateway_id: Option<&str>,
    vpc_peering_connection_id: Option<&str>,
    carrier_gateway_id: Option<&str>,
    core_network_arn: Option<&str>,
) -> Result<ReplaceRouteOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.replace_route().route_table_id(route_table_id);

    if let Some(cidr) = destination_cidr_block {
        request = request.destination_cidr_block(cidr);
    }
    if let Some(ipv6_cidr) = destination_ipv6_cidr_block {
        request = request.destination_ipv6_cidr_block(ipv6_cidr);
    }
    if let Some(prefix_list) = destination_prefix_list_id {
        request = request.destination_prefix_list_id(prefix_list);
    }
    if let Some(egress_igw) = egress_only_internet_gateway_id {
        request = request.egress_only_internet_gateway_id(egress_igw);
    }
    if let Some(gw) = gateway_id {
        request = request.gateway_id(gw);
    }
    if let Some(inst) = instance_id {
        request = request.instance_id(inst);
    }
    if let Some(local_gw) = local_gateway_id {
        request = request.local_gateway_id(local_gw);
    }
    if let Some(nat_gw) = nat_gateway_id {
        request = request.nat_gateway_id(nat_gw);
    }
    if let Some(eni) = network_interface_id {
        request = request.network_interface_id(eni);
    }
    if let Some(tgw) = transit_gateway_id {
        request = request.transit_gateway_id(tgw);
    }
    if let Some(pcx) = vpc_peering_connection_id {
        request = request.vpc_peering_connection_id(pcx);
    }
    if let Some(cgw) = carrier_gateway_id {
        request = request.carrier_gateway_id(cgw);
    }
    if let Some(arn) = core_network_arn {
        request = request.core_network_arn(arn);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to replace route: {}", e))?;

    Ok(ReplaceRouteOutput { success: true })
}

/// Associate Route Table
///
/// Associates a subnet or gateway with a route table.
///
pub async fn associate_route_table(
    route_table_id: &str,
    gateway_id: Option<&str>,
    region: Option<&str>,
    subnet_id: Option<&str>,
) -> Result<AssociateRouteTableOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.associate_route_table().route_table_id(route_table_id);

    if let Some(gw) = gateway_id {
        request = request.gateway_id(gw);
    }
    if let Some(subnet) = subnet_id {
        request = request.subnet_id(subnet);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to associate route table: {}", e))?;

    let association_state = response
        .association_state()
        .map(|s| RouteTableAssociationState {
            state: s.state().map(|st| st.as_str().to_string()).unwrap_or_default(),
            status_message: s.status_message().map(|m| m.to_string()),
        })
        .unwrap_or(RouteTableAssociationState {
            state: "associated".to_string(),
            status_message: None,
        });

    Ok(AssociateRouteTableOutput {
        association_id: response.association_id().unwrap_or_default().to_string(),
        association_state,
        success: true,
    })
}

/// Disassociate Route Table
///
/// Disassociates a subnet or gateway from a route table.
///
pub async fn disassociate_route_table(
    association_id: &str,
    region: Option<&str>,
) -> Result<DisassociateRouteTableOutput, String> {
    let client = create_client(region).await?;

    client
        .disassociate_route_table()
        .association_id(association_id)
        .send()
        .await
        .map_err(|e| format!("Failed to disassociate route table: {}", e))?;

    Ok(DisassociateRouteTableOutput { success: true })
}

/// Allocate Address
///
/// Allocates an Elastic IP address to your AWS account.
///
pub async fn allocate_address(
    address: Option<&str>,
    customer_owned_ipv4_pool: Option<&str>,
    domain: Option<&str>,
    network_border_group: Option<&str>,
    public_ipv4_pool: Option<&str>,
    region: Option<&str>,
    tags: Option<VpcTags>,
) -> Result<AllocateAddressOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.allocate_address();

    if let Some(addr) = address {
        request = request.address(addr);
    }
    if let Some(pool) = customer_owned_ipv4_pool {
        request = request.customer_owned_ipv4_pool(pool);
    }
    if let Some(d) = domain {
        let domain_type = match d.to_lowercase().as_str() {
            "standard" => aws_sdk_ec2::types::DomainType::Standard,
            _ => aws_sdk_ec2::types::DomainType::Vpc,
        };
        request = request.domain(domain_type);
    }
    if let Some(nbg) = network_border_group {
        request = request.network_border_group(nbg);
    }
    if let Some(pool) = public_ipv4_pool {
        request = request.public_ipv4_pool(pool);
    }
    if let Some(tag_spec) = tags_to_tag_specification(tags, ResourceType::ElasticIp) {
        request = request.tag_specifications(tag_spec);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to allocate address: {}", e))?;

    Ok(AllocateAddressOutput {
        allocation_id: response.allocation_id().unwrap_or_default().to_string(),
        public_ip: response.public_ip().unwrap_or_default().to_string(),
        domain: response
            .domain()
            .map(|d| d.as_str().to_string())
            .unwrap_or_else(|| "vpc".to_string()),
        carrier_ip: response.carrier_ip().map(|s| s.to_string()),
        customer_owned_ip: response.customer_owned_ip().map(|s| s.to_string()),
        customer_owned_ipv4_pool: response.customer_owned_ipv4_pool().map(|s| s.to_string()),
        network_border_group: response.network_border_group().map(|s| s.to_string()),
        public_ipv4_pool: response.public_ipv4_pool().map(|s| s.to_string()),
        success: true,
    })
}

/// Release Address
///
/// Releases the specified Elastic IP address.
///
pub async fn release_address(
    allocation_id: Option<&str>,
    network_border_group: Option<&str>,
    public_ip: Option<&str>,
    region: Option<&str>,
) -> Result<ReleaseAddressOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.release_address();

    if let Some(alloc_id) = allocation_id {
        request = request.allocation_id(alloc_id);
    }
    if let Some(nbg) = network_border_group {
        request = request.network_border_group(nbg);
    }
    if let Some(ip) = public_ip {
        request = request.public_ip(ip);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to release address: {}", e))?;

    Ok(ReleaseAddressOutput { success: true })
}

/// Describe Addresses
///
/// Describes the specified Elastic IP addresses.
///
pub async fn describe_addresses(
    allocation_ids: Option<Vec<String>>,
    filters: Option<Vec<VpcFilter>>,
    public_ips: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<DescribeAddressesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.describe_addresses();

    if let Some(ids) = allocation_ids {
        request = request.set_allocation_ids(Some(ids));
    }
    if let Some(ips) = public_ips {
        request = request.set_public_ips(Some(ips));
    }
    if let Some(f) = convert_filters(filters) {
        request = request.set_filters(Some(f));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe addresses: {}", e))?;

    let addresses: Vec<Address> = response
        .addresses()
        .iter()
        .map(|a| Address {
            allocation_id: a.allocation_id().map(|s| s.to_string()),
            association_id: a.association_id().map(|s| s.to_string()),
            domain: a.domain().map(|d| d.as_str().to_string()),
            instance_id: a.instance_id().map(|s| s.to_string()),
            network_interface_id: a.network_interface_id().map(|s| s.to_string()),
            network_interface_owner_id: a.network_interface_owner_id().map(|s| s.to_string()),
            private_ip_address: a.private_ip_address().map(|s| s.to_string()),
            public_ip: a.public_ip().map(|s| s.to_string()),
            tags: convert_tags(Some(a.tags())),
            public_ipv4_pool: a.public_ipv4_pool().map(|s| s.to_string()),
            network_border_group: a.network_border_group().map(|s| s.to_string()),
            customer_owned_ip: a.customer_owned_ip().map(|s| s.to_string()),
            customer_owned_ipv4_pool: a.customer_owned_ipv4_pool().map(|s| s.to_string()),
            carrier_ip: a.carrier_ip().map(|s| s.to_string()),
        })
        .collect();

    Ok(DescribeAddressesOutput { addresses })
}

/// Associate Address
///
/// Associates an Elastic IP address with an instance or network interface.
///
pub async fn associate_address(
    allocation_id: Option<&str>,
    allow_reassociation: Option<bool>,
    instance_id: Option<&str>,
    network_interface_id: Option<&str>,
    private_ip_address: Option<&str>,
    public_ip: Option<&str>,
    region: Option<&str>,
) -> Result<AssociateAddressOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.associate_address();

    if let Some(alloc_id) = allocation_id {
        request = request.allocation_id(alloc_id);
    }
    if let Some(allow) = allow_reassociation {
        request = request.allow_reassociation(allow);
    }
    if let Some(inst) = instance_id {
        request = request.instance_id(inst);
    }
    if let Some(eni) = network_interface_id {
        request = request.network_interface_id(eni);
    }
    if let Some(private_ip) = private_ip_address {
        request = request.private_ip_address(private_ip);
    }
    if let Some(ip) = public_ip {
        request = request.public_ip(ip);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to associate address: {}", e))?;

    Ok(AssociateAddressOutput {
        association_id: response.association_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Disassociate Address
///
/// Disassociates an Elastic IP address from the instance or network interface it's associated with.
///
pub async fn disassociate_address(
    association_id: Option<&str>,
    public_ip: Option<&str>,
    region: Option<&str>,
) -> Result<DisassociateAddressOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.disassociate_address();

    if let Some(assoc_id) = association_id {
        request = request.association_id(assoc_id);
    }
    if let Some(ip) = public_ip {
        request = request.public_ip(ip);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to disassociate address: {}", e))?;

    Ok(DisassociateAddressOutput { success: true })
}

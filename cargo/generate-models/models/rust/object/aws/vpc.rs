// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_vpc
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpc {
    pub account_id: String,
    pub cidr_block: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub enable_dns_hostnames: bool,
    #[serde(default)]
    pub enable_dns_support: bool,
    pub instance_tenancy: String,
    pub ipv6_cidr_block: Option<String>,
    #[serde(default)]
    pub is_default: bool,
    pub owner_id: Option<String>,
    pub region: Option<String>,
    pub state: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

impl AwsVpc {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_subnet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcSubnet {
    pub availability_zone: Option<String>,
    pub availability_zone_id: Option<String>,
    pub available_ip_address_count: Option<i64>,
    pub cidr_block: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub default_for_az: bool,
    pub ipv6_cidr_block: Option<String>,
    #[serde(default)]
    pub map_public_ip_on_launch: bool,
    pub owner_id: Option<String>,
    pub state: String,
    pub subnet_arn: Option<String>,
    pub subnet_id: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_vpc.id
    pub vpc_id: String,
}

impl AwsVpcSubnet {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_internet_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcInternetGateway {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub internet_gateway_id: String,
    pub owner_id: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsVpcInternetGateway {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_internet_gateway_attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcInternetGatewayAttachment {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_vpc_internet_gateway.id
    pub internet_gateway_id: String,
    pub state: String,
    /// Reference: aws_vpc.id
    pub vpc_id: String,
}

impl AwsVpcInternetGatewayAttachment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_nat_gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcNatGateway {
    pub account_id: String,
    pub allocation_id: Option<String>,
    pub connectivity_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub nat_gateway_id: String,
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub state: String,
    pub subnet_id: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: Option<String>,
}

impl AwsVpcNatGateway {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_route_table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcRouteTable {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub owner_id: Option<String>,
    pub route_table_id: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_vpc.id
    pub vpc_id: String,
}

impl AwsVpcRouteTable {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcRoute {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub destination_cidr_block: Option<String>,
    pub destination_ipv6_cidr_block: Option<String>,
    pub gateway_id: Option<String>,
    pub instance_id: Option<String>,
    pub nat_gateway_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub origin: String,
    /// Reference: aws_vpc_route_table.id
    pub route_table_id: String,
    pub state: String,
    pub transit_gateway_id: Option<String>,
    pub vpc_peering_connection_id: Option<String>,
}

impl AwsVpcRoute {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_network_acl
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcNetworkAcl {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_default: bool,
    pub network_acl_id: String,
    pub owner_id: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_vpc.id
    pub vpc_id: String,
}

impl AwsVpcNetworkAcl {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_vpc_peering_connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsVpcPeeringConnection {
    pub accepter_vpc_id: Option<String>,
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub requester_vpc_id: Option<String>,
    pub status: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_peering_connection_id: String,
}

impl AwsVpcPeeringConnection {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


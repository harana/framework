// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_ec2_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Instance {
    pub account_id: String,
    pub ami_id: Option<String>,
    pub availability_zone: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub ebs_optimized: bool,
    pub iam_instance_profile: Option<String>,
    pub instance_id: String,
    pub instance_type: String,
    pub key_name: Option<String>,
    pub launch_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub monitoring_enabled: bool,
    pub platform: Option<String>,
    pub private_dns_name: Option<String>,
    pub private_ip_address: Option<String>,
    pub public_dns_name: Option<String>,
    pub public_ip_address: Option<String>,
    pub region: Option<String>,
    pub state: String,
    pub subnet_id: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: Option<String>,
}

impl AwsEc2Instance {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ec2_ami
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Ami {
    pub account_id: String,
    pub architecture: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub image_id: String,
    #[serde(default)]
    pub is_public: bool,
    pub name: Option<String>,
    pub owner_id: Option<String>,
    pub platform: Option<String>,
    pub region: Option<String>,
    pub root_device_type: String,
    pub state: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub virtualization_type: String,
}

impl AwsEc2Ami {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ec2_security_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2SecurityGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub group_id: String,
    pub group_name: String,
    pub region: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: Option<String>,
}

impl AwsEc2SecurityGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ec2_security_group_rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2SecurityGroupRule {
    pub cidr_ipv4: Option<String>,
    pub cidr_ipv6: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub direction: String,
    pub from_port: Option<i64>,
    pub ip_protocol: String,
    pub referenced_group_id: Option<String>,
    /// Reference: aws_ec2_security_group.id
    pub security_group_id: String,
    pub to_port: Option<i64>,
}

impl AwsEc2SecurityGroupRule {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ec2_key_pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2KeyPair {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fingerprint: Option<String>,
    pub key_name: String,
    pub key_pair_id: Option<String>,
    pub key_type: String,
    pub region: Option<String>,
    pub tags: Option<String>,
}

impl AwsEc2KeyPair {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ec2_elastic_ip
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2ElasticIp {
    pub account_id: String,
    pub allocation_id: String,
    pub association_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub domain: String,
    pub instance_id: Option<String>,
    pub network_interface_id: Option<String>,
    pub private_ip_address: Option<String>,
    pub public_ip: Option<String>,
    pub region: Option<String>,
    pub tags: Option<String>,
}

impl AwsEc2ElasticIp {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ec2_volume
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Volume {
    pub account_id: String,
    pub availability_zone: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub encrypted: bool,
    pub iops: Option<i64>,
    pub kms_key_id: Option<String>,
    pub region: Option<String>,
    pub size: i64,
    pub snapshot_id: Option<String>,
    pub state: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub volume_id: String,
    pub volume_type: String,
}

impl AwsEc2Volume {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ec2_snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Snapshot {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub encrypted: bool,
    pub kms_key_id: Option<String>,
    pub owner_id: Option<String>,
    pub progress: Option<String>,
    pub region: Option<String>,
    pub snapshot_id: String,
    pub state: String,
    pub tags: Option<String>,
    pub volume_id: Option<String>,
    pub volume_size: Option<i64>,
}

impl AwsEc2Snapshot {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


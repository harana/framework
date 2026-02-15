// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_rds_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsInstance {
    pub account_id: String,
    pub allocated_storage: Option<i64>,
    #[serde(default)]
    pub auto_minor_version_upgrade: bool,
    pub availability_zone: Option<String>,
    pub backup_retention_period: i64,
    pub ca_certificate_identifier: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_cluster_identifier: Option<String>,
    pub db_instance_class: String,
    pub db_instance_identifier: String,
    pub db_name: Option<String>,
    pub db_subnet_group_name: Option<String>,
    #[serde(default)]
    pub deletion_protection: bool,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i64>,
    pub engine: String,
    pub engine_version: Option<String>,
    #[serde(default)]
    pub iam_auth_enabled: bool,
    pub iops: Option<i64>,
    pub kms_key_id: Option<String>,
    pub master_username: Option<String>,
    #[serde(default)]
    pub multi_az: bool,
    #[serde(default)]
    pub performance_insights_enabled: bool,
    pub port: Option<i64>,
    #[serde(default)]
    pub publicly_accessible: bool,
    pub region: Option<String>,
    pub status: String,
    #[serde(default)]
    pub storage_encrypted: bool,
    pub storage_type: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsRdsInstance {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_rds_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsCluster {
    pub account_id: String,
    pub availability_zones: Option<String>,
    pub backup_retention_period: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database_name: Option<String>,
    pub db_cluster_identifier: String,
    #[serde(default)]
    pub deletion_protection: bool,
    pub endpoint: Option<String>,
    pub engine: String,
    pub engine_mode: String,
    pub engine_version: Option<String>,
    #[serde(default)]
    pub iam_auth_enabled: bool,
    pub kms_key_id: Option<String>,
    pub master_username: Option<String>,
    #[serde(default)]
    pub multi_az: bool,
    pub port: Option<i64>,
    pub reader_endpoint: Option<String>,
    pub region: Option<String>,
    pub status: String,
    #[serde(default)]
    pub storage_encrypted: bool,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsRdsCluster {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_rds_subnet_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsSubnetGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_subnet_group_description: Option<String>,
    pub db_subnet_group_name: String,
    pub region: Option<String>,
    pub status: Option<String>,
    pub subnet_ids: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: Option<String>,
}

impl AwsRdsSubnetGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_rds_parameter_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsParameterGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_parameter_group_family: Option<String>,
    pub db_parameter_group_name: String,
    pub description: Option<String>,
    pub region: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsRdsParameterGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_rds_snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsSnapshot {
    pub account_id: String,
    pub allocated_storage: Option<i64>,
    pub availability_zone: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_instance_identifier: Option<String>,
    pub db_snapshot_identifier: String,
    #[serde(default)]
    pub encrypted: bool,
    pub engine: Option<String>,
    pub engine_version: Option<String>,
    pub kms_key_id: Option<String>,
    pub region: Option<String>,
    pub snapshot_type: String,
    pub status: String,
    pub storage_type: Option<String>,
    pub tags: Option<String>,
}

impl AwsRdsSnapshot {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


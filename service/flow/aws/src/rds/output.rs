use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// === Action Outputs ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeDbInstancesOutput {
    pub db_instances: Vec<DBInstance>,
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebootDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDbClusterOutput {
    pub db_cluster: DBCluster,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDbClusterOutput {
    pub db_cluster: DBCluster,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeDbClustersOutput {
    pub db_clusters: Vec<DBCluster>,
    pub marker: Option<String>,
}

// === Class Types ===

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DBInstance {
    pub db_instance_identifier: String,
    pub db_instance_class: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub db_instance_status: String,
    pub master_username: Option<String>,
    pub db_name: Option<String>,
    pub endpoint: Option<Endpoint>,
    pub allocated_storage: i32,
    pub instance_create_time: Option<DateTime<Utc>>,
    pub preferred_backup_window: Option<String>,
    pub backup_retention_period: i32,
    pub db_security_groups: Vec<DBSecurityGroupMembership>,
    pub vpc_security_groups: Vec<VpcSecurityGroupMembership>,
    pub db_parameter_groups: Vec<DBParameterGroupStatus>,
    pub availability_zone: Option<String>,
    pub db_subnet_group: Option<DBSubnetGroup>,
    pub preferred_maintenance_window: Option<String>,
    pub pending_modified_values: Option<PendingModifiedValues>,
    pub latest_restorable_time: Option<DateTime<Utc>>,
    pub multi_az: bool,
    pub auto_minor_version_upgrade: bool,
    pub read_replica_source_db_instance_identifier: Option<String>,
    pub read_replica_db_instance_identifiers: Vec<String>,
    pub license_model: Option<String>,
    pub iops: Option<i32>,
    pub option_group_memberships: Vec<OptionGroupMembership>,
    pub publicly_accessible: bool,
    pub storage_type: Option<String>,
    pub db_cluster_identifier: Option<String>,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub dbi_resource_id: String,
    pub ca_certificate_identifier: Option<String>,
    pub deletion_protection: bool,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DBCluster {
    pub db_cluster_identifier: String,
    pub db_cluster_arn: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub engine_mode: Option<String>,
    pub status: String,
    pub master_username: Option<String>,
    pub database_name: Option<String>,
    pub endpoint: Option<String>,
    pub reader_endpoint: Option<String>,
    pub port: Option<i32>,
    pub allocated_storage: i32,
    pub availability_zones: Vec<String>,
    pub backup_retention_period: i32,
    pub db_cluster_members: Vec<DBClusterMember>,
    pub vpc_security_groups: Vec<VpcSecurityGroupMembership>,
    pub db_subnet_group: Option<String>,
    pub deletion_protection: bool,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub cluster_create_time: Option<DateTime<Utc>>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub address: String,
    pub port: i32,
    pub hosted_zone_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBSecurityGroupMembership {
    pub db_security_group_name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcSecurityGroupMembership {
    pub vpc_security_group_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBParameterGroupStatus {
    pub db_parameter_group_name: String,
    pub parameter_apply_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBSubnetGroup {
    pub db_subnet_group_name: String,
    pub db_subnet_group_description: Option<String>,
    pub vpc_id: Option<String>,
    pub subnet_group_status: String,
    pub subnets: Vec<Subnet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub subnet_identifier: String,
    pub subnet_availability_zone: Option<AvailabilityZone>,
    pub subnet_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityZone {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PendingModifiedValues {
    pub db_instance_class: Option<String>,
    pub allocated_storage: Option<i32>,
    pub master_user_password: Option<String>,
    pub port: Option<i32>,
    pub backup_retention_period: Option<i32>,
    pub multi_az: Option<bool>,
    pub engine_version: Option<String>,
    pub license_model: Option<String>,
    pub iops: Option<i32>,
    pub db_instance_identifier: Option<String>,
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionGroupMembership {
    pub option_group_name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBClusterMember {
    pub db_instance_identifier: String,
    pub is_cluster_writer: bool,
    pub db_cluster_parameter_group_status: Option<String>,
    pub promotion_tier: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdsFilter {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudwatchLogsExportConfiguration {
    pub enable_log_types: Vec<String>,
    pub disable_log_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorFeature {
    pub name: String,
    pub value: String,
}

// === Enums ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    Standard,
    Gp2,
    Gp3,
    Io1,
    Io2,
}

impl Default for StorageType {
    fn default() -> Self {
        StorageType::Gp2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    IPV4,
    DUAL,
}

// === Internal Storage Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredDbInstance {
    pub db_instance_identifier: String,
    pub db_instance_class: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub status: String,
    pub master_username: Option<String>,
    pub master_user_password: Option<String>,
    pub db_name: Option<String>,
    pub allocated_storage: i32,
    pub max_allocated_storage: Option<i32>,
    pub backup_retention_period: i32,
    pub availability_zone: Option<String>,
    pub multi_az: bool,
    pub auto_minor_version_upgrade: bool,
    pub publicly_accessible: bool,
    pub storage_type: StorageType,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub iops: Option<i32>,
    pub port: i32,
    pub db_cluster_identifier: Option<String>,
    pub db_subnet_group_name: Option<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub db_parameter_group_name: Option<String>,
    pub deletion_protection: bool,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredDbCluster {
    pub db_cluster_identifier: String,
    pub db_cluster_arn: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub engine_mode: Option<String>,
    pub status: String,
    pub master_username: Option<String>,
    pub database_name: Option<String>,
    pub port: i32,
    pub allocated_storage: i32,
    pub availability_zones: Vec<String>,
    pub backup_retention_period: i32,
    pub db_subnet_group_name: Option<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub deletion_protection: bool,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub endpoint: Option<String>,
    pub reader_endpoint: Option<String>,
    pub members: Vec<String>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub region: String,
}

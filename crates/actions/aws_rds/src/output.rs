// Harana Actions - AWS RDS Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// create_db_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

// delete_db_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

// describe_db_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeDbInstancesOutput {
    pub db_instances: Vec<DBInstance>,
    pub marker: Option<String>,
}

// modify_db_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

// start_db_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

// stop_db_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

// reboot_db_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebootDbInstanceOutput {
    pub db_instance: DBInstance,
    pub success: bool,
}

// create_db_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDbClusterOutput {
    pub db_cluster: DBCluster,
    pub success: bool,
}

// delete_db_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDbClusterOutput {
    pub db_cluster: DBCluster,
    pub success: bool,
}

// describe_db_clusters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeDbClustersOutput {
    pub db_clusters: Vec<DBCluster>,
    pub marker: Option<String>,
}

// modify_db_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyDbClusterOutput {
    pub db_cluster: DBCluster,
    pub success: bool,
}

// start_db_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartDbClusterOutput {
    pub db_cluster: DBCluster,
    pub success: bool,
}

// stop_db_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopDbClusterOutput {
    pub db_cluster: DBCluster,
    pub success: bool,
}

// create_db_snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDbSnapshotOutput {
    pub db_snapshot: DBSnapshot,
    pub success: bool,
}

// delete_db_snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDbSnapshotOutput {
    pub db_snapshot: DBSnapshot,
    pub success: bool,
}

// describe_db_snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeDbSnapshotsOutput {
    pub db_snapshots: Vec<DBSnapshot>,
    pub marker: Option<String>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBInstance {
    pub db_instance_identifier: String,
    pub db_instance_class: String,
    pub engine: String,
    pub db_instance_status: String,
    pub master_username: Option<String>,
    pub db_name: Option<String>,
    pub endpoint: Option<Endpoint>,
    pub allocated_storage: i32,
    pub instance_create_time: Option<DateTime<Utc>>,
    pub preferred_backup_window: Option<String>,
    pub backup_retention_period: i32,
    pub db_security_groups: Option<Vec<DBSecurityGroupMembership>>,
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
    pub db_parameter_groups: Option<Vec<DBParameterGroupStatus>>,
    pub availability_zone: Option<String>,
    pub db_subnet_group: Option<DBSubnetGroup>,
    pub preferred_maintenance_window: Option<String>,
    pub pending_modified_values: Option<PendingModifiedValues>,
    pub latest_restorable_time: Option<DateTime<Utc>>,
    pub multi_az: bool,
    pub engine_version: Option<String>,
    pub auto_minor_version_upgrade: bool,
    pub read_replica_source_db_instance_identifier: Option<String>,
    pub read_replica_db_instance_identifiers: Option<Vec<String>>,
    pub license_model: Option<String>,
    pub iops: Option<i32>,
    pub option_group_memberships: Option<Vec<OptionGroupMembership>>,
    pub character_set_name: Option<String>,
    pub secondary_availability_zone: Option<String>,
    pub publicly_accessible: bool,
    pub status_infos: Option<Vec<DBInstanceStatusInfo>>,
    pub storage_type: Option<String>,
    pub tde_credential_arn: Option<String>,
    pub db_instance_port: i32,
    pub db_cluster_identifier: Option<String>,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub dbi_resource_id: Option<String>,
    pub ca_certificate_identifier: Option<String>,
    pub domain_memberships: Option<Vec<DomainMembership>>,
    pub copy_tags_to_snapshot: bool,
    pub monitoring_interval: Option<i32>,
    pub enhanced_monitoring_resource_arn: Option<String>,
    pub monitoring_role_arn: Option<String>,
    pub promotion_tier: Option<i32>,
    pub db_instance_arn: Option<String>,
    pub timezone: Option<String>,
    pub iam_database_authentication_enabled: bool,
    pub performance_insights_enabled: Option<bool>,
    pub performance_insights_kms_key_id: Option<String>,
    pub performance_insights_retention_period: Option<i32>,
    pub enabled_cloudwatch_logs_exports: Option<Vec<String>>,
    pub deletion_protection: bool,
    pub max_allocated_storage: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBCluster {
    pub db_cluster_identifier: String,
    pub db_cluster_arn: Option<String>,
    pub status: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub engine_mode: Option<String>,
    pub allocated_storage: Option<i32>,
    pub availability_zones: Option<Vec<String>>,
    pub backup_retention_period: Option<i32>,
    pub character_set_name: Option<String>,
    pub database_name: Option<String>,
    pub db_cluster_parameter_group: Option<String>,
    pub db_subnet_group: Option<String>,
    pub db_cluster_members: Option<Vec<DBClusterMember>>,
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
    pub endpoint: Option<String>,
    pub reader_endpoint: Option<String>,
    pub multi_az: Option<bool>,
    pub master_username: Option<String>,
    pub port: Option<i32>,
    pub preferred_backup_window: Option<String>,
    pub preferred_maintenance_window: Option<String>,
    pub storage_encrypted: Option<bool>,
    pub kms_key_id: Option<String>,
    pub cluster_create_time: Option<DateTime<Utc>>,
    pub iam_database_authentication_enabled: Option<bool>,
    pub deletion_protection: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBSnapshot {
    pub db_snapshot_identifier: String,
    pub db_instance_identifier: String,
    pub snapshot_create_time: Option<DateTime<Utc>>,
    pub engine: String,
    pub allocated_storage: i32,
    pub status: String,
    pub port: i32,
    pub availability_zone: Option<String>,
    pub vpc_id: Option<String>,
    pub instance_create_time: Option<DateTime<Utc>>,
    pub master_username: Option<String>,
    pub engine_version: Option<String>,
    pub license_model: Option<String>,
    pub snapshot_type: Option<String>,
    pub iops: Option<i32>,
    pub option_group_name: Option<String>,
    pub percent_progress: i32,
    pub source_region: Option<String>,
    pub source_db_snapshot_identifier: Option<String>,
    pub storage_type: Option<String>,
    pub tde_credential_arn: Option<String>,
    pub encrypted: bool,
    pub kms_key_id: Option<String>,
    pub db_snapshot_arn: Option<String>,
    pub timezone: Option<String>,
    pub iam_database_authentication_enabled: bool,
    pub processor_features: Option<Vec<ProcessorFeature>>,
    pub dbi_resource_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub address: Option<String>,
    pub port: Option<i32>,
    pub hosted_zone_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBSecurityGroupMembership {
    pub db_security_group_name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcSecurityGroupMembership {
    pub vpc_security_group_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBParameterGroupStatus {
    pub db_parameter_group_name: Option<String>,
    pub parameter_apply_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBSubnetGroup {
    pub db_subnet_group_name: Option<String>,
    pub db_subnet_group_description: Option<String>,
    pub vpc_id: Option<String>,
    pub subnet_group_status: Option<String>,
    pub subnets: Option<Vec<Subnet>>,
    pub db_subnet_group_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub subnet_identifier: Option<String>,
    pub subnet_availability_zone: Option<AvailabilityZone>,
    pub subnet_outpost: Option<Outpost>,
    pub subnet_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityZone {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outpost {
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub ca_certificate_identifier: Option<String>,
    pub db_subnet_group_name: Option<String>,
    pub pending_cloudwatch_logs_exports: Option<Value>,
    pub processor_features: Option<Vec<ProcessorFeature>>,
    pub iam_database_authentication_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionGroupMembership {
    pub option_group_name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBInstanceStatusInfo {
    pub status_type: Option<String>,
    pub normal: Option<bool>,
    pub status: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainMembership {
    pub domain: Option<String>,
    pub status: Option<String>,
    pub fqdn: Option<String>,
    pub iam_role_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBClusterMember {
    pub db_instance_identifier: Option<String>,
    pub is_cluster_writer: Option<bool>,
    pub db_cluster_parameter_group_status: Option<String>,
    pub promotion_tier: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorFeature {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudwatchLogsExportConfiguration {
    pub enable_log_types: Option<Vec<String>>,
    pub disable_log_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdsFilter {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdsTags {
    pub tags: HashMap<String, String>,
}

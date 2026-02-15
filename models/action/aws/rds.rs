// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbInstanceInput {
    pub allocated_storage: i64,
    #[serde(default)]
    pub auto_minor_version_upgrade: bool,
    pub availability_zone: String,
    pub backup_retention_period: i64,
    pub ca_certificate_identifier: String,
    pub character_set_name: String,
    #[serde(default)]
    pub copy_tags_to_snapshot: bool,
    pub db_cluster_identifier: String,
    pub db_instance_class: String,
    pub db_instance_identifier: String,
    pub db_name: String,
    pub db_parameter_group_name: String,
    pub db_security_groups: Vec<String>,
    pub db_subnet_group_name: String,
    #[serde(default)]
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_cloudwatch_logs_exports: Vec<String>,
    #[serde(default)]
    pub enable_customer_owned_ip: bool,
    #[serde(default)]
    pub enable_iam_database_authentication: bool,
    #[serde(default)]
    pub enable_performance_insights: bool,
    pub engine: String,
    pub engine_version: String,
    pub iops: i64,
    pub kms_key_id: String,
    pub license_model: String,
    pub manage_master_user_password: bool,
    pub master_user_password: String,
    pub master_user_secret_kms_key_id: String,
    pub master_username: String,
    pub max_allocated_storage: i64,
    pub monitoring_interval: i64,
    pub monitoring_role_arn: String,
    #[serde(default)]
    pub multi_az: bool,
    pub nchar_character_set_name: String,
    pub network_type: String,
    pub option_group_name: String,
    pub performance_insights_kms_key_id: String,
    pub performance_insights_retention_period: i64,
    pub port: i64,
    pub preferred_backup_window: String,
    pub preferred_maintenance_window: String,
    pub processor_features: Vec<String>,
    #[serde(default)]
    pub publicly_accessible: bool,
    pub region: String,
    #[serde(default)]
    pub storage_encrypted: bool,
    pub storage_throughput: i64,
    pub storage_type: String,
    pub tags: String,
    pub tde_credential_arn: String,
    pub tde_credential_password: String,
    pub timezone: String,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbInstanceOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbInstanceInput {
    pub db_instance_identifier: String,
    #[serde(default)]
    pub delete_automated_backups: bool,
    pub final_db_snapshot_identifier: String,
    pub region: String,
    #[serde(default)]
    pub skip_final_snapshot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbInstanceOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbInstancesInput {
    pub db_instance_identifier: String,
    pub filters: Vec<String>,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbInstancesOutput {
    pub db_instances: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbInstanceInput {
    pub allocated_storage: i64,
    #[serde(default)]
    pub allow_major_version_upgrade: bool,
    #[serde(default)]
    pub apply_immediately: bool,
    pub auto_minor_version_upgrade: bool,
    pub automation_mode: String,
    pub backup_retention_period: i64,
    pub ca_certificate_identifier: String,
    pub cloudwatch_logs_export_configuration: String,
    pub copy_tags_to_snapshot: bool,
    pub db_instance_class: String,
    pub db_instance_identifier: String,
    pub db_parameter_group_name: String,
    pub db_port_number: i64,
    pub db_security_groups: Vec<String>,
    pub db_subnet_group_name: String,
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_customer_owned_ip: bool,
    pub enable_iam_database_authentication: bool,
    pub enable_performance_insights: bool,
    pub engine_version: String,
    pub iops: i64,
    pub license_model: String,
    pub manage_master_user_password: bool,
    pub master_user_password: String,
    pub master_user_secret_kms_key_id: String,
    pub max_allocated_storage: i64,
    pub monitoring_interval: i64,
    pub monitoring_role_arn: String,
    pub multi_az: bool,
    pub network_type: String,
    pub new_db_instance_identifier: String,
    pub option_group_name: String,
    pub performance_insights_kms_key_id: String,
    pub performance_insights_retention_period: i64,
    pub preferred_backup_window: String,
    pub preferred_maintenance_window: String,
    pub processor_features: Vec<String>,
    pub promotion_tier: i64,
    pub publicly_accessible: bool,
    pub region: String,
    pub replica_mode: String,
    pub resume_full_automation_mode_minutes: i64,
    pub rotate_master_user_password: bool,
    pub storage_throughput: i64,
    pub storage_type: String,
    pub tde_credential_arn: String,
    pub tde_credential_password: String,
    pub use_default_processor_features: bool,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbInstanceOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartDbInstanceInput {
    pub db_instance_identifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartDbInstanceOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopDbInstanceInput {
    pub db_instance_identifier: String,
    pub db_snapshot_identifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopDbInstanceOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RebootDbInstanceInput {
    pub db_instance_identifier: String,
    #[serde(default)]
    pub force_failover: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RebootDbInstanceOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbClusterInput {
    pub allocated_storage: i64,
    #[serde(default)]
    pub auto_minor_version_upgrade: bool,
    pub availability_zones: Vec<String>,
    pub backtrack_window: i64,
    pub backup_retention_period: i64,
    pub character_set_name: String,
    #[serde(default)]
    pub copy_tags_to_snapshot: bool,
    pub database_name: String,
    pub db_cluster_identifier: String,
    pub db_cluster_instance_class: String,
    pub db_cluster_parameter_group_name: String,
    pub db_subnet_group_name: String,
    pub db_system_id: String,
    #[serde(default)]
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_cloudwatch_logs_exports: Vec<String>,
    #[serde(default)]
    pub enable_global_write_forwarding: bool,
    #[serde(default)]
    pub enable_http_endpoint: bool,
    #[serde(default)]
    pub enable_iam_database_authentication: bool,
    #[serde(default)]
    pub enable_performance_insights: bool,
    pub engine: String,
    pub engine_mode: String,
    pub engine_version: String,
    pub global_cluster_identifier: String,
    pub iops: i64,
    pub kms_key_id: String,
    pub manage_master_user_password: bool,
    pub master_user_password: String,
    pub master_user_secret_kms_key_id: String,
    pub master_username: String,
    pub monitoring_interval: i64,
    pub monitoring_role_arn: String,
    pub network_type: String,
    pub option_group_name: String,
    pub performance_insights_kms_key_id: String,
    pub performance_insights_retention_period: i64,
    pub port: i64,
    pub preferred_backup_window: String,
    pub preferred_maintenance_window: String,
    #[serde(default)]
    pub publicly_accessible: bool,
    pub region: String,
    pub replication_source_identifier: String,
    pub scaling_configuration: String,
    pub serverless_v2_scaling_configuration: String,
    #[serde(default)]
    pub storage_encrypted: bool,
    pub storage_type: String,
    pub tags: String,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbClusterOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbClusterInput {
    pub db_cluster_identifier: String,
    #[serde(default)]
    pub delete_automated_backups: bool,
    pub final_db_snapshot_identifier: String,
    pub region: String,
    #[serde(default)]
    pub skip_final_snapshot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbClusterOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClustersInput {
    pub db_cluster_identifier: String,
    pub filters: Vec<String>,
    #[serde(default)]
    pub include_shared: bool,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClustersOutput {
    pub db_clusters: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbClusterInput {
    pub allocated_storage: i64,
    #[serde(default)]
    pub allow_major_version_upgrade: bool,
    #[serde(default)]
    pub apply_immediately: bool,
    pub auto_minor_version_upgrade: bool,
    pub backtrack_window: i64,
    pub backup_retention_period: i64,
    pub cloudwatch_logs_export_configuration: String,
    pub copy_tags_to_snapshot: bool,
    pub db_cluster_identifier: String,
    pub db_cluster_instance_class: String,
    pub db_cluster_parameter_group_name: String,
    pub db_instance_parameter_group_name: String,
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_global_write_forwarding: bool,
    pub enable_http_endpoint: bool,
    pub enable_iam_database_authentication: bool,
    pub enable_performance_insights: bool,
    pub engine_version: String,
    pub iops: i64,
    pub manage_master_user_password: bool,
    pub master_user_password: String,
    pub master_user_secret_kms_key_id: String,
    pub monitoring_interval: i64,
    pub monitoring_role_arn: String,
    pub network_type: String,
    pub new_db_cluster_identifier: String,
    pub option_group_name: String,
    pub performance_insights_kms_key_id: String,
    pub performance_insights_retention_period: i64,
    pub port: i64,
    pub preferred_backup_window: String,
    pub preferred_maintenance_window: String,
    pub region: String,
    pub rotate_master_user_password: bool,
    pub scaling_configuration: String,
    pub serverless_v2_scaling_configuration: String,
    pub storage_type: String,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbClusterOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartDbClusterInput {
    pub db_cluster_identifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartDbClusterOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopDbClusterInput {
    pub db_cluster_identifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopDbClusterOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FailoverDbClusterInput {
    pub db_cluster_identifier: String,
    pub region: String,
    pub target_db_instance_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FailoverDbClusterOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbSnapshotInput {
    pub db_instance_identifier: String,
    pub db_snapshot_identifier: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbSnapshotOutput {
    pub db_snapshot: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbSnapshotInput {
    pub db_snapshot_identifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbSnapshotOutput {
    pub db_snapshot: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbSnapshotsInput {
    pub db_instance_identifier: String,
    pub db_snapshot_identifier: String,
    pub dbi_resource_id: String,
    pub filters: Vec<String>,
    #[serde(default)]
    pub include_public: bool,
    #[serde(default)]
    pub include_shared: bool,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
    pub snapshot_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbSnapshotsOutput {
    pub db_snapshots: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyDbSnapshotInput {
    #[serde(default)]
    pub copy_option_group: bool,
    #[serde(default)]
    pub copy_tags: bool,
    pub kms_key_id: String,
    pub option_group_name: String,
    pub pre_signed_url: String,
    pub region: String,
    pub source_db_snapshot_identifier: String,
    pub source_region: String,
    pub tags: String,
    pub target_db_snapshot_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyDbSnapshotOutput {
    pub db_snapshot: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbInstanceFromDbSnapshotInput {
    pub auto_minor_version_upgrade: bool,
    pub availability_zone: String,
    pub copy_tags_to_snapshot: bool,
    pub custom_iam_instance_profile: String,
    pub db_instance_class: String,
    pub db_instance_identifier: String,
    pub db_name: String,
    pub db_parameter_group_name: String,
    pub db_snapshot_identifier: String,
    pub db_subnet_group_name: String,
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_cloudwatch_logs_exports: Vec<String>,
    pub enable_customer_owned_ip: bool,
    pub enable_iam_database_authentication: bool,
    pub engine: String,
    pub iops: i64,
    pub license_model: String,
    pub multi_az: bool,
    pub network_type: String,
    pub option_group_name: String,
    pub port: i64,
    pub processor_features: Vec<String>,
    pub publicly_accessible: bool,
    pub region: String,
    pub storage_throughput: i64,
    pub storage_type: String,
    pub tags: String,
    pub tde_credential_arn: String,
    pub tde_credential_password: String,
    pub use_default_processor_features: bool,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbInstanceFromDbSnapshotOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbInstanceToPointInTimeInput {
    pub auto_minor_version_upgrade: bool,
    pub availability_zone: String,
    pub copy_tags_to_snapshot: bool,
    pub custom_iam_instance_profile: String,
    pub db_instance_class: String,
    pub db_name: String,
    pub db_parameter_group_name: String,
    pub db_subnet_group_name: String,
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_cloudwatch_logs_exports: Vec<String>,
    pub enable_customer_owned_ip: bool,
    pub enable_iam_database_authentication: bool,
    pub engine: String,
    pub iops: i64,
    pub license_model: String,
    pub max_allocated_storage: i64,
    pub multi_az: bool,
    pub network_type: String,
    pub option_group_name: String,
    pub port: i64,
    pub processor_features: Vec<String>,
    pub publicly_accessible: bool,
    pub region: String,
    pub restore_time: chrono::DateTime<chrono::Utc>,
    pub source_db_instance_automated_backups_arn: String,
    pub source_db_instance_identifier: String,
    pub source_dbi_resource_id: String,
    pub storage_throughput: i64,
    pub storage_type: String,
    pub tags: String,
    pub target_db_instance_identifier: String,
    pub tde_credential_arn: String,
    pub tde_credential_password: String,
    pub use_default_processor_features: bool,
    #[serde(default)]
    pub use_latest_restorable_time: bool,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbInstanceToPointInTimeOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbClusterSnapshotInput {
    pub db_cluster_identifier: String,
    pub db_cluster_snapshot_identifier: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbClusterSnapshotOutput {
    pub db_cluster_snapshot: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbClusterSnapshotInput {
    pub db_cluster_snapshot_identifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbClusterSnapshotOutput {
    pub db_cluster_snapshot: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClusterSnapshotsInput {
    pub db_cluster_identifier: String,
    pub db_cluster_snapshot_identifier: String,
    pub filters: Vec<String>,
    #[serde(default)]
    pub include_public: bool,
    #[serde(default)]
    pub include_shared: bool,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
    pub snapshot_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClusterSnapshotsOutput {
    pub db_cluster_snapshots: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyDbClusterSnapshotInput {
    #[serde(default)]
    pub copy_tags: bool,
    pub kms_key_id: String,
    pub pre_signed_url: String,
    pub region: String,
    pub source_db_cluster_snapshot_identifier: String,
    pub source_region: String,
    pub tags: String,
    pub target_db_cluster_snapshot_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyDbClusterSnapshotOutput {
    pub db_cluster_snapshot: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbClusterFromSnapshotInput {
    pub availability_zones: Vec<String>,
    pub backtrack_window: i64,
    pub copy_tags_to_snapshot: bool,
    pub database_name: String,
    pub db_cluster_identifier: String,
    pub db_cluster_instance_class: String,
    pub db_cluster_parameter_group_name: String,
    pub db_subnet_group_name: String,
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_cloudwatch_logs_exports: Vec<String>,
    pub enable_iam_database_authentication: bool,
    pub engine: String,
    pub engine_mode: String,
    pub engine_version: String,
    pub iops: i64,
    pub kms_key_id: String,
    pub network_type: String,
    pub option_group_name: String,
    pub port: i64,
    pub publicly_accessible: bool,
    pub region: String,
    pub scaling_configuration: String,
    pub serverless_v2_scaling_configuration: String,
    pub snapshot_identifier: String,
    pub storage_type: String,
    pub tags: String,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbClusterFromSnapshotOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbClusterToPointInTimeInput {
    pub backtrack_window: i64,
    pub copy_tags_to_snapshot: bool,
    pub db_cluster_identifier: String,
    pub db_cluster_instance_class: String,
    pub db_cluster_parameter_group_name: String,
    pub db_subnet_group_name: String,
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_cloudwatch_logs_exports: Vec<String>,
    pub enable_iam_database_authentication: bool,
    pub engine_mode: String,
    pub iops: i64,
    pub kms_key_id: String,
    pub network_type: String,
    pub option_group_name: String,
    pub port: i64,
    pub publicly_accessible: bool,
    pub region: String,
    pub restore_to_time: chrono::DateTime<chrono::Utc>,
    pub restore_type: String,
    pub scaling_configuration: String,
    pub serverless_v2_scaling_configuration: String,
    pub source_db_cluster_identifier: String,
    pub storage_type: String,
    pub tags: String,
    #[serde(default)]
    pub use_latest_restorable_time: bool,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreDbClusterToPointInTimeOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbSubnetGroupInput {
    pub db_subnet_group_description: String,
    pub db_subnet_group_name: String,
    pub region: String,
    pub subnet_ids: Vec<String>,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbSubnetGroupOutput {
    pub db_subnet_group: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbSubnetGroupInput {
    pub db_subnet_group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbSubnetGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbSubnetGroupsInput {
    pub db_subnet_group_name: String,
    pub filters: Vec<String>,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbSubnetGroupsOutput {
    pub db_subnet_groups: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbSubnetGroupInput {
    pub db_subnet_group_description: String,
    pub db_subnet_group_name: String,
    pub region: String,
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbSubnetGroupOutput {
    pub db_subnet_group: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbParameterGroupInput {
    pub db_parameter_group_family: String,
    pub db_parameter_group_name: String,
    pub description: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbParameterGroupOutput {
    pub db_parameter_group: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbParameterGroupInput {
    pub db_parameter_group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbParameterGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbParameterGroupsInput {
    pub db_parameter_group_name: String,
    pub filters: Vec<String>,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbParameterGroupsOutput {
    pub db_parameter_groups: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbParameterGroupInput {
    pub db_parameter_group_name: String,
    pub parameters: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbParameterGroupOutput {
    pub db_parameter_group_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResetDbParameterGroupInput {
    pub db_parameter_group_name: String,
    pub parameters: Vec<String>,
    pub region: String,
    #[serde(default)]
    pub reset_all_parameters: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResetDbParameterGroupOutput {
    pub db_parameter_group_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbParametersInput {
    pub db_parameter_group_name: String,
    pub filters: Vec<String>,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbParametersOutput {
    pub marker: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbClusterParameterGroupInput {
    pub db_cluster_parameter_group_name: String,
    pub db_parameter_group_family: String,
    pub description: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbClusterParameterGroupOutput {
    pub db_cluster_parameter_group: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbClusterParameterGroupInput {
    pub db_cluster_parameter_group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDbClusterParameterGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClusterParameterGroupsInput {
    pub db_cluster_parameter_group_name: String,
    pub filters: Vec<String>,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClusterParameterGroupsOutput {
    pub db_cluster_parameter_groups: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbClusterParameterGroupInput {
    pub db_cluster_parameter_group_name: String,
    pub parameters: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyDbClusterParameterGroupOutput {
    pub db_cluster_parameter_group_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOptionGroupInput {
    pub engine_name: String,
    pub major_engine_version: String,
    pub option_group_description: String,
    pub option_group_name: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOptionGroupOutput {
    pub option_group: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOptionGroupInput {
    pub option_group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOptionGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOptionGroupsInput {
    pub engine_name: String,
    pub filters: Vec<String>,
    pub major_engine_version: String,
    pub marker: String,
    pub max_records: i64,
    pub option_group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOptionGroupsOutput {
    pub marker: String,
    pub option_groups_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyOptionGroupInput {
    #[serde(default)]
    pub apply_immediately: bool,
    pub option_group_name: String,
    pub options_to_include: Vec<String>,
    pub options_to_remove: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyOptionGroupOutput {
    pub option_group: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbInstanceReadReplicaInput {
    pub auto_minor_version_upgrade: bool,
    pub availability_zone: String,
    pub copy_tags_to_snapshot: bool,
    pub custom_iam_instance_profile: String,
    pub db_instance_class: String,
    pub db_instance_identifier: String,
    pub db_parameter_group_name: String,
    pub db_subnet_group_name: String,
    pub deletion_protection: bool,
    pub domain: String,
    pub domain_iam_role_name: String,
    pub enable_cloudwatch_logs_exports: Vec<String>,
    pub enable_iam_database_authentication: bool,
    pub enable_performance_insights: bool,
    pub iops: i64,
    pub kms_key_id: String,
    pub max_allocated_storage: i64,
    pub monitoring_interval: i64,
    pub monitoring_role_arn: String,
    pub multi_az: bool,
    pub network_type: String,
    pub option_group_name: String,
    pub performance_insights_kms_key_id: String,
    pub performance_insights_retention_period: i64,
    pub port: i64,
    pub pre_signed_url: String,
    pub processor_features: Vec<String>,
    pub publicly_accessible: bool,
    pub region: String,
    pub replica_mode: String,
    pub source_db_instance_identifier: String,
    pub source_region: String,
    pub storage_throughput: i64,
    pub storage_type: String,
    pub tags: String,
    pub use_default_processor_features: bool,
    pub vpc_security_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDbInstanceReadReplicaOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PromoteReadReplicaInput {
    pub backup_retention_period: i64,
    pub db_instance_identifier: String,
    pub preferred_backup_window: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PromoteReadReplicaOutput {
    pub db_instance: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PromoteReadReplicaDbClusterInput {
    pub db_cluster_identifier: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PromoteReadReplicaDbClusterOutput {
    pub db_cluster: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbEngineVersionsInput {
    pub db_parameter_group_family: String,
    #[serde(default)]
    pub default_only: bool,
    pub engine: String,
    pub engine_version: String,
    pub filters: Vec<String>,
    #[serde(default)]
    pub include_all: bool,
    #[serde(default)]
    pub list_supported_character_sets: bool,
    #[serde(default)]
    pub list_supported_timezones: bool,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbEngineVersionsOutput {
    pub db_engine_versions: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOrderableDbInstanceOptionsInput {
    pub availability_zone_group: String,
    pub db_instance_class: String,
    pub engine: String,
    pub engine_version: String,
    pub filters: Vec<String>,
    pub license_model: String,
    pub marker: String,
    pub max_records: i64,
    pub region: String,
    pub vpc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOrderableDbInstanceOptionsOutput {
    pub marker: String,
    pub orderable_db_instance_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTagsToResourceInput {
    pub region: String,
    pub resource_name: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTagsToResourceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTagsFromResourceInput {
    pub region: String,
    pub resource_name: String,
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTagsFromResourceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourceInput {
    pub filters: Vec<String>,
    pub region: String,
    pub resource_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsForResourceOutput {
    pub tag_list: Vec<String>,
}

#[async_trait]
pub trait RdsAction {
    async fn create_db_instance(&self, input: CreateDbInstanceInput) -> Result<CreateDbInstanceOutput, Box<dyn std::error::Error>>;
    async fn delete_db_instance(&self, input: DeleteDbInstanceInput) -> Result<DeleteDbInstanceOutput, Box<dyn std::error::Error>>;
    async fn describe_db_instances(&self, input: DescribeDbInstancesInput) -> Result<DescribeDbInstancesOutput, Box<dyn std::error::Error>>;
    async fn modify_db_instance(&self, input: ModifyDbInstanceInput) -> Result<ModifyDbInstanceOutput, Box<dyn std::error::Error>>;
    async fn start_db_instance(&self, input: StartDbInstanceInput) -> Result<StartDbInstanceOutput, Box<dyn std::error::Error>>;
    async fn stop_db_instance(&self, input: StopDbInstanceInput) -> Result<StopDbInstanceOutput, Box<dyn std::error::Error>>;
    async fn reboot_db_instance(&self, input: RebootDbInstanceInput) -> Result<RebootDbInstanceOutput, Box<dyn std::error::Error>>;
    async fn create_db_cluster(&self, input: CreateDbClusterInput) -> Result<CreateDbClusterOutput, Box<dyn std::error::Error>>;
    async fn delete_db_cluster(&self, input: DeleteDbClusterInput) -> Result<DeleteDbClusterOutput, Box<dyn std::error::Error>>;
    async fn describe_db_clusters(&self, input: DescribeDbClustersInput) -> Result<DescribeDbClustersOutput, Box<dyn std::error::Error>>;
    async fn modify_db_cluster(&self, input: ModifyDbClusterInput) -> Result<ModifyDbClusterOutput, Box<dyn std::error::Error>>;
    async fn start_db_cluster(&self, input: StartDbClusterInput) -> Result<StartDbClusterOutput, Box<dyn std::error::Error>>;
    async fn stop_db_cluster(&self, input: StopDbClusterInput) -> Result<StopDbClusterOutput, Box<dyn std::error::Error>>;
    async fn failover_db_cluster(&self, input: FailoverDbClusterInput) -> Result<FailoverDbClusterOutput, Box<dyn std::error::Error>>;
    async fn create_db_snapshot(&self, input: CreateDbSnapshotInput) -> Result<CreateDbSnapshotOutput, Box<dyn std::error::Error>>;
    async fn delete_db_snapshot(&self, input: DeleteDbSnapshotInput) -> Result<DeleteDbSnapshotOutput, Box<dyn std::error::Error>>;
    async fn describe_db_snapshots(&self, input: DescribeDbSnapshotsInput) -> Result<DescribeDbSnapshotsOutput, Box<dyn std::error::Error>>;
    async fn copy_db_snapshot(&self, input: CopyDbSnapshotInput) -> Result<CopyDbSnapshotOutput, Box<dyn std::error::Error>>;
    async fn restore_db_instance_from_db_snapshot(&self, input: RestoreDbInstanceFromDbSnapshotInput) -> Result<RestoreDbInstanceFromDbSnapshotOutput, Box<dyn std::error::Error>>;
    async fn restore_db_instance_to_point_in_time(&self, input: RestoreDbInstanceToPointInTimeInput) -> Result<RestoreDbInstanceToPointInTimeOutput, Box<dyn std::error::Error>>;
    async fn create_db_cluster_snapshot(&self, input: CreateDbClusterSnapshotInput) -> Result<CreateDbClusterSnapshotOutput, Box<dyn std::error::Error>>;
    async fn delete_db_cluster_snapshot(&self, input: DeleteDbClusterSnapshotInput) -> Result<DeleteDbClusterSnapshotOutput, Box<dyn std::error::Error>>;
    async fn describe_db_cluster_snapshots(&self, input: DescribeDbClusterSnapshotsInput) -> Result<DescribeDbClusterSnapshotsOutput, Box<dyn std::error::Error>>;
    async fn copy_db_cluster_snapshot(&self, input: CopyDbClusterSnapshotInput) -> Result<CopyDbClusterSnapshotOutput, Box<dyn std::error::Error>>;
    async fn restore_db_cluster_from_snapshot(&self, input: RestoreDbClusterFromSnapshotInput) -> Result<RestoreDbClusterFromSnapshotOutput, Box<dyn std::error::Error>>;
    async fn restore_db_cluster_to_point_in_time(&self, input: RestoreDbClusterToPointInTimeInput) -> Result<RestoreDbClusterToPointInTimeOutput, Box<dyn std::error::Error>>;
    async fn create_db_subnet_group(&self, input: CreateDbSubnetGroupInput) -> Result<CreateDbSubnetGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_db_subnet_group(&self, input: DeleteDbSubnetGroupInput) -> Result<DeleteDbSubnetGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_db_subnet_groups(&self, input: DescribeDbSubnetGroupsInput) -> Result<DescribeDbSubnetGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_db_subnet_group(&self, input: ModifyDbSubnetGroupInput) -> Result<ModifyDbSubnetGroupOutput, Box<dyn std::error::Error>>;
    async fn create_db_parameter_group(&self, input: CreateDbParameterGroupInput) -> Result<CreateDbParameterGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_db_parameter_group(&self, input: DeleteDbParameterGroupInput) -> Result<DeleteDbParameterGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_db_parameter_groups(&self, input: DescribeDbParameterGroupsInput) -> Result<DescribeDbParameterGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_db_parameter_group(&self, input: ModifyDbParameterGroupInput) -> Result<ModifyDbParameterGroupOutput, Box<dyn std::error::Error>>;
    async fn reset_db_parameter_group(&self, input: ResetDbParameterGroupInput) -> Result<ResetDbParameterGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_db_parameters(&self, input: DescribeDbParametersInput) -> Result<DescribeDbParametersOutput, Box<dyn std::error::Error>>;
    async fn create_db_cluster_parameter_group(&self, input: CreateDbClusterParameterGroupInput) -> Result<CreateDbClusterParameterGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_db_cluster_parameter_group(&self, input: DeleteDbClusterParameterGroupInput) -> Result<DeleteDbClusterParameterGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_db_cluster_parameter_groups(&self, input: DescribeDbClusterParameterGroupsInput) -> Result<DescribeDbClusterParameterGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_db_cluster_parameter_group(&self, input: ModifyDbClusterParameterGroupInput) -> Result<ModifyDbClusterParameterGroupOutput, Box<dyn std::error::Error>>;
    async fn create_option_group(&self, input: CreateOptionGroupInput) -> Result<CreateOptionGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_option_group(&self, input: DeleteOptionGroupInput) -> Result<DeleteOptionGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_option_groups(&self, input: DescribeOptionGroupsInput) -> Result<DescribeOptionGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_option_group(&self, input: ModifyOptionGroupInput) -> Result<ModifyOptionGroupOutput, Box<dyn std::error::Error>>;
    async fn create_db_instance_read_replica(&self, input: CreateDbInstanceReadReplicaInput) -> Result<CreateDbInstanceReadReplicaOutput, Box<dyn std::error::Error>>;
    async fn promote_read_replica(&self, input: PromoteReadReplicaInput) -> Result<PromoteReadReplicaOutput, Box<dyn std::error::Error>>;
    async fn promote_read_replica_db_cluster(&self, input: PromoteReadReplicaDbClusterInput) -> Result<PromoteReadReplicaDbClusterOutput, Box<dyn std::error::Error>>;
    async fn describe_db_engine_versions(&self, input: DescribeDbEngineVersionsInput) -> Result<DescribeDbEngineVersionsOutput, Box<dyn std::error::Error>>;
    async fn describe_orderable_db_instance_options(&self, input: DescribeOrderableDbInstanceOptionsInput) -> Result<DescribeOrderableDbInstanceOptionsOutput, Box<dyn std::error::Error>>;
    async fn add_tags_to_resource(&self, input: AddTagsToResourceInput) -> Result<AddTagsToResourceOutput, Box<dyn std::error::Error>>;
    async fn remove_tags_from_resource(&self, input: RemoveTagsFromResourceInput) -> Result<RemoveTagsFromResourceOutput, Box<dyn std::error::Error>>;
    async fn list_tags_for_resource(&self, input: ListTagsForResourceInput) -> Result<ListTagsForResourceOutput, Box<dyn std::error::Error>>;
}

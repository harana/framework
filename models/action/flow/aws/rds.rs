// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbInstancesOutput {
    pub db_instances: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClustersOutput {
    pub db_clusters: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbSnapshotsOutput {
    pub db_snapshots: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClusterSnapshotsOutput {
    pub db_cluster_snapshots: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbSubnetGroupsOutput {
    pub db_subnet_groups: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbParameterGroupsOutput {
    pub db_parameter_groups: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbParametersOutput {
    pub marker: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbClusterParameterGroupsOutput {
    pub db_cluster_parameter_groups: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOptionGroupsOutput {
    pub marker: String,
    pub option_groups_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeDbEngineVersionsOutput {
    pub db_engine_versions: Vec<String>,
    pub marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOrderableDbInstanceOptionsOutput {
    pub marker: String,
    pub orderable_db_instance_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsInstance {
    pub account_id: String,
    pub allocated_storage: i64,
    #[serde(default)]
    pub auto_minor_version_upgrade: bool,
    pub availability_zone: String,
    pub backup_retention_period: i64,
    pub ca_certificate_identifier: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_cluster_identifier: String,
    pub db_instance_schema: String,
    pub db_instance_identifier: String,
    pub db_name: String,
    pub db_subnet_group_name: String,
    #[serde(default)]
    pub deletion_protection: bool,
    pub endpoint_address: String,
    pub endpoint_port: i64,
    pub engine: String,
    pub engine_version: String,
    #[serde(default)]
    pub iam_auth_enabled: bool,
    pub iops: i64,
    pub kms_key_id: String,
    pub master_username: String,
    #[serde(default)]
    pub multi_az: bool,
    #[serde(default)]
    pub performance_insights_enabled: bool,
    pub port: i64,
    #[serde(default)]
    pub publicly_accessible: bool,
    pub region: String,
    pub status: String,
    #[serde(default)]
    pub storage_encrypted: bool,
    pub storage_type: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsCluster {
    pub account_id: String,
    pub availability_zones: String,
    pub backup_retention_period: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database_name: String,
    pub db_cluster_identifier: String,
    #[serde(default)]
    pub deletion_protection: bool,
    pub endpoint: String,
    pub engine: String,
    pub engine_mode: String,
    pub engine_version: String,
    #[serde(default)]
    pub iam_auth_enabled: bool,
    pub kms_key_id: String,
    pub master_username: String,
    #[serde(default)]
    pub multi_az: bool,
    pub port: i64,
    pub reader_endpoint: String,
    pub region: String,
    pub status: String,
    #[serde(default)]
    pub storage_encrypted: bool,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsSubnetGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_subnet_group_description: String,
    pub db_subnet_group_name: String,
    pub region: String,
    pub status: String,
    pub subnet_ids: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsParameterGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_parameter_group_family: String,
    pub db_parameter_group_name: String,
    pub description: String,
    pub region: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsRdsSnapshot {
    pub account_id: String,
    pub allocated_storage: i64,
    pub availability_zone: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub db_instance_identifier: String,
    pub db_snapshot_identifier: String,
    #[serde(default)]
    pub encrypted: bool,
    pub engine: String,
    pub engine_version: String,
    pub kms_key_id: String,
    pub region: String,
    pub snapshot_type: String,
    pub status: String,
    pub storage_type: String,
    pub tags: String,
}

#[async_trait]
pub trait RdsAction {
    async fn create_db_instance(&self, allocated_storage: i64, auto_minor_version_upgrade: bool, availability_zone: String, backup_retention_period: i64, ca_certificate_identifier: String, character_set_name: String, copy_tags_to_snapshot: bool, db_cluster_identifier: String, db_instance_schema: String, db_instance_identifier: String, db_name: String, db_parameter_group_name: String, db_security_groups: Vec<String>, db_subnet_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_cloudwatch_logs_exports: Vec<String>, enable_customer_owned_ip: bool, enable_iam_database_authentication: bool, enable_performance_insights: bool, engine: String, engine_version: String, iops: i64, kms_key_id: String, license_model: String, manage_master_user_password: bool, master_user_password: String, master_user_secret_kms_key_id: String, master_username: String, max_allocated_storage: i64, monitoring_interval: i64, monitoring_role_arn: String, multi_az: bool, nchar_character_set_name: String, network_type: String, option_group_name: String, performance_insights_kms_key_id: String, performance_insights_retention_period: i64, port: i64, preferred_backup_window: String, preferred_maintenance_window: String, processor_features: Vec<String>, publicly_accessible: bool, region: String, storage_encrypted: bool, storage_throughput: i64, storage_type: String, tags: String, tde_credential_arn: String, tde_credential_password: String, timezone: String, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_db_instance(&self, db_instance_identifier: String, delete_automated_backups: bool, final_db_snapshot_identifier: String, region: String, skip_final_snapshot: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_db_instances(&self, db_instance_identifier: String, filters: Vec<String>, marker: String, max_records: i64, region: String) -> Result<DescribeDbInstancesOutput, Box<dyn std::error::Error>>;
    async fn modify_db_instance(&self, allocated_storage: i64, allow_major_version_upgrade: bool, apply_immediately: bool, auto_minor_version_upgrade: bool, automation_mode: String, backup_retention_period: i64, ca_certificate_identifier: String, cloudwatch_logs_export_configuration: String, copy_tags_to_snapshot: bool, db_instance_schema: String, db_instance_identifier: String, db_parameter_group_name: String, db_port_number: i64, db_security_groups: Vec<String>, db_subnet_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_customer_owned_ip: bool, enable_iam_database_authentication: bool, enable_performance_insights: bool, engine_version: String, iops: i64, license_model: String, manage_master_user_password: bool, master_user_password: String, master_user_secret_kms_key_id: String, max_allocated_storage: i64, monitoring_interval: i64, monitoring_role_arn: String, multi_az: bool, network_type: String, new_db_instance_identifier: String, option_group_name: String, performance_insights_kms_key_id: String, performance_insights_retention_period: i64, preferred_backup_window: String, preferred_maintenance_window: String, processor_features: Vec<String>, promotion_tier: i64, publicly_accessible: bool, region: String, replica_mode: String, resume_full_automation_mode_minutes: i64, rotate_master_user_password: bool, storage_throughput: i64, storage_type: String, tde_credential_arn: String, tde_credential_password: String, use_default_processor_features: bool, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn start_db_instance(&self, db_instance_identifier: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn stop_db_instance(&self, db_instance_identifier: String, db_snapshot_identifier: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn reboot_db_instance(&self, db_instance_identifier: String, force_failover: bool, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_db_cluster(&self, allocated_storage: i64, auto_minor_version_upgrade: bool, availability_zones: Vec<String>, backtrack_window: i64, backup_retention_period: i64, character_set_name: String, copy_tags_to_snapshot: bool, database_name: String, db_cluster_identifier: String, db_cluster_instance_schema: String, db_cluster_parameter_group_name: String, db_subnet_group_name: String, db_system_id: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_cloudwatch_logs_exports: Vec<String>, enable_global_write_forwarding: bool, enable_http_endpoint: bool, enable_iam_database_authentication: bool, enable_performance_insights: bool, engine: String, engine_mode: String, engine_version: String, global_cluster_identifier: String, iops: i64, kms_key_id: String, manage_master_user_password: bool, master_user_password: String, master_user_secret_kms_key_id: String, master_username: String, monitoring_interval: i64, monitoring_role_arn: String, network_type: String, option_group_name: String, performance_insights_kms_key_id: String, performance_insights_retention_period: i64, port: i64, preferred_backup_window: String, preferred_maintenance_window: String, publicly_accessible: bool, region: String, replication_source_identifier: String, scaling_configuration: String, serverless_v2_scaling_configuration: String, storage_encrypted: bool, storage_type: String, tags: String, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_db_cluster(&self, db_cluster_identifier: String, delete_automated_backups: bool, final_db_snapshot_identifier: String, region: String, skip_final_snapshot: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_db_clusters(&self, db_cluster_identifier: String, filters: Vec<String>, include_shared: bool, marker: String, max_records: i64, region: String) -> Result<DescribeDbClustersOutput, Box<dyn std::error::Error>>;
    async fn modify_db_cluster(&self, allocated_storage: i64, allow_major_version_upgrade: bool, apply_immediately: bool, auto_minor_version_upgrade: bool, backtrack_window: i64, backup_retention_period: i64, cloudwatch_logs_export_configuration: String, copy_tags_to_snapshot: bool, db_cluster_identifier: String, db_cluster_instance_schema: String, db_cluster_parameter_group_name: String, db_instance_parameter_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_global_write_forwarding: bool, enable_http_endpoint: bool, enable_iam_database_authentication: bool, enable_performance_insights: bool, engine_version: String, iops: i64, manage_master_user_password: bool, master_user_password: String, master_user_secret_kms_key_id: String, monitoring_interval: i64, monitoring_role_arn: String, network_type: String, new_db_cluster_identifier: String, option_group_name: String, performance_insights_kms_key_id: String, performance_insights_retention_period: i64, port: i64, preferred_backup_window: String, preferred_maintenance_window: String, region: String, rotate_master_user_password: bool, scaling_configuration: String, serverless_v2_scaling_configuration: String, storage_type: String, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn start_db_cluster(&self, db_cluster_identifier: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn stop_db_cluster(&self, db_cluster_identifier: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn failover_db_cluster(&self, db_cluster_identifier: String, region: String, target_db_instance_identifier: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_db_snapshot(&self, db_instance_identifier: String, db_snapshot_identifier: String, region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_db_snapshot(&self, db_snapshot_identifier: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_db_snapshots(&self, db_instance_identifier: String, db_snapshot_identifier: String, dbi_resource_id: String, filters: Vec<String>, include_public: bool, include_shared: bool, marker: String, max_records: i64, region: String, snapshot_type: String) -> Result<DescribeDbSnapshotsOutput, Box<dyn std::error::Error>>;
    async fn copy_db_snapshot(&self, copy_option_group: bool, copy_tags: bool, kms_key_id: String, option_group_name: String, pre_signed_url: String, region: String, source_db_snapshot_identifier: String, source_region: String, tags: String, target_db_snapshot_identifier: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn restore_db_instance_from_db_snapshot(&self, auto_minor_version_upgrade: bool, availability_zone: String, copy_tags_to_snapshot: bool, custom_iam_instance_profile: String, db_instance_schema: String, db_instance_identifier: String, db_name: String, db_parameter_group_name: String, db_snapshot_identifier: String, db_subnet_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_cloudwatch_logs_exports: Vec<String>, enable_customer_owned_ip: bool, enable_iam_database_authentication: bool, engine: String, iops: i64, license_model: String, multi_az: bool, network_type: String, option_group_name: String, port: i64, processor_features: Vec<String>, publicly_accessible: bool, region: String, storage_throughput: i64, storage_type: String, tags: String, tde_credential_arn: String, tde_credential_password: String, use_default_processor_features: bool, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn restore_db_instance_to_point_in_time(&self, auto_minor_version_upgrade: bool, availability_zone: String, copy_tags_to_snapshot: bool, custom_iam_instance_profile: String, db_instance_schema: String, db_name: String, db_parameter_group_name: String, db_subnet_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_cloudwatch_logs_exports: Vec<String>, enable_customer_owned_ip: bool, enable_iam_database_authentication: bool, engine: String, iops: i64, license_model: String, max_allocated_storage: i64, multi_az: bool, network_type: String, option_group_name: String, port: i64, processor_features: Vec<String>, publicly_accessible: bool, region: String, restore_time: chrono::DateTime<chrono::Utc>, source_db_instance_automated_backups_arn: String, source_db_instance_identifier: String, source_dbi_resource_id: String, storage_throughput: i64, storage_type: String, tags: String, target_db_instance_identifier: String, tde_credential_arn: String, tde_credential_password: String, use_default_processor_features: bool, use_latest_restorable_time: bool, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_db_cluster_snapshot(&self, db_cluster_identifier: String, db_cluster_snapshot_identifier: String, region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_db_cluster_snapshot(&self, db_cluster_snapshot_identifier: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_db_cluster_snapshots(&self, db_cluster_identifier: String, db_cluster_snapshot_identifier: String, filters: Vec<String>, include_public: bool, include_shared: bool, marker: String, max_records: i64, region: String, snapshot_type: String) -> Result<DescribeDbClusterSnapshotsOutput, Box<dyn std::error::Error>>;
    async fn copy_db_cluster_snapshot(&self, copy_tags: bool, kms_key_id: String, pre_signed_url: String, region: String, source_db_cluster_snapshot_identifier: String, source_region: String, tags: String, target_db_cluster_snapshot_identifier: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn restore_db_cluster_from_snapshot(&self, availability_zones: Vec<String>, backtrack_window: i64, copy_tags_to_snapshot: bool, database_name: String, db_cluster_identifier: String, db_cluster_instance_schema: String, db_cluster_parameter_group_name: String, db_subnet_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_cloudwatch_logs_exports: Vec<String>, enable_iam_database_authentication: bool, engine: String, engine_mode: String, engine_version: String, iops: i64, kms_key_id: String, network_type: String, option_group_name: String, port: i64, publicly_accessible: bool, region: String, scaling_configuration: String, serverless_v2_scaling_configuration: String, snapshot_identifier: String, storage_type: String, tags: String, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn restore_db_cluster_to_point_in_time(&self, backtrack_window: i64, copy_tags_to_snapshot: bool, db_cluster_identifier: String, db_cluster_instance_schema: String, db_cluster_parameter_group_name: String, db_subnet_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_cloudwatch_logs_exports: Vec<String>, enable_iam_database_authentication: bool, engine_mode: String, iops: i64, kms_key_id: String, network_type: String, option_group_name: String, port: i64, publicly_accessible: bool, region: String, restore_to_time: chrono::DateTime<chrono::Utc>, restore_type: String, scaling_configuration: String, serverless_v2_scaling_configuration: String, source_db_cluster_identifier: String, storage_type: String, tags: String, use_latest_restorable_time: bool, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_db_subnet_group(&self, db_subnet_group_description: String, db_subnet_group_name: String, region: String, subnet_ids: Vec<String>, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_db_subnet_group(&self, db_subnet_group_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_db_subnet_groups(&self, db_subnet_group_name: String, filters: Vec<String>, marker: String, max_records: i64, region: String) -> Result<DescribeDbSubnetGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_db_subnet_group(&self, db_subnet_group_description: String, db_subnet_group_name: String, region: String, subnet_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_db_parameter_group(&self, db_parameter_group_family: String, db_parameter_group_name: String, description: String, region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_db_parameter_group(&self, db_parameter_group_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_db_parameter_groups(&self, db_parameter_group_name: String, filters: Vec<String>, marker: String, max_records: i64, region: String) -> Result<DescribeDbParameterGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_db_parameter_group(&self, db_parameter_group_name: String, parameters: Vec<String>, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn reset_db_parameter_group(&self, db_parameter_group_name: String, parameters: Vec<String>, region: String, reset_all_parameters: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_db_parameters(&self, db_parameter_group_name: String, filters: Vec<String>, marker: String, max_records: i64, region: String, source: String) -> Result<DescribeDbParametersOutput, Box<dyn std::error::Error>>;
    async fn create_db_cluster_parameter_group(&self, db_cluster_parameter_group_name: String, db_parameter_group_family: String, description: String, region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_db_cluster_parameter_group(&self, db_cluster_parameter_group_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_db_cluster_parameter_groups(&self, db_cluster_parameter_group_name: String, filters: Vec<String>, marker: String, max_records: i64, region: String) -> Result<DescribeDbClusterParameterGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_db_cluster_parameter_group(&self, db_cluster_parameter_group_name: String, parameters: Vec<String>, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_option_group(&self, engine_name: String, major_engine_version: String, option_group_description: String, option_group_name: String, region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_option_group(&self, option_group_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_option_groups(&self, engine_name: String, filters: Vec<String>, major_engine_version: String, marker: String, max_records: i64, option_group_name: String, region: String) -> Result<DescribeOptionGroupsOutput, Box<dyn std::error::Error>>;
    async fn modify_option_group(&self, apply_immediately: bool, option_group_name: String, options_to_include: Vec<String>, options_to_remove: Vec<String>, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_db_instance_read_replica(&self, auto_minor_version_upgrade: bool, availability_zone: String, copy_tags_to_snapshot: bool, custom_iam_instance_profile: String, db_instance_schema: String, db_instance_identifier: String, db_parameter_group_name: String, db_subnet_group_name: String, deletion_protection: bool, domain: String, domain_iam_role_name: String, enable_cloudwatch_logs_exports: Vec<String>, enable_iam_database_authentication: bool, enable_performance_insights: bool, iops: i64, kms_key_id: String, max_allocated_storage: i64, monitoring_interval: i64, monitoring_role_arn: String, multi_az: bool, network_type: String, option_group_name: String, performance_insights_kms_key_id: String, performance_insights_retention_period: i64, port: i64, pre_signed_url: String, processor_features: Vec<String>, publicly_accessible: bool, region: String, replica_mode: String, source_db_instance_identifier: String, source_region: String, storage_throughput: i64, storage_type: String, tags: String, use_default_processor_features: bool, vpc_security_group_ids: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn promote_read_replica(&self, backup_retention_period: i64, db_instance_identifier: String, preferred_backup_window: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn promote_read_replica_db_cluster(&self, db_cluster_identifier: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn describe_db_engine_versions(&self, db_parameter_group_family: String, default_only: bool, engine: String, engine_version: String, filters: Vec<String>, include_all: bool, list_supported_character_sets: bool, list_supported_timezones: bool, marker: String, max_records: i64, region: String) -> Result<DescribeDbEngineVersionsOutput, Box<dyn std::error::Error>>;
    async fn describe_orderable_db_instance_options(&self, availability_zone_group: String, db_instance_schema: String, engine: String, engine_version: String, filters: Vec<String>, license_model: String, marker: String, max_records: i64, region: String, vpc: bool) -> Result<DescribeOrderableDbInstanceOptionsOutput, Box<dyn std::error::Error>>;
    async fn add_tags_to_resource(&self, region: String, resource_name: String, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_tags_from_resource(&self, region: String, resource_name: String, tag_keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_tags_for_resource(&self, filters: Vec<String>, region: String, resource_name: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

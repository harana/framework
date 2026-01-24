// Harana Actions - AWS RDS Module
// This module provides AWS RDS actions.

#![warn(missing_docs)]

pub mod output;

use output::*;

/// Create DB Instance
pub async fn create_db_instance(
    db_instance_class: &str,
    db_instance_identifier: &str,
    engine: &str,
    allocated_storage: Option<i32>,
    auto_minor_version_upgrade: Option<bool>,
    availability_zone: Option<&str>,
    backup_retention_period: Option<i32>,
    ca_certificate_identifier: Option<&str>,
    character_set_name: Option<&str>,
    copy_tags_to_snapshot: Option<bool>,
    db_cluster_identifier: Option<&str>,
    db_name: Option<&str>,
    db_parameter_group_name: Option<&str>,
    db_security_groups: Option<Vec<String>>,
    db_subnet_group_name: Option<&str>,
    deletion_protection: Option<bool>,
    domain: Option<&str>,
    domain_iam_role_name: Option<&str>,
    enable_cloudwatch_logs_exports: Option<Vec<String>>,
    enable_customer_owned_ip: Option<bool>,
    enable_iam_database_authentication: Option<bool>,
    enable_performance_insights: Option<bool>,
    engine_version: Option<&str>,
    iops: Option<i32>,
    kms_key_id: Option<&str>,
    license_model: Option<&str>,
    manage_master_user_password: Option<bool>,
    master_user_password: Option<&str>,
    master_user_secret_kms_key_id: Option<&str>,
    master_username: Option<&str>,
    max_allocated_storage: Option<i32>,
    monitoring_interval: Option<i32>,
    monitoring_role_arn: Option<&str>,
    multi_az: Option<bool>,
    nchar_character_set_name: Option<&str>,
    network_type: Option<&str>,
    option_group_name: Option<&str>,
    performance_insights_kms_key_id: Option<&str>,
    performance_insights_retention_period: Option<i32>,
    port: Option<i32>,
    preferred_backup_window: Option<&str>,
    preferred_maintenance_window: Option<&str>,
    processor_features: Option<Vec<ProcessorFeature>>,
    publicly_accessible: Option<bool>,
    region: Option<&str>,
    storage_encrypted: Option<bool>,
    storage_throughput: Option<i32>,
    storage_type: Option<&str>,
    tags: Option<RdsTags>,
    tde_credential_arn: Option<&str>,
    tde_credential_password: Option<&str>,
    timezone: Option<&str>,
    vpc_security_group_ids: Option<Vec<String>>,
) -> Result<CreateDbInstanceOutput, String> {
    unimplemented!("create_db_instance")
}

/// Delete DB Instance
pub async fn delete_db_instance(
    db_instance_identifier: &str,
    delete_automated_backups: Option<bool>,
    final_db_snapshot_identifier: Option<&str>,
    region: Option<&str>,
    skip_final_snapshot: Option<bool>,
) -> Result<DeleteDbInstanceOutput, String> {
    unimplemented!("delete_db_instance")
}

/// Describe DB Instances
pub async fn describe_db_instances(
    db_instance_identifier: Option<&str>,
    filters: Option<Vec<RdsFilter>>,
    marker: Option<&str>,
    max_records: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeDbInstancesOutput, String> {
    unimplemented!("describe_db_instances")
}

/// Modify DB Instance
pub async fn modify_db_instance(
    db_instance_identifier: &str,
    allocated_storage: Option<i32>,
    allow_major_version_upgrade: Option<bool>,
    apply_immediately: Option<bool>,
    auto_minor_version_upgrade: Option<bool>,
    automation_mode: Option<&str>,
    backup_retention_period: Option<i32>,
    ca_certificate_identifier: Option<&str>,
    cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    copy_tags_to_snapshot: Option<bool>,
    db_instance_class: Option<&str>,
    db_parameter_group_name: Option<&str>,
    db_port_number: Option<i32>,
    db_security_groups: Option<Vec<String>>,
    db_subnet_group_name: Option<&str>,
    deletion_protection: Option<bool>,
    domain: Option<&str>,
    domain_iam_role_name: Option<&str>,
    enable_customer_owned_ip: Option<bool>,
    enable_iam_database_authentication: Option<bool>,
    enable_performance_insights: Option<bool>,
    engine_version: Option<&str>,
    iops: Option<i32>,
    license_model: Option<&str>,
    manage_master_user_password: Option<bool>,
    master_user_password: Option<&str>,
    master_user_secret_kms_key_id: Option<&str>,
    max_allocated_storage: Option<i32>,
    monitoring_interval: Option<i32>,
    monitoring_role_arn: Option<&str>,
    multi_az: Option<bool>,
    network_type: Option<&str>,
    new_db_instance_identifier: Option<&str>,
    option_group_name: Option<&str>,
    performance_insights_kms_key_id: Option<&str>,
    performance_insights_retention_period: Option<i32>,
    preferred_backup_window: Option<&str>,
    preferred_maintenance_window: Option<&str>,
    processor_features: Option<Vec<ProcessorFeature>>,
    promotion_tier: Option<i32>,
    publicly_accessible: Option<bool>,
    region: Option<&str>,
    replica_mode: Option<&str>,
    resume_full_automation_mode_minutes: Option<i32>,
    rotate_master_user_password: Option<bool>,
    storage_throughput: Option<i32>,
    storage_type: Option<&str>,
    tde_credential_arn: Option<&str>,
    tde_credential_password: Option<&str>,
    use_default_processor_features: Option<bool>,
    vpc_security_group_ids: Option<Vec<String>>,
) -> Result<ModifyDbInstanceOutput, String> {
    unimplemented!("modify_db_instance")
}

/// Start DB Instance
pub async fn start_db_instance(
    db_instance_identifier: &str,
    region: Option<&str>,
) -> Result<StartDbInstanceOutput, String> {
    unimplemented!("start_db_instance")
}

/// Stop DB Instance
pub async fn stop_db_instance(
    db_instance_identifier: &str,
    db_snapshot_identifier: Option<&str>,
    region: Option<&str>,
) -> Result<StopDbInstanceOutput, String> {
    unimplemented!("stop_db_instance")
}

/// Reboot DB Instance
pub async fn reboot_db_instance(
    db_instance_identifier: &str,
    force_failover: Option<bool>,
    region: Option<&str>,
) -> Result<RebootDbInstanceOutput, String> {
    unimplemented!("reboot_db_instance")
}

/// Create DB Cluster
pub async fn create_db_cluster(
    db_cluster_identifier: &str,
    engine: &str,
    allocated_storage: Option<i32>,
    auto_minor_version_upgrade: Option<bool>,
    availability_zones: Option<Vec<String>>,
    backtrack_window: Option<i64>,
    backup_retention_period: Option<i32>,
    character_set_name: Option<&str>,
    copy_tags_to_snapshot: Option<bool>,
    database_name: Option<&str>,
    db_cluster_instance_class: Option<&str>,
    db_cluster_parameter_group_name: Option<&str>,
    db_subnet_group_name: Option<&str>,
    db_system_id: Option<&str>,
    deletion_protection: Option<bool>,
    domain: Option<&str>,
    domain_iam_role_name: Option<&str>,
    enable_cloudwatch_logs_exports: Option<Vec<String>>,
    enable_global_write_forwarding: Option<bool>,
    enable_http_endpoint: Option<bool>,
    enable_iam_database_authentication: Option<bool>,
    enable_performance_insights: Option<bool>,
    engine_mode: Option<&str>,
    engine_version: Option<&str>,
    global_cluster_identifier: Option<&str>,
    iops: Option<i32>,
    kms_key_id: Option<&str>,
    manage_master_user_password: Option<bool>,
    master_user_password: Option<&str>,
    master_user_secret_kms_key_id: Option<&str>,
    master_username: Option<&str>,
    port: Option<i32>,
    preferred_backup_window: Option<&str>,
    preferred_maintenance_window: Option<&str>,
    region: Option<&str>,
    storage_encrypted: Option<bool>,
    storage_type: Option<&str>,
    tags: Option<RdsTags>,
    vpc_security_group_ids: Option<Vec<String>>,
) -> Result<CreateDbClusterOutput, String> {
    unimplemented!("create_db_cluster")
}

/// Delete DB Cluster
pub async fn delete_db_cluster(
    db_cluster_identifier: &str,
    final_db_snapshot_identifier: Option<&str>,
    region: Option<&str>,
    skip_final_snapshot: Option<bool>,
) -> Result<DeleteDbClusterOutput, String> {
    unimplemented!("delete_db_cluster")
}

/// Describe DB Clusters
pub async fn describe_db_clusters(
    db_cluster_identifier: Option<&str>,
    filters: Option<Vec<RdsFilter>>,
    include_shared: Option<bool>,
    marker: Option<&str>,
    max_records: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeDbClustersOutput, String> {
    unimplemented!("describe_db_clusters")
}

/// Modify DB Cluster
pub async fn modify_db_cluster(
    db_cluster_identifier: &str,
    allow_major_version_upgrade: Option<bool>,
    apply_immediately: Option<bool>,
    auto_minor_version_upgrade: Option<bool>,
    backtrack_window: Option<i64>,
    backup_retention_period: Option<i32>,
    cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    copy_tags_to_snapshot: Option<bool>,
    db_cluster_instance_class: Option<&str>,
    db_cluster_parameter_group_name: Option<&str>,
    deletion_protection: Option<bool>,
    domain: Option<&str>,
    domain_iam_role_name: Option<&str>,
    enable_global_write_forwarding: Option<bool>,
    enable_http_endpoint: Option<bool>,
    enable_iam_database_authentication: Option<bool>,
    enable_performance_insights: Option<bool>,
    engine_version: Option<&str>,
    iops: Option<i32>,
    manage_master_user_password: Option<bool>,
    master_user_password: Option<&str>,
    master_user_secret_kms_key_id: Option<&str>,
    new_db_cluster_identifier: Option<&str>,
    option_group_name: Option<&str>,
    performance_insights_kms_key_id: Option<&str>,
    performance_insights_retention_period: Option<i32>,
    port: Option<i32>,
    preferred_backup_window: Option<&str>,
    preferred_maintenance_window: Option<&str>,
    region: Option<&str>,
    rotate_master_user_password: Option<bool>,
    storage_type: Option<&str>,
    vpc_security_group_ids: Option<Vec<String>>,
) -> Result<ModifyDbClusterOutput, String> {
    unimplemented!("modify_db_cluster")
}

/// Start DB Cluster
pub async fn start_db_cluster(
    db_cluster_identifier: &str,
    region: Option<&str>,
) -> Result<StartDbClusterOutput, String> {
    unimplemented!("start_db_cluster")
}

/// Stop DB Cluster
pub async fn stop_db_cluster(
    db_cluster_identifier: &str,
    region: Option<&str>,
) -> Result<StopDbClusterOutput, String> {
    unimplemented!("stop_db_cluster")
}

/// Create DB Snapshot
pub async fn create_db_snapshot(
    db_instance_identifier: &str,
    db_snapshot_identifier: &str,
    region: Option<&str>,
    tags: Option<RdsTags>,
) -> Result<CreateDbSnapshotOutput, String> {
    unimplemented!("create_db_snapshot")
}

/// Delete DB Snapshot
pub async fn delete_db_snapshot(
    db_snapshot_identifier: &str,
    region: Option<&str>,
) -> Result<DeleteDbSnapshotOutput, String> {
    unimplemented!("delete_db_snapshot")
}

/// Describe DB Snapshots
pub async fn describe_db_snapshots(
    db_instance_identifier: Option<&str>,
    db_snapshot_identifier: Option<&str>,
    filters: Option<Vec<RdsFilter>>,
    include_public: Option<bool>,
    include_shared: Option<bool>,
    marker: Option<&str>,
    max_records: Option<i32>,
    region: Option<&str>,
    snapshot_type: Option<&str>,
) -> Result<DescribeDbSnapshotsOutput, String> {
    unimplemented!("describe_db_snapshots")
}

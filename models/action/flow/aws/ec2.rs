// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunInstancesOutput {
    pub instances: Vec<String>,
    pub owner_id: String,
    pub reservation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetConsoleOutputOutput {
    pub instance_id: String,
    pub output: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPasswordDataOutput {
    pub instance_id: String,
    pub password_data: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSnapshotOutput {
    pub description: String,
    pub encrypted: bool,
    pub owner_id: String,
    pub progress: String,
    pub snapshot_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub state: String,
    pub volume_id: String,
    pub volume_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVolumeOutput {
    pub attachments: Vec<String>,
    pub availability_zone: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub encrypted: bool,
    pub iops: i64,
    pub kms_key_id: String,
    pub multi_attach_enabled: bool,
    pub size: i64,
    pub snapshot_id: String,
    pub state: String,
    pub throughput: i64,
    pub volume_id: String,
    pub volume_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachVolumeOutput {
    pub attach_time: chrono::DateTime<chrono::Utc>,
    pub delete_on_termination: bool,
    pub device: String,
    pub instance_id: String,
    pub state: String,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachVolumeOutput {
    pub attach_time: chrono::DateTime<chrono::Utc>,
    pub delete_on_termination: bool,
    pub device: String,
    pub instance_id: String,
    pub state: String,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyVolumeOutput {
    pub modification_state: String,
    pub original_iops: i64,
    pub original_size: i64,
    pub original_throughput: i64,
    pub original_volume_type: String,
    pub progress: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub target_iops: i64,
    pub target_size: i64,
    pub target_throughput: i64,
    pub target_volume_type: String,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateKeyPairOutput {
    pub key_fingerprint: String,
    pub key_material: String,
    pub key_name: String,
    pub key_pair_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImportKeyPairOutput {
    pub key_fingerprint: String,
    pub key_name: String,
    pub key_pair_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLaunchTemplateOutput {
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub default_version_number: i64,
    pub latest_version_number: i64,
    pub launch_template_id: String,
    pub launch_template_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchTemplateOutput {
    pub launch_template_id: String,
    pub launch_template_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchTemplateVersionsOutput {
    pub successfully_deleted_launch_template_versions: Vec<String>,
    pub unsuccessfully_deleted_launch_template_versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachNetworkInterfaceOutput {
    pub attachment_id: String,
    pub network_card_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePlacementGroupOutput {
    pub group_arn: String,
    pub group_id: String,
    pub group_name: String,
    pub partition_count: i64,
    pub state: String,
    pub strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFleetOutput {
    pub errors: Vec<String>,
    pub fleet_id: String,
    pub instances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFleetsOutput {
    pub successful_fleet_deletions: Vec<String>,
    pub unsuccessful_fleet_deletions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Instance {
    pub account_id: String,
    pub ami_id: String,
    pub availability_zone: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub ebs_optimized: bool,
    pub iam_instance_profile: String,
    pub instance_id: String,
    pub instance_type: String,
    pub key_name: String,
    pub launch_time: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub monitoring_enabled: bool,
    pub platform: String,
    pub private_dns_name: String,
    pub private_ip_address: String,
    pub public_dns_name: String,
    pub public_ip_address: String,
    pub region: String,
    pub state: String,
    pub subnet_id: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Ami {
    pub account_id: String,
    pub architecture: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub image_id: String,
    #[serde(default)]
    pub is_public: bool,
    pub name: String,
    pub owner_id: String,
    pub platform: String,
    pub region: String,
    pub root_device_type: String,
    pub state: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub virtualization_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2SecurityGroup {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub group_id: String,
    pub group_name: String,
    pub region: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub vpc_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2SecurityGroupRule {
    pub cidr_ipv4: String,
    pub cidr_ipv6: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub direction: String,
    pub from_port: i64,
    pub ip_protocol: String,
    pub referenced_group_id: String,
    pub security_group_id: String,
    pub to_port: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2KeyPair {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fingerprint: String,
    pub key_name: String,
    pub key_pair_id: String,
    pub key_type: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2ElasticIp {
    pub account_id: String,
    pub allocation_id: String,
    pub association_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub domain: String,
    pub instance_id: String,
    pub network_interface_id: String,
    pub private_ip_address: String,
    pub public_ip: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Volume {
    pub account_id: String,
    pub availability_zone: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub encrypted: bool,
    pub iops: i64,
    pub kms_key_id: String,
    pub region: String,
    pub size: i64,
    pub snapshot_id: String,
    pub state: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub volume_id: String,
    pub volume_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEc2Snapshot {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub encrypted: bool,
    pub kms_key_id: String,
    pub owner_id: String,
    pub progress: String,
    pub region: String,
    pub snapshot_id: String,
    pub state: String,
    pub tags: String,
    pub volume_id: String,
    pub volume_size: i64,
}

#[async_trait]
pub trait Ec2Action {
    async fn run_instances(&self, block_device_mappings: Vec<String>, capacity_reservation_specification: String, cpu_options: String, credit_specification: String, disable_api_stop: bool, disable_api_termination: bool, ebs_optimized: bool, elastic_gpu_specification: Vec<String>, elastic_inference_accelerators: Vec<String>, enclave_options: String, hibernation_options: String, iam_instance_profile: String, image_id: String, instance_initiated_shutdown_behavior: String, instance_market_options: String, instance_type: String, ipv6_address_count: i64, ipv6_addresses: Vec<String>, kernel_id: String, key_name: String, launch_template: String, license_specifications: Vec<String>, maintenance_options: String, max_count: i64, metadata_options: String, min_count: i64, monitoring: bool, network_interfaces: Vec<String>, placement: String, private_dns_name_options: String, private_ip_address: String, ramdisk_id: String, region: String, security_group_ids: Vec<String>, security_groups: Vec<String>, subnet_id: String, tags: String, user_data: String) -> Result<RunInstancesOutput, Box<dyn std::error::Error>>;
    async fn start_instances(&self, instance_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn stop_instances(&self, force: bool, hibernate: bool, instance_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn reboot_instances(&self, instance_ids: Vec<String>, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn terminate_instances(&self, instance_ids: Vec<String>, region: String, terminating_instances: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_instances(&self, filters: Vec<String>, instance_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_instance_status(&self, filters: Vec<String>, include_all_instances: bool, instance_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn modify_instance_attribute(&self, attribute: String, block_device_mappings: Vec<String>, disable_api_stop: bool, disable_api_termination: bool, ebs_optimized: bool, ena_support: bool, groups: Vec<String>, instance_id: String, instance_initiated_shutdown_behavior: String, instance_type: String, kernel: String, ramdisk: String, region: String, source_dest_check: bool, sriov_net_support: String, user_data: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_console_output(&self, instance_id: String, latest: bool, region: String) -> Result<GetConsoleOutputOutput, Box<dyn std::error::Error>>;
    async fn get_password_data(&self, instance_id: String, region: String) -> Result<GetPasswordDataOutput, Box<dyn std::error::Error>>;
    async fn create_image(&self, block_device_mappings: Vec<String>, description: String, instance_id: String, name: String, no_reboot: bool, region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn deregister_image(&self, image_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_images(&self, executable_users: Vec<String>, filters: Vec<String>, image_ids: Vec<String>, include_deprecated: bool, owners: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn copy_image(&self, description: String, destination_region: String, encrypted: bool, kms_key_id: String, name: String, region: String, source_image_id: String, source_region: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn modify_image_attribute(&self, attribute: String, description: String, image_id: String, launch_permission: String, operation_type: String, region: String, user_groups: Vec<String>, user_ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_snapshot(&self, description: String, outpost_arn: String, region: String, tags: String, volume_id: String) -> Result<CreateSnapshotOutput, Box<dyn std::error::Error>>;
    async fn delete_snapshot(&self, region: String, snapshot_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_snapshots(&self, filters: Vec<String>, owner_ids: Vec<String>, region: String, restorable_by_user_ids: Vec<String>, snapshot_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn copy_snapshot(&self, description: String, destination_region: String, encrypted: bool, kms_key_id: String, presigned_url: String, region: String, source_region: String, source_snapshot_id: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_volume(&self, availability_zone: String, encrypted: bool, iops: i64, kms_key_id: String, multi_attach_enabled: bool, outpost_arn: String, region: String, size: i64, snapshot_id: String, tags: String, throughput: i64, volume_type: String) -> Result<CreateVolumeOutput, Box<dyn std::error::Error>>;
    async fn delete_volume(&self, region: String, volume_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_volumes(&self, filters: Vec<String>, region: String, volume_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn attach_volume(&self, device: String, instance_id: String, region: String, volume_id: String) -> Result<AttachVolumeOutput, Box<dyn std::error::Error>>;
    async fn detach_volume(&self, device: String, force: bool, instance_id: String, region: String, volume_id: String) -> Result<DetachVolumeOutput, Box<dyn std::error::Error>>;
    async fn modify_volume(&self, iops: i64, multi_attach_enabled: bool, region: String, size: i64, throughput: i64, volume_id: String, volume_type: String) -> Result<ModifyVolumeOutput, Box<dyn std::error::Error>>;
    async fn create_key_pair(&self, key_format: String, key_name: String, key_type: String, region: String, tags: String) -> Result<CreateKeyPairOutput, Box<dyn std::error::Error>>;
    async fn delete_key_pair(&self, key_name: String, key_pair_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_key_pairs(&self, filters: Vec<String>, include_public_key: bool, key_names: Vec<String>, key_pair_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn import_key_pair(&self, key_name: String, public_key_material: String, region: String, tags: String) -> Result<ImportKeyPairOutput, Box<dyn std::error::Error>>;
    async fn create_launch_template(&self, launch_template_data: String, launch_template_name: String, region: String, tags: String, version_description: String) -> Result<CreateLaunchTemplateOutput, Box<dyn std::error::Error>>;
    async fn delete_launch_template(&self, launch_template_id: String, launch_template_name: String, region: String) -> Result<DeleteLaunchTemplateOutput, Box<dyn std::error::Error>>;
    async fn describe_launch_templates(&self, filters: Vec<String>, launch_template_ids: Vec<String>, launch_template_names: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_launch_template_version(&self, launch_template_data: String, launch_template_id: String, launch_template_name: String, region: String, source_version: String, version_description: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_launch_template_versions(&self, launch_template_id: String, launch_template_name: String, region: String, versions: Vec<String>) -> Result<DeleteLaunchTemplateVersionsOutput, Box<dyn std::error::Error>>;
    async fn modify_launch_template(&self, default_version: String, launch_template_id: String, launch_template_name: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_network_interface(&self, description: String, groups: Vec<String>, interface_type: String, ipv4_prefix_count: i64, ipv4_prefixes: Vec<String>, ipv6_address_count: i64, ipv6_addresses: Vec<String>, ipv6_prefix_count: i64, ipv6_prefixes: Vec<String>, private_ip_address: String, private_ip_addresses: Vec<String>, region: String, secondary_private_ip_address_count: i64, subnet_id: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_network_interface(&self, network_interface_id: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_network_interfaces(&self, filters: Vec<String>, network_interface_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn attach_network_interface(&self, device_index: i64, instance_id: String, network_card_index: i64, network_interface_id: String, region: String) -> Result<AttachNetworkInterfaceOutput, Box<dyn std::error::Error>>;
    async fn detach_network_interface(&self, attachment_id: String, force: bool, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn modify_network_interface_attribute(&self, attachment: String, description: String, groups: Vec<String>, network_interface_id: String, region: String, source_dest_check: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_placement_group(&self, group_name: String, partition_count: i64, region: String, spread_level: String, strategy: String, tags: String) -> Result<CreatePlacementGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_placement_group(&self, group_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_placement_groups(&self, filters: Vec<String>, group_ids: Vec<String>, group_names: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_instance_types(&self, filters: Vec<String>, instance_types: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_availability_zones(&self, all_availability_zones: bool, filters: Vec<String>, region: String, zone_ids: Vec<String>, zone_names: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_regions(&self, all_regions: bool, filters: Vec<String>, region_names: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_tags(&self, region: String, resources: Vec<String>, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_tags(&self, region: String, resources: Vec<String>, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_tags(&self, filters: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn request_spot_instances(&self, availability_zone_group: String, block_duration_minutes: i64, instance_count: i64, instance_interruption_behavior: String, launch_group: String, launch_specification: String, region: String, spot_price: String, tags: String, type: String, valid_from: chrono::DateTime<chrono::Utc>, valid_until: chrono::DateTime<chrono::Utc>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn cancel_spot_instance_requests(&self, region: String, spot_instance_request_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_spot_instance_requests(&self, filters: Vec<String>, region: String, spot_instance_request_ids: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn describe_spot_price_history(&self, availability_zone: String, end_time: chrono::DateTime<chrono::Utc>, filters: Vec<String>, instance_types: Vec<String>, product_descriptions: Vec<String>, region: String, start_time: chrono::DateTime<chrono::Utc>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_fleet(&self, context: String, excess_capacity_termination_policy: String, launch_template_configs: Vec<String>, on_demand_options: String, region: String, replace_unhealthy_instances: bool, spot_options: String, tags: String, target_capacity_specification: String, terminate_instances_with_expiration: bool, type: String, valid_from: chrono::DateTime<chrono::Utc>, valid_until: chrono::DateTime<chrono::Utc>) -> Result<CreateFleetOutput, Box<dyn std::error::Error>>;
    async fn delete_fleets(&self, fleet_ids: Vec<String>, region: String, terminate_instances: bool) -> Result<DeleteFleetsOutput, Box<dyn std::error::Error>>;
    async fn describe_fleets(&self, filters: Vec<String>, fleet_ids: Vec<String>, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn modify_fleet(&self, context: String, excess_capacity_termination_policy: String, fleet_id: String, launch_template_configs: Vec<String>, region: String, target_capacity_specification: String) -> Result<(), Box<dyn std::error::Error>>;
}

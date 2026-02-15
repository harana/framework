// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunInstancesInput {
    pub block_device_mappings: Vec<String>,
    pub capacity_reservation_specification: String,
    pub cpu_options: String,
    pub credit_specification: String,
    #[serde(default)]
    pub disable_api_stop: bool,
    #[serde(default)]
    pub disable_api_termination: bool,
    #[serde(default)]
    pub ebs_optimized: bool,
    pub elastic_gpu_specification: Vec<String>,
    pub elastic_inference_accelerators: Vec<String>,
    pub enclave_options: String,
    pub hibernation_options: String,
    pub iam_instance_profile: String,
    pub image_id: String,
    pub instance_initiated_shutdown_behavior: String,
    pub instance_market_options: String,
    pub instance_type: String,
    pub ipv6_address_count: i64,
    pub ipv6_addresses: Vec<String>,
    pub kernel_id: String,
    pub key_name: String,
    pub launch_template: String,
    pub license_specifications: Vec<String>,
    pub maintenance_options: String,
    pub max_count: i64,
    pub metadata_options: String,
    pub min_count: i64,
    #[serde(default)]
    pub monitoring: bool,
    pub network_interfaces: Vec<String>,
    pub placement: String,
    pub private_dns_name_options: String,
    pub private_ip_address: String,
    pub ramdisk_id: String,
    pub region: String,
    pub security_group_ids: Vec<String>,
    pub security_groups: Vec<String>,
    pub subnet_id: String,
    pub tags: String,
    pub user_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunInstancesOutput {
    pub instances: Vec<String>,
    pub owner_id: String,
    pub reservation_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartInstancesInput {
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartInstancesOutput {
    pub starting_instances: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopInstancesInput {
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub hibernate: bool,
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopInstancesOutput {
    pub stopping_instances: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RebootInstancesInput {
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RebootInstancesOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateInstancesInput {
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateInstancesOutput {
    pub success: bool,
    pub terminating_instances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstancesInput {
    pub filters: Vec<String>,
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstancesOutput {
    pub reservations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstanceStatusInput {
    pub filters: Vec<String>,
    #[serde(default)]
    pub include_all_instances: bool,
    pub instance_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstanceStatusOutput {
    pub instance_statuses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyInstanceAttributeInput {
    pub attribute: String,
    pub block_device_mappings: Vec<String>,
    pub disable_api_stop: bool,
    pub disable_api_termination: bool,
    pub ebs_optimized: bool,
    pub ena_support: bool,
    pub groups: Vec<String>,
    pub instance_id: String,
    pub instance_initiated_shutdown_behavior: String,
    pub instance_type: String,
    pub kernel: String,
    pub ramdisk: String,
    pub region: String,
    pub source_dest_check: bool,
    pub sriov_net_support: String,
    pub user_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyInstanceAttributeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetConsoleOutputInput {
    pub instance_id: String,
    #[serde(default)]
    pub latest: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetConsoleOutputOutput {
    pub instance_id: String,
    pub output: String,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPasswordDataInput {
    pub instance_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPasswordDataOutput {
    pub instance_id: String,
    pub password_data: String,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateImageInput {
    pub block_device_mappings: Vec<String>,
    pub description: String,
    pub instance_id: String,
    pub name: String,
    #[serde(default)]
    pub no_reboot: bool,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateImageOutput {
    pub image_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeregisterImageInput {
    pub image_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeregisterImageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeImagesInput {
    pub executable_users: Vec<String>,
    pub filters: Vec<String>,
    pub image_ids: Vec<String>,
    #[serde(default)]
    pub include_deprecated: bool,
    pub owners: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeImagesOutput {
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyImageInput {
    pub description: String,
    pub destination_region: String,
    #[serde(default)]
    pub encrypted: bool,
    pub kms_key_id: String,
    pub name: String,
    pub region: String,
    pub source_image_id: String,
    pub source_region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyImageOutput {
    pub image_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyImageAttributeInput {
    pub attribute: String,
    pub description: String,
    pub image_id: String,
    pub launch_permission: String,
    pub operation_type: String,
    pub region: String,
    pub user_groups: Vec<String>,
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyImageAttributeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSnapshotInput {
    pub description: String,
    pub outpost_arn: String,
    pub region: String,
    pub tags: String,
    pub volume_id: String,
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
    pub success: bool,
    pub volume_id: String,
    pub volume_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSnapshotInput {
    pub region: String,
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSnapshotOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSnapshotsInput {
    pub filters: Vec<String>,
    pub owner_ids: Vec<String>,
    pub region: String,
    pub restorable_by_user_ids: Vec<String>,
    pub snapshot_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSnapshotsOutput {
    pub snapshots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopySnapshotInput {
    pub description: String,
    pub destination_region: String,
    #[serde(default)]
    pub encrypted: bool,
    pub kms_key_id: String,
    pub presigned_url: String,
    pub region: String,
    pub source_region: String,
    pub source_snapshot_id: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopySnapshotOutput {
    pub snapshot_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVolumeInput {
    pub availability_zone: String,
    #[serde(default)]
    pub encrypted: bool,
    pub iops: i64,
    pub kms_key_id: String,
    #[serde(default)]
    pub multi_attach_enabled: bool,
    pub outpost_arn: String,
    pub region: String,
    pub size: i64,
    pub snapshot_id: String,
    pub tags: String,
    pub throughput: i64,
    pub volume_type: String,
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
    pub success: bool,
    pub throughput: i64,
    pub volume_id: String,
    pub volume_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVolumeInput {
    pub region: String,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteVolumeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVolumesInput {
    pub filters: Vec<String>,
    pub region: String,
    pub volume_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeVolumesOutput {
    pub volumes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachVolumeInput {
    pub device: String,
    pub instance_id: String,
    pub region: String,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachVolumeOutput {
    pub attach_time: chrono::DateTime<chrono::Utc>,
    pub delete_on_termination: bool,
    pub device: String,
    pub instance_id: String,
    pub state: String,
    pub success: bool,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachVolumeInput {
    pub device: String,
    #[serde(default)]
    pub force: bool,
    pub instance_id: String,
    pub region: String,
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
    pub success: bool,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyVolumeInput {
    pub iops: i64,
    pub multi_attach_enabled: bool,
    pub region: String,
    pub size: i64,
    pub throughput: i64,
    pub volume_id: String,
    pub volume_type: String,
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
    pub success: bool,
    pub target_iops: i64,
    pub target_size: i64,
    pub target_throughput: i64,
    pub target_volume_type: String,
    pub volume_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateKeyPairInput {
    pub key_format: String,
    pub key_name: String,
    pub key_type: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateKeyPairOutput {
    pub key_fingerprint: String,
    pub key_material: String,
    pub key_name: String,
    pub key_pair_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteKeyPairInput {
    pub key_name: String,
    pub key_pair_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteKeyPairOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeKeyPairsInput {
    pub filters: Vec<String>,
    #[serde(default)]
    pub include_public_key: bool,
    pub key_names: Vec<String>,
    pub key_pair_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeKeyPairsOutput {
    pub key_pairs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImportKeyPairInput {
    pub key_name: String,
    pub public_key_material: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImportKeyPairOutput {
    pub key_fingerprint: String,
    pub key_name: String,
    pub key_pair_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLaunchTemplateInput {
    pub launch_template_data: String,
    pub launch_template_name: String,
    pub region: String,
    pub tags: String,
    pub version_description: String,
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
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchTemplateInput {
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchTemplateOutput {
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLaunchTemplatesInput {
    pub filters: Vec<String>,
    pub launch_template_ids: Vec<String>,
    pub launch_template_names: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeLaunchTemplatesOutput {
    pub launch_templates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLaunchTemplateVersionInput {
    pub launch_template_data: String,
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub region: String,
    pub source_version: String,
    pub version_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLaunchTemplateVersionOutput {
    pub launch_template_version: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchTemplateVersionsInput {
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub region: String,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteLaunchTemplateVersionsOutput {
    pub successfully_deleted_launch_template_versions: Vec<String>,
    pub unsuccessfully_deleted_launch_template_versions: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyLaunchTemplateInput {
    pub default_version: String,
    pub launch_template_id: String,
    pub launch_template_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyLaunchTemplateOutput {
    pub launch_template: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkInterfaceInput {
    pub description: String,
    pub groups: Vec<String>,
    pub interface_type: String,
    pub ipv4_prefix_count: i64,
    pub ipv4_prefixes: Vec<String>,
    pub ipv6_address_count: i64,
    pub ipv6_addresses: Vec<String>,
    pub ipv6_prefix_count: i64,
    pub ipv6_prefixes: Vec<String>,
    pub private_ip_address: String,
    pub private_ip_addresses: Vec<String>,
    pub region: String,
    pub secondary_private_ip_address_count: i64,
    pub subnet_id: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkInterfaceOutput {
    pub network_interface: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNetworkInterfaceInput {
    pub network_interface_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteNetworkInterfaceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNetworkInterfacesInput {
    pub filters: Vec<String>,
    pub network_interface_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeNetworkInterfacesOutput {
    pub network_interfaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachNetworkInterfaceInput {
    pub device_index: i64,
    pub instance_id: String,
    pub network_card_index: i64,
    pub network_interface_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachNetworkInterfaceOutput {
    pub attachment_id: String,
    pub network_card_index: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachNetworkInterfaceInput {
    pub attachment_id: String,
    #[serde(default)]
    pub force: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DetachNetworkInterfaceOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyNetworkInterfaceAttributeInput {
    pub attachment: String,
    pub description: String,
    pub groups: Vec<String>,
    pub network_interface_id: String,
    pub region: String,
    pub source_dest_check: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyNetworkInterfaceAttributeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePlacementGroupInput {
    pub group_name: String,
    pub partition_count: i64,
    pub region: String,
    pub spread_level: String,
    pub strategy: String,
    pub tags: String,
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
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePlacementGroupInput {
    pub group_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePlacementGroupOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribePlacementGroupsInput {
    pub filters: Vec<String>,
    pub group_ids: Vec<String>,
    pub group_names: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribePlacementGroupsOutput {
    pub placement_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstanceTypesInput {
    pub filters: Vec<String>,
    pub instance_types: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeInstanceTypesOutput {
    pub instance_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAvailabilityZonesInput {
    #[serde(default)]
    pub all_availability_zones: bool,
    pub filters: Vec<String>,
    pub region: String,
    pub zone_ids: Vec<String>,
    pub zone_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeAvailabilityZonesOutput {
    pub availability_zones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRegionsInput {
    #[serde(default)]
    pub all_regions: bool,
    pub filters: Vec<String>,
    pub region_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRegionsOutput {
    pub regions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTagsInput {
    pub region: String,
    pub resources: Vec<String>,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTagsInput {
    pub region: String,
    pub resources: Vec<String>,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTagsInput {
    pub filters: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeTagsOutput {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestSpotInstancesInput {
    pub availability_zone_group: String,
    pub block_duration_minutes: i64,
    pub instance_count: i64,
    pub instance_interruption_behavior: String,
    pub launch_group: String,
    pub launch_specification: String,
    pub region: String,
    pub spot_price: String,
    pub tags: String,
    pub type: String,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_until: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestSpotInstancesOutput {
    pub spot_instance_requests: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelSpotInstanceRequestsInput {
    pub region: String,
    pub spot_instance_request_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelSpotInstanceRequestsOutput {
    pub cancelled_spot_instance_requests: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSpotInstanceRequestsInput {
    pub filters: Vec<String>,
    pub region: String,
    pub spot_instance_request_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSpotInstanceRequestsOutput {
    pub spot_instance_requests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSpotPriceHistoryInput {
    pub availability_zone: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub filters: Vec<String>,
    pub instance_types: Vec<String>,
    pub product_descriptions: Vec<String>,
    pub region: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeSpotPriceHistoryOutput {
    pub spot_price_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFleetInput {
    pub context: String,
    pub excess_capacity_termination_policy: String,
    pub launch_template_configs: Vec<String>,
    pub on_demand_options: String,
    pub region: String,
    #[serde(default)]
    pub replace_unhealthy_instances: bool,
    pub spot_options: String,
    pub tags: String,
    pub target_capacity_specification: String,
    #[serde(default)]
    pub terminate_instances_with_expiration: bool,
    pub type: String,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_until: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFleetOutput {
    pub errors: Vec<String>,
    pub fleet_id: String,
    pub instances: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFleetsInput {
    pub fleet_ids: Vec<String>,
    pub region: String,
    pub terminate_instances: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFleetsOutput {
    pub successful_fleet_deletions: Vec<String>,
    pub unsuccessful_fleet_deletions: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeFleetsInput {
    pub filters: Vec<String>,
    pub fleet_ids: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeFleetsOutput {
    pub fleets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyFleetInput {
    pub context: String,
    pub excess_capacity_termination_policy: String,
    pub fleet_id: String,
    pub launch_template_configs: Vec<String>,
    pub region: String,
    pub target_capacity_specification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModifyFleetOutput {
    pub success: bool,
}

#[async_trait]
pub trait Ec2Action {
    async fn run_instances(&self, input: RunInstancesInput) -> Result<RunInstancesOutput, Box<dyn std::error::Error>>;
    async fn start_instances(&self, input: StartInstancesInput) -> Result<StartInstancesOutput, Box<dyn std::error::Error>>;
    async fn stop_instances(&self, input: StopInstancesInput) -> Result<StopInstancesOutput, Box<dyn std::error::Error>>;
    async fn reboot_instances(&self, input: RebootInstancesInput) -> Result<RebootInstancesOutput, Box<dyn std::error::Error>>;
    async fn terminate_instances(&self, input: TerminateInstancesInput) -> Result<TerminateInstancesOutput, Box<dyn std::error::Error>>;
    async fn describe_instances(&self, input: DescribeInstancesInput) -> Result<DescribeInstancesOutput, Box<dyn std::error::Error>>;
    async fn describe_instance_status(&self, input: DescribeInstanceStatusInput) -> Result<DescribeInstanceStatusOutput, Box<dyn std::error::Error>>;
    async fn modify_instance_attribute(&self, input: ModifyInstanceAttributeInput) -> Result<ModifyInstanceAttributeOutput, Box<dyn std::error::Error>>;
    async fn get_console_output(&self, input: GetConsoleOutputInput) -> Result<GetConsoleOutputOutput, Box<dyn std::error::Error>>;
    async fn get_password_data(&self, input: GetPasswordDataInput) -> Result<GetPasswordDataOutput, Box<dyn std::error::Error>>;
    async fn create_image(&self, input: CreateImageInput) -> Result<CreateImageOutput, Box<dyn std::error::Error>>;
    async fn deregister_image(&self, input: DeregisterImageInput) -> Result<DeregisterImageOutput, Box<dyn std::error::Error>>;
    async fn describe_images(&self, input: DescribeImagesInput) -> Result<DescribeImagesOutput, Box<dyn std::error::Error>>;
    async fn copy_image(&self, input: CopyImageInput) -> Result<CopyImageOutput, Box<dyn std::error::Error>>;
    async fn modify_image_attribute(&self, input: ModifyImageAttributeInput) -> Result<ModifyImageAttributeOutput, Box<dyn std::error::Error>>;
    async fn create_snapshot(&self, input: CreateSnapshotInput) -> Result<CreateSnapshotOutput, Box<dyn std::error::Error>>;
    async fn delete_snapshot(&self, input: DeleteSnapshotInput) -> Result<DeleteSnapshotOutput, Box<dyn std::error::Error>>;
    async fn describe_snapshots(&self, input: DescribeSnapshotsInput) -> Result<DescribeSnapshotsOutput, Box<dyn std::error::Error>>;
    async fn copy_snapshot(&self, input: CopySnapshotInput) -> Result<CopySnapshotOutput, Box<dyn std::error::Error>>;
    async fn create_volume(&self, input: CreateVolumeInput) -> Result<CreateVolumeOutput, Box<dyn std::error::Error>>;
    async fn delete_volume(&self, input: DeleteVolumeInput) -> Result<DeleteVolumeOutput, Box<dyn std::error::Error>>;
    async fn describe_volumes(&self, input: DescribeVolumesInput) -> Result<DescribeVolumesOutput, Box<dyn std::error::Error>>;
    async fn attach_volume(&self, input: AttachVolumeInput) -> Result<AttachVolumeOutput, Box<dyn std::error::Error>>;
    async fn detach_volume(&self, input: DetachVolumeInput) -> Result<DetachVolumeOutput, Box<dyn std::error::Error>>;
    async fn modify_volume(&self, input: ModifyVolumeInput) -> Result<ModifyVolumeOutput, Box<dyn std::error::Error>>;
    async fn create_key_pair(&self, input: CreateKeyPairInput) -> Result<CreateKeyPairOutput, Box<dyn std::error::Error>>;
    async fn delete_key_pair(&self, input: DeleteKeyPairInput) -> Result<DeleteKeyPairOutput, Box<dyn std::error::Error>>;
    async fn describe_key_pairs(&self, input: DescribeKeyPairsInput) -> Result<DescribeKeyPairsOutput, Box<dyn std::error::Error>>;
    async fn import_key_pair(&self, input: ImportKeyPairInput) -> Result<ImportKeyPairOutput, Box<dyn std::error::Error>>;
    async fn create_launch_template(&self, input: CreateLaunchTemplateInput) -> Result<CreateLaunchTemplateOutput, Box<dyn std::error::Error>>;
    async fn delete_launch_template(&self, input: DeleteLaunchTemplateInput) -> Result<DeleteLaunchTemplateOutput, Box<dyn std::error::Error>>;
    async fn describe_launch_templates(&self, input: DescribeLaunchTemplatesInput) -> Result<DescribeLaunchTemplatesOutput, Box<dyn std::error::Error>>;
    async fn create_launch_template_version(&self, input: CreateLaunchTemplateVersionInput) -> Result<CreateLaunchTemplateVersionOutput, Box<dyn std::error::Error>>;
    async fn delete_launch_template_versions(&self, input: DeleteLaunchTemplateVersionsInput) -> Result<DeleteLaunchTemplateVersionsOutput, Box<dyn std::error::Error>>;
    async fn modify_launch_template(&self, input: ModifyLaunchTemplateInput) -> Result<ModifyLaunchTemplateOutput, Box<dyn std::error::Error>>;
    async fn create_network_interface(&self, input: CreateNetworkInterfaceInput) -> Result<CreateNetworkInterfaceOutput, Box<dyn std::error::Error>>;
    async fn delete_network_interface(&self, input: DeleteNetworkInterfaceInput) -> Result<DeleteNetworkInterfaceOutput, Box<dyn std::error::Error>>;
    async fn describe_network_interfaces(&self, input: DescribeNetworkInterfacesInput) -> Result<DescribeNetworkInterfacesOutput, Box<dyn std::error::Error>>;
    async fn attach_network_interface(&self, input: AttachNetworkInterfaceInput) -> Result<AttachNetworkInterfaceOutput, Box<dyn std::error::Error>>;
    async fn detach_network_interface(&self, input: DetachNetworkInterfaceInput) -> Result<DetachNetworkInterfaceOutput, Box<dyn std::error::Error>>;
    async fn modify_network_interface_attribute(&self, input: ModifyNetworkInterfaceAttributeInput) -> Result<ModifyNetworkInterfaceAttributeOutput, Box<dyn std::error::Error>>;
    async fn create_placement_group(&self, input: CreatePlacementGroupInput) -> Result<CreatePlacementGroupOutput, Box<dyn std::error::Error>>;
    async fn delete_placement_group(&self, input: DeletePlacementGroupInput) -> Result<DeletePlacementGroupOutput, Box<dyn std::error::Error>>;
    async fn describe_placement_groups(&self, input: DescribePlacementGroupsInput) -> Result<DescribePlacementGroupsOutput, Box<dyn std::error::Error>>;
    async fn describe_instance_types(&self, input: DescribeInstanceTypesInput) -> Result<DescribeInstanceTypesOutput, Box<dyn std::error::Error>>;
    async fn describe_availability_zones(&self, input: DescribeAvailabilityZonesInput) -> Result<DescribeAvailabilityZonesOutput, Box<dyn std::error::Error>>;
    async fn describe_regions(&self, input: DescribeRegionsInput) -> Result<DescribeRegionsOutput, Box<dyn std::error::Error>>;
    async fn create_tags(&self, input: CreateTagsInput) -> Result<CreateTagsOutput, Box<dyn std::error::Error>>;
    async fn delete_tags(&self, input: DeleteTagsInput) -> Result<DeleteTagsOutput, Box<dyn std::error::Error>>;
    async fn describe_tags(&self, input: DescribeTagsInput) -> Result<DescribeTagsOutput, Box<dyn std::error::Error>>;
    async fn request_spot_instances(&self, input: RequestSpotInstancesInput) -> Result<RequestSpotInstancesOutput, Box<dyn std::error::Error>>;
    async fn cancel_spot_instance_requests(&self, input: CancelSpotInstanceRequestsInput) -> Result<CancelSpotInstanceRequestsOutput, Box<dyn std::error::Error>>;
    async fn describe_spot_instance_requests(&self, input: DescribeSpotInstanceRequestsInput) -> Result<DescribeSpotInstanceRequestsOutput, Box<dyn std::error::Error>>;
    async fn describe_spot_price_history(&self, input: DescribeSpotPriceHistoryInput) -> Result<DescribeSpotPriceHistoryOutput, Box<dyn std::error::Error>>;
    async fn create_fleet(&self, input: CreateFleetInput) -> Result<CreateFleetOutput, Box<dyn std::error::Error>>;
    async fn delete_fleets(&self, input: DeleteFleetsInput) -> Result<DeleteFleetsOutput, Box<dyn std::error::Error>>;
    async fn describe_fleets(&self, input: DescribeFleetsInput) -> Result<DescribeFleetsOutput, Box<dyn std::error::Error>>;
    async fn modify_fleet(&self, input: ModifyFleetInput) -> Result<ModifyFleetOutput, Box<dyn std::error::Error>>;
}

// Harana Actions - AWS EC2 Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// run_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunInstancesOutput {
    pub instances: Vec<Instance>,
    pub owner_id: String,
    pub reservation_id: String,
    pub success: bool,
}

// start_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartInstancesOutput {
    pub starting_instances: Vec<InstanceStateChange>,
    pub success: bool,
}

// stop_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopInstancesOutput {
    pub stopping_instances: Vec<InstanceStateChange>,
    pub success: bool,
}

// reboot_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebootInstancesOutput {
    pub success: bool,
}

// terminate_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateInstancesOutput {
    pub success: bool,
    pub terminating_instances: Vec<InstanceStateChange>,
}

// describe_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeInstancesOutput {
    pub reservations: Vec<Reservation>,
}

// describe_instance_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeInstanceStatusOutput {
    pub instance_statuses: Vec<InstanceStatus>,
    pub next_token: Option<String>,
}

// create_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageOutput {
    pub image_id: String,
    pub success: bool,
}

// describe_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeImagesOutput {
    pub images: Vec<Ec2Image>,
}

// create_security_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSecurityGroupOutput {
    pub group_id: String,
    pub success: bool,
}

// delete_security_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSecurityGroupOutput {
    pub success: bool,
}

// describe_security_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeSecurityGroupsOutput {
    pub security_groups: Vec<SecurityGroup>,
    pub next_token: Option<String>,
}

// authorize_security_group_ingress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeSecurityGroupIngressOutput {
    pub success: bool,
}

// revoke_security_group_ingress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeSecurityGroupIngressOutput {
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub instance_id: String,
    pub instance_type: String,
    pub image_id: String,
    pub state: InstanceState,
    pub private_ip_address: Option<String>,
    pub public_ip_address: Option<String>,
    pub private_dns_name: Option<String>,
    pub public_dns_name: Option<String>,
    pub subnet_id: Option<String>,
    pub vpc_id: Option<String>,
    pub key_name: Option<String>,
    pub launch_time: String,
    pub availability_zone: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceState {
    pub code: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStateChange {
    pub instance_id: String,
    pub current_state: InstanceState,
    pub previous_state: InstanceState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub reservation_id: String,
    pub owner_id: String,
    pub instances: Vec<Instance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStatus {
    pub instance_id: String,
    pub instance_state: InstanceState,
    pub availability_zone: String,
    pub system_status: StatusSummary,
    pub instance_status: StatusSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSummary {
    pub status: String,
    pub details: Vec<StatusDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDetail {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ec2Image {
    pub image_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub state: String,
    pub owner_id: String,
    pub creation_date: Option<String>,
    pub architecture: String,
    pub platform: Option<String>,
    pub root_device_type: String,
    pub virtualization_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGroup {
    pub group_id: String,
    pub group_name: String,
    pub description: String,
    pub vpc_id: Option<String>,
    pub owner_id: String,
    pub ip_permissions: Vec<IpPermission>,
    pub ip_permissions_egress: Vec<IpPermission>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpPermission {
    pub ip_protocol: String,
    pub from_port: Option<i32>,
    pub to_port: Option<i32>,
    pub ip_ranges: Vec<IpRange>,
    pub user_id_group_pairs: Vec<UserIdGroupPair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRange {
    pub cidr_ip: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdGroupPair {
    pub group_id: String,
    pub user_id: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ec2Tags {
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ec2Filter {
    pub name: String,
    pub values: Vec<String>,
}

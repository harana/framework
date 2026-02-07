// Harana Actions - AWS ECS Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// create_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClusterOutput {
    pub cluster: Cluster,
    pub success: bool,
}

// delete_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteClusterOutput {
    pub cluster: Cluster,
    pub success: bool,
}

// describe_clusters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeClustersOutput {
    pub clusters: Vec<Cluster>,
    pub failures: Vec<Failure>,
}

// list_clusters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListClustersOutput {
    pub cluster_arns: Vec<String>,
    pub next_token: Option<String>,
}

// register_task_definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterTaskDefinitionOutput {
    pub task_definition: TaskDefinition,
    pub success: bool,
}

// deregister_task_definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeregisterTaskDefinitionOutput {
    pub task_definition: TaskDefinition,
    pub success: bool,
}

// describe_task_definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeTaskDefinitionOutput {
    pub task_definition: TaskDefinition,
}

// list_task_definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTaskDefinitionsOutput {
    pub task_definition_arns: Vec<String>,
    pub next_token: Option<String>,
}

// create_service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceOutput {
    pub service: Service,
    pub success: bool,
}

// update_service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServiceOutput {
    pub service: Service,
    pub success: bool,
}

// delete_service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteServiceOutput {
    pub service: Service,
    pub success: bool,
}

// describe_services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeServicesOutput {
    pub services: Vec<Service>,
    pub failures: Vec<Failure>,
}

// list_services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListServicesOutput {
    pub service_arns: Vec<String>,
    pub next_token: Option<String>,
}

// run_task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTaskOutput {
    pub tasks: Vec<Task>,
    pub failures: Vec<Failure>,
    pub success: bool,
}

// stop_task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTaskOutput {
    pub task: Task,
    pub success: bool,
}

// describe_tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeTasksOutput {
    pub tasks: Vec<Task>,
    pub failures: Vec<Failure>,
}

// list_tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksOutput {
    pub task_arns: Vec<String>,
    pub next_token: Option<String>,
}

// list_container_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContainerInstancesOutput {
    pub container_instance_arns: Vec<String>,
    pub next_token: Option<String>,
}

// describe_container_instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeContainerInstancesOutput {
    pub container_instances: Vec<ContainerInstance>,
    pub failures: Vec<Failure>,
}

// update_container_instances_state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContainerInstancesStateOutput {
    pub container_instances: Vec<ContainerInstance>,
    pub failures: Vec<Failure>,
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub cluster_arn: Option<String>,
    pub cluster_name: Option<String>,
    pub status: Option<String>,
    pub registered_container_instances_count: i32,
    pub running_tasks_count: i32,
    pub pending_tasks_count: i32,
    pub active_services_count: i32,
    pub capacity_providers: Vec<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDefinition {
    pub task_definition_arn: Option<String>,
    pub family: Option<String>,
    pub revision: i32,
    pub status: Option<String>,
    pub container_definitions: Vec<ContainerDefinition>,
    pub requires_compatibilities: Vec<String>,
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub network_mode: Option<String>,
    pub execution_role_arn: Option<String>,
    pub task_role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDefinition {
    pub name: Option<String>,
    pub image: Option<String>,
    pub cpu: i32,
    pub memory: Option<i32>,
    pub memory_reservation: Option<i32>,
    pub essential: bool,
    pub port_mappings: Vec<PortMapping>,
    pub environment: Vec<KeyValuePair>,
    pub command: Vec<String>,
    pub entry_point: Vec<String>,
    pub log_configuration: Option<LogConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: Option<i32>,
    pub host_port: Option<i32>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfiguration {
    pub log_driver: String,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub service_arn: Option<String>,
    pub service_name: Option<String>,
    pub cluster_arn: Option<String>,
    pub status: Option<String>,
    pub desired_count: i32,
    pub running_count: i32,
    pub pending_count: i32,
    pub launch_type: Option<String>,
    pub task_definition: Option<String>,
    pub deployments: Vec<Deployment>,
    pub load_balancers: Vec<LoadBalancer>,
    pub network_configuration: Option<NetworkConfiguration>,
    pub created_at: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub id: Option<String>,
    pub status: Option<String>,
    pub task_definition: Option<String>,
    pub desired_count: i32,
    pub running_count: i32,
    pub pending_count: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub launch_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    pub target_group_arn: Option<String>,
    pub load_balancer_name: Option<String>,
    pub container_name: Option<String>,
    pub container_port: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsVpcConfiguration {
    pub subnets: Vec<String>,
    pub security_groups: Vec<String>,
    pub assign_public_ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_arn: Option<String>,
    pub task_definition_arn: Option<String>,
    pub cluster_arn: Option<String>,
    pub container_instance_arn: Option<String>,
    pub last_status: Option<String>,
    pub desired_status: Option<String>,
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub containers: Vec<Container>,
    pub started_at: Option<String>,
    pub stopped_at: Option<String>,
    pub stopped_reason: Option<String>,
    pub launch_type: Option<String>,
    pub group: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub container_arn: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub last_status: Option<String>,
    pub exit_code: Option<i32>,
    pub reason: Option<String>,
    pub network_bindings: Vec<NetworkBinding>,
    pub network_interfaces: Vec<NetworkInterface>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkBinding {
    pub bind_ip: Option<String>,
    pub container_port: Option<i32>,
    pub host_port: Option<i32>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub attachment_id: Option<String>,
    pub private_ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInstance {
    pub container_instance_arn: Option<String>,
    pub ec2_instance_id: Option<String>,
    pub status: Option<String>,
    pub status_reason: Option<String>,
    pub running_tasks_count: i32,
    pub pending_tasks_count: i32,
    pub agent_connected: bool,
    pub registered_at: Option<String>,
    pub remaining_resources: Vec<Resource>,
    pub registered_resources: Vec<Resource>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub integer_value: i32,
    pub string_set_value: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Failure {
    pub arn: Option<String>,
    pub reason: Option<String>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsTags {
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsFilter {
    pub name: String,
    pub values: Vec<String>,
}

// Input types for complex parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDefinitionInput {
    pub name: String,
    pub image: String,
    pub cpu: Option<i32>,
    pub memory: Option<i32>,
    pub memory_reservation: Option<i32>,
    pub essential: Option<bool>,
    pub port_mappings: Option<Vec<PortMappingInput>>,
    pub environment: Option<Vec<KeyValuePairInput>>,
    pub command: Option<Vec<String>>,
    pub entry_point: Option<Vec<String>>,
    pub log_configuration: Option<LogConfigurationInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMappingInput {
    pub container_port: i32,
    pub host_port: Option<i32>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValuePairInput {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfigurationInput {
    pub log_driver: String,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfigurationInput {
    pub awsvpc_configuration: AwsVpcConfigurationInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsVpcConfigurationInput {
    pub subnets: Vec<String>,
    pub security_groups: Option<Vec<String>>,
    pub assign_public_ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerInput {
    pub target_group_arn: Option<String>,
    pub load_balancer_name: Option<String>,
    pub container_name: String,
    pub container_port: i32,
}

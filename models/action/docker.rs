// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildInput {
    pub build_args: std::collections::HashMap<String, String>,
    pub context: String,
    pub dockerfile: String,
    #[serde(default)]
    pub no_cache: bool,
    pub platform: String,
    #[serde(default)]
    pub pull: bool,
    pub tags: Vec<String>,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildOutput {
    pub image_id: String,
    pub size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushInput {
    #[serde(default)]
    pub all_tags: bool,
    pub image: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PushOutput {
    pub digest: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullInput {
    #[serde(default)]
    pub all_tags: bool,
    pub image: String,
    pub platform: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullOutput {
    pub digest: String,
    pub image_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListImagesInput {
    #[serde(default)]
    pub all: bool,
    pub filters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListImagesOutput {
    pub images: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveImageInput {
    #[serde(default)]
    pub force: bool,
    pub image: String,
    #[serde(default)]
    pub no_prune: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveImageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagInput {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunInput {
    pub command: Vec<String>,
    #[serde(default)]
    pub detach: bool,
    pub entrypoint: String,
    pub env: std::collections::HashMap<String, String>,
    pub image: String,
    pub labels: std::collections::HashMap<String, String>,
    pub name: String,
    pub network: String,
    pub ports: Vec<String>,
    #[serde(default)]
    pub remove: bool,
    pub restart: String,
    pub volumes: Vec<String>,
    pub working_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunOutput {
    pub container_id: String,
    pub exit_code: i64,
    pub logs: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartInput {
    pub container: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopInput {
    pub container: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestartInput {
    pub container: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestartOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveContainerInput {
    pub container: String,
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub volumes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveContainerOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListContainersInput {
    #[serde(default)]
    pub all: bool,
    pub filters: String,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListContainersOutput {
    pub containers: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogsInput {
    pub container: String,
    #[serde(default)]
    pub follow: bool,
    pub since: String,
    pub tail: i64,
    #[serde(default)]
    pub timestamps: bool,
    pub until: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogsOutput {
    pub logs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecInput {
    pub command: Vec<String>,
    pub container: String,
    #[serde(default)]
    pub detach: bool,
    pub env: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub privileged: bool,
    #[serde(default)]
    pub tty: bool,
    pub user: String,
    pub working_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecOutput {
    pub exit_code: i64,
    pub output: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InspectContainerInput {
    pub container: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InspectContainerOutput {
    pub config: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub image: String,
    pub name: String,
    pub network_settings: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkInput {
    pub driver: String,
    pub labels: std::collections::HashMap<String, String>,
    pub name: String,
    pub options: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNetworkOutput {
    pub network_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveNetworkInput {
    pub network: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveNetworkOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListNetworksInput {
    pub filters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListNetworksOutput {
    pub networks: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVolumeInput {
    pub driver: String,
    pub labels: std::collections::HashMap<String, String>,
    pub name: String,
    pub options: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVolumeOutput {
    pub success: bool,
    pub volume_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveVolumeInput {
    #[serde(default)]
    pub force: bool,
    pub volume: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveVolumeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVolumesInput {
    pub filters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVolumesOutput {
    pub total: i64,
    pub volumes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginInput {
    pub password: String,
    pub registry: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogoutInput {
    pub registry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogoutOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerImage {
    pub id: String,
    pub tags: Vec<String>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub size: i64,
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub state: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub ports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerContainerState {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    pub exit_code: i64,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerContainerConfig {
    pub hostname: String,
    pub user: String,
    pub env: Vec<String>,
    pub cmd: Vec<String>,
    pub working_dir: String,
    pub entrypoint: Vec<String>,
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetworkSettings {
    pub networks: std::collections::HashMap<String, String>,
    pub ip_address: String,
    pub gateway: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetworkEndpoint {
    pub network_id: String,
    pub ip_address: String,
    pub gateway: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerPortMapping {
    pub container_port: i64,
    pub host_port: i64,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerVolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetwork {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerVolume {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerImageFilters {
    pub dangling: bool,
    pub label: Vec<String>,
    pub reference: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerContainerFilters {
    pub id: Vec<String>,
    pub name: Vec<String>,
    pub status: Vec<String>,
    pub label: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetworkFilters {
    pub driver: Vec<String>,
    pub id: Vec<String>,
    pub name: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerVolumeFilters {
    pub dangling: bool,
    pub driver: Vec<String>,
    pub name: Vec<String>,
}

#[async_trait]
pub trait DockerAction {
    async fn build(&self, input: BuildInput) -> Result<BuildOutput, Box<dyn std::error::Error>>;
    async fn push(&self, input: PushInput) -> Result<PushOutput, Box<dyn std::error::Error>>;
    async fn pull(&self, input: PullInput) -> Result<PullOutput, Box<dyn std::error::Error>>;
    async fn list_images(&self, input: ListImagesInput) -> Result<ListImagesOutput, Box<dyn std::error::Error>>;
    async fn remove_image(&self, input: RemoveImageInput) -> Result<RemoveImageOutput, Box<dyn std::error::Error>>;
    async fn tag(&self, input: TagInput) -> Result<TagOutput, Box<dyn std::error::Error>>;
    async fn run(&self, input: RunInput) -> Result<RunOutput, Box<dyn std::error::Error>>;
    async fn start(&self, input: StartInput) -> Result<StartOutput, Box<dyn std::error::Error>>;
    async fn stop(&self, input: StopInput) -> Result<StopOutput, Box<dyn std::error::Error>>;
    async fn restart(&self, input: RestartInput) -> Result<RestartOutput, Box<dyn std::error::Error>>;
    async fn remove_container(&self, input: RemoveContainerInput) -> Result<RemoveContainerOutput, Box<dyn std::error::Error>>;
    async fn list_containers(&self, input: ListContainersInput) -> Result<ListContainersOutput, Box<dyn std::error::Error>>;
    async fn logs(&self, input: LogsInput) -> Result<LogsOutput, Box<dyn std::error::Error>>;
    async fn exec(&self, input: ExecInput) -> Result<ExecOutput, Box<dyn std::error::Error>>;
    async fn inspect_container(&self, input: InspectContainerInput) -> Result<InspectContainerOutput, Box<dyn std::error::Error>>;
    async fn create_network(&self, input: CreateNetworkInput) -> Result<CreateNetworkOutput, Box<dyn std::error::Error>>;
    async fn remove_network(&self, input: RemoveNetworkInput) -> Result<RemoveNetworkOutput, Box<dyn std::error::Error>>;
    async fn list_networks(&self, input: ListNetworksInput) -> Result<ListNetworksOutput, Box<dyn std::error::Error>>;
    async fn create_volume(&self, input: CreateVolumeInput) -> Result<CreateVolumeOutput, Box<dyn std::error::Error>>;
    async fn remove_volume(&self, input: RemoveVolumeInput) -> Result<RemoveVolumeOutput, Box<dyn std::error::Error>>;
    async fn list_volumes(&self, input: ListVolumesInput) -> Result<ListVolumesOutput, Box<dyn std::error::Error>>;
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, Box<dyn std::error::Error>>;
    async fn logout(&self, input: LogoutInput) -> Result<LogoutOutput, Box<dyn std::error::Error>>;
}

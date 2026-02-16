// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildOutput {
    pub image_id: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PullOutput {
    pub digest: String,
    pub image_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListImagesOutput {
    pub images: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunOutput {
    pub container_id: String,
    pub exit_code: i64,
    pub logs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListContainersOutput {
    pub containers: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExecOutput {
    pub exit_code: i64,
    pub output: String,
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
pub struct ListNetworksOutput {
    pub networks: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListVolumesOutput {
    pub total: i64,
    pub volumes: Vec<String>,
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
    async fn build(&self, build_args: std::collections::HashMap<String, String>, context: String, dockerfile: String, no_cache: bool, platform: String, pull: bool, tags: Vec<String>, target: String) -> Result<BuildOutput, Box<dyn std::error::Error>>;
    async fn push(&self, all_tags: bool, image: String, tag: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn pull(&self, all_tags: bool, image: String, platform: String, tag: String) -> Result<PullOutput, Box<dyn std::error::Error>>;
    async fn list_images(&self, all: bool, filters: String) -> Result<ListImagesOutput, Box<dyn std::error::Error>>;
    async fn remove_image(&self, force: bool, image: String, no_prune: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn tag(&self, source: String, target: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn run(&self, command: Vec<String>, detach: bool, entrypoint: String, env: std::collections::HashMap<String, String>, image: String, labels: std::collections::HashMap<String, String>, name: String, network: String, ports: Vec<String>, remove: bool, restart: String, volumes: Vec<String>, working_dir: String) -> Result<RunOutput, Box<dyn std::error::Error>>;
    async fn start(&self, container: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self, container: String, timeout: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn restart(&self, container: String, timeout: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_container(&self, container: String, force: bool, volumes: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_containers(&self, all: bool, filters: String, limit: i64) -> Result<ListContainersOutput, Box<dyn std::error::Error>>;
    async fn logs(&self, container: String, follow: bool, since: String, tail: i64, timestamps: bool, until: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn exec(&self, command: Vec<String>, container: String, detach: bool, env: std::collections::HashMap<String, String>, privileged: bool, tty: bool, user: String, working_dir: String) -> Result<ExecOutput, Box<dyn std::error::Error>>;
    async fn inspect_container(&self, container: String) -> Result<InspectContainerOutput, Box<dyn std::error::Error>>;
    async fn create_network(&self, driver: String, labels: std::collections::HashMap<String, String>, name: String, options: std::collections::HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn remove_network(&self, network: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_networks(&self, filters: String) -> Result<ListNetworksOutput, Box<dyn std::error::Error>>;
    async fn create_volume(&self, driver: String, labels: std::collections::HashMap<String, String>, name: String, options: std::collections::HashMap<String, String>, volume_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_volume(&self, force: bool, volume: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_volumes(&self, filters: String) -> Result<ListVolumesOutput, Box<dyn std::error::Error>>;
    async fn login(&self, password: String, registry: String, username: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn logout(&self, registry: String) -> Result<(), Box<dyn std::error::Error>>;
}

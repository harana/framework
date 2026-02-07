// Harana Actions - Docker Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildOutput {
    pub image_id: String,
    pub size: i64,
    pub success: bool,
}

// push
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushOutput {
    pub digest: String,
    pub success: bool,
}

// pull
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullOutput {
    pub digest: String,
    pub image_id: String,
    pub success: bool,
}

// list_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListImagesOutput {
    pub images: Vec<DockerImage>,
    pub total: i32,
}

// remove_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveImageOutput {
    pub success: bool,
}

// tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagOutput {
    pub success: bool,
}

// run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOutput {
    pub container_id: String,
    pub exit_code: Option<i32>,
    pub logs: Option<String>,
    pub success: bool,
}

// start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOutput {
    pub success: bool,
}

// stop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopOutput {
    pub success: bool,
}

// restart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartOutput {
    pub success: bool,
}

// remove_container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveContainerOutput {
    pub success: bool,
}

// list_containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContainersOutput {
    pub containers: Vec<DockerContainer>,
    pub total: i32,
}

// logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsOutput {
    pub logs: String,
}

// exec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecOutput {
    pub exit_code: i32,
    pub output: String,
    pub success: bool,
}

// inspect_container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectContainerOutput {
    pub container: DockerContainerDetails,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerImage {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainer {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: String,
    pub status: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerDetails {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: DockerContainerState,
    pub config: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerState {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerPortMapping {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerVolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerImageFilters {
    pub dangling: Option<bool>,
    pub label: Option<Vec<String>>,
    pub reference: Option<Vec<String>>,
}

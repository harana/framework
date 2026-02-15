// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// docker_image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerImage {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub digest: Option<String>,
    pub image_id: String,
    pub platform: Option<String>,
    pub repository: Option<String>,
    pub size: Option<i64>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl DockerImage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_container
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerContainer {
    pub command: Option<String>,
    pub container_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub exit_code: Option<i64>,
    pub image_id: String,
    pub labels: Option<String>,
    pub name: Option<String>,
    pub network: Option<String>,
    pub ports: Option<String>,
    pub restart_policy: String,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub stopped_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl DockerContainer {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_network
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetwork {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub driver: String,
    pub labels: Option<String>,
    pub name: String,
    pub network_id: String,
    pub subnet: Option<String>,
}

impl DockerNetwork {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_volume
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerVolume {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub driver: String,
    pub labels: Option<String>,
    pub mountpoint: Option<String>,
    pub name: String,
}

impl DockerVolume {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_container_state
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

impl DockerContainerState {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_container_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerContainerConfig {
    pub hostname: String,
    pub user: String,
    pub env: Vec<String>,
    pub cmd: Vec<String>,
    pub working_dir: String,
    pub entrypoint: Vec<String>,
    pub labels: String,
}

impl DockerContainerConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_network_settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetworkSettings {
    pub networks: String,
    pub ip_address: String,
    pub gateway: String,
}

impl DockerNetworkSettings {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_network_endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetworkEndpoint {
    pub network_id: String,
    pub ip_address: String,
    pub gateway: String,
}

impl DockerNetworkEndpoint {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_port_mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerPortMapping {
    pub container_port: i64,
    pub host_port: i64,
    pub protocol: String,
}

impl DockerPortMapping {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_volume_mount
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerVolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

impl DockerVolumeMount {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_image_filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerImageFilters {
    pub dangling: bool,
    pub label: Vec<String>,
    pub reference: Vec<String>,
}

impl DockerImageFilters {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_container_filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerContainerFilters {
    pub id: Vec<String>,
    pub name: Vec<String>,
    pub status: Vec<String>,
    pub label: Vec<String>,
}

impl DockerContainerFilters {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_network_filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerNetworkFilters {
    pub driver: Vec<String>,
    pub id: Vec<String>,
    pub name: Vec<String>,
}

impl DockerNetworkFilters {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// docker_volume_filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DockerVolumeFilters {
    pub dangling: bool,
    pub driver: Vec<String>,
    pub name: Vec<String>,
}

impl DockerVolumeFilters {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


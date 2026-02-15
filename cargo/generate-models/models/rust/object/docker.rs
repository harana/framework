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


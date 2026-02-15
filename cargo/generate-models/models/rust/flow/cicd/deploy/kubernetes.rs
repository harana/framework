// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_kubernetes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetes {
    pub context: String,
    #[serde(default)]
    pub enabled: bool,
    pub health_check: Option<String>,
    pub helm: Option<String>,
    pub image: String,
    pub kubectl: Option<String>,
    pub kustomize: Option<String>,
    pub manifests: Vec<String>,
    pub namespace: String,
    pub post_deploy: Vec<String>,
    pub pre_deploy: Vec<String>,
    pub replicas: i64,
    pub rolling_update: Option<String>,
    pub strategy: String,
}

impl DeployKubernetes {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_kubernetes_rolling_update
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesRollingUpdate {
    pub max_surge: i64,
    pub max_unavailable: i64,
}

impl DeployKubernetesRollingUpdate {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_kubernetes_helm
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesHelm {
    pub chart: String,
    #[serde(default)]
    pub enabled: bool,
    pub release_name: String,
    pub set: Vec<String>,
    pub timeout: i64,
    pub values_file: Option<String>,
    #[serde(default)]
    pub wait: bool,
}

impl DeployKubernetesHelm {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_kubernetes_kubectl
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesKubectl {
    pub apply_options: Vec<String>,
    pub version: Option<String>,
}

impl DeployKubernetesKubectl {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_kubernetes_kustomize
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesKustomize {
    pub dir: String,
    #[serde(default)]
    pub enabled: bool,
}

impl DeployKubernetesKustomize {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_kubernetes_health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesHealthCheck {
    #[serde(default)]
    pub enabled: bool,
    pub endpoint: String,
    pub interval: i64,
    pub timeout: i64,
}

impl DeployKubernetesHealthCheck {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


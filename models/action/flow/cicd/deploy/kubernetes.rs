// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetes {
    pub context: String,
    #[serde(default)]
    pub enabled: bool,
    pub health_check: String,
    pub helm: String,
    pub image: String,
    pub kubectl: String,
    pub kustomize: String,
    pub manifests: Vec<String>,
    pub namespace: String,
    pub post_deploy: Vec<String>,
    pub pre_deploy: Vec<String>,
    pub replicas: i64,
    pub rolling_update: String,
    pub strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesRollingUpdate {
    pub max_surge: i64,
    pub max_unavailable: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesHelm {
    pub chart: String,
    #[serde(default)]
    pub enabled: bool,
    pub release_name: String,
    pub set: Vec<String>,
    pub timeout: i64,
    pub values_file: String,
    #[serde(default)]
    pub wait: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesKubectl {
    pub apply_options: Vec<String>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesKustomize {
    pub dir: String,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKubernetesHealthCheck {
    #[serde(default)]
    pub enabled: bool,
    pub endpoint: String,
    pub interval: i64,
    pub timeout: i64,
}


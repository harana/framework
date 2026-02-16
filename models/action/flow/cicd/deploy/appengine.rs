// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAppengine {
    #[serde(default)]
    pub enabled: bool,
    pub env_variables: std::collections::HashMap<String, String>,
    pub instance_type: String,
    pub project_id: String,
    #[serde(default)]
    pub promote_traffic: bool,
    pub region: String,
    pub runtime: String,
    pub scaling: String,
    pub service: String,
    #[serde(default)]
    pub stop_previous_version: bool,
    pub version: String,
    pub vpc_access_connector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAppengineScaling {
    pub max_instances: i64,
    pub min_instances: i64,
    pub target_cpu_utilization: f64,
    pub target_throughput_utilization: f64,
    pub type: String,
}


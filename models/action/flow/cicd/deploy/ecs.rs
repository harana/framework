// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployEcs {
    #[serde(default)]
    pub assign_public_ip: bool,
    #[serde(default)]
    pub auto_rollback: bool,
    pub cluster: String,
    pub container_name: String,
    pub cpu: String,
    pub deployment: String,
    pub desired_count: i64,
    #[serde(default)]
    pub enabled: bool,
    pub health_check_grace_period: i64,
    pub image: String,
    pub launch_type: String,
    pub load_balancer: String,
    pub memory: String,
    pub network_mode: String,
    pub region: String,
    pub security_groups: Vec<String>,
    pub service: String,
    pub subnets: Vec<String>,
    pub task_definition: String,
    #[serde(default)]
    pub wait_for_stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployEcsDeployment {
    pub maximum_percent: i64,
    pub minimum_healthy_percent: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployEcsLoadBalancer {
    pub container_port: i64,
    pub target_group_arn: String,
}


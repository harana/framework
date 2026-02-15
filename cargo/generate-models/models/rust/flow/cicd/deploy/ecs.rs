// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_ecs
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
    pub deployment: Option<String>,
    pub desired_count: i64,
    #[serde(default)]
    pub enabled: bool,
    pub health_check_grace_period: i64,
    pub image: String,
    pub launch_type: String,
    pub load_balancer: Option<String>,
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

impl DeployEcs {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_ecs_deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployEcsDeployment {
    pub maximum_percent: i64,
    pub minimum_healthy_percent: i64,
}

impl DeployEcsDeployment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_ecs_load_balancer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployEcsLoadBalancer {
    pub container_port: i64,
    pub target_group_arn: String,
}

impl DeployEcsLoadBalancer {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


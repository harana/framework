// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_ecs_cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcsCluster {
    pub account_id: String,
    pub active_services_count: i64,
    pub capacity_providers: Option<String>,
    pub cluster_arn: Option<String>,
    pub cluster_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub pending_tasks_count: i64,
    pub region: Option<String>,
    pub registered_container_instances_count: i64,
    pub running_tasks_count: i64,
    pub settings: Option<String>,
    pub status: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEcsCluster {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ecs_service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcsService {
    /// Reference: aws_ecs_cluster.id
    pub cluster_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub desired_count: i64,
    pub launch_type: String,
    pub load_balancers: Option<String>,
    pub network_configuration: Option<String>,
    pub pending_count: i64,
    pub platform_version: Option<String>,
    pub running_count: i64,
    pub scheduling_strategy: String,
    pub service_arn: Option<String>,
    pub service_name: String,
    pub status: String,
    pub tags: Option<String>,
    pub task_definition: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEcsService {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ecs_task_definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcsTaskDefinition {
    pub account_id: String,
    pub container_definitions: Option<String>,
    pub cpu: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub execution_role_arn: Option<String>,
    pub family: String,
    pub memory: Option<String>,
    pub network_mode: String,
    pub region: Option<String>,
    pub requires_compatibilities: Option<String>,
    pub revision: i64,
    pub status: String,
    pub tags: Option<String>,
    pub task_definition_arn: Option<String>,
    pub task_role_arn: Option<String>,
}

impl AwsEcsTaskDefinition {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ecs_task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcsTask {
    /// Reference: aws_ecs_cluster.id
    pub cluster_id: String,
    pub connectivity: String,
    pub container_instance_arn: Option<String>,
    pub cpu: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub desired_status: String,
    pub group: Option<String>,
    pub last_status: String,
    pub launch_type: String,
    pub memory: Option<String>,
    pub platform_version: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub stopped_at: Option<chrono::DateTime<chrono::Utc>>,
    pub stopped_reason: Option<String>,
    pub tags: Option<String>,
    pub task_arn: String,
    pub task_definition_arn: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEcsTask {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


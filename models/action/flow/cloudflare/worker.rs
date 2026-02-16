// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceBindingFetchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVersionOutput {
    pub id: String,
    pub message: String,
    pub tag: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorker {
    pub account_id: String,
    pub compatibility_date: String,
    pub compatibility_flags: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: String,
    pub script_name: String,
    pub size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkerServiceBinding {
    pub binding_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub environment: String,
    pub target_service: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub worker_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkerCronTrigger {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cron: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub worker_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkerVersion {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message: String,
    pub tag: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version_id: String,
    pub worker_id: String,
}

#[async_trait]
pub trait WorkerAction {
    async fn fetch(&self, body: String, headers: String, method: String, service_binding: String, url: String) -> Result<FetchOutput, Box<dyn std::error::Error>>;
    async fn get_var(&self, name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_secret(&self, name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn scheduled(&self, cron: String, service_binding: String) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>>;
    async fn wait_until(&self, promise: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn pass_through(&self, request: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn service_binding_fetch(&self, body: String, headers: String, method: String, service: String, url: String) -> Result<ServiceBindingFetchOutput, Box<dyn std::error::Error>>;
    async fn get_version(&self, binding: String) -> Result<GetVersionOutput, Box<dyn std::error::Error>>;
}

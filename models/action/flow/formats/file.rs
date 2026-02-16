// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoOutput {
    pub created: chrono::DateTime<chrono::Utc>,
    pub is_directory: bool,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub permissions: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct File {
    pub path: String,
    pub name: String,
    pub size: i64,
    pub content_type: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub is_directory: bool,
    pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileDocument {
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_directory: bool,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub path: String,
    pub permissions: String,
    pub size: i64,
}

#[async_trait]
pub trait FileAction {
    async fn read(&self, mode: String, path: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn write(&self, content: String, mode: String, overwrite: bool, path: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, path: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn copy(&self, from: String, overwrite: bool, to: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn move(&self, from: String, overwrite: bool, to: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn exists(&self, path: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn info(&self, path: String) -> Result<InfoOutput, Box<dyn std::error::Error>>;
    async fn list_directory(&self, path: String, pattern: String, recursive: bool) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_directory(&self, path: String, recursive: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_directory(&self, path: String, recursive: bool) -> Result<(), Box<dyn std::error::Error>>;
}

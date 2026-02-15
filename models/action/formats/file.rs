// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReadInput {
    pub mode: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReadOutput {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WriteInput {
    pub content: String,
    pub mode: String,
    #[serde(default)]
    pub overwrite: bool,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WriteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyInput {
    pub from: String,
    #[serde(default)]
    pub overwrite: bool,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MoveInput {
    pub from: String,
    #[serde(default)]
    pub overwrite: bool,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MoveOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExistsInput {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExistsOutput {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoInput {
    pub path: String,
}

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
pub struct ListDirectoryInput {
    pub path: String,
    pub pattern: String,
    #[serde(default)]
    pub recursive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListDirectoryOutput {
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDirectoryInput {
    pub path: String,
    #[serde(default)]
    pub recursive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDirectoryOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDirectoryInput {
    pub path: String,
    #[serde(default)]
    pub recursive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteDirectoryOutput {
    pub success: bool,
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

#[async_trait]
pub trait FileAction {
    async fn read(&self, input: ReadInput) -> Result<ReadOutput, Box<dyn std::error::Error>>;
    async fn write(&self, input: WriteInput) -> Result<WriteOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn copy(&self, input: CopyInput) -> Result<CopyOutput, Box<dyn std::error::Error>>;
    async fn move(&self, input: MoveInput) -> Result<MoveOutput, Box<dyn std::error::Error>>;
    async fn exists(&self, input: ExistsInput) -> Result<ExistsOutput, Box<dyn std::error::Error>>;
    async fn info(&self, input: InfoInput) -> Result<InfoOutput, Box<dyn std::error::Error>>;
    async fn list_directory(&self, input: ListDirectoryInput) -> Result<ListDirectoryOutput, Box<dyn std::error::Error>>;
    async fn create_directory(&self, input: CreateDirectoryInput) -> Result<CreateDirectoryOutput, Box<dyn std::error::Error>>;
    async fn delete_directory(&self, input: DeleteDirectoryInput) -> Result<DeleteDirectoryOutput, Box<dyn std::error::Error>>;
}

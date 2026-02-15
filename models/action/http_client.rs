// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub headers: String,
    pub query_params: String,
    pub timeout: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostInput {
    pub body: String,
    pub content_type: String,
    pub headers: String,
    pub query_params: String,
    pub timeout: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutInput {
    pub body: String,
    pub content_type: String,
    pub headers: String,
    pub query_params: String,
    pub timeout: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PatchInput {
    pub body: String,
    pub content_type: String,
    pub headers: String,
    pub query_params: String,
    pub timeout: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PatchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub headers: String,
    pub query_params: String,
    pub timeout: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadInput {
    pub headers: String,
    pub timeout: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadOutput {
    pub content: String,
    pub content_type: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadInput {
    pub file: String,
    pub filename: String,
    pub headers: String,
    pub timeout: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadOutput {
    pub body: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlQueryInput {
    pub headers: String,
    pub query: String,
    pub timeout: i64,
    pub url: String,
    pub variables: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlQueryOutput {
    pub data: String,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: String,
    pub query_params: String,
    pub body: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Headers {
    pub content_type: String,
    pub authorization: String,
    pub accept: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryParams {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Variables {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphQlError {
    pub message: String,
    pub path: Vec<String>,
    pub locations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphQlErrorLocation {
    pub line: i64,
    pub column: i64,
}

#[async_trait]
pub trait HttpClientAction {
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn post(&self, input: PostInput) -> Result<PostOutput, Box<dyn std::error::Error>>;
    async fn put(&self, input: PutInput) -> Result<PutOutput, Box<dyn std::error::Error>>;
    async fn patch(&self, input: PatchInput) -> Result<PatchOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn download(&self, input: DownloadInput) -> Result<DownloadOutput, Box<dyn std::error::Error>>;
    async fn upload(&self, input: UploadInput) -> Result<UploadOutput, Box<dyn std::error::Error>>;
    async fn graphql_query(&self, input: GraphqlQueryInput) -> Result<GraphqlQueryOutput, Box<dyn std::error::Error>>;
}

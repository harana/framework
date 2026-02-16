// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployS3 {
    pub acl: String,
    pub bucket: String,
    pub cache_control: String,
    pub cloudfront: String,
    pub content_encoding: String,
    pub credentials_env: String,
    pub destination_prefix: String,
    #[serde(default)]
    pub enabled: bool,
    pub metadata: std::collections::HashMap<String, String>,
    pub region: String,
    pub source_dir: String,
    pub sync: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployS3Cloudfront {
    pub distribution_id: String,
    #[serde(default)]
    pub enabled: bool,
    pub invalidate_paths: Vec<String>,
    #[serde(default)]
    pub wait_for_invalidation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployS3Sync {
    #[serde(default)]
    pub delete: bool,
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAwsCredentials {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
}


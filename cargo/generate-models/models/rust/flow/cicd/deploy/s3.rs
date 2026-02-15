// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_s3
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployS3 {
    pub acl: Option<String>,
    pub bucket: String,
    pub cache_control: Option<String>,
    pub cloudfront: Option<String>,
    pub content_encoding: Option<String>,
    pub credentials_env: Option<String>,
    pub destination_prefix: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub metadata: String,
    pub region: String,
    pub source_dir: String,
    pub sync: Option<String>,
}

impl DeployS3 {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_s3_cloudfront
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

impl DeployS3Cloudfront {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_s3_sync
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployS3Sync {
    #[serde(default)]
    pub delete: bool,
    pub exclude: Vec<String>,
}

impl DeployS3Sync {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_aws_credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAwsCredentials {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
}

impl DeployAwsCredentials {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


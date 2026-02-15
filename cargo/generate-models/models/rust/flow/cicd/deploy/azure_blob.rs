// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_azure_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAzureBlob {
    pub cdn: Option<String>,
    pub container: String,
    pub content_type_mapping: String,
    pub credentials_env: Option<String>,
    pub destination_prefix: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub public_access: Option<String>,
    pub source_dir: String,
    pub storage_account: String,
}

impl DeployAzureBlob {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_azure_blob_cdn
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAzureBlobCdn {
    #[serde(default)]
    pub enabled: bool,
    pub endpoint: String,
    pub profile: String,
    #[serde(default)]
    pub purge_content: bool,
}

impl DeployAzureBlobCdn {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_azure_blob_credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAzureBlobCredentials {
    pub azure_storage_connection_string: String,
}

impl DeployAzureBlobCredentials {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


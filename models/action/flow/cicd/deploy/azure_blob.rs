// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAzureBlob {
    pub cdn: String,
    pub container: String,
    pub content_type_mapping: std::collections::HashMap<String, String>,
    pub credentials_env: String,
    pub destination_prefix: String,
    #[serde(default)]
    pub enabled: bool,
    pub public_access: String,
    pub source_dir: String,
    pub storage_account: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployAzureBlobCredentials {
    pub azure_storage_connection_string: String,
}


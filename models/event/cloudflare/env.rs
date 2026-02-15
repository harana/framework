// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareEnvVariableRead {
    pub name: String,
    pub found: bool,
    #[serde(default = "chrono::Utc::now")]
    pub read_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareEnvVariableRead {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareSecretRead {
    pub name: String,
    #[serde(default = "chrono::Utc::now")]
    pub read_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareSecretRead {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareSecretsStoreSecretRead {
    pub store_binding: String,
    pub name: String,
    #[serde(default = "chrono::Utc::now")]
    pub read_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareSecretsStoreSecretRead {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareEnvVariablesListed {
    pub variable_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub listed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareEnvVariablesListed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


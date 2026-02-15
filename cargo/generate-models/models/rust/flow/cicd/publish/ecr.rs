// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// publish_ecr
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishEcr {
    pub access_key_env: String,
    #[serde(default)]
    pub create_repository: bool,
    #[serde(default)]
    pub enabled: bool,
    pub image: String,
    #[serde(default)]
    pub image_scanning: bool,
    pub image_tag_mutability: String,
    pub lifecycle_policy: Option<String>,
    pub region: String,
    pub registry_id: Option<String>,
    pub repository: String,
    pub secret_key_env: String,
    pub tags: Vec<String>,
}

impl PublishEcr {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


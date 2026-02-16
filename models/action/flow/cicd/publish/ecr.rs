// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    pub lifecycle_policy: String,
    pub region: String,
    pub registry_id: String,
    pub repository: String,
    pub secret_key_env: String,
    pub tags: Vec<String>,
}


// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// publish_dockerhub
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishDockerhub {
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub image: String,
    pub password_env: String,
    pub readme: Option<String>,
    pub registry: String,
    pub tags: Vec<String>,
    pub username_env: String,
}

impl PublishDockerhub {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PublishDockerhub {
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    pub image: String,
    pub password_env: String,
    pub readme: String,
    pub registry: String,
    pub tags: Vec<String>,
    pub username_env: String,
}


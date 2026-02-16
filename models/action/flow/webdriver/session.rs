// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewSessionOutput {
    pub browser: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusOutput {
    pub message: String,
    pub ready: bool,
}

#[async_trait]
pub trait SessionAction {
    async fn new_session(&self, browser: String, capabilities: String, headless: bool, server_url: String) -> Result<NewSessionOutput, Box<dyn std::error::Error>>;
    async fn quit(&self, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn status(&self, session_id: String) -> Result<StatusOutput, Box<dyn std::error::Error>>;
}

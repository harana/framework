// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTimeoutsOutput {
    pub implicit_wait_ms: i64,
    pub page_load_ms: i64,
    pub script_ms: i64,
}

#[async_trait]
pub trait TimeoutAction {
    async fn get_timeouts(&self, session_id: String) -> Result<GetTimeoutsOutput, Box<dyn std::error::Error>>;
    async fn set_implicit_wait_timeout(&self, session_id: String, timeout_ms: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_page_load_timeout(&self, session_id: String, timeout_ms: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_script_timeout(&self, session_id: String, timeout_ms: i64) -> Result<(), Box<dyn std::error::Error>>;
}

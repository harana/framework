// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait JavascriptAction {
    async fn execute(&self, args: Vec<String>, script: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn execute_async(&self, args: Vec<String>, script: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
}

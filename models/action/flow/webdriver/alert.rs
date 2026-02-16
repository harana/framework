// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AlertAction {
    async fn get_alert_text(&self, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn accept_alert(&self, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn dismiss_alert(&self, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_alert_text(&self, keys: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
}

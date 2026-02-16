// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait NavigationAction {
    async fn goto(&self, session_id: String, url: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn current_url(&self, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn title(&self, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn source(&self, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn back(&self, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn forward(&self, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn refresh(&self, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
}

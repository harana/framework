// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ElementInteractionAction {
    async fn click(&self, element_id: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn send_keys(&self, element_id: String, keys: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn clear(&self, element_id: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn focus(&self, element_id: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn scroll_into_view(&self, element_id: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn drag_to(&self, session_id: String, source_element_id: String, target_element_id: String) -> Result<(), Box<dyn std::error::Error>>;
}

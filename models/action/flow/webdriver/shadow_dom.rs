// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ShadowDomAction {
    async fn get_shadow_root(&self, element_id: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
}

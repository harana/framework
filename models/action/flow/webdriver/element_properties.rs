// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RectOutput {
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}

#[async_trait]
pub trait ElementPropertiesAction {
    async fn text(&self, element_id: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn attr(&self, attribute: String, element_id: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn prop(&self, element_id: String, property: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn css_value(&self, element_id: String, property: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn tag_name(&self, element_id: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn rect(&self, element_id: String, session_id: String) -> Result<RectOutput, Box<dyn std::error::Error>>;
    async fn inner_html(&self, element_id: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn outer_html(&self, element_id: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn is_displayed(&self, element_id: String, session_id: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn is_enabled(&self, element_id: String, session_id: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn is_selected(&self, element_id: String, session_id: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn is_clickable(&self, element_id: String, session_id: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn is_present(&self, element_id: String, session_id: String) -> Result<bool, Box<dyn std::error::Error>>;
}

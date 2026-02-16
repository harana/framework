// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ScreenshotAction {
    async fn screenshot(&self, path: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn screenshot_as_png(&self, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn screenshot_as_png_base64(&self, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn element_screenshot(&self, element_id: String, path: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn element_screenshot_as_png(&self, element_id: String, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
}

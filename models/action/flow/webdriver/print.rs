// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait PrintAction {
    async fn print_page(&self, background: bool, margin_bottom: f64, margin_left: f64, margin_right: f64, margin_top: f64, orientation: String, page_height: f64, page_width: f64, scale: f64, session_id: String, shrink_to_fit: bool) -> Result<String, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ClassAction {
    async fn web_driver_capabilities(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn web_driver_proxy(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn web_driver_element(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn web_driver_cookie(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn web_driver_action(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn x_path_query_result(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn element_rect(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn window_rect(&self) -> Result<(), Box<dyn std::error::Error>>;
}

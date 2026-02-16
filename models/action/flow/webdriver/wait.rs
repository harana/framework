// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitForElementOutput {
    pub element: String,
    pub found: bool,
    pub wait_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitForElementsOutput {
    pub elements: Vec<String>,
    pub found: bool,
    pub total: i64,
    pub wait_ms: i64,
}

#[async_trait]
pub trait WaitAction {
    async fn wait_for_element(&self, by: String, condition: String, interval_ms: i64, timeout_ms: i64) -> Result<WaitForElementOutput, Box<dyn std::error::Error>>;
    async fn wait_for_elements(&self, by: String, interval_ms: i64, timeout_ms: i64) -> Result<WaitForElementsOutput, Box<dyn std::error::Error>>;
    async fn wait_until(&self, condition: String, element_id: String, interval_ms: i64, timeout_ms: i64, met: bool, wait_ms: i64) -> Result<(), Box<dyn std::error::Error>>;
}

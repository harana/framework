// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindOutput {
    pub element: String,
    pub found: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindAllOutput {
    pub elements: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindChildOutput {
    pub element: String,
    pub found: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindAllChildrenOutput {
    pub elements: Vec<String>,
    pub total: i64,
}

#[async_trait]
pub trait ElementFindingAction {
    async fn find(&self, by: String, session_id: String, value: String) -> Result<FindOutput, Box<dyn std::error::Error>>;
    async fn find_all(&self, by: String, session_id: String, value: String) -> Result<FindAllOutput, Box<dyn std::error::Error>>;
    async fn find_child(&self, by: String, parent_element_id: String, session_id: String, value: String) -> Result<FindChildOutput, Box<dyn std::error::Error>>;
    async fn find_all_children(&self, by: String, parent_element_id: String, session_id: String, value: String) -> Result<FindAllChildrenOutput, Box<dyn std::error::Error>>;
    async fn active_element(&self, session_id: String) -> Result<String, Box<dyn std::error::Error>>;
}

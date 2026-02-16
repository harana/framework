// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RandomValue {
    pub seed: i64,
    pub value: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RandomSeed {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub seed_value: i64,
}

#[async_trait]
pub trait RandomAction {
    async fn uuid(&self, version: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn bytes(&self, length: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn string(&self, charset: String, length: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn number(&self, integer: bool, max: f64, min: f64) -> Result<f64, Box<dyn std::error::Error>>;
    async fn choice(&self, items: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn shuffle(&self, items: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

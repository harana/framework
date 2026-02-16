// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NumberValue {
    pub value: f64,
    pub precision: i64,
    pub formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MathExpression {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub expression: String,
    pub result: f64,
    pub variables: String,
}

#[async_trait]
pub trait MathAction {
    async fn round(&self, precision: i64, value: f64) -> Result<f64, Box<dyn std::error::Error>>;
    async fn floor(&self, value: f64) -> Result<f64, Box<dyn std::error::Error>>;
    async fn ceil(&self, value: f64) -> Result<f64, Box<dyn std::error::Error>>;
    async fn abs(&self, value: f64) -> Result<f64, Box<dyn std::error::Error>>;
    async fn min(&self, values: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>>;
    async fn max(&self, values: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>>;
    async fn sum(&self, values: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>>;
    async fn average(&self, values: Vec<f64>) -> Result<f64, Box<dyn std::error::Error>>;
    async fn percentage(&self, precision: i64, total: f64, value: f64) -> Result<f64, Box<dyn std::error::Error>>;
}

// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RoundInput {
    pub precision: i64,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RoundOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FloorInput {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FloorOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CeilInput {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CeilOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AbsInput {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AbsOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MinInput {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MinOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaxInput {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MaxOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SumInput {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SumOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AverageInput {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AverageOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PercentageInput {
    pub precision: i64,
    pub total: f64,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PercentageOutput {
    pub result: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NumberValue {
    pub value: f64,
    pub precision: i64,
    pub formatted: String,
}

#[async_trait]
pub trait MathAction {
    async fn round(&self, input: RoundInput) -> Result<RoundOutput, Box<dyn std::error::Error>>;
    async fn floor(&self, input: FloorInput) -> Result<FloorOutput, Box<dyn std::error::Error>>;
    async fn ceil(&self, input: CeilInput) -> Result<CeilOutput, Box<dyn std::error::Error>>;
    async fn abs(&self, input: AbsInput) -> Result<AbsOutput, Box<dyn std::error::Error>>;
    async fn min(&self, input: MinInput) -> Result<MinOutput, Box<dyn std::error::Error>>;
    async fn max(&self, input: MaxInput) -> Result<MaxOutput, Box<dyn std::error::Error>>;
    async fn sum(&self, input: SumInput) -> Result<SumOutput, Box<dyn std::error::Error>>;
    async fn average(&self, input: AverageInput) -> Result<AverageOutput, Box<dyn std::error::Error>>;
    async fn percentage(&self, input: PercentageInput) -> Result<PercentageOutput, Box<dyn std::error::Error>>;
}

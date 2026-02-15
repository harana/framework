// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UuidInput {
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UuidOutput {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BytesInput {
    pub length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BytesOutput {
    pub bytes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StringInput {
    pub charset: String,
    pub length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StringOutput {
    pub string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NumberInput {
    #[serde(default)]
    pub integer: bool,
    pub max: f64,
    pub min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NumberOutput {
    pub number: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChoiceInput {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChoiceOutput {
    pub item: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShuffleInput {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShuffleOutput {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RandomValue {
    pub seed: i64,
    pub value: String,
    pub type: String,
}

#[async_trait]
pub trait RandomAction {
    async fn uuid(&self, input: UuidInput) -> Result<UuidOutput, Box<dyn std::error::Error>>;
    async fn bytes(&self, input: BytesInput) -> Result<BytesOutput, Box<dyn std::error::Error>>;
    async fn string(&self, input: StringInput) -> Result<StringOutput, Box<dyn std::error::Error>>;
    async fn number(&self, input: NumberInput) -> Result<NumberOutput, Box<dyn std::error::Error>>;
    async fn choice(&self, input: ChoiceInput) -> Result<ChoiceOutput, Box<dyn std::error::Error>>;
    async fn shuffle(&self, input: ShuffleInput) -> Result<ShuffleOutput, Box<dyn std::error::Error>>;
}

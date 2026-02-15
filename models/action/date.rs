// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NowInput {
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NowOutput {
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormatInput {
    pub date: String,
    pub format: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FormatOutput {
    pub formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub date: String,
    pub format: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddInput {
    pub amount: i64,
    pub date: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubtractInput {
    pub amount: i64,
    pub date: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubtractOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DiffInput {
    pub end: String,
    pub start: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DiffOutput {
    pub difference: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConvertTimezoneInput {
    pub date: String,
    pub from_timezone: String,
    pub to_timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConvertTimezoneOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartOfInput {
    pub date: String,
    pub period: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartOfOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EndOfInput {
    pub date: String,
    pub period: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EndOfOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Date {
    pub timestamp: String,
    pub timezone: String,
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
}

#[async_trait]
pub trait DateAction {
    async fn now(&self, input: NowInput) -> Result<NowOutput, Box<dyn std::error::Error>>;
    async fn format(&self, input: FormatInput) -> Result<FormatOutput, Box<dyn std::error::Error>>;
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn add(&self, input: AddInput) -> Result<AddOutput, Box<dyn std::error::Error>>;
    async fn subtract(&self, input: SubtractInput) -> Result<SubtractOutput, Box<dyn std::error::Error>>;
    async fn diff(&self, input: DiffInput) -> Result<DiffOutput, Box<dyn std::error::Error>>;
    async fn convert_timezone(&self, input: ConvertTimezoneInput) -> Result<ConvertTimezoneOutput, Box<dyn std::error::Error>>;
    async fn start_of(&self, input: StartOfInput) -> Result<StartOfOutput, Box<dyn std::error::Error>>;
    async fn end_of(&self, input: EndOfInput) -> Result<EndOfOutput, Box<dyn std::error::Error>>;
}

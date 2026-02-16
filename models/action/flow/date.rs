// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DateFormat {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub format_string: String,
    #[serde(default)]
    pub is_active: bool,
    pub locale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Timezone {
    pub abbreviation: String,
    pub dst_offset: i64,
    pub name: String,
    pub utc_offset: i64,
}

#[async_trait]
pub trait DateAction {
    async fn now(&self, timezone: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn format(&self, date: String, format: String, timezone: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn parse(&self, date: String, format: String, timezone: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn add(&self, amount: i64, date: String, unit: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn subtract(&self, amount: i64, date: String, unit: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn diff(&self, end: String, start: String, unit: String) -> Result<f64, Box<dyn std::error::Error>>;
    async fn convert_timezone(&self, date: String, from_timezone: String, to_timezone: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn start_of(&self, date: String, period: String, timezone: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn end_of(&self, date: String, period: String, timezone: String) -> Result<String, Box<dyn std::error::Error>>;
}

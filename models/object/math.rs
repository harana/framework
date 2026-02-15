// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MathExpression {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub expression: String,
    pub result: Option<f64>,
    pub variables: Option<String>,
}

impl MathExpression {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NumberValue {
    pub value: f64,
    pub precision: i64,
    pub formatted: String,
}

impl NumberValue {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


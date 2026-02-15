// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiModel {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub input_token_limit: Option<i64>,
    pub model_id: String,
    pub output_token_limit: Option<i64>,
    pub supported_generation_methods: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: Option<String>,
}

impl GoogleGeminiModel {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiSafetySetting {
    pub category: String,
    pub model_id: String,
    pub threshold: String,
}

impl GoogleGeminiSafetySetting {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiSafetyRating {
    #[serde(default)]
    pub blocked: bool,
    pub category: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub probability: String,
}

impl GoogleGeminiSafetyRating {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiChatMessage {
    pub content: String,
    pub conversation_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub role: String,
}

impl GoogleGeminiChatMessage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiConversation {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub status: String,
    pub title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

impl GoogleGeminiConversation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiEmbedding {
    pub content_hash: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dimensions: Option<i64>,
    pub embedding: Option<String>,
    pub model_id: String,
    pub task_type: String,
}

impl GoogleGeminiEmbedding {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiUsage {
    pub candidates_token_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub prompt_token_count: Option<i64>,
    pub total_token_count: Option<i64>,
}

impl GoogleGeminiUsage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiParameterRange {
    pub max_value: Option<f64>,
    pub min_value: Option<f64>,
    pub model_id: String,
    pub parameter_name: String,
}

impl GoogleGeminiParameterRange {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiModel {
    pub model_id: String,
    pub name: String,
    pub version: String,
    pub input_token_limit: i64,
    pub output_token_limit: i64,
    pub supported_generation_methods: Vec<String>,
}

impl GeminiModel {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiSafetySetting {
    pub category: String,
    pub threshold: String,
}

impl GeminiSafetySetting {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiSafetyRating {
    pub category: String,
    pub probability: String,
    pub blocked: bool,
}

impl GeminiSafetyRating {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiUsage {
    pub prompt_token_count: i64,
    pub candidates_token_count: i64,
    pub total_token_count: i64,
}

impl GeminiUsage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiChatMessage {
    pub role: String,
    pub content: String,
}

impl GeminiChatMessage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiEmbedding {
    pub values: Vec<f64>,
    pub dimensions: i64,
}

impl GeminiEmbedding {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiParameterRange {
    pub min: f64,
    pub max: f64,
    pub default: f64,
}

impl GeminiParameterRange {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractSchema {
    pub fields: Vec<String>,
}

impl ExtractSchema {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractSchemaField {
    pub name: String,
    pub type: String,
    pub description: String,
    pub required: bool,
}

impl ExtractSchemaField {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractedData {
    pub values: std::collections::HashMap<String, String>,
}

impl ExtractedData {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


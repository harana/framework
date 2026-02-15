// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// google_gemini_model
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_gemini_safety_setting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiSafetySetting {
    pub category: String,
    /// Reference: google_gemini_model.id
    pub model_id: String,
    pub threshold: String,
}

impl GoogleGeminiSafetySetting {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_gemini_safety_rating
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_gemini_chat_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiChatMessage {
    pub content: String,
    /// Reference: google_gemini_conversation.id
    pub conversation_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub role: String,
}

impl GoogleGeminiChatMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_gemini_conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiConversation {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: google_gemini_model.id
    pub model_id: String,
    pub status: String,
    pub title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

impl GoogleGeminiConversation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_gemini_embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiEmbedding {
    pub content_hash: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dimensions: Option<i64>,
    pub embedding: Option<String>,
    /// Reference: google_gemini_model.id
    pub model_id: String,
    pub task_type: String,
}

impl GoogleGeminiEmbedding {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_gemini_usage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiUsage {
    pub candidates_token_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: google_gemini_model.id
    pub model_id: String,
    pub prompt_token_count: Option<i64>,
    pub total_token_count: Option<i64>,
}

impl GoogleGeminiUsage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// google_gemini_parameter_range
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiParameterRange {
    pub max_value: Option<f64>,
    pub min_value: Option<f64>,
    /// Reference: google_gemini_model.id
    pub model_id: String,
    pub parameter_name: String,
}

impl GoogleGeminiParameterRange {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


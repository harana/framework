// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// ai_model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiModel {
    pub context_window: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub max_output_tokens: Option<i64>,
    pub provider: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

impl AiModel {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ai_model_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiModelConfig {
    pub frequency_penalty: f64,
    pub max_tokens: Option<i64>,
    /// Reference: ai_model.id
    pub model_id: String,
    pub presence_penalty: f64,
    pub stop_sequences: Option<String>,
    pub temperature: f64,
    pub top_p: f64,
}

impl AiModelConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ai_prompt_template
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiPromptTemplate {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    /// Reference: ai_model.id
    pub model_id: Option<String>,
    pub system_prompt: Option<String>,
    pub template: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub variables: Option<String>,
}

impl AiPromptTemplate {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ai_conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiConversation {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<String>,
    /// Reference: ai_model.id
    pub model_id: String,
    pub status: String,
    pub title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub user_id: String,
}

impl AiConversation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ai_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiMessage {
    pub content: String,
    /// Reference: ai_conversation.id
    pub conversation_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<String>,
    pub role: String,
    pub tokens_used: Option<i64>,
}

impl AiMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ai_embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiEmbedding {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dimensions: i64,
    pub metadata: Option<String>,
    /// Reference: ai_model.id
    pub model_id: String,
    pub source_id: Option<String>,
    pub source_type: Option<String>,
    pub vector: String,
}

impl AiEmbedding {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// ai_usage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiUsage {
    pub completion_tokens: i64,
    pub cost: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: ai_model.id
    pub model_id: String,
    pub prompt_tokens: i64,
    pub request_type: String,
    pub total_tokens: i64,
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl AiUsage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


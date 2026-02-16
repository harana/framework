// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateContentOutput {
    pub finish_reason: String,
    pub safety_ratings: Vec<String>,
    pub text: String,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatOutput {
    pub finish_reason: String,
    pub response: String,
    pub safety_ratings: Vec<String>,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateMultimodalOutput {
    pub finish_reason: String,
    pub safety_ratings: Vec<String>,
    pub text: String,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedContentOutput {
    pub dimensions: i64,
    pub embedding: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchEmbedOutput {
    pub count: i64,
    pub embeddings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListModelsOutput {
    pub models: Vec<String>,
    pub next_page_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetModelOutput {
    pub description: String,
    pub display_name: String,
    pub input_token_limit: i64,
    pub name: String,
    pub output_token_limit: i64,
    pub supported_generation_methods: Vec<String>,
    pub temperature: String,
    pub top_k: String,
    pub top_p: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnalyzeImageOutput {
    pub description: String,
    pub safety_ratings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateCodeOutput {
    pub code: String,
    pub explanation: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SummarizeOutput {
    pub original_length: i64,
    pub summary: String,
    pub summary_length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslateOutput {
    pub detected_source_language: String,
    pub translated_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractStructuredOutput {
    pub confidence: f64,
    pub data: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiSafetySetting {
    pub category: String,
    pub threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiSafetyRating {
    pub category: String,
    pub probability: String,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiUsage {
    pub prompt_token_count: i64,
    pub candidates_token_count: i64,
    pub total_token_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiEmbedding {
    pub values: Vec<f64>,
    pub dimensions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeminiParameterRange {
    pub min: f64,
    pub max: f64,
    pub default: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractSchema {
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractSchemaField {
    pub name: String,
    pub type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractedData {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiModel {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub display_name: String,
    pub input_token_limit: i64,
    pub model_id: String,
    pub output_token_limit: i64,
    pub supported_generation_methods: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiSafetySetting {
    pub category: String,
    pub model_id: String,
    pub threshold: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiChatMessage {
    pub content: String,
    pub conversation_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiConversation {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub status: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiEmbedding {
    pub content_hash: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dimensions: i64,
    pub embedding: String,
    pub model_id: String,
    pub task_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiUsage {
    pub candidates_token_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub prompt_token_count: i64,
    pub total_token_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleGeminiParameterRange {
    pub max_value: f64,
    pub min_value: f64,
    pub model_id: String,
    pub parameter_name: String,
}

#[async_trait]
pub trait GeminiAction {
    async fn generate_content(&self, max_output_tokens: i64, model: String, prompt: String, safety_settings: Vec<String>, stop_sequences: Vec<String>, temperature: f64, top_k: i64, top_p: f64) -> Result<GenerateContentOutput, Box<dyn std::error::Error>>;
    async fn chat(&self, max_output_tokens: i64, messages: Vec<String>, model: String, safety_settings: Vec<String>, temperature: f64, top_k: i64, top_p: f64) -> Result<ChatOutput, Box<dyn std::error::Error>>;
    async fn generate_multimodal(&self, audio: Vec<String>, images: Vec<String>, max_output_tokens: i64, model: String, prompt: String, temperature: f64, videos: Vec<String>) -> Result<GenerateMultimodalOutput, Box<dyn std::error::Error>>;
    async fn stream_generate(&self, max_output_tokens: i64, model: String, prompt: String, temperature: f64) -> Result<String, Box<dyn std::error::Error>>;
    async fn embed_content(&self, content: String, model: String, task_type: String) -> Result<EmbedContentOutput, Box<dyn std::error::Error>>;
    async fn batch_embed(&self, contents: Vec<String>, model: String, task_type: String) -> Result<BatchEmbedOutput, Box<dyn std::error::Error>>;
    async fn count_tokens(&self, content: String, model: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn list_models(&self, page_size: i64, page_token: String) -> Result<ListModelsOutput, Box<dyn std::error::Error>>;
    async fn get_model(&self, model: String) -> Result<GetModelOutput, Box<dyn std::error::Error>>;
    async fn analyze_image(&self, image: String, model: String, prompt: String, temperature: f64) -> Result<AnalyzeImageOutput, Box<dyn std::error::Error>>;
    async fn generate_code(&self, language: String, max_output_tokens: i64, model: String, prompt: String, temperature: f64) -> Result<GenerateCodeOutput, Box<dyn std::error::Error>>;
    async fn summarize(&self, max_length: i64, model: String, style: String, text: String) -> Result<SummarizeOutput, Box<dyn std::error::Error>>;
    async fn translate(&self, model: String, source_language: String, target_language: String, text: String) -> Result<TranslateOutput, Box<dyn std::error::Error>>;
    async fn extract_structured(&self, model: String, schema: String, text: String) -> Result<ExtractStructuredOutput, Box<dyn std::error::Error>>;
}

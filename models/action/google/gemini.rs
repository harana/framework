// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateContentInput {
    pub max_output_tokens: i64,
    pub model: String,
    pub prompt: String,
    pub safety_settings: Vec<String>,
    pub stop_sequences: Vec<String>,
    pub temperature: f64,
    pub top_k: i64,
    pub top_p: f64,
}

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
pub struct ChatInput {
    pub max_output_tokens: i64,
    pub messages: Vec<String>,
    pub model: String,
    pub safety_settings: Vec<String>,
    pub temperature: f64,
    pub top_k: i64,
    pub top_p: f64,
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
pub struct GenerateMultimodalInput {
    pub audio: Vec<String>,
    pub images: Vec<String>,
    pub max_output_tokens: i64,
    pub model: String,
    pub prompt: String,
    pub temperature: f64,
    pub videos: Vec<String>,
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
pub struct StreamGenerateInput {
    pub max_output_tokens: i64,
    pub model: String,
    pub prompt: String,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamGenerateOutput {
    pub stream_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedContentInput {
    pub content: String,
    pub model: String,
    pub task_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedContentOutput {
    pub dimensions: i64,
    pub embedding: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchEmbedInput {
    pub contents: Vec<String>,
    pub model: String,
    pub task_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchEmbedOutput {
    pub count: i64,
    pub embeddings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CountTokensInput {
    pub content: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CountTokensOutput {
    pub total_tokens: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListModelsInput {
    pub page_size: i64,
    pub page_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListModelsOutput {
    pub models: Vec<String>,
    pub next_page_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetModelInput {
    pub model: String,
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
pub struct AnalyzeImageInput {
    pub image: String,
    pub model: String,
    pub prompt: String,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnalyzeImageOutput {
    pub description: String,
    pub safety_ratings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateCodeInput {
    pub language: String,
    pub max_output_tokens: i64,
    pub model: String,
    pub prompt: String,
    pub temperature: f64,
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
pub struct SummarizeInput {
    pub max_length: i64,
    pub model: String,
    pub style: String,
    pub text: String,
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
pub struct TranslateInput {
    pub model: String,
    pub source_language: String,
    pub target_language: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslateOutput {
    pub detected_source_language: String,
    pub translated_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractStructuredInput {
    pub model: String,
    pub schema: String,
    pub text: String,
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

#[async_trait]
pub trait GeminiAction {
    async fn generate_content(&self, input: GenerateContentInput) -> Result<GenerateContentOutput, Box<dyn std::error::Error>>;
    async fn chat(&self, input: ChatInput) -> Result<ChatOutput, Box<dyn std::error::Error>>;
    async fn generate_multimodal(&self, input: GenerateMultimodalInput) -> Result<GenerateMultimodalOutput, Box<dyn std::error::Error>>;
    async fn stream_generate(&self, input: StreamGenerateInput) -> Result<StreamGenerateOutput, Box<dyn std::error::Error>>;
    async fn embed_content(&self, input: EmbedContentInput) -> Result<EmbedContentOutput, Box<dyn std::error::Error>>;
    async fn batch_embed(&self, input: BatchEmbedInput) -> Result<BatchEmbedOutput, Box<dyn std::error::Error>>;
    async fn count_tokens(&self, input: CountTokensInput) -> Result<CountTokensOutput, Box<dyn std::error::Error>>;
    async fn list_models(&self, input: ListModelsInput) -> Result<ListModelsOutput, Box<dyn std::error::Error>>;
    async fn get_model(&self, input: GetModelInput) -> Result<GetModelOutput, Box<dyn std::error::Error>>;
    async fn analyze_image(&self, input: AnalyzeImageInput) -> Result<AnalyzeImageOutput, Box<dyn std::error::Error>>;
    async fn generate_code(&self, input: GenerateCodeInput) -> Result<GenerateCodeOutput, Box<dyn std::error::Error>>;
    async fn summarize(&self, input: SummarizeInput) -> Result<SummarizeOutput, Box<dyn std::error::Error>>;
    async fn translate(&self, input: TranslateInput) -> Result<TranslateOutput, Box<dyn std::error::Error>>;
    async fn extract_structured(&self, input: ExtractStructuredInput) -> Result<ExtractStructuredOutput, Box<dyn std::error::Error>>;
}

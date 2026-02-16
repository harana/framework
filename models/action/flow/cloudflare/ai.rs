// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextEmbeddingsOutput {
    pub data: Vec<String>,
    pub shape: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SpeechRecognitionOutput {
    pub text: String,
    pub vtt: String,
    pub word_count: i64,
    pub words: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkersAiModel {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_name: String,
    pub task_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkersAiInference {
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: i64,
    pub input_tokens: i64,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub output_tokens: i64,
    pub status: String,
    #[serde(default)]
    pub stream: bool,
}

#[async_trait]
pub trait AiAction {
    async fn run(&self, binding: String, model: String, params: String, stream: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn text_generation(&self, binding: String, frequency_penalty: f64, max_tokens: i64, messages: Vec<String>, model: String, presence_penalty: f64, prompt: String, repetition_penalty: f64, stream: bool, temperature: f64, top_k: i64, top_p: f64) -> Result<String, Box<dyn std::error::Error>>;
    async fn text_embeddings(&self, binding: String, model: String, text: Vec<String>) -> Result<TextEmbeddingsOutput, Box<dyn std::error::Error>>;
    async fn text_classification(&self, binding: String, model: String, text: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn translation(&self, binding: String, model: String, source_lang: String, target_lang: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn image_classification(&self, binding: String, image: String, model: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn object_detection(&self, binding: String, image: String, model: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn text_to_image(&self, binding: String, guidance: f64, height: i64, model: String, num_steps: i64, prompt: String, width: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn speech_recognition(&self, audio: String, binding: String, model: String, source_lang: String) -> Result<SpeechRecognitionOutput, Box<dyn std::error::Error>>;
    async fn text_to_speech(&self, binding: String, model: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn summarization(&self, binding: String, input_text: String, max_length: i64, model: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn image_to_text(&self, binding: String, image: String, max_tokens: i64, model: String, prompt: String) -> Result<String, Box<dyn std::error::Error>>;
}

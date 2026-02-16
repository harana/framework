// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateTextOutput {
    pub text: String,
    pub tokens_used: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletionOutput {
    pub response: String,
    pub tokens_used: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateEmbeddingsOutput {
    pub dimensions: i64,
    pub embeddings: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateImageOutput {
    pub base64: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranscribeAudioOutput {
    pub duration: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClassifyTextOutput {
    pub confidence: f64,
    pub label: String,
    pub scores: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxLoadModelOutput {
    pub input_shapes: Vec<String>,
    pub model_id: String,
    pub output_shapes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxInferenceOutput {
    pub inference_time_ms: f64,
    pub outputs: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxInferenceBytesOutput {
    pub inference_time_ms: f64,
    pub outputs: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxModelInfoOutput {
    pub input_names: Vec<String>,
    pub input_shapes: Vec<String>,
    pub output_names: Vec<String>,
    pub output_shapes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxClassifyImageOutput {
    pub inference_time_ms: f64,
    pub predictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxDetectObjectsOutput {
    pub detections: Vec<String>,
    pub inference_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxTextEmbeddingOutput {
    pub dimensions: i64,
    pub embedding: Vec<f64>,
    pub inference_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiModel {
    pub model_id: String,
    pub name: String,
    pub provider: String,
    pub max_tokens: i64,
    pub temperature: f64,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiEntity {
    pub text: String,
    pub type: String,
    pub start: i64,
    pub end: i64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiTensorShape {
    pub name: String,
    pub dtype: String,
    pub shape: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiClassificationPrediction {
    pub label: String,
    pub confidence: f64,
    pub index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiObjectDetection {
    pub label: String,
    pub confidence: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiModelConfig {
    pub frequency_penalty: f64,
    pub max_tokens: i64,
    pub model_id: String,
    pub presence_penalty: f64,
    pub stop_sequences: String,
    pub temperature: f64,
    pub top_p: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiPromptTemplate {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub model_id: String,
    pub system_prompt: String,
    pub template: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub variables: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiConversation {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub model_id: String,
    pub status: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiMessage {
    pub content: String,
    pub conversation_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub role: String,
    pub tokens_used: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiEmbedding {
    pub content: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dimensions: i64,
    pub metadata: String,
    pub model_id: String,
    pub source_id: String,
    pub source_type: String,
    pub vector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AiUsage {
    pub completion_tokens: i64,
    pub cost: f64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub prompt_tokens: i64,
    pub request_type: String,
    pub total_tokens: i64,
    pub user_id: String,
}

#[async_trait]
pub trait AiAction {
    async fn generate_text(&self, max_tokens: i64, model: String, prompt: String, temperature: f64) -> Result<GenerateTextOutput, Box<dyn std::error::Error>>;
    async fn chat_completion(&self, max_tokens: i64, messages: Vec<String>, model: String, system_prompt: String, temperature: f64) -> Result<ChatCompletionOutput, Box<dyn std::error::Error>>;
    async fn generate_embeddings(&self, model: String, text: String) -> Result<GenerateEmbeddingsOutput, Box<dyn std::error::Error>>;
    async fn generate_image(&self, model: String, prompt: String, quality: String, size: String) -> Result<GenerateImageOutput, Box<dyn std::error::Error>>;
    async fn transcribe_audio(&self, audio: String, language: String, model: String) -> Result<TranscribeAudioOutput, Box<dyn std::error::Error>>;
    async fn classify_text(&self, labels: Vec<String>, model: String, text: String) -> Result<ClassifyTextOutput, Box<dyn std::error::Error>>;
    async fn summarize_text(&self, max_length: i64, model: String, text: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn extract_entities(&self, entity_types: Vec<String>, model: String, text: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn onnx_load_model(&self, model_path: String, optimize: bool) -> Result<OnnxLoadModelOutput, Box<dyn std::error::Error>>;
    async fn onnx_inference(&self, input_data: std::collections::HashMap<String, String>, model_id: String) -> Result<OnnxInferenceOutput, Box<dyn std::error::Error>>;
    async fn onnx_inference_bytes(&self, input_data: std::collections::HashMap<String, String>, model_bytes: String, optimize: bool) -> Result<OnnxInferenceBytesOutput, Box<dyn std::error::Error>>;
    async fn onnx_model_info(&self, model_path: String) -> Result<OnnxModelInfoOutput, Box<dyn std::error::Error>>;
    async fn onnx_unload_model(&self, model_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn onnx_classify_image(&self, image: String, labels: Vec<String>, model_id: String, top_k: i64) -> Result<OnnxClassifyImageOutput, Box<dyn std::error::Error>>;
    async fn onnx_detect_objects(&self, confidence_threshold: f64, image: String, labels: Vec<String>, model_id: String, nms_threshold: f64) -> Result<OnnxDetectObjectsOutput, Box<dyn std::error::Error>>;
    async fn onnx_text_embedding(&self, model_id: String, text: String) -> Result<OnnxTextEmbeddingOutput, Box<dyn std::error::Error>>;
}

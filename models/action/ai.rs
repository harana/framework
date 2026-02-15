// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateTextInput {
    pub max_tokens: i64,
    pub model: String,
    pub prompt: String,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateTextOutput {
    pub text: String,
    pub tokens_used: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletionInput {
    pub max_tokens: i64,
    pub messages: Vec<String>,
    pub model: String,
    pub system_prompt: String,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletionOutput {
    pub response: String,
    pub tokens_used: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateEmbeddingsInput {
    pub model: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateEmbeddingsOutput {
    pub dimensions: i64,
    pub embeddings: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateImageInput {
    pub model: String,
    pub prompt: String,
    pub quality: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateImageOutput {
    pub base64: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranscribeAudioInput {
    pub audio: String,
    pub language: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranscribeAudioOutput {
    pub duration: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClassifyTextInput {
    pub labels: Vec<String>,
    pub model: String,
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
pub struct SummarizeTextInput {
    pub max_length: i64,
    pub model: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SummarizeTextOutput {
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractEntitiesInput {
    pub entity_types: Vec<String>,
    pub model: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractEntitiesOutput {
    pub entities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxLoadModelInput {
    pub model_path: String,
    #[serde(default)]
    pub optimize: bool,
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
pub struct OnnxInferenceInput {
    pub input_data: std::collections::HashMap<String, String>,
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxInferenceOutput {
    pub inference_time_ms: f64,
    pub outputs: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxInferenceBytesInput {
    pub input_data: std::collections::HashMap<String, String>,
    pub model_bytes: String,
    #[serde(default)]
    pub optimize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxInferenceBytesOutput {
    pub inference_time_ms: f64,
    pub outputs: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxModelInfoInput {
    pub model_path: String,
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
pub struct OnnxUnloadModelInput {
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxUnloadModelOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxClassifyImageInput {
    pub image: String,
    pub labels: Vec<String>,
    pub model_id: String,
    pub top_k: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxClassifyImageOutput {
    pub inference_time_ms: f64,
    pub predictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxDetectObjectsInput {
    pub confidence_threshold: f64,
    pub image: String,
    pub labels: Vec<String>,
    pub model_id: String,
    pub nms_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxDetectObjectsOutput {
    pub detections: Vec<String>,
    pub inference_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnnxTextEmbeddingInput {
    pub model_id: String,
    pub text: String,
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

#[async_trait]
pub trait AiAction {
    async fn generate_text(&self, input: GenerateTextInput) -> Result<GenerateTextOutput, Box<dyn std::error::Error>>;
    async fn chat_completion(&self, input: ChatCompletionInput) -> Result<ChatCompletionOutput, Box<dyn std::error::Error>>;
    async fn generate_embeddings(&self, input: GenerateEmbeddingsInput) -> Result<GenerateEmbeddingsOutput, Box<dyn std::error::Error>>;
    async fn generate_image(&self, input: GenerateImageInput) -> Result<GenerateImageOutput, Box<dyn std::error::Error>>;
    async fn transcribe_audio(&self, input: TranscribeAudioInput) -> Result<TranscribeAudioOutput, Box<dyn std::error::Error>>;
    async fn classify_text(&self, input: ClassifyTextInput) -> Result<ClassifyTextOutput, Box<dyn std::error::Error>>;
    async fn summarize_text(&self, input: SummarizeTextInput) -> Result<SummarizeTextOutput, Box<dyn std::error::Error>>;
    async fn extract_entities(&self, input: ExtractEntitiesInput) -> Result<ExtractEntitiesOutput, Box<dyn std::error::Error>>;
    async fn onnx_load_model(&self, input: OnnxLoadModelInput) -> Result<OnnxLoadModelOutput, Box<dyn std::error::Error>>;
    async fn onnx_inference(&self, input: OnnxInferenceInput) -> Result<OnnxInferenceOutput, Box<dyn std::error::Error>>;
    async fn onnx_inference_bytes(&self, input: OnnxInferenceBytesInput) -> Result<OnnxInferenceBytesOutput, Box<dyn std::error::Error>>;
    async fn onnx_model_info(&self, input: OnnxModelInfoInput) -> Result<OnnxModelInfoOutput, Box<dyn std::error::Error>>;
    async fn onnx_unload_model(&self, input: OnnxUnloadModelInput) -> Result<OnnxUnloadModelOutput, Box<dyn std::error::Error>>;
    async fn onnx_classify_image(&self, input: OnnxClassifyImageInput) -> Result<OnnxClassifyImageOutput, Box<dyn std::error::Error>>;
    async fn onnx_detect_objects(&self, input: OnnxDetectObjectsInput) -> Result<OnnxDetectObjectsOutput, Box<dyn std::error::Error>>;
    async fn onnx_text_embedding(&self, input: OnnxTextEmbeddingInput) -> Result<OnnxTextEmbeddingOutput, Box<dyn std::error::Error>>;
}

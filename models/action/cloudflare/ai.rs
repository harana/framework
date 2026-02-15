// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunInput {
    pub binding: String,
    pub model: String,
    pub params: String,
    #[serde(default)]
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunOutput {
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextGenerationInput {
    pub binding: String,
    pub frequency_penalty: f64,
    pub max_tokens: i64,
    pub messages: Vec<String>,
    pub model: String,
    pub presence_penalty: f64,
    pub prompt: String,
    pub repetition_penalty: f64,
    #[serde(default)]
    pub stream: bool,
    pub temperature: f64,
    pub top_k: i64,
    pub top_p: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextGenerationOutput {
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextEmbeddingsInput {
    pub binding: String,
    pub model: String,
    pub text: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextEmbeddingsOutput {
    pub data: Vec<String>,
    pub shape: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextClassificationInput {
    pub binding: String,
    pub model: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextClassificationOutput {
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslationInput {
    pub binding: String,
    pub model: String,
    pub source_lang: String,
    pub target_lang: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TranslationOutput {
    pub translated_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageClassificationInput {
    pub binding: String,
    pub image: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageClassificationOutput {
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ObjectDetectionInput {
    pub binding: String,
    pub image: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ObjectDetectionOutput {
    pub detections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextToImageInput {
    pub binding: String,
    pub guidance: f64,
    pub height: i64,
    pub model: String,
    pub num_steps: i64,
    pub prompt: String,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextToImageOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SpeechRecognitionInput {
    pub audio: String,
    pub binding: String,
    pub model: String,
    pub source_lang: String,
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
pub struct TextToSpeechInput {
    pub binding: String,
    pub model: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextToSpeechOutput {
    pub audio: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SummarizationInput {
    pub binding: String,
    pub input_text: String,
    pub max_length: i64,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SummarizationOutput {
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageToTextInput {
    pub binding: String,
    pub image: String,
    pub max_tokens: i64,
    pub model: String,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageToTextOutput {
    pub description: String,
}

#[async_trait]
pub trait AiAction {
    async fn run(&self, input: RunInput) -> Result<RunOutput, Box<dyn std::error::Error>>;
    async fn text_generation(&self, input: TextGenerationInput) -> Result<TextGenerationOutput, Box<dyn std::error::Error>>;
    async fn text_embeddings(&self, input: TextEmbeddingsInput) -> Result<TextEmbeddingsOutput, Box<dyn std::error::Error>>;
    async fn text_classification(&self, input: TextClassificationInput) -> Result<TextClassificationOutput, Box<dyn std::error::Error>>;
    async fn translation(&self, input: TranslationInput) -> Result<TranslationOutput, Box<dyn std::error::Error>>;
    async fn image_classification(&self, input: ImageClassificationInput) -> Result<ImageClassificationOutput, Box<dyn std::error::Error>>;
    async fn object_detection(&self, input: ObjectDetectionInput) -> Result<ObjectDetectionOutput, Box<dyn std::error::Error>>;
    async fn text_to_image(&self, input: TextToImageInput) -> Result<TextToImageOutput, Box<dyn std::error::Error>>;
    async fn speech_recognition(&self, input: SpeechRecognitionInput) -> Result<SpeechRecognitionOutput, Box<dyn std::error::Error>>;
    async fn text_to_speech(&self, input: TextToSpeechInput) -> Result<TextToSpeechOutput, Box<dyn std::error::Error>>;
    async fn summarization(&self, input: SummarizationInput) -> Result<SummarizationOutput, Box<dyn std::error::Error>>;
    async fn image_to_text(&self, input: ImageToTextInput) -> Result<ImageToTextOutput, Box<dyn std::error::Error>>;
}

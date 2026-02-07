// Harana Actions - Cloudflare AI Module Output Types

use serde::{Deserialize, Serialize};

// run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOutput {
    pub response: serde_json::Value,
}

// text_generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextGenerationOutput {
    pub response: String,
}

// text_embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEmbeddingsOutput {
    pub data: Vec<Vec<f32>>,
    pub shape: Vec<i32>,
}

// text_classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextClassificationOutput {
    pub labels: Vec<WorkersAiClassification>,
}

// translation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationOutput {
    pub translated_text: String,
}

// image_classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageClassificationOutput {
    pub labels: Vec<WorkersAiClassification>,
}

// object_detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDetectionOutput {
    pub detections: Vec<WorkersAiDetection>,
}

// text_to_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToImageOutput {
    pub image: Vec<u8>,
}

// speech_recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechRecognitionOutput {
    pub text: String,
    pub vtt: String,
    pub word_count: i32,
    pub words: Vec<WorkersAiWord>,
}

// text_to_speech
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToSpeechOutput {
    pub audio: Vec<u8>,
}

// summarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizationOutput {
    pub summary: String,
}

// image_to_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageToTextOutput {
    pub description: String,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkersAiMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkersAiClassification {
    pub label: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkersAiDetection {
    pub label: String,
    pub score: f64,
    pub box_coords: WorkersAiBoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkersAiBoundingBox {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkersAiWord {
    pub word: String,
    pub start: f64,
    pub end: f64,
}

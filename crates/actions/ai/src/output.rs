// Harana Actions - AI Module Output Types
// Auto-generated output structs for AI action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// generate_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateTextOutput {
    pub text: String,
    pub tokens_used: i32,
}

// chat_completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionOutput {
    pub response: String,
    pub tokens_used: i32,
}

// generate_embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateEmbeddingsOutput {
    pub dimensions: i32,
    pub embeddings: Vec<f32>,
}

// generate_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateImageOutput {
    pub base64: String,
    pub url: String,
}

// transcribe_audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscribeAudioOutput {
    pub duration: f32,
    pub text: String,
}

// classify_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifyTextOutput {
    pub confidence: f32,
    pub label: String,
    pub scores: HashMap<String, serde_json::Value>,
}

// summarize_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeTextOutput {
    pub summary: String,
}

// extract_entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractEntitiesOutput {
    pub entities: Vec<HashMap<String, String>>,
}

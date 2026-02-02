// Harana Actions - Ai Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// chat_completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionOutput {
    pub response: String,
    pub tokens_used: i32,
}

// classify_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifyTextOutput {
    pub confidence: f64,
    pub label: String,
    pub scores: HashMap<String, Value>,
}

// extract_entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractEntitiesOutput {
    pub entities: Vec<HashMap<String, Value>>,
}

// generate_embeddings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateEmbeddingsOutput {
    pub dimensions: i32,
    pub embeddings: Vec<f64>,
}

// generate_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateImageOutput {
    pub url: String,
    pub base64: String,
}

// generate_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateTextOutput {
    pub text: String,
    pub tokens_used: i32,
}

// summarize_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeTextOutput {
    pub summary: String,
}

// transcribe_audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscribeAudioOutput {
    pub duration: f64,
    pub text: String,
}

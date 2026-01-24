// Harana Actions - Google Gemini Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// generate_content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateContentOutput {
    pub finish_reason: String,
    pub safety_ratings: Vec<GeminiSafetyRating>,
    pub text: String,
    pub usage: GeminiUsage,
}

// chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatOutput {
    pub finish_reason: String,
    pub response: String,
    pub safety_ratings: Vec<GeminiSafetyRating>,
    pub usage: GeminiUsage,
}

// generate_multimodal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMultimodalOutput {
    pub finish_reason: String,
    pub safety_ratings: Vec<GeminiSafetyRating>,
    pub text: String,
    pub usage: GeminiUsage,
}

// stream_generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamGenerateOutput {
    pub stream_id: String,
    pub success: bool,
}

// embed_content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedContentOutput {
    pub dimensions: i32,
    pub embedding: Vec<f32>,
}

// batch_embed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchEmbedOutput {
    pub count: i32,
    pub embeddings: Vec<GeminiEmbedding>,
}

// count_tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountTokensOutput {
    pub total_tokens: i32,
}

// list_models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelsOutput {
    pub models: Vec<GeminiModel>,
    pub next_page_token: Option<String>,
}

// get_model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelOutput {
    pub description: Option<String>,
    pub display_name: String,
    pub input_token_limit: i32,
    pub model: String,
    pub output_token_limit: i32,
    pub supported_generation_methods: Vec<String>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiSafetyRating {
    pub category: String,
    pub probability: String,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiUsage {
    pub prompt_token_count: i32,
    pub candidates_token_count: i32,
    pub total_token_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiChatMessage {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: Option<String>,
    pub inline_data: Option<GeminiInlineData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiInlineData {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiSafetySetting {
    pub category: String,
    pub threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiEmbedding {
    pub values: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiModel {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub input_token_limit: i32,
    pub output_token_limit: i32,
}

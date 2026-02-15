// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiModelRun {
    pub binding: String,
    pub model: String,
    #[serde(default)]
    pub stream: bool,
    #[serde(default = "chrono::Utc::now")]
    pub run_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiModelRun {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiTextGenerated {
    pub binding: String,
    pub model: String,
    pub max_tokens: Option<i64>,
    pub temperature: Option<f64>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default = "chrono::Utc::now")]
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiTextGenerated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiTextEmbedded {
    pub binding: String,
    pub model: String,
    pub text_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub embedded_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiTextEmbedded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiTextClassified {
    pub binding: String,
    pub model: String,
    pub label_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub classified_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiTextClassified {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiTranslationRun {
    pub binding: String,
    pub model: String,
    pub source_lang: String,
    pub target_lang: String,
    #[serde(default = "chrono::Utc::now")]
    pub translated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiTranslationRun {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiImageClassified {
    pub binding: String,
    pub model: String,
    pub label_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub classified_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiImageClassified {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiObjectDetected {
    pub binding: String,
    pub model: String,
    pub detection_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub detected_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiObjectDetected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiImageGenerated {
    pub binding: String,
    pub model: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiImageGenerated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiSpeechRecognized {
    pub binding: String,
    pub model: String,
    pub word_count: Option<i64>,
    pub source_lang: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub recognized_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiSpeechRecognized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiSpeechSynthesized {
    pub binding: String,
    pub model: String,
    #[serde(default = "chrono::Utc::now")]
    pub synthesized_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiSpeechSynthesized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiTextSummarized {
    pub binding: String,
    pub model: String,
    pub max_length: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub summarized_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiTextSummarized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiImageDescribed {
    pub binding: String,
    pub model: String,
    #[serde(default = "chrono::Utc::now")]
    pub described_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiImageDescribed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAiModelError {
    pub binding: String,
    pub model: String,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAiModelError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


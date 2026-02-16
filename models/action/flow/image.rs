// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResizeOutput {
    pub height: i64,
    pub image: String,
    pub size: i64,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CropOutput {
    pub height: i64,
    pub image: String,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompressOutput {
    pub compression_ratio: f64,
    pub image: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateOutput {
    pub height: i64,
    pub image: String,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThumbnailOutput {
    pub height: i64,
    pub image: String,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMetadataOutput {
    pub color_space: String,
    pub format: String,
    pub has_alpha: bool,
    pub height: i64,
    pub orientation: i64,
    pub size: i64,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConvertFormatOutput {
    pub format: String,
    pub image: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Image {
    pub source: String,
    pub width: i64,
    pub height: i64,
    pub format: String,
    pub size: i64,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageVariant {
    pub blob_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub format: String,
    pub height: i64,
    pub image_id: String,
    pub quality: i64,
    pub size: i64,
    pub variant_type: String,
    pub width: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageOperation {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error_message: String,
    pub image_id: String,
    pub operation: String,
    pub parameters: String,
    pub status: String,
}

#[async_trait]
pub trait ImageAction {
    async fn resize(&self, fit: String, format: String, height: i64, maintain_aspect_ratio: bool, source: String, width: i64) -> Result<ResizeOutput, Box<dyn std::error::Error>>;
    async fn crop(&self, height: i64, source: String, width: i64, x: i64, y: i64) -> Result<CropOutput, Box<dyn std::error::Error>>;
    async fn compress(&self, format: String, progressive: bool, quality: i64, source: String) -> Result<CompressOutput, Box<dyn std::error::Error>>;
    async fn rotate(&self, angle: i64, background: String, source: String) -> Result<RotateOutput, Box<dyn std::error::Error>>;
    async fn thumbnail(&self, fit: String, format: String, size: i64, source: String) -> Result<ThumbnailOutput, Box<dyn std::error::Error>>;
    async fn blur(&self, sigma: f64, source: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn greyscale(&self, source: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn flip(&self, direction: String, source: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn sharpen(&self, sigma: f64, source: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn brightness(&self, brightness: f64, source: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn contrast(&self, contrast: f64, source: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn tint(&self, color: String, source: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_metadata(&self, source: String) -> Result<GetMetadataOutput, Box<dyn std::error::Error>>;
    async fn convert_format(&self, format: String, quality: i64, source: String) -> Result<ConvertFormatOutput, Box<dyn std::error::Error>>;
    async fn watermark(&self, opacity: f64, position: String, source: String, watermark: String) -> Result<String, Box<dyn std::error::Error>>;
}

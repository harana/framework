// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResizeInput {
    pub fit: String,
    pub format: String,
    pub height: i64,
    #[serde(default)]
    pub maintain_aspect_ratio: bool,
    pub source: String,
    pub width: i64,
}

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
pub struct CropInput {
    pub height: i64,
    pub source: String,
    pub width: i64,
    pub x: i64,
    pub y: i64,
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
pub struct CompressInput {
    pub format: String,
    #[serde(default)]
    pub progressive: bool,
    pub quality: i64,
    pub source: String,
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
pub struct RotateInput {
    pub angle: i64,
    pub background: String,
    pub source: String,
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
pub struct ThumbnailInput {
    pub fit: String,
    pub format: String,
    pub size: i64,
    pub source: String,
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
pub struct BlurInput {
    pub sigma: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlurOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GreyscaleInput {
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GreyscaleOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlipInput {
    pub direction: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FlipOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SharpenInput {
    pub sigma: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SharpenOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrightnessInput {
    pub brightness: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrightnessOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContrastInput {
    pub contrast: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContrastOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TintInput {
    pub color: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TintOutput {
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMetadataInput {
    pub source: String,
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
pub struct ConvertFormatInput {
    pub format: String,
    pub quality: i64,
    pub source: String,
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
pub struct WatermarkInput {
    pub opacity: f64,
    pub position: String,
    pub source: String,
    pub watermark: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WatermarkOutput {
    pub image: String,
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

#[async_trait]
pub trait ImageAction {
    async fn resize(&self, input: ResizeInput) -> Result<ResizeOutput, Box<dyn std::error::Error>>;
    async fn crop(&self, input: CropInput) -> Result<CropOutput, Box<dyn std::error::Error>>;
    async fn compress(&self, input: CompressInput) -> Result<CompressOutput, Box<dyn std::error::Error>>;
    async fn rotate(&self, input: RotateInput) -> Result<RotateOutput, Box<dyn std::error::Error>>;
    async fn thumbnail(&self, input: ThumbnailInput) -> Result<ThumbnailOutput, Box<dyn std::error::Error>>;
    async fn blur(&self, input: BlurInput) -> Result<BlurOutput, Box<dyn std::error::Error>>;
    async fn greyscale(&self, input: GreyscaleInput) -> Result<GreyscaleOutput, Box<dyn std::error::Error>>;
    async fn flip(&self, input: FlipInput) -> Result<FlipOutput, Box<dyn std::error::Error>>;
    async fn sharpen(&self, input: SharpenInput) -> Result<SharpenOutput, Box<dyn std::error::Error>>;
    async fn brightness(&self, input: BrightnessInput) -> Result<BrightnessOutput, Box<dyn std::error::Error>>;
    async fn contrast(&self, input: ContrastInput) -> Result<ContrastOutput, Box<dyn std::error::Error>>;
    async fn tint(&self, input: TintInput) -> Result<TintOutput, Box<dyn std::error::Error>>;
    async fn get_metadata(&self, input: GetMetadataInput) -> Result<GetMetadataOutput, Box<dyn std::error::Error>>;
    async fn convert_format(&self, input: ConvertFormatInput) -> Result<ConvertFormatOutput, Box<dyn std::error::Error>>;
    async fn watermark(&self, input: WatermarkInput) -> Result<WatermarkOutput, Box<dyn std::error::Error>>;
}

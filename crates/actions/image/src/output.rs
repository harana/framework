// Harana Actions - Image Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// blur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlurOutput {
    pub image: String
}

// brightness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessOutput {
    pub image: String
}

// compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressOutput {
    pub compression_ratio: f64,
    pub size: i32,
    pub image: String
}

// contrast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastOutput {
    pub image: String
}

// convert_format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertFormatOutput {
    pub size: i32,
    pub image: String,
    pub format: String
}

// crop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropOutput {
    pub height: i32,
    pub image: String,
    pub width: i32
}

// flip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlipOutput {
    pub image: String
}

// get_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetadataOutput {
    pub height: i32,
    pub format: String,
    pub orientation: i32,
    pub width: i32,
    pub size: i32,
    pub has_alpha: bool,
    pub color_space: String
}

// greyscale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreyscaleOutput {
    pub image: String
}

// resize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeOutput {
    pub image: String,
    pub height: i32,
    pub size: i32,
    pub width: i32
}

// rotate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateOutput {
    pub image: String,
    pub width: i32,
    pub height: i32
}

// sharpen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharpenOutput {
    pub image: String
}

// thumbnail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailOutput {
    pub image: String,
    pub width: i32,
    pub height: i32
}

// tint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintOutput {
    pub image: String
}

// watermark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkOutput {
    pub image: String
}

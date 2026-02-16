// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConvertOutput {
    pub color: String,
    pub components: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub format: String,
    pub hex: String,
    pub hsl: String,
    pub rgb: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeneratePaletteOutput {
    pub colors: Vec<String>,
    pub palette_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContrastRatioOutput {
    pub ratio: f64,
    pub wcag_aa: bool,
    pub wcag_aaa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessibleTextColorOutput {
    pub contrast_ratio: f64,
    pub text_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LuminanceOutput {
    pub is_dark: bool,
    pub is_light: bool,
    pub luminance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub detected_format: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Color {
    pub hex: String,
    pub rgb: String,
    pub hsl: String,
    pub hsv: String,
    pub cmyk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RgbColor {
    pub r: i64,
    pub g: i64,
    pub b: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HslColor {
    pub h: i64,
    pub s: f64,
    pub l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HsvColor {
    pub h: i64,
    pub s: f64,
    pub v: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CmykColor {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorComponents {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
    pub alpha: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorPalette {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub palette_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorPaletteEntry {
    pub color_id: String,
    pub palette_id: String,
    pub sort_order: i64,
}

#[async_trait]
pub trait ColorAction {
    async fn convert(&self, color: String, from_format: String, to_format: String) -> Result<ConvertOutput, Box<dyn std::error::Error>>;
    async fn parse(&self, color: String) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn generate_palette(&self, base_color: String, count: i64, palette_type: String) -> Result<GeneratePaletteOutput, Box<dyn std::error::Error>>;
    async fn lighten(&self, amount: f64, color: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn darken(&self, amount: f64, color: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn saturate(&self, amount: f64, color: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn desaturate(&self, amount: f64, color: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn mix(&self, color1: String, color2: String, weight: f64) -> Result<String, Box<dyn std::error::Error>>;
    async fn contrast_ratio(&self, color1: String, color2: String) -> Result<ContrastRatioOutput, Box<dyn std::error::Error>>;
    async fn accessible_text_color(&self, background_color: String, prefer_white: bool) -> Result<AccessibleTextColorOutput, Box<dyn std::error::Error>>;
    async fn invert(&self, color: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn gradient(&self, end_color: String, start_color: String, steps: i64) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn luminance(&self, color: String) -> Result<LuminanceOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, color: String, format: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn random(&self, format: String, hue: String, luminosity: String) -> Result<String, Box<dyn std::error::Error>>;
}

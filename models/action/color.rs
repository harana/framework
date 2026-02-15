// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConvertInput {
    pub color: String,
    pub from_format: String,
    pub to_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConvertOutput {
    pub color: String,
    pub components: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseInput {
    pub color: String,
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
pub struct GeneratePaletteInput {
    pub base_color: String,
    pub count: i64,
    pub palette_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeneratePaletteOutput {
    pub colors: Vec<String>,
    pub palette_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LightenInput {
    pub amount: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LightenOutput {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DarkenInput {
    pub amount: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DarkenOutput {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SaturateInput {
    pub amount: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SaturateOutput {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DesaturateInput {
    pub amount: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DesaturateOutput {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MixInput {
    pub color1: String,
    pub color2: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MixOutput {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContrastRatioInput {
    pub color1: String,
    pub color2: String,
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
pub struct AccessibleTextColorInput {
    pub background_color: String,
    #[serde(default)]
    pub prefer_white: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessibleTextColorOutput {
    pub contrast_ratio: f64,
    pub text_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvertInput {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InvertOutput {
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GradientInput {
    pub end_color: String,
    pub start_color: String,
    pub steps: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GradientOutput {
    pub colors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LuminanceInput {
    pub color: String,
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
pub struct ValidateInput {
    pub color: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub detected_format: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RandomInput {
    pub format: String,
    pub hue: String,
    pub luminosity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RandomOutput {
    pub color: String,
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

#[async_trait]
pub trait ColorAction {
    async fn convert(&self, input: ConvertInput) -> Result<ConvertOutput, Box<dyn std::error::Error>>;
    async fn parse(&self, input: ParseInput) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn generate_palette(&self, input: GeneratePaletteInput) -> Result<GeneratePaletteOutput, Box<dyn std::error::Error>>;
    async fn lighten(&self, input: LightenInput) -> Result<LightenOutput, Box<dyn std::error::Error>>;
    async fn darken(&self, input: DarkenInput) -> Result<DarkenOutput, Box<dyn std::error::Error>>;
    async fn saturate(&self, input: SaturateInput) -> Result<SaturateOutput, Box<dyn std::error::Error>>;
    async fn desaturate(&self, input: DesaturateInput) -> Result<DesaturateOutput, Box<dyn std::error::Error>>;
    async fn mix(&self, input: MixInput) -> Result<MixOutput, Box<dyn std::error::Error>>;
    async fn contrast_ratio(&self, input: ContrastRatioInput) -> Result<ContrastRatioOutput, Box<dyn std::error::Error>>;
    async fn accessible_text_color(&self, input: AccessibleTextColorInput) -> Result<AccessibleTextColorOutput, Box<dyn std::error::Error>>;
    async fn invert(&self, input: InvertInput) -> Result<InvertOutput, Box<dyn std::error::Error>>;
    async fn gradient(&self, input: GradientInput) -> Result<GradientOutput, Box<dyn std::error::Error>>;
    async fn luminance(&self, input: LuminanceInput) -> Result<LuminanceOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
    async fn random(&self, input: RandomInput) -> Result<RandomOutput, Box<dyn std::error::Error>>;
}

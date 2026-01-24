// Harana Actions - Color Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// convert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOutput {
    pub color: String,
    pub components: ColorComponents,
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub format: String,
    pub hex: String,
    pub hsl: HslColor,
    pub rgb: RgbColor,
    pub valid: bool,
}

// generate_palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePaletteOutput {
    pub colors: Vec<String>,
    pub palette_type: String,
}

// lighten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightenOutput {
    pub color: String,
}

// darken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkenOutput {
    pub color: String,
}

// saturate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaturateOutput {
    pub color: String,
}

// desaturate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesaturateOutput {
    pub color: String,
}

// mix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixOutput {
    pub color: String,
}

// contrast_ratio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastRatioOutput {
    pub ratio: f64,
    pub wcag_aa: bool,
    pub wcag_aaa: bool,
}

// accessible_text_color
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleTextColorOutput {
    pub contrast_ratio: f64,
    pub text_color: String,
}

// invert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvertOutput {
    pub color: String,
}

// gradient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientOutput {
    pub colors: Vec<String>,
    pub css: String,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorComponents {
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
    pub h: Option<f64>,
    pub s: Option<f64>,
    pub l: Option<f64>,
    pub v: Option<f64>,
    pub c: Option<f64>,
    pub m: Option<f64>,
    pub y: Option<f64>,
    pub k: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HslColor {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: Option<f64>,
}

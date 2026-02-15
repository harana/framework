// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// color
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Color {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub format: String,
    pub hex: String,
    pub label: Option<String>,
}

impl Color {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// color_palette
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorPalette {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub palette_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ColorPalette {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// color_palette_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorPaletteEntry {
    /// Reference: color.id
    pub color_id: String,
    /// Reference: color_palette.id
    pub palette_id: String,
    pub sort_order: i64,
}

impl ColorPaletteEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// rgb_color
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RgbColor {
    pub r: i64,
    pub g: i64,
    pub b: i64,
}

impl RgbColor {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// hsl_color
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HslColor {
    pub h: i64,
    pub s: f64,
    pub l: f64,
}

impl HslColor {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// hsv_color
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HsvColor {
    pub h: i64,
    pub s: f64,
    pub v: f64,
}

impl HsvColor {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cmyk_color
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CmykColor {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

impl CmykColor {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// color_components
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorComponents {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
    pub alpha: f64,
}

impl ColorComponents {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


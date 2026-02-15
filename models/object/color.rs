// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorPalette {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub palette_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ColorPalette {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorPaletteEntry {
    pub color_id: String,
    pub palette_id: String,
    pub sort_order: i64,
}

impl ColorPaletteEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RgbColor {
    pub r: i64,
    pub g: i64,
    pub b: i64,
}

impl RgbColor {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HslColor {
    pub h: i64,
    pub s: f64,
    pub l: f64,
}

impl HslColor {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HsvColor {
    pub h: i64,
    pub s: f64,
    pub v: f64,
}

impl HsvColor {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CmykColor {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

impl CmykColor {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColorComponents {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
    pub alpha: f64,
}

impl ColorComponents {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


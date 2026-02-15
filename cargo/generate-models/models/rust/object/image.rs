// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Image {
    /// Reference: blob_object.id
    pub blob_id: Option<String>,
    pub color_space: Option<String>,
    pub content_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub format: String,
    #[serde(default)]
    pub has_alpha: bool,
    pub height: i64,
    pub orientation: i64,
    pub size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub width: i64,
}

impl Image {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// image_variant
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageVariant {
    /// Reference: blob_object.id
    pub blob_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub format: String,
    pub height: i64,
    /// Reference: image.id
    pub image_id: String,
    pub quality: i64,
    pub size: i64,
    pub variant_type: String,
    pub width: i64,
}

impl ImageVariant {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// image_operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImageOperation {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error_message: Option<String>,
    /// Reference: image.id
    pub image_id: String,
    pub operation: String,
    pub parameters: Option<String>,
    pub status: String,
}

impl ImageOperation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


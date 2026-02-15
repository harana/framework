// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// pdf_document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfDocument {
    pub author: Option<String>,
    /// Reference: blob_object.id
    pub blob_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub creator: Option<String>,
    #[serde(default)]
    pub is_encrypted: bool,
    pub keywords: Option<String>,
    pub modification_date: Option<chrono::DateTime<chrono::Utc>>,
    pub page_count: i64,
    pub producer: Option<String>,
    pub size: i64,
    pub subject: Option<String>,
    pub title: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl PdfDocument {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_template
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfTemplate {
    pub content_template: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub footer_template: Option<String>,
    pub header_template: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub orientation: String,
    pub page_size: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl PdfTemplate {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_generation_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfGenerationLog {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error_message: Option<String>,
    pub output_size: Option<i64>,
    pub page_count: Option<i64>,
    pub source: String,
    pub status: String,
    /// Reference: pdf_template.id
    pub template_id: Option<String>,
}

impl PdfGenerationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


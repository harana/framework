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

/// pdf_margin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfMargin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl PdfMargin {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfOptions {
    pub scale: f64,
    pub print_background: bool,
    pub prefer_css_page_size: bool,
}

impl PdfOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_template_data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfTemplateData {
    pub values: String,
}

impl PdfTemplateData {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_page_text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfPageText {
    pub page_number: i64,
    pub text: String,
}

impl PdfPageText {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_extracted_image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfExtractedImage {
    pub page_number: i64,
    pub image: String,
    pub format: String,
    pub width: i64,
    pub height: i64,
}

impl PdfExtractedImage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfPermissions {
    pub print: bool,
    pub copy: bool,
    pub modify: bool,
    pub annotate: bool,
}

impl PdfPermissions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_form_fields
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfFormFields {
    pub fields: Vec<String>,
}

impl PdfFormFields {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_form_field_value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfFormFieldValue {
    pub name: String,
    pub value: String,
}

impl PdfFormFieldValue {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// pdf_form_field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfFormField {
    pub name: String,
    pub type: String,
    pub value: String,
    pub required: bool,
    pub options: Vec<String>,
}

impl PdfFormField {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


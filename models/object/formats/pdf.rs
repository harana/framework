// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfDocument {
    pub author: Option<String>,
    pub blob_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub template_id: Option<String>,
}

impl PdfGenerationLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfMargin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl PdfMargin {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfOptions {
    pub scale: f64,
    pub print_background: bool,
    pub prefer_css_page_size: bool,
}

impl PdfOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfTemplateData {
    pub values: std::collections::HashMap<String, String>,
}

impl PdfTemplateData {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfPageText {
    pub page_number: i64,
    pub text: String,
}

impl PdfPageText {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfPermissions {
    pub print: bool,
    pub copy: bool,
    pub modify: bool,
    pub annotate: bool,
}

impl PdfPermissions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfFormFields {
    pub fields: Vec<String>,
}

impl PdfFormFields {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfFormFieldValue {
    pub name: String,
    pub value: String,
}

impl PdfFormFieldValue {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}


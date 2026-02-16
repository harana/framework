// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FromHtmlOutput {
    pub page_count: i64,
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FromTemplateOutput {
    pub page_count: i64,
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeOutput {
    pub page_count: i64,
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SplitOutput {
    pub documents: Vec<String>,
    pub page_counts: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractPagesOutput {
    pub page_count: i64,
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractTextOutput {
    pub pages: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractImagesOutput {
    pub count: i64,
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddWatermarkOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompressOutput {
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
    pub pdf: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecryptOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMetadataOutput {
    pub author: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub creator: String,
    pub encrypted: bool,
    pub keywords: Vec<String>,
    pub modification_date: chrono::DateTime<chrono::Utc>,
    pub page_count: i64,
    pub producer: String,
    pub size: i64,
    pub subject: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotatePagesOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddPageNumbersOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToImagesOutput {
    pub count: i64,
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FillFormOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFormFieldsOutput {
    pub count: i64,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfDocument {
    pub content: String,
    pub page_count: i64,
    pub size: i64,
    pub title: String,
    pub author: String,
    pub orientation: String,
    pub page_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfMargin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfOptions {
    pub scale: f64,
    pub print_background: bool,
    pub prefer_css_page_size: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfTemplateData {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfPageText {
    pub page_number: i64,
    pub text: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfPermissions {
    pub print: bool,
    pub copy: bool,
    pub modify: bool,
    pub annotate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfFormFields {
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfFormFieldValue {
    pub name: String,
    pub value: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfTemplate {
    pub content_template: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub footer_template: String,
    pub header_template: String,
    #[serde(default)]
    pub is_active: bool,
    pub orientation: String,
    pub page_size: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PdfGenerationLog {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error_message: String,
    pub output_size: i64,
    pub page_count: i64,
    pub source: String,
    pub status: String,
    pub template_id: String,
}

#[async_trait]
pub trait PdfAction {
    async fn from_html(&self, footer_template: String, header_template: String, html: String, margin: String, options: String, orientation: String, page_size: String) -> Result<FromHtmlOutput, Box<dyn std::error::Error>>;
    async fn from_template(&self, data: String, options: String, orientation: String, page_size: String, template_id: String) -> Result<FromTemplateOutput, Box<dyn std::error::Error>>;
    async fn merge(&self, documents: Vec<String>, output_name: String) -> Result<MergeOutput, Box<dyn std::error::Error>>;
    async fn split(&self, document: String, pages: Vec<i64>, ranges: Vec<String>) -> Result<SplitOutput, Box<dyn std::error::Error>>;
    async fn extract_pages(&self, document: String, end_page: i64, start_page: i64) -> Result<ExtractPagesOutput, Box<dyn std::error::Error>>;
    async fn extract_text(&self, document: String, pages: Vec<i64>, preserve_layout: bool) -> Result<ExtractTextOutput, Box<dyn std::error::Error>>;
    async fn extract_images(&self, document: String, format: String, pages: Vec<i64>) -> Result<ExtractImagesOutput, Box<dyn std::error::Error>>;
    async fn add_watermark(&self, document: String, opacity: f64, pages: Vec<i64>, position: String, watermark_image: String, watermark_text: String) -> Result<AddWatermarkOutput, Box<dyn std::error::Error>>;
    async fn compress(&self, document: String, quality: String, remove_metadata: bool) -> Result<CompressOutput, Box<dyn std::error::Error>>;
    async fn encrypt(&self, document: String, owner_password: String, permissions: String, user_password: String) -> Result<EncryptOutput, Box<dyn std::error::Error>>;
    async fn decrypt(&self, document: String, password: String) -> Result<DecryptOutput, Box<dyn std::error::Error>>;
    async fn get_metadata(&self, document: String) -> Result<GetMetadataOutput, Box<dyn std::error::Error>>;
    async fn set_metadata(&self, author: String, creator: String, document: String, keywords: Vec<String>, subject: String, title: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn rotate_pages(&self, angle: String, document: String, pages: Vec<i64>) -> Result<RotatePagesOutput, Box<dyn std::error::Error>>;
    async fn add_page_numbers(&self, alignment: String, document: String, font_size: i64, format: String, position: String, start_page: i64) -> Result<AddPageNumbersOutput, Box<dyn std::error::Error>>;
    async fn to_images(&self, document: String, dpi: i64, format: String, pages: Vec<i64>) -> Result<ToImagesOutput, Box<dyn std::error::Error>>;
    async fn fill_form(&self, document: String, fields: String, flatten: bool) -> Result<FillFormOutput, Box<dyn std::error::Error>>;
    async fn get_form_fields(&self, document: String) -> Result<GetFormFieldsOutput, Box<dyn std::error::Error>>;
}

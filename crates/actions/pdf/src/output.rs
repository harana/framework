// Harana Actions - PDF Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// from_html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromHtmlOutput {
    pub page_count: i32,
    pub pdf: Vec<u8>,
    pub size: i64,
}

// from_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromTemplateOutput {
    pub page_count: i32,
    pub pdf: Vec<u8>,
    pub size: i64,
}

// merge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeOutput {
    pub page_count: i32,
    pub pdf: Vec<u8>,
    pub size: i64,
}

// split
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitOutput {
    pub documents: Vec<Vec<u8>>,
    pub page_counts: Vec<i32>,
}

// extract_pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractPagesOutput {
    pub page_count: i32,
    pub pdf: Vec<u8>,
    pub size: i64,
}

// extract_text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractTextOutput {
    pub pages: Vec<PdfPageText>,
    pub text: String,
}

// extract_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractImagesOutput {
    pub count: i32,
    pub images: Vec<PdfExtractedImage>,
}

// add_watermark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddWatermarkOutput {
    pub pdf: Vec<u8>,
    pub size: i64,
}

// compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressOutput {
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
    pub pdf: Vec<u8>,
}

// encrypt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptOutput {
    pub pdf: Vec<u8>,
    pub success: bool,
}

// decrypt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptOutput {
    pub pdf: Vec<u8>,
    pub success: bool,
}

// get_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetadataOutput {
    pub author: Option<String>,
    pub creation_date: Option<String>,
    pub creator: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub modification_date: Option<String>,
    pub page_count: i32,
    pub producer: Option<String>,
    pub subject: Option<String>,
    pub title: Option<String>,
}

// set_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMetadataOutput {
    pub pdf: Vec<u8>,
    pub success: bool,
}

// rotate_pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotatePagesOutput {
    pub pdf: Vec<u8>,
    pub success: bool,
}

// add_page_numbers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPageNumbersOutput {
    pub pdf: Vec<u8>,
    pub size: i64,
}

// to_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToImagesOutput {
    pub count: i32,
    pub images: Vec<Vec<u8>>,
}

// fill_form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FillFormOutput {
    pub pdf: Vec<u8>,
    pub size: i64,
}

// get_form_fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFormFieldsOutput {
    pub count: i32,
    pub fields: Vec<PdfFormField>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPageText {
    pub page_number: i32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfExtractedImage {
    pub page_number: i32,
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMargin {
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfOptions {
    pub print_background: Option<bool>,
    pub scale: Option<f64>,
    pub prefer_css_page_size: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfTemplateData {
    pub data: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfFormField {
    pub name: String,
    pub field_type: String,
    pub value: String,
    pub required: bool,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfFormFieldValue {
    pub name: String,
    pub value: String,
}

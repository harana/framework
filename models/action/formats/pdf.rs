// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FromHtmlInput {
    pub footer_template: String,
    pub header_template: String,
    pub html: String,
    pub margin: String,
    pub options: String,
    pub orientation: String,
    pub page_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FromHtmlOutput {
    pub page_count: i64,
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FromTemplateInput {
    pub data: String,
    pub options: String,
    pub orientation: String,
    pub page_size: String,
    pub template_id: String,
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
pub struct MergeInput {
    pub documents: Vec<String>,
    pub output_name: String,
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
pub struct SplitInput {
    pub document: String,
    pub pages: Vec<i64>,
    pub ranges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SplitOutput {
    pub documents: Vec<String>,
    pub page_counts: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractPagesInput {
    pub document: String,
    pub end_page: i64,
    pub start_page: i64,
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
pub struct ExtractTextInput {
    pub document: String,
    pub pages: Vec<i64>,
    #[serde(default)]
    pub preserve_layout: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractTextOutput {
    pub pages: Vec<String>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractImagesInput {
    pub document: String,
    pub format: String,
    pub pages: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtractImagesOutput {
    pub count: i64,
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddWatermarkInput {
    pub document: String,
    pub opacity: f64,
    pub pages: Vec<i64>,
    pub position: String,
    pub watermark_image: String,
    pub watermark_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddWatermarkOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompressInput {
    pub document: String,
    pub quality: String,
    #[serde(default)]
    pub remove_metadata: bool,
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
pub struct EncryptInput {
    pub document: String,
    pub owner_password: String,
    pub permissions: String,
    pub user_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecryptInput {
    pub document: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecryptOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMetadataInput {
    pub document: String,
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
pub struct SetMetadataInput {
    pub author: String,
    pub creator: String,
    pub document: String,
    pub keywords: Vec<String>,
    pub subject: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetMetadataOutput {
    pub pdf: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotatePagesInput {
    pub angle: String,
    pub document: String,
    pub pages: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotatePagesOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddPageNumbersInput {
    pub alignment: String,
    pub document: String,
    pub font_size: i64,
    pub format: String,
    pub position: String,
    pub start_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddPageNumbersOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToImagesInput {
    pub document: String,
    pub dpi: i64,
    pub format: String,
    pub pages: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToImagesOutput {
    pub count: i64,
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FillFormInput {
    pub document: String,
    pub fields: String,
    #[serde(default)]
    pub flatten: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FillFormOutput {
    pub pdf: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFormFieldsInput {
    pub document: String,
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

#[async_trait]
pub trait PdfAction {
    async fn from_html(&self, input: FromHtmlInput) -> Result<FromHtmlOutput, Box<dyn std::error::Error>>;
    async fn from_template(&self, input: FromTemplateInput) -> Result<FromTemplateOutput, Box<dyn std::error::Error>>;
    async fn merge(&self, input: MergeInput) -> Result<MergeOutput, Box<dyn std::error::Error>>;
    async fn split(&self, input: SplitInput) -> Result<SplitOutput, Box<dyn std::error::Error>>;
    async fn extract_pages(&self, input: ExtractPagesInput) -> Result<ExtractPagesOutput, Box<dyn std::error::Error>>;
    async fn extract_text(&self, input: ExtractTextInput) -> Result<ExtractTextOutput, Box<dyn std::error::Error>>;
    async fn extract_images(&self, input: ExtractImagesInput) -> Result<ExtractImagesOutput, Box<dyn std::error::Error>>;
    async fn add_watermark(&self, input: AddWatermarkInput) -> Result<AddWatermarkOutput, Box<dyn std::error::Error>>;
    async fn compress(&self, input: CompressInput) -> Result<CompressOutput, Box<dyn std::error::Error>>;
    async fn encrypt(&self, input: EncryptInput) -> Result<EncryptOutput, Box<dyn std::error::Error>>;
    async fn decrypt(&self, input: DecryptInput) -> Result<DecryptOutput, Box<dyn std::error::Error>>;
    async fn get_metadata(&self, input: GetMetadataInput) -> Result<GetMetadataOutput, Box<dyn std::error::Error>>;
    async fn set_metadata(&self, input: SetMetadataInput) -> Result<SetMetadataOutput, Box<dyn std::error::Error>>;
    async fn rotate_pages(&self, input: RotatePagesInput) -> Result<RotatePagesOutput, Box<dyn std::error::Error>>;
    async fn add_page_numbers(&self, input: AddPageNumbersInput) -> Result<AddPageNumbersOutput, Box<dyn std::error::Error>>;
    async fn to_images(&self, input: ToImagesInput) -> Result<ToImagesOutput, Box<dyn std::error::Error>>;
    async fn fill_form(&self, input: FillFormInput) -> Result<FillFormOutput, Box<dyn std::error::Error>>;
    async fn get_form_fields(&self, input: GetFormFieldsInput) -> Result<GetFormFieldsOutput, Box<dyn std::error::Error>>;
}

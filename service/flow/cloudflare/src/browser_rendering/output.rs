// Harana Actions - Cloudflare Browser Rendering Module Output Types

use serde::{Deserialize, Serialize};

// render
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderOutput {
    pub content: String,
    pub status_code: i32,
}

// screenshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotOutput {
    pub image: Vec<u8>,
}

// extract_content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractContentOutput {
    pub content: String,
    pub title: String,
}

// pdf
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfOutput {
    pub pdf: Vec<u8>,
}

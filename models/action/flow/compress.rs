// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GzipCompressOutput {
    pub compressed: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GzipDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZstdCompressOutput {
    pub compressed: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZstdDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrotliCompressOutput {
    pub compressed: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrotliDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Lz4CompressOutput {
    pub compressed: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Lz4DecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeflateCompressOutput {
    pub compressed: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeflateDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AutoCompressOutput {
    pub algorithm_used: String,
    pub compressed: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AutoDecompressOutput {
    pub algorithm_detected: String,
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamCompressOutput {
    pub compressed_chunk: String,
    pub total_input: i64,
    pub total_output: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamDecompressOutput {
    pub decompressed_chunk: String,
    pub total_input: i64,
    pub total_output: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompressedData {
    pub algorithm: String,
    pub original_size: i64,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompressionJob {
    pub algorithm: String,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub compressed_size: i64,
    pub compression_level: i64,
    pub compression_ratio: f64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub direction: String,
    pub error_message: String,
    pub original_size: i64,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait CompressAction {
    async fn gzip_compress(&self, data: String, level: i64) -> Result<GzipCompressOutput, Box<dyn std::error::Error>>;
    async fn gzip_decompress(&self, data: String) -> Result<GzipDecompressOutput, Box<dyn std::error::Error>>;
    async fn zstd_compress(&self, data: String, level: i64) -> Result<ZstdCompressOutput, Box<dyn std::error::Error>>;
    async fn zstd_decompress(&self, data: String) -> Result<ZstdDecompressOutput, Box<dyn std::error::Error>>;
    async fn brotli_compress(&self, data: String, quality: i64) -> Result<BrotliCompressOutput, Box<dyn std::error::Error>>;
    async fn brotli_decompress(&self, data: String) -> Result<BrotliDecompressOutput, Box<dyn std::error::Error>>;
    async fn lz4_compress(&self, data: String, high_compression: bool) -> Result<Lz4CompressOutput, Box<dyn std::error::Error>>;
    async fn lz4_decompress(&self, data: String) -> Result<Lz4DecompressOutput, Box<dyn std::error::Error>>;
    async fn deflate_compress(&self, data: String, level: i64) -> Result<DeflateCompressOutput, Box<dyn std::error::Error>>;
    async fn deflate_decompress(&self, data: String) -> Result<DeflateDecompressOutput, Box<dyn std::error::Error>>;
    async fn auto_compress(&self, algorithm: String, data: String, optimize_for: String) -> Result<AutoCompressOutput, Box<dyn std::error::Error>>;
    async fn auto_decompress(&self, data: String) -> Result<AutoDecompressOutput, Box<dyn std::error::Error>>;
    async fn stream_compress(&self, algorithm: String, chunk: String, final: bool, stream_id: String) -> Result<StreamCompressOutput, Box<dyn std::error::Error>>;
    async fn stream_decompress(&self, algorithm: String, chunk: String, final: bool, stream_id: String) -> Result<StreamDecompressOutput, Box<dyn std::error::Error>>;
}

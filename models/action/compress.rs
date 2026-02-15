// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GzipCompressInput {
    pub data: String,
    pub level: i64,
}

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
pub struct GzipDecompressInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GzipDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZstdCompressInput {
    pub data: String,
    pub level: i64,
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
pub struct ZstdDecompressInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZstdDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrotliCompressInput {
    pub data: String,
    pub quality: i64,
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
pub struct BrotliDecompressInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BrotliDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Lz4CompressInput {
    pub data: String,
    #[serde(default)]
    pub high_compression: bool,
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
pub struct Lz4DecompressInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Lz4DecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeflateCompressInput {
    pub data: String,
    pub level: i64,
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
pub struct DeflateDecompressInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeflateDecompressOutput {
    pub decompressed: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AutoCompressInput {
    pub algorithm: String,
    pub data: String,
    pub optimize_for: String,
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
pub struct AutoDecompressInput {
    pub data: String,
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
pub struct StreamCompressInput {
    pub algorithm: String,
    pub chunk: String,
    #[serde(default)]
    pub final: bool,
    pub stream_id: String,
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
pub struct StreamDecompressInput {
    pub algorithm: String,
    pub chunk: String,
    #[serde(default)]
    pub final: bool,
    pub stream_id: String,
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

#[async_trait]
pub trait CompressAction {
    async fn gzip_compress(&self, input: GzipCompressInput) -> Result<GzipCompressOutput, Box<dyn std::error::Error>>;
    async fn gzip_decompress(&self, input: GzipDecompressInput) -> Result<GzipDecompressOutput, Box<dyn std::error::Error>>;
    async fn zstd_compress(&self, input: ZstdCompressInput) -> Result<ZstdCompressOutput, Box<dyn std::error::Error>>;
    async fn zstd_decompress(&self, input: ZstdDecompressInput) -> Result<ZstdDecompressOutput, Box<dyn std::error::Error>>;
    async fn brotli_compress(&self, input: BrotliCompressInput) -> Result<BrotliCompressOutput, Box<dyn std::error::Error>>;
    async fn brotli_decompress(&self, input: BrotliDecompressInput) -> Result<BrotliDecompressOutput, Box<dyn std::error::Error>>;
    async fn lz4_compress(&self, input: Lz4CompressInput) -> Result<Lz4CompressOutput, Box<dyn std::error::Error>>;
    async fn lz4_decompress(&self, input: Lz4DecompressInput) -> Result<Lz4DecompressOutput, Box<dyn std::error::Error>>;
    async fn deflate_compress(&self, input: DeflateCompressInput) -> Result<DeflateCompressOutput, Box<dyn std::error::Error>>;
    async fn deflate_decompress(&self, input: DeflateDecompressInput) -> Result<DeflateDecompressOutput, Box<dyn std::error::Error>>;
    async fn auto_compress(&self, input: AutoCompressInput) -> Result<AutoCompressOutput, Box<dyn std::error::Error>>;
    async fn auto_decompress(&self, input: AutoDecompressInput) -> Result<AutoDecompressOutput, Box<dyn std::error::Error>>;
    async fn stream_compress(&self, input: StreamCompressInput) -> Result<StreamCompressOutput, Box<dyn std::error::Error>>;
    async fn stream_decompress(&self, input: StreamDecompressInput) -> Result<StreamDecompressOutput, Box<dyn std::error::Error>>;
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OptimizeFor {
    Speed,
    Size,
    #[default]
    Balanced,
}

// gzip_compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GzipCompressOutput {
    pub compressed: Vec<u8>,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

// gzip_decompress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GzipDecompressOutput {
    pub decompressed: Vec<u8>,
    pub size: i64,
}

// zstd_compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZstdCompressOutput {
    pub compressed: Vec<u8>,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

// zstd_decompress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZstdDecompressOutput {
    pub decompressed: Vec<u8>,
    pub size: i64,
}

// brotli_compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrotliCompressOutput {
    pub compressed: Vec<u8>,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

// brotli_decompress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrotliDecompressOutput {
    pub decompressed: Vec<u8>,
    pub size: i64,
}

// lz4_compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lz4CompressOutput {
    pub compressed: Vec<u8>,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

// lz4_decompress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lz4DecompressOutput {
    pub decompressed: Vec<u8>,
    pub size: i64,
}

// deflate_compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeflateCompressOutput {
    pub compressed: Vec<u8>,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

// deflate_decompress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeflateDecompressOutput {
    pub decompressed: Vec<u8>,
    pub size: i64,
}

// auto_compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCompressOutput {
    pub algorithm_used: String,
    pub compressed: Vec<u8>,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

// auto_decompress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDecompressOutput {
    pub algorithm_detected: String,
    pub decompressed: Vec<u8>,
    pub size: i64,
}

// stream_compress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCompressOutput {
    pub compressed_chunk: Vec<u8>,
    pub total_input: i64,
    pub total_output: i64,
}

// stream_decompress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDecompressOutput {
    pub decompressed_chunk: Vec<u8>,
    pub total_input: i64,
    pub total_output: i64,
}

pub mod output;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Mutex;
use flate2::Compression;
use flate2::read::{GzDecoder, DeflateDecoder};
use flate2::write::{GzEncoder, DeflateEncoder};
use output::*;

// Thread-safe storage for streaming state
lazy_static::lazy_static! {
    static ref COMPRESS_STREAMS: Mutex<HashMap<String, StreamState>> = Mutex::new(HashMap::new());
    static ref DECOMPRESS_STREAMS: Mutex<HashMap<String, StreamState>> = Mutex::new(HashMap::new());
}

struct StreamState {
    total_input: i64,
    total_output: i64,
}

/// Compress Data With Gzip
pub async fn gzip_compress(
    data: &[u8],
    level: Option<i32>,
) -> Result<GzipCompressOutput, String> {
    let level = level.unwrap_or(6).clamp(0, 9) as u32;
    let original_size = data.len() as i64;
    
    let mut encoder = GzEncoder::new(Vec::new(), Compression::new(level));
    encoder.write_all(data).map_err(|e| format!("Gzip compression failed: {}", e))?;
    let compressed = encoder.finish().map_err(|e| format!("Gzip finish failed: {}", e))?;
    
    let compressed_size = compressed.len() as i64;
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(GzipCompressOutput {
        compressed,
        compressed_size,
        compression_ratio,
        original_size,
    })
}

/// Decompress Gzip Data
pub async fn gzip_decompress(
    data: &[u8],
) -> Result<GzipDecompressOutput, String> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| format!("Gzip decompression failed: {}", e))?;
    
    let size = decompressed.len() as i64;
    
    Ok(GzipDecompressOutput { decompressed, size })
}

/// Compress Data With Zstd
pub async fn zstd_compress(
    data: &[u8],
    level: Option<i32>,
) -> Result<ZstdCompressOutput, String> {
    let level = level.unwrap_or(3).clamp(1, 22);
    let original_size = data.len() as i64;
    
    let compressed = zstd::encode_all(data, level)
        .map_err(|e| format!("Zstd compression failed: {}", e))?;
    
    let compressed_size = compressed.len() as i64;
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(ZstdCompressOutput {
        compressed,
        compressed_size,
        compression_ratio,
        original_size,
    })
}

/// Decompress Zstd Data
pub async fn zstd_decompress(
    data: &[u8],
) -> Result<ZstdDecompressOutput, String> {
    let decompressed = zstd::decode_all(data)
        .map_err(|e| format!("Zstd decompression failed: {}", e))?;
    
    let size = decompressed.len() as i64;
    
    Ok(ZstdDecompressOutput { decompressed, size })
}

/// Compress Data With Brotli
pub async fn brotli_compress(
    data: &[u8],
    quality: Option<i32>,
) -> Result<BrotliCompressOutput, String> {
    let quality = quality.unwrap_or(6).clamp(0, 11) as u32;
    let original_size = data.len() as i64;
    
    let mut compressed = Vec::new();
    let mut params = brotli::enc::BrotliEncoderParams::default();
    params.quality = quality as i32;
    
    brotli::BrotliCompress(&mut &data[..], &mut compressed, &params)
        .map_err(|e| format!("Brotli compression failed: {}", e))?;
    
    let compressed_size = compressed.len() as i64;
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(BrotliCompressOutput {
        compressed,
        compressed_size,
        compression_ratio,
        original_size,
    })
}

/// Decompress Brotli Data
pub async fn brotli_decompress(
    data: &[u8],
) -> Result<BrotliDecompressOutput, String> {
    let mut decompressed = Vec::new();
    brotli::BrotliDecompress(&mut &data[..], &mut decompressed)
        .map_err(|e| format!("Brotli decompression failed: {}", e))?;
    
    let size = decompressed.len() as i64;
    
    Ok(BrotliDecompressOutput { decompressed, size })
}

/// Compress Data With LZ4
pub async fn lz4_compress(
    data: &[u8],
    _high_compression: Option<bool>,
) -> Result<Lz4CompressOutput, String> {
    let original_size = data.len() as i64;
    
    // Use compress_prepend_size which prepends the uncompressed size
    // This is needed for decompress_size_prepended to work correctly
    let compressed = lz4_flex::compress_prepend_size(data);
    
    let compressed_size = compressed.len() as i64;
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(Lz4CompressOutput {
        compressed,
        compressed_size,
        compression_ratio,
        original_size,
    })
}

/// Decompress LZ4 Data
pub async fn lz4_decompress(
    data: &[u8],
) -> Result<Lz4DecompressOutput, String> {
    let decompressed = lz4_flex::decompress_size_prepended(data)
        .map_err(|e| format!("LZ4 decompression failed: {}", e))?;
    
    let size = decompressed.len() as i64;
    
    Ok(Lz4DecompressOutput { decompressed, size })
}

/// Compress Data With Deflate
pub async fn deflate_compress(
    data: &[u8],
    level: Option<i32>,
) -> Result<DeflateCompressOutput, String> {
    let level = level.unwrap_or(6).clamp(0, 9) as u32;
    let original_size = data.len() as i64;
    
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::new(level));
    encoder.write_all(data).map_err(|e| format!("Deflate compression failed: {}", e))?;
    let compressed = encoder.finish().map_err(|e| format!("Deflate finish failed: {}", e))?;
    
    let compressed_size = compressed.len() as i64;
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(DeflateCompressOutput {
        compressed,
        compressed_size,
        compression_ratio,
        original_size,
    })
}

/// Decompress Deflate Data
pub async fn deflate_decompress(
    data: &[u8],
) -> Result<DeflateDecompressOutput, String> {
    let mut decoder = DeflateDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| format!("Deflate decompression failed: {}", e))?;
    
    let size = decompressed.len() as i64;
    
    Ok(DeflateDecompressOutput { decompressed, size })
}

/// Auto Compress Data - selects best algorithm or uses specified one
pub async fn auto_compress(
    data: &[u8],
    algorithm: Option<&str>,
    optimize_for: Option<OptimizeFor>,
) -> Result<AutoCompressOutput, String> {
    let optimize = optimize_for.unwrap_or(OptimizeFor::Balanced);
    let original_size = data.len() as i64;
    
    // If algorithm is specified, use it
    let algo = algorithm.unwrap_or_else(|| {
        // Select algorithm based on optimization strategy
        match optimize {
            OptimizeFor::Speed => "lz4",
            OptimizeFor::Size => "zstd",
            OptimizeFor::Balanced => "gzip",
        }
    });
    
    // Adjust compression level based on optimization
    let (compressed, algorithm_used) = match algo.to_lowercase().as_str() {
        "gzip" | "gz" => {
            let level = match optimize {
                OptimizeFor::Speed => Some(1),
                OptimizeFor::Size => Some(9),
                OptimizeFor::Balanced => None,
            };
            let result = gzip_compress(data, level).await?;
            (result.compressed, "gzip".to_string())
        }
        "zstd" | "zstandard" => {
            let level = match optimize {
                OptimizeFor::Speed => Some(1),
                OptimizeFor::Size => Some(19),
                OptimizeFor::Balanced => None,
            };
            let result = zstd_compress(data, level).await?;
            (result.compressed, "zstd".to_string())
        }
        "brotli" | "br" => {
            let quality = match optimize {
                OptimizeFor::Speed => Some(1),
                OptimizeFor::Size => Some(11),
                OptimizeFor::Balanced => None,
            };
            let result = brotli_compress(data, quality).await?;
            (result.compressed, "brotli".to_string())
        }
        "lz4" => {
            let result = lz4_compress(data, None).await?;
            (result.compressed, "lz4".to_string())
        }
        "deflate" => {
            let level = match optimize {
                OptimizeFor::Speed => Some(1),
                OptimizeFor::Size => Some(9),
                OptimizeFor::Balanced => None,
            };
            let result = deflate_compress(data, level).await?;
            (result.compressed, "deflate".to_string())
        }
        _ => return Err(format!("Unknown compression algorithm: {}", algo)),
    };
    
    let compressed_size = compressed.len() as i64;
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(AutoCompressOutput {
        algorithm_used,
        compressed,
        compressed_size,
        compression_ratio,
        original_size,
    })
}

/// Auto Decompress Data - detects algorithm from magic bytes
pub async fn auto_decompress(
    data: &[u8],
) -> Result<AutoDecompressOutput, String> {
    if data.len() < 2 {
        return Err("Data too short to determine compression format".to_string());
    }
    
    // Detect compression format from magic bytes
    let (decompressed, algorithm_detected) = if data.starts_with(&[0x1f, 0x8b]) {
        // Gzip magic bytes
        let result = gzip_decompress(data).await?;
        (result.decompressed, "gzip".to_string())
    } else if data.starts_with(&[0x28, 0xb5, 0x2f, 0xfd]) {
        // Zstd magic bytes
        let result = zstd_decompress(data).await?;
        (result.decompressed, "zstd".to_string())
    } else if data.len() >= 4 && data[0] == 0x04 && data[1] == 0x22 && data[2] == 0x4d && data[3] == 0x18 {
        // LZ4 frame magic bytes
        let result = lz4_decompress(data).await?;
        (result.decompressed, "lz4".to_string())
    } else {
        // Try brotli first, then deflate as fallback
        if let Ok(result) = brotli_decompress(data).await {
            (result.decompressed, "brotli".to_string())
        } else if let Ok(result) = deflate_decompress(data).await {
            (result.decompressed, "deflate".to_string())
        } else {
            return Err("Unable to detect compression format".to_string());
        }
    };
    
    let size = decompressed.len() as i64;
    
    Ok(AutoDecompressOutput {
        algorithm_detected,
        decompressed,
        size,
    })
}

/// Stream Compress - compress data in chunks
pub async fn stream_compress(
    stream_id: &str,
    chunk: &[u8],
    algorithm: Option<&str>,
    is_final: Option<bool>,
) -> Result<StreamCompressOutput, String> {
    let algo = algorithm.unwrap_or("gzip");
    let is_final = is_final.unwrap_or(false);
    
    // Compress the chunk using the specified algorithm
    let compressed_chunk = match algo.to_lowercase().as_str() {
        "gzip" | "gz" => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(chunk).map_err(|e| format!("Gzip compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Gzip finish failed: {}", e))?
        }
        "zstd" | "zstandard" => {
            zstd::encode_all(chunk, 3)
                .map_err(|e| format!("Zstd compression failed: {}", e))?
        }
        "brotli" | "br" => {
            let mut compressed = Vec::new();
            let params = brotli::enc::BrotliEncoderParams::default();
            brotli::BrotliCompress(&mut &chunk[..], &mut compressed, &params)
                .map_err(|e| format!("Brotli compression failed: {}", e))?;
            compressed
        }
        _ => return Err(format!("Unsupported streaming algorithm: {}", algo)),
    };
    
    // Update stream state
    let (total_input, total_output) = {
        let mut streams = COMPRESS_STREAMS.lock().map_err(|e| format!("Lock error: {}", e))?;
        let state = streams.entry(stream_id.to_string()).or_insert(StreamState {
            total_input: 0,
            total_output: 0,
        });
        state.total_input += chunk.len() as i64;
        state.total_output += compressed_chunk.len() as i64;
        let totals = (state.total_input, state.total_output);
        
        // Clean up state if this is the final chunk
        if is_final {
            streams.remove(stream_id);
        }
        
        totals
    };
    
    Ok(StreamCompressOutput {
        compressed_chunk,
        total_input,
        total_output,
    })
}

/// Stream Decompress - decompress data in chunks
pub async fn stream_decompress(
    stream_id: &str,
    chunk: &[u8],
    algorithm: Option<&str>,
    is_final: Option<bool>,
) -> Result<StreamDecompressOutput, String> {
    let algo = algorithm.unwrap_or("gzip");
    let is_final = is_final.unwrap_or(false);
    
    // Decompress the chunk using the specified algorithm
    let decompressed_chunk = match algo.to_lowercase().as_str() {
        "gzip" | "gz" => {
            let mut decoder = GzDecoder::new(chunk);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)
                .map_err(|e| format!("Gzip decompression failed: {}", e))?;
            decompressed
        }
        "zstd" | "zstandard" => {
            zstd::decode_all(chunk)
                .map_err(|e| format!("Zstd decompression failed: {}", e))?
        }
        "brotli" | "br" => {
            let mut decompressed = Vec::new();
            brotli::BrotliDecompress(&mut &chunk[..], &mut decompressed)
                .map_err(|e| format!("Brotli decompression failed: {}", e))?;
            decompressed
        }
        _ => return Err(format!("Unsupported streaming algorithm: {}", algo)),
    };
    
    // Update stream state
    let (total_input, total_output) = {
        let mut streams = DECOMPRESS_STREAMS.lock().map_err(|e| format!("Lock error: {}", e))?;
        let state = streams.entry(stream_id.to_string()).or_insert(StreamState {
            total_input: 0,
            total_output: 0,
        });
        state.total_input += chunk.len() as i64;
        state.total_output += decompressed_chunk.len() as i64;
        let totals = (state.total_input, state.total_output);
        
        // Clean up state if this is the final chunk
        if is_final {
            streams.remove(stream_id);
        }
        
        totals
    };
    
    Ok(StreamDecompressOutput {
        decompressed_chunk,
        total_input,
        total_output,
    })
}

#[cfg(test)]
mod tests;

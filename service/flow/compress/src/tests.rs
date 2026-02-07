#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_DATA: &[u8] = b"Hello, World! This is a test string for compression. \
        It needs to be reasonably long to see meaningful compression ratios. \
        The quick brown fox jumps over the lazy dog. \
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

    // Gzip tests
    #[tokio::test]
    async fn test_gzip_compress_decompress() {
        let result = gzip_compress(TEST_DATA, None).await.unwrap();
        assert!(!result.compressed.is_empty());
        assert!(result.compressed_size > 0);
        assert!(result.original_size == TEST_DATA.len() as i64);
        
        let decompressed = gzip_decompress(&result.compressed).await.unwrap();
        assert_eq!(decompressed.decompressed, TEST_DATA);
        assert_eq!(decompressed.size, TEST_DATA.len() as i64);
    }

    #[tokio::test]
    async fn test_gzip_compress_with_level() {
        let low = gzip_compress(TEST_DATA, Some(1)).await.unwrap();
        let high = gzip_compress(TEST_DATA, Some(9)).await.unwrap();
        
        // Higher compression level should produce smaller or equal output
        assert!(high.compressed_size <= low.compressed_size || high.compressed_size == low.compressed_size);
    }

    #[tokio::test]
    async fn test_gzip_empty_data() {
        let result = gzip_compress(&[], None).await.unwrap();
        assert_eq!(result.original_size, 0);
        
        let decompressed = gzip_decompress(&result.compressed).await.unwrap();
        assert!(decompressed.decompressed.is_empty());
    }

    // Zstd tests
    #[tokio::test]
    async fn test_zstd_compress_decompress() {
        let result = zstd_compress(TEST_DATA, None).await.unwrap();
        assert!(!result.compressed.is_empty());
        assert!(result.compressed_size > 0);
        
        let decompressed = zstd_decompress(&result.compressed).await.unwrap();
        assert_eq!(decompressed.decompressed, TEST_DATA);
    }

    #[tokio::test]
    async fn test_zstd_compress_with_level() {
        let low = zstd_compress(TEST_DATA, Some(1)).await.unwrap();
        let high = zstd_compress(TEST_DATA, Some(19)).await.unwrap();
        
        // Both should decompress correctly
        let d1 = zstd_decompress(&low.compressed).await.unwrap();
        let d2 = zstd_decompress(&high.compressed).await.unwrap();
        assert_eq!(d1.decompressed, TEST_DATA);
        assert_eq!(d2.decompressed, TEST_DATA);
    }

    // Brotli tests
    #[tokio::test]
    async fn test_brotli_compress_decompress() {
        let result = brotli_compress(TEST_DATA, None).await.unwrap();
        assert!(!result.compressed.is_empty());
        assert!(result.compressed_size > 0);
        
        let decompressed = brotli_decompress(&result.compressed).await.unwrap();
        assert_eq!(decompressed.decompressed, TEST_DATA);
    }

    #[tokio::test]
    async fn test_brotli_compress_with_quality() {
        let low = brotli_compress(TEST_DATA, Some(1)).await.unwrap();
        let high = brotli_compress(TEST_DATA, Some(11)).await.unwrap();
        
        let d1 = brotli_decompress(&low.compressed).await.unwrap();
        let d2 = brotli_decompress(&high.compressed).await.unwrap();
        assert_eq!(d1.decompressed, TEST_DATA);
        assert_eq!(d2.decompressed, TEST_DATA);
    }

    // LZ4 tests
    #[tokio::test]
    async fn test_lz4_compress_decompress() {
        let result = lz4_compress(TEST_DATA, None).await.unwrap();
        assert!(!result.compressed.is_empty());
        assert!(result.compressed_size > 0);
        
        let decompressed = lz4_decompress(&result.compressed).await.unwrap();
        assert_eq!(decompressed.decompressed, TEST_DATA);
    }

    #[tokio::test]
    async fn test_lz4_high_compression() {
        let normal = lz4_compress(TEST_DATA, Some(false)).await.unwrap();
        let high = lz4_compress(TEST_DATA, Some(true)).await.unwrap();
        
        let d1 = lz4_decompress(&normal.compressed).await.unwrap();
        let d2 = lz4_decompress(&high.compressed).await.unwrap();
        assert_eq!(d1.decompressed, TEST_DATA);
        assert_eq!(d2.decompressed, TEST_DATA);
    }

    // Deflate tests
    #[tokio::test]
    async fn test_deflate_compress_decompress() {
        let result = deflate_compress(TEST_DATA, None).await.unwrap();
        assert!(!result.compressed.is_empty());
        assert!(result.compressed_size > 0);
        
        let decompressed = deflate_decompress(&result.compressed).await.unwrap();
        assert_eq!(decompressed.decompressed, TEST_DATA);
    }

    #[tokio::test]
    async fn test_deflate_compress_with_level() {
        let low = deflate_compress(TEST_DATA, Some(1)).await.unwrap();
        let high = deflate_compress(TEST_DATA, Some(9)).await.unwrap();
        
        let d1 = deflate_decompress(&low.compressed).await.unwrap();
        let d2 = deflate_decompress(&high.compressed).await.unwrap();
        assert_eq!(d1.decompressed, TEST_DATA);
        assert_eq!(d2.decompressed, TEST_DATA);
    }

    // Auto compress/decompress tests
    #[tokio::test]
    async fn test_auto_compress_gzip() {
        let result = auto_compress(TEST_DATA, Some("gzip"), None).await.unwrap();
        assert_eq!(result.algorithm_used, "gzip");
        assert!(!result.compressed.is_empty());
    }

    #[tokio::test]
    async fn test_auto_compress_zstd() {
        let result = auto_compress(TEST_DATA, Some("zstd"), None).await.unwrap();
        assert_eq!(result.algorithm_used, "zstd");
        assert!(!result.compressed.is_empty());
    }

    #[tokio::test]
    async fn test_auto_compress_brotli() {
        let result = auto_compress(TEST_DATA, Some("brotli"), None).await.unwrap();
        assert_eq!(result.algorithm_used, "brotli");
        assert!(!result.compressed.is_empty());
    }

    #[tokio::test]
    async fn test_auto_compress_lz4() {
        let result = auto_compress(TEST_DATA, Some("lz4"), None).await.unwrap();
        assert_eq!(result.algorithm_used, "lz4");
        assert!(!result.compressed.is_empty());
    }

    #[tokio::test]
    async fn test_auto_compress_deflate() {
        let result = auto_compress(TEST_DATA, Some("deflate"), None).await.unwrap();
        assert_eq!(result.algorithm_used, "deflate");
        assert!(!result.compressed.is_empty());
    }

    #[tokio::test]
    async fn test_auto_compress_default() {
        let result = auto_compress(TEST_DATA, None, None).await.unwrap();
        assert_eq!(result.algorithm_used, "gzip"); // Default is gzip for Balanced
    }

    #[tokio::test]
    async fn test_auto_compress_optimize_for_speed() {
        let result = auto_compress(TEST_DATA, None, Some(OptimizeFor::Speed)).await.unwrap();
        assert_eq!(result.algorithm_used, "lz4"); // Speed uses lz4
    }

    #[tokio::test]
    async fn test_auto_compress_optimize_for_size() {
        let result = auto_compress(TEST_DATA, None, Some(OptimizeFor::Size)).await.unwrap();
        assert_eq!(result.algorithm_used, "zstd"); // Size uses zstd
    }

    #[tokio::test]
    async fn test_auto_compress_unknown_algorithm() {
        let result = auto_compress(TEST_DATA, Some("unknown"), None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_auto_decompress_gzip() {
        let compressed = gzip_compress(TEST_DATA, None).await.unwrap();
        let result = auto_decompress(&compressed.compressed).await.unwrap();
        assert_eq!(result.algorithm_detected, "gzip");
        assert_eq!(result.decompressed, TEST_DATA);
    }

    #[tokio::test]
    async fn test_auto_decompress_zstd() {
        let compressed = zstd_compress(TEST_DATA, None).await.unwrap();
        let result = auto_decompress(&compressed.compressed).await.unwrap();
        assert_eq!(result.algorithm_detected, "zstd");
        assert_eq!(result.decompressed, TEST_DATA);
    }

    // Compression ratio tests
    #[tokio::test]
    async fn test_compression_ratio() {
        let result = gzip_compress(TEST_DATA, None).await.unwrap();
        assert!(result.compression_ratio > 0.0);
        assert!(result.compression_ratio < 1.0);
    }

    #[tokio::test]
    async fn test_compression_ratio_empty() {
        let result = gzip_compress(&[], None).await.unwrap();
        assert_eq!(result.compression_ratio, 0.0);
    }

    // Binary data tests
    #[tokio::test]
    async fn test_binary_data() {
        let binary_data: Vec<u8> = (0..=255).collect();
        
        let compressed = zstd_compress(&binary_data, None).await.unwrap();
        let decompressed = zstd_decompress(&compressed.compressed).await.unwrap();
        assert_eq!(decompressed.decompressed, binary_data);
    }

    // Large data tests
    #[tokio::test]
    async fn test_large_data() {
        let large_data: Vec<u8> = vec![b'A'; 100_000];
        
        let compressed = zstd_compress(&large_data, None).await.unwrap();
        assert!(compressed.compressed_size < large_data.len() as i64);
        
        let decompressed = zstd_decompress(&compressed.compressed).await.unwrap();
        assert_eq!(decompressed.decompressed, large_data);
    }

    // Stream compress/decompress tests
    #[tokio::test]
    async fn test_stream_compress_gzip() {
        let result = stream_compress("test-stream-1", TEST_DATA, Some("gzip"), Some(true)).await.unwrap();
        assert!(!result.compressed_chunk.is_empty());
        assert_eq!(result.total_input, TEST_DATA.len() as i64);
        assert!(result.total_output > 0);
    }

    #[tokio::test]
    async fn test_stream_compress_zstd() {
        let result = stream_compress("test-stream-2", TEST_DATA, Some("zstd"), Some(true)).await.unwrap();
        assert!(!result.compressed_chunk.is_empty());
        assert_eq!(result.total_input, TEST_DATA.len() as i64);
    }

    #[tokio::test]
    async fn test_stream_compress_brotli() {
        let result = stream_compress("test-stream-3", TEST_DATA, Some("brotli"), Some(true)).await.unwrap();
        assert!(!result.compressed_chunk.is_empty());
        assert_eq!(result.total_input, TEST_DATA.len() as i64);
    }

    #[tokio::test]
    async fn test_stream_decompress_gzip() {
        let compressed = gzip_compress(TEST_DATA, None).await.unwrap();
        let result = stream_decompress("test-stream-4", &compressed.compressed, Some("gzip"), Some(true)).await.unwrap();
        assert_eq!(result.decompressed_chunk, TEST_DATA);
        assert_eq!(result.total_input, compressed.compressed_size);
    }

    #[tokio::test]
    async fn test_stream_decompress_zstd() {
        let compressed = zstd_compress(TEST_DATA, None).await.unwrap();
        let result = stream_decompress("test-stream-5", &compressed.compressed, Some("zstd"), Some(true)).await.unwrap();
        assert_eq!(result.decompressed_chunk, TEST_DATA);
    }

    #[tokio::test]
    async fn test_stream_decompress_brotli() {
        let compressed = brotli_compress(TEST_DATA, None).await.unwrap();
        let result = stream_decompress("test-stream-6", &compressed.compressed, Some("brotli"), Some(true)).await.unwrap();
        assert_eq!(result.decompressed_chunk, TEST_DATA);
    }

    #[tokio::test]
    async fn test_stream_compress_multiple_chunks() {
        let chunk1 = b"First chunk of data. ";
        let chunk2 = b"Second chunk of data. ";
        let chunk3 = b"Third and final chunk.";
        
        let r1 = stream_compress("test-multi-stream", chunk1, Some("gzip"), Some(false)).await.unwrap();
        assert_eq!(r1.total_input, chunk1.len() as i64);
        
        let r2 = stream_compress("test-multi-stream", chunk2, Some("gzip"), Some(false)).await.unwrap();
        assert_eq!(r2.total_input, (chunk1.len() + chunk2.len()) as i64);
        
        let r3 = stream_compress("test-multi-stream", chunk3, Some("gzip"), Some(true)).await.unwrap();
        assert_eq!(r3.total_input, (chunk1.len() + chunk2.len() + chunk3.len()) as i64);
    }

    #[tokio::test]
    async fn test_stream_compress_unsupported_algorithm() {
        let result = stream_compress("test-unsupported", TEST_DATA, Some("lz4"), Some(true)).await;
        assert!(result.is_err());
    }
}

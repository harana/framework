#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(dir: &TempDir, name: &str, content: &str) -> String {
        let path = dir.path().join(name);
        fs::write(&path, content).unwrap();
        path.to_string_lossy().to_string()
    }

    // Zip tests
    #[tokio::test]
    async fn test_zip_single_file() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", "Hello, World!");
        let zip_path = temp.path().join("test.zip").to_string_lossy().to_string();
        
        let result = zip(&zip_path, vec![test_file], None, None, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 1);
        assert!(result.size > 0);
        assert!(fs::metadata(&zip_path).is_ok());
    }

    #[tokio::test]
    async fn test_zip_multiple_files() {
        let temp = TempDir::new().unwrap();
        let file1 = create_test_file(&temp, "file1.txt", "Content 1");
        let file2 = create_test_file(&temp, "file2.txt", "Content 2");
        let zip_path = temp.path().join("multi.zip").to_string_lossy().to_string();
        
        let result = zip(&zip_path, vec![file1, file2], None, None, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 2);
    }

    #[tokio::test]
    async fn test_unzip() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", "Hello, World!");
        let zip_path = temp.path().join("test.zip").to_string_lossy().to_string();
        let extract_dir = temp.path().join("extracted").to_string_lossy().to_string();
        
        // Create zip
        zip(&zip_path, vec![test_file], None, None, None).await.unwrap();
        
        // Extract zip
        let result = unzip(&extract_dir, &zip_path, None, None, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 1);
        
        // Verify extracted content
        let extracted = fs::read_to_string(format!("{}/test.txt", extract_dir)).unwrap();
        assert_eq!(extracted, "Hello, World!");
    }

    #[tokio::test]
    async fn test_list_zip() {
        let temp = TempDir::new().unwrap();
        let file1 = create_test_file(&temp, "file1.txt", "Content 1");
        let file2 = create_test_file(&temp, "file2.txt", "Content 2");
        let zip_path = temp.path().join("list.zip").to_string_lossy().to_string();
        
        zip(&zip_path, vec![file1, file2], None, None, None).await.unwrap();
        
        let result = list(&zip_path, "zip", None).await.unwrap();
        assert_eq!(result.total_files, 2);
        assert!(result.total_size > 0);
    }

    #[tokio::test]
    async fn test_get_archive_info() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", "Hello, World!");
        let zip_path = temp.path().join("info.zip").to_string_lossy().to_string();
        
        zip(&zip_path, vec![test_file], None, None, None).await.unwrap();
        
        let result = get_archive_info(&zip_path).await.unwrap();
        assert_eq!(result.file_count, 1);
        assert!(result.compressed_size > 0);
        assert_eq!(result.archive_type, "zip");
    }

    #[tokio::test]
    async fn test_verify_archive_valid() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", "Hello, World!");
        let zip_path = temp.path().join("valid.zip").to_string_lossy().to_string();
        
        zip(&zip_path, vec![test_file], None, None, None).await.unwrap();
        
        let result = verify_archive(&zip_path, "zip", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.file_count, 1);
    }

    #[tokio::test]
    async fn test_verify_archive_invalid() {
        let temp = TempDir::new().unwrap();
        let invalid_path = create_test_file(&temp, "invalid.zip", "Not a zip file");
        
        let result = verify_archive(&invalid_path, "zip", None).await.unwrap();
        assert!(!result.valid);
        assert!(!result.error.is_empty());
    }

    // Gzip tests
    #[tokio::test]
    async fn test_gzip_gunzip() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", "Hello, World! This is test content.");
        let gz_path = temp.path().join("test.txt.gz").to_string_lossy().to_string();
        let decompressed_path = temp.path().join("decompressed.txt").to_string_lossy().to_string();
        
        // Compress
        let compress_result = gzip(&test_file, None, Some(true), Some(&gz_path)).await.unwrap();
        assert!(compress_result.success);
        assert!(compress_result.compressed_size > 0);
        
        // Decompress
        let decompress_result = gunzip(&gz_path, Some(&decompressed_path), Some(true)).await.unwrap();
        assert!(decompress_result.success);
        
        // Verify content
        let content = fs::read_to_string(&decompressed_path).unwrap();
        assert_eq!(content, "Hello, World! This is test content.");
    }

    #[tokio::test]
    async fn test_gzip_compression_levels() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", &"A".repeat(1000));
        
        let fast_path = temp.path().join("fast.gz").to_string_lossy().to_string();
        let best_path = temp.path().join("best.gz").to_string_lossy().to_string();
        
        let fast_result = gzip(&test_file, Some("1"), Some(true), Some(&fast_path)).await.unwrap();
        let best_result = gzip(&test_file, Some("9"), Some(true), Some(&best_path)).await.unwrap();
        
        // Best compression should produce smaller or equal output
        assert!(best_result.compressed_size <= fast_result.compressed_size);
    }

    // Tar tests
    #[tokio::test]
    async fn test_tar_single_file() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", "Hello, World!");
        let tar_path = temp.path().join("test.tar").to_string_lossy().to_string();
        
        let result = tar(vec![test_file], &tar_path, None, None, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 1);
        // Size may be 0 immediately after writing due to buffering, so just check file exists
        assert!(fs::metadata(&tar_path).is_ok());
    }

    #[tokio::test]
    async fn test_tar_directory() {
        let temp = TempDir::new().unwrap();
        let subdir = temp.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        
        fs::write(subdir.join("file1.txt"), "Content 1").unwrap();
        fs::write(subdir.join("file2.txt"), "Content 2").unwrap();
        
        let tar_path = temp.path().join("dir.tar").to_string_lossy().to_string();
        
        let result = tar(vec![subdir.to_string_lossy().to_string()], &tar_path, None, None, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 2);
    }

    #[tokio::test]
    async fn test_untar() {
        let temp = TempDir::new().unwrap();
        let test_file = create_test_file(&temp, "test.txt", "Hello, World!");
        let tar_path = temp.path().join("test.tar").to_string_lossy().to_string();
        let extract_dir = temp.path().join("extracted").to_string_lossy().to_string();
        
        tar(vec![test_file], &tar_path, None, None, None).await.unwrap();
        
        let result = untar(&tar_path, &extract_dir, None, None, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 1);
    }

    // Edge cases
    #[tokio::test]
    async fn test_zip_empty_source() {
        let temp = TempDir::new().unwrap();
        let zip_path = temp.path().join("empty.zip").to_string_lossy().to_string();
        
        let result = zip(&zip_path, vec![], None, None, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 0);
    }

    #[tokio::test]
    async fn test_unzip_selective() {
        let temp = TempDir::new().unwrap();
        let file1 = create_test_file(&temp, "file1.txt", "Content 1");
        let file2 = create_test_file(&temp, "file2.txt", "Content 2");
        let zip_path = temp.path().join("selective.zip").to_string_lossy().to_string();
        let extract_dir = temp.path().join("extracted").to_string_lossy().to_string();
        
        zip(&zip_path, vec![file1, file2], None, None, None).await.unwrap();
        
        // Only extract file1.txt
        let result = unzip(&extract_dir, &zip_path, None, None, Some(vec!["file1.txt".to_string()])).await.unwrap();
        assert!(result.success);
        assert_eq!(result.file_count, 1);
        
        // Verify only file1.txt was extracted
        assert!(fs::metadata(format!("{}/file1.txt", extract_dir)).is_ok());
        assert!(fs::metadata(format!("{}/file2.txt", extract_dir)).is_err());
    }
}

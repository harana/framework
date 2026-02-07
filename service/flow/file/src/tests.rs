// Harana Actions - File Module Tests

use super::*;
use tempfile::TempDir;

// ============================================================================
// copy tests
// ============================================================================

#[tokio::test]
async fn test_copy_success() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("source.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    std::fs::write(&source, "hello world").unwrap();
    
    let result = copy(source.to_str().unwrap(), dest.to_str().unwrap(), None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
    assert!(dest.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "hello world");
}

#[tokio::test]
async fn test_copy_overwrite_false() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("source.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    std::fs::write(&source, "source content").unwrap();
    std::fs::write(&dest, "existing content").unwrap();
    
    let result = copy(source.to_str().unwrap(), dest.to_str().unwrap(), Some(false)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already exists"));
}

#[tokio::test]
async fn test_copy_overwrite_true() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("source.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    std::fs::write(&source, "new content").unwrap();
    std::fs::write(&dest, "old content").unwrap();
    
    let result = copy(source.to_str().unwrap(), dest.to_str().unwrap(), Some(true)).await;
    assert!(result.is_ok());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "new content");
}

#[tokio::test]
async fn test_copy_source_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("nonexistent.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    let result = copy(source.to_str().unwrap(), dest.to_str().unwrap(), None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

// ============================================================================
// create_directory tests
// ============================================================================

#[tokio::test]
async fn test_create_directory_simple() {
    let temp_dir = TempDir::new().unwrap();
    let new_dir = temp_dir.path().join("new_dir");
    
    let result = create_directory(new_dir.to_str().unwrap(), None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
    assert!(new_dir.exists());
    assert!(new_dir.is_dir());
}

#[tokio::test]
async fn test_create_directory_recursive() {
    let temp_dir = TempDir::new().unwrap();
    let nested_dir = temp_dir.path().join("a").join("b").join("c");
    
    let result = create_directory(nested_dir.to_str().unwrap(), Some(true)).await;
    assert!(result.is_ok());
    assert!(nested_dir.exists());
}

#[tokio::test]
async fn test_create_directory_non_recursive_fails() {
    let temp_dir = TempDir::new().unwrap();
    let nested_dir = temp_dir.path().join("a").join("b").join("c");
    
    let result = create_directory(nested_dir.to_str().unwrap(), Some(false)).await;
    assert!(result.is_err());
}

// ============================================================================
// delete tests
// ============================================================================

#[tokio::test]
async fn test_delete_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("to_delete.txt");
    std::fs::write(&file_path, "content").unwrap();
    
    let result = delete(file_path.to_str().unwrap()).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
    assert!(!file_path.exists());
}

#[tokio::test]
async fn test_delete_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent.txt");
    
    let result = delete(file_path.to_str().unwrap()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

#[tokio::test]
async fn test_delete_directory_fails() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("a_directory");
    std::fs::create_dir(&dir_path).unwrap();
    
    let result = delete(dir_path.to_str().unwrap()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("directory"));
}

// ============================================================================
// delete_directory tests
// ============================================================================

#[tokio::test]
async fn test_delete_directory_empty() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("empty_dir");
    std::fs::create_dir(&dir_path).unwrap();
    
    let result = delete_directory(dir_path.to_str().unwrap(), Some(false)).await;
    assert!(result.is_ok());
    assert!(!dir_path.exists());
}

#[tokio::test]
async fn test_delete_directory_recursive() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("dir_with_contents");
    std::fs::create_dir(&dir_path).unwrap();
    std::fs::write(dir_path.join("file.txt"), "content").unwrap();
    std::fs::create_dir(dir_path.join("subdir")).unwrap();
    
    let result = delete_directory(dir_path.to_str().unwrap(), Some(true)).await;
    assert!(result.is_ok());
    assert!(!dir_path.exists());
}

#[tokio::test]
async fn test_delete_directory_non_recursive_fails_if_not_empty() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("dir_with_contents");
    std::fs::create_dir(&dir_path).unwrap();
    std::fs::write(dir_path.join("file.txt"), "content").unwrap();
    
    let result = delete_directory(dir_path.to_str().unwrap(), Some(false)).await;
    assert!(result.is_err());
}

// ============================================================================
// exists tests
// ============================================================================

#[tokio::test]
async fn test_exists_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("exists.txt");
    std::fs::write(&file_path, "content").unwrap();
    
    let result = exists(file_path.to_str().unwrap()).await;
    assert!(result.is_ok());
    assert!(result.unwrap().exists);
}

#[tokio::test]
async fn test_exists_directory() {
    let temp_dir = TempDir::new().unwrap();
    
    let result = exists(temp_dir.path().to_str().unwrap()).await;
    assert!(result.is_ok());
    assert!(result.unwrap().exists);
}

#[tokio::test]
async fn test_exists_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent.txt");
    
    let result = exists(file_path.to_str().unwrap()).await;
    assert!(result.is_ok());
    assert!(!result.unwrap().exists);
}

// ============================================================================
// info tests
// ============================================================================

#[tokio::test]
async fn test_info_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("info_test.txt");
    std::fs::write(&file_path, "hello world").unwrap();
    
    let result = info(file_path.to_str().unwrap()).await;
    assert!(result.is_ok());
    let info = result.unwrap();
    assert!(!info.is_directory);
    assert_eq!(info.size, 11); // "hello world".len()
}

#[tokio::test]
async fn test_info_directory() {
    let temp_dir = TempDir::new().unwrap();
    
    let result = info(temp_dir.path().to_str().unwrap()).await;
    assert!(result.is_ok());
    let info = result.unwrap();
    assert!(info.is_directory);
}

#[tokio::test]
async fn test_info_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent.txt");
    
    let result = info(file_path.to_str().unwrap()).await;
    assert!(result.is_err());
}

// ============================================================================
// list_directory tests
// ============================================================================

#[tokio::test]
async fn test_list_directory_simple() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("a.txt"), "").unwrap();
    std::fs::write(temp_dir.path().join("b.txt"), "").unwrap();
    std::fs::create_dir(temp_dir.path().join("subdir")).unwrap();
    
    let result = list_directory(temp_dir.path().to_str().unwrap(), None, None).await;
    assert!(result.is_ok());
    let files = result.unwrap().files;
    assert_eq!(files.len(), 3);
}

#[tokio::test]
async fn test_list_directory_recursive() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("root.txt"), "").unwrap();
    std::fs::create_dir(temp_dir.path().join("subdir")).unwrap();
    std::fs::write(temp_dir.path().join("subdir").join("nested.txt"), "").unwrap();
    
    let result = list_directory(temp_dir.path().to_str().unwrap(), Some(true), None).await;
    assert!(result.is_ok());
    let files = result.unwrap().files;
    assert_eq!(files.len(), 3); // root.txt, subdir, subdir/nested.txt
}

#[tokio::test]
async fn test_list_directory_with_pattern() {
    let temp_dir = TempDir::new().unwrap();
    std::fs::write(temp_dir.path().join("file1.txt"), "").unwrap();
    std::fs::write(temp_dir.path().join("file2.txt"), "").unwrap();
    std::fs::write(temp_dir.path().join("data.json"), "").unwrap();
    
    let result = list_directory(temp_dir.path().to_str().unwrap(), None, Some("*.txt")).await;
    assert!(result.is_ok());
    let files = result.unwrap().files;
    assert_eq!(files.len(), 2);
}

#[tokio::test]
async fn test_list_directory_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("nonexistent");
    
    let result = list_directory(dir_path.to_str().unwrap(), None, None).await;
    assert!(result.is_err());
}

// ============================================================================
// move tests
// ============================================================================

#[tokio::test]
async fn test_move_success() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("source.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    std::fs::write(&source, "content to move").unwrap();
    
    let result = r#move(source.to_str().unwrap(), dest.to_str().unwrap(), None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
    assert!(!source.exists());
    assert!(dest.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "content to move");
}

#[tokio::test]
async fn test_move_overwrite_false() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("source.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    std::fs::write(&source, "source content").unwrap();
    std::fs::write(&dest, "existing content").unwrap();
    
    let result = r#move(source.to_str().unwrap(), dest.to_str().unwrap(), Some(false)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already exists"));
}

#[tokio::test]
async fn test_move_overwrite_true() {
    let temp_dir = TempDir::new().unwrap();
    let source = temp_dir.path().join("source.txt");
    let dest = temp_dir.path().join("dest.txt");
    
    std::fs::write(&source, "new content").unwrap();
    std::fs::write(&dest, "old content").unwrap();
    
    let result = r#move(source.to_str().unwrap(), dest.to_str().unwrap(), Some(true)).await;
    assert!(result.is_ok());
    assert!(!source.exists());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "new content");
}

// ============================================================================
// read tests
// ============================================================================

#[tokio::test]
async fn test_read_text() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("read_test.txt");
    std::fs::write(&file_path, "hello world").unwrap();
    
    let result = read(file_path.to_str().unwrap(), None).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, "hello world");
}

#[tokio::test]
async fn test_read_text_explicit() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("read_test.txt");
    std::fs::write(&file_path, "text content").unwrap();
    
    let result = read(file_path.to_str().unwrap(), Some("text")).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, "text content");
}

#[tokio::test]
async fn test_read_binary() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("binary_test.bin");
    std::fs::write(&file_path, &[0x00, 0x01, 0xff]).unwrap();
    
    let result = read(file_path.to_str().unwrap(), Some("binary")).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, "0001ff");
}

#[tokio::test]
async fn test_read_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nonexistent.txt");
    
    let result = read(file_path.to_str().unwrap(), None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

// ============================================================================
// write tests
// ============================================================================

#[tokio::test]
async fn test_write_text() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("write_test.txt");
    
    let result = write(file_path.to_str().unwrap(), "hello world", None, None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
    assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "hello world");
}

#[tokio::test]
async fn test_write_creates_parent_dirs() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("nested").join("dirs").join("file.txt");
    
    let result = write(file_path.to_str().unwrap(), "content", None, None).await;
    assert!(result.is_ok());
    assert!(file_path.exists());
}

#[tokio::test]
async fn test_write_overwrite_false() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("existing.txt");
    std::fs::write(&file_path, "existing content").unwrap();
    
    let result = write(file_path.to_str().unwrap(), "new content", Some(false), None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already exists"));
}

#[tokio::test]
async fn test_write_overwrite_true() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("existing.txt");
    std::fs::write(&file_path, "old content").unwrap();
    
    let result = write(file_path.to_str().unwrap(), "new content", Some(true), None).await;
    assert!(result.is_ok());
    assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "new content");
}

#[tokio::test]
async fn test_write_binary() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("binary_write.bin");
    
    let result = write(file_path.to_str().unwrap(), "0001ff", None, Some("binary")).await;
    assert!(result.is_ok());
    assert_eq!(std::fs::read(&file_path).unwrap(), vec![0x00, 0x01, 0xff]);
}

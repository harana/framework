// Harana Actions - Excel Module Tests

use super::*;
use tempfile::TempDir;
use std::path::PathBuf;

/// Helper to create a test Excel file and return its path
async fn create_test_excel() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.xlsx");
    
    let data = vec![
        {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Alice".to_string()));
            map.insert("age".to_string(), Value::Number(30.into()));
            map.insert("active".to_string(), Value::Bool(true));
            map
        },
        {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Bob".to_string()));
            map.insert("age".to_string(), Value::Number(25.into()));
            map.insert("active".to_string(), Value::Bool(false));
            map
        },
    ];
    
    write(
        file_path.to_str().unwrap(),
        data,
        Some(vec!["name".to_string(), "age".to_string(), "active".to_string()]),
        Some(true),
        Some("TestSheet"),
    ).await.unwrap();
    
    (temp_dir, file_path)
}

// ============================================================================
// write tests
// ============================================================================

#[tokio::test]
async fn test_write_simple() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("output.xlsx");
    
    let data = vec![
        {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Alice".to_string()));
            map.insert("age".to_string(), Value::Number(30.into()));
            map
        },
    ];
    
    let result = write(file_path.to_str().unwrap(), data, None, None, None).await;
    assert!(result.is_ok());
    assert!(file_path.exists());
}

#[tokio::test]
async fn test_write_with_custom_headers() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("output.xlsx");
    
    let data = vec![
        {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Alice".to_string()));
            map
        },
    ];
    
    let result = write(
        file_path.to_str().unwrap(),
        data,
        Some(vec!["name".to_string()]),
        Some(true),
        None,
    ).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_write_no_headers() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("output.xlsx");
    
    let data = vec![
        {
            let mut map = HashMap::new();
            map.insert("value".to_string(), Value::String("test".to_string()));
            map
        },
    ];
    
    let result = write(
        file_path.to_str().unwrap(),
        data,
        None,
        Some(false),
        None,
    ).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_write_custom_sheet_name() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("output.xlsx");
    
    let data = vec![
        {
            let mut map = HashMap::new();
            map.insert("test".to_string(), Value::String("data".to_string()));
            map
        },
    ];
    
    let result = write(
        file_path.to_str().unwrap(),
        data,
        None,
        None,
        Some("CustomSheet"),
    ).await;
    assert!(result.is_ok());
    
    // Verify the sheet exists
    let sheets = get_sheets(file_path.to_str().unwrap()).await.unwrap();
    assert!(sheets.sheets.contains(&"CustomSheet".to_string()));
}

#[tokio::test]
async fn test_write_empty_data() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("output.xlsx");
    
    let result = write(file_path.to_str().unwrap(), vec![], None, None, None).await;
    assert!(result.is_ok());
}

// ============================================================================
// get_sheets tests
// ============================================================================

#[tokio::test]
async fn test_get_sheets() {
    let (_temp_dir, file_path) = create_test_excel().await;
    
    let result = get_sheets(file_path.to_str().unwrap()).await;
    assert!(result.is_ok());
    let sheets = result.unwrap().sheets;
    assert!(!sheets.is_empty());
    assert!(sheets.contains(&"TestSheet".to_string()));
}

#[tokio::test]
async fn test_get_sheets_file_not_found() {
    let result = get_sheets("/nonexistent/file.xlsx").await;
    assert!(result.is_err());
}

// ============================================================================
// read tests
// ============================================================================

#[tokio::test]
async fn test_read_with_headers() {
    let (_temp_dir, file_path) = create_test_excel().await;
    
    let result = read(file_path.to_str().unwrap(), Some(true), None, None).await;
    assert!(result.is_ok());
    let rows = result.unwrap().rows;
    assert_eq!(rows.len(), 2);
    assert!(rows[0].contains_key("name"));
}

#[tokio::test]
async fn test_read_without_headers() {
    let (_temp_dir, file_path) = create_test_excel().await;
    
    let result = read(file_path.to_str().unwrap(), Some(false), None, None).await;
    assert!(result.is_ok());
    let rows = result.unwrap().rows;
    // Without headers, we get 3 rows (header row is treated as data)
    assert_eq!(rows.len(), 3);
}

#[tokio::test]
async fn test_read_specific_sheet() {
    let (_temp_dir, file_path) = create_test_excel().await;
    
    let result = read(file_path.to_str().unwrap(), Some(true), Some("TestSheet"), None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_read_file_not_found() {
    let result = read("/nonexistent/file.xlsx", None, None, None).await;
    assert!(result.is_err());
}

// ============================================================================
// read_sheet tests
// ============================================================================

#[tokio::test]
async fn test_read_sheet() {
    let (_temp_dir, file_path) = create_test_excel().await;
    
    let result = read_sheet("TestSheet", file_path.to_str().unwrap(), None, Some(true)).await;
    assert!(result.is_ok());
    let rows = result.unwrap().rows;
    assert_eq!(rows.len(), 2);
}

#[tokio::test]
async fn test_read_sheet_not_found() {
    let (_temp_dir, file_path) = create_test_excel().await;
    
    let result = read_sheet("NonexistentSheet", file_path.to_str().unwrap(), None, None).await;
    assert!(result.is_err());
}

// ============================================================================
// round-trip tests
// ============================================================================

#[tokio::test]
async fn test_write_then_read() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("roundtrip.xlsx");
    
    // Write data
    let original_data = vec![
        {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::Number(1.into()));
            map.insert("name".to_string(), Value::String("Test".to_string()));
            map
        },
        {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Value::Number(2.into()));
            map.insert("name".to_string(), Value::String("Data".to_string()));
            map
        },
    ];
    
    let headers = vec!["id".to_string(), "name".to_string()];
    
    write(
        file_path.to_str().unwrap(),
        original_data,
        Some(headers.clone()),
        Some(true),
        None,
    ).await.unwrap();
    
    // Read it back
    let result = read(file_path.to_str().unwrap(), Some(true), None, None).await;
    assert!(result.is_ok());
    let rows = result.unwrap().rows;
    assert_eq!(rows.len(), 2);
    
    // Verify content
    assert_eq!(rows[0].get("name").unwrap(), &Value::String("Test".to_string()));
    assert_eq!(rows[1].get("name").unwrap(), &Value::String("Data".to_string()));
}

// ============================================================================
// helper function tests
// ============================================================================

#[test]
fn test_parse_range() {
    let result = parse_range("A1:D10");
    assert!(result.is_some());
    let (start_row, start_col, end_row, end_col) = result.unwrap();
    assert_eq!(start_row, 0);
    assert_eq!(start_col, 0);
    assert_eq!(end_row, 9);
    assert_eq!(end_col, 3);
}

#[test]
fn test_parse_range_with_letters() {
    let result = parse_range("B2:Z100");
    assert!(result.is_some());
    let (start_row, start_col, end_row, end_col) = result.unwrap();
    assert_eq!(start_row, 1);
    assert_eq!(start_col, 1);
    assert_eq!(end_row, 99);
    assert_eq!(end_col, 25);
}

#[test]
fn test_parse_range_invalid() {
    assert!(parse_range("invalid").is_none());
    assert!(parse_range("A1").is_none());
    assert!(parse_range("").is_none());
}

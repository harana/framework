use super::*;

#[tokio::test]
async fn test_upload_and_download_blob() {
    clear_all_data();
    
    let content = b"Hello, World!".to_vec();
    
    let upload_result = upload_blob(
        "test-bucket".to_string(),
        "test/file.txt".to_string(),
        content.clone(),
        Some("text/plain".to_string()),
        None,
    ).await;

    assert!(!upload_result.url.is_empty());
    assert!(!upload_result.etag.is_empty());
    assert!(upload_result.url.contains("test-bucket"));

    let download_result = download_blob(
        "test-bucket".to_string(),
        "test/file.txt".to_string(),
    ).await;

    assert_eq!(download_result.content, content);
    assert_eq!(download_result.content_type, "text/plain");
    assert_eq!(download_result.size, content.len() as i64);
}

#[tokio::test]
async fn test_delete_blob() {
    clear_all_data();
    
    // Upload a blob
    upload_blob(
        "test-bucket".to_string(),
        "to-delete.txt".to_string(),
        b"Delete me".to_vec(),
        None,
        None,
    ).await;

    // Verify it exists
    let exists_result = exists(
        "test-bucket".to_string(),
        "to-delete.txt".to_string(),
    ).await;
    assert!(exists_result.exists);

    // Delete it
    let delete_result = delete_blob(
        "test-bucket".to_string(),
        "to-delete.txt".to_string(),
    ).await;
    assert!(delete_result.success);

    // Verify it's gone
    let exists_result2 = exists(
        "test-bucket".to_string(),
        "to-delete.txt".to_string(),
    ).await;
    assert!(!exists_result2.exists);
}

#[tokio::test]
async fn test_list_blobs() {
    clear_all_data();
    
    // Upload multiple blobs
    for i in 0..5 {
        upload_blob(
            "list-bucket".to_string(),
            format!("folder/file{}.txt", i),
            format!("Content {}", i).into_bytes(),
            None,
            None,
        ).await;
    }

    // List with prefix
    let list_result = list_blobs(
        "list-bucket".to_string(),
        Some("folder/".to_string()),
        Some(3),
        None,
    ).await;

    assert_eq!(list_result.blobs.len(), 3);
    assert!(list_result.next_token.is_some());
}

#[tokio::test]
async fn test_get_blob_metadata() {
    clear_all_data();
    
    let mut metadata_values = std::collections::HashMap::new();
    metadata_values.insert("author".to_string(), "test-user".to_string());
    
    let metadata = BlobMetadata { values: metadata_values };

    upload_blob(
        "meta-bucket".to_string(),
        "with-meta.txt".to_string(),
        b"Test content with metadata".to_vec(),
        Some("text/plain".to_string()),
        Some(metadata),
    ).await;

    let meta_result = get_blob_metadata(
        "meta-bucket".to_string(),
        "with-meta.txt".to_string(),
    ).await;

    assert_eq!(meta_result.content_type, "text/plain");
    assert_eq!(meta_result.size, 26);
    assert!(!meta_result.etag.is_empty());
    assert_eq!(
        meta_result.metadata.values.get("author"),
        Some(&"test-user".to_string())
    );
}

#[tokio::test]
async fn test_copy_blob() {
    clear_all_data();
    
    // Upload source blob
    upload_blob(
        "source-bucket".to_string(),
        "source-file.txt".to_string(),
        b"Copy this content".to_vec(),
        Some("text/plain".to_string()),
        None,
    ).await;

    // Copy to new location
    let copy_result = copy_blob(
        "source-bucket".to_string(),
        "source-file.txt".to_string(),
        "dest-bucket".to_string(),
        "dest-file.txt".to_string(),
    ).await;

    assert!(copy_result.success);
    assert!(!copy_result.etag.is_empty());

    // Verify the copy exists
    let download_result = download_blob(
        "dest-bucket".to_string(),
        "dest-file.txt".to_string(),
    ).await;

    assert_eq!(download_result.content, b"Copy this content".to_vec());
}

#[tokio::test]
async fn test_copy_nonexistent_blob() {
    clear_all_data();
    
    let copy_result = copy_blob(
        "source-bucket".to_string(),
        "nonexistent.txt".to_string(),
        "dest-bucket".to_string(),
        "dest-file.txt".to_string(),
    ).await;

    assert!(!copy_result.success);
}

#[tokio::test]
async fn test_generate_presigned_url() {
    clear_all_data();
    
    let presigned_result = generate_presigned_url(
        "presign-bucket".to_string(),
        "presigned-file.txt".to_string(),
        Some(PresignedUrlOperation::Get),
        Some(3600),
    ).await;

    assert!(presigned_result.url.contains("presign-bucket"));
    assert!(presigned_result.url.contains("presigned-file.txt"));
    assert!(presigned_result.url.contains("X-Signature"));
    assert!(presigned_result.expires_at > Utc::now());
}

#[tokio::test]
async fn test_exists_nonexistent() {
    clear_all_data();
    
    let exists_result = exists(
        "nonexistent-bucket".to_string(),
        "nonexistent-file.txt".to_string(),
    ).await;

    assert!(!exists_result.exists);
}

#[tokio::test]
async fn test_upload_with_default_content_type() {
    clear_all_data();
    
    upload_blob(
        "test-bucket".to_string(),
        "binary-file.bin".to_string(),
        vec![0u8, 1, 2, 3, 4],
        None, // No content type specified
        None,
    ).await;

    let meta_result = get_blob_metadata(
        "test-bucket".to_string(),
        "binary-file.bin".to_string(),
    ).await;

    assert_eq!(meta_result.content_type, "application/octet-stream");
}

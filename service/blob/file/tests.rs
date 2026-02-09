use super::*;
use crate::store::{BlobMetadata, BlobStore, ListOptions, PutOptions};
use tempfile::TempDir;

async fn make_store() -> (FileBlobStore, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = FileBlobStore::new(dir.path()).await.unwrap();
    (store, dir)
}

#[tokio::test]
async fn test_put_and_get() {
    let (store, _dir) = make_store().await;
    let data = b"hello blob";
    let info = store
        .put("test.txt", data, PutOptions::new().with_content_type("text/plain"))
        .await
        .unwrap();

    assert_eq!(info.key, "test.txt");
    assert_eq!(info.size, data.len() as u64);
    assert!(info.etag.is_some());
    assert_eq!(info.content_type.as_deref(), Some("text/plain"));

    let retrieved = store.get("test.txt").await.unwrap();
    assert_eq!(retrieved, data);
}

#[tokio::test]
async fn test_get_not_found() {
    let (store, _dir) = make_store().await;
    let result = store.get("nonexistent").await;
    assert!(matches!(result, Err(BlobError::NotFound(_))));
}

#[tokio::test]
async fn test_head() {
    let (store, _dir) = make_store().await;
    store.put("doc.bin", b"binary data", PutOptions::new()).await.unwrap();

    let info = store.head("doc.bin").await.unwrap();
    assert_eq!(info.key, "doc.bin");
    assert_eq!(info.size, 11);
}

#[tokio::test]
async fn test_delete() {
    let (store, _dir) = make_store().await;
    store.put("tmp.txt", b"data", PutOptions::new()).await.unwrap();
    store.delete("tmp.txt").await.unwrap();
    let result = store.get("tmp.txt").await;
    assert!(matches!(result, Err(BlobError::NotFound(_))));
}

#[tokio::test]
async fn test_delete_idempotent() {
    let (store, _dir) = make_store().await;
    // Deleting a non-existent key should not error.
    store.delete("nope").await.unwrap();
}

#[tokio::test]
async fn test_exists() {
    let (store, _dir) = make_store().await;
    assert!(!store.exists("x").await.unwrap());
    store.put("x", b"y", PutOptions::new()).await.unwrap();
    assert!(store.exists("x").await.unwrap());
}

#[tokio::test]
async fn test_put_if_not_exists() {
    let (store, _dir) = make_store().await;
    store.put("key", b"v1", PutOptions::new()).await.unwrap();
    let result = store.put("key", b"v2", PutOptions::new().if_not_exists()).await;
    assert!(matches!(result, Err(BlobError::AlreadyExists(_))));
}

#[tokio::test]
async fn test_list_with_prefix() {
    let (store, _dir) = make_store().await;
    store.put("images/a.png", b"a", PutOptions::new()).await.unwrap();
    store.put("images/b.png", b"b", PutOptions::new()).await.unwrap();
    store.put("docs/c.txt", b"c", PutOptions::new()).await.unwrap();

    let resp = store.list(ListOptions::new().with_prefix("images/")).await.unwrap();
    assert_eq!(resp.objects.len(), 2);
    assert!(resp.objects.iter().all(|o| o.key.starts_with("images/")));
}

#[tokio::test]
async fn test_list_pagination() {
    let (store, _dir) = make_store().await;
    for i in 0..5 {
        store.put(&format!("item{i}"), b"x", PutOptions::new()).await.unwrap();
    }

    let resp = store.list(ListOptions::new().with_limit(3)).await.unwrap();
    assert_eq!(resp.objects.len(), 3);
    assert!(resp.truncated);
    assert!(resp.cursor.is_some());

    let resp2 = store
        .list(ListOptions::new().with_cursor(resp.cursor.unwrap()))
        .await
        .unwrap();
    assert_eq!(resp2.objects.len(), 2);
    assert!(!resp2.truncated);
}

#[tokio::test]
async fn test_copy() {
    let (store, _dir) = make_store().await;
    let meta = BlobMetadata::new().with_content_type("application/pdf");
    store
        .put("src.pdf", b"pdf-data", PutOptions::new().with_metadata(meta))
        .await
        .unwrap();

    let info = store.copy("src.pdf", "dst.pdf").await.unwrap();
    assert_eq!(info.key, "dst.pdf");

    let data = store.get("dst.pdf").await.unwrap();
    assert_eq!(data, b"pdf-data");

    // Source should still exist.
    assert!(store.exists("src.pdf").await.unwrap());
}

#[tokio::test]
async fn test_rename() {
    let (store, _dir) = make_store().await;
    store.put("old", b"data", PutOptions::new()).await.unwrap();
    let info = store.rename("old", "new").await.unwrap();
    assert_eq!(info.key, "new");
    assert!(!store.exists("old").await.unwrap());
    assert!(store.exists("new").await.unwrap());
}

#[tokio::test]
async fn test_get_with_metadata() {
    let (store, _dir) = make_store().await;
    let meta = BlobMetadata::new()
        .with_content_type("image/png")
        .with_custom("author", "test");
    store
        .put("pic.png", b"png-bytes", PutOptions::new().with_metadata(meta))
        .await
        .unwrap();

    let (data, metadata) = store.get_with_metadata("pic.png").await.unwrap();
    assert_eq!(data, b"png-bytes");
    assert_eq!(metadata.content_type.as_deref(), Some("image/png"));
    assert_eq!(metadata.custom.get("author").map(|s| s.as_str()), Some("test"));
}

#[tokio::test]
async fn test_delete_many() {
    let (store, _dir) = make_store().await;
    store.put("a", b"1", PutOptions::new()).await.unwrap();
    store.put("b", b"2", PutOptions::new()).await.unwrap();
    store.put("c", b"3", PutOptions::new()).await.unwrap();

    store.delete_many(&["a", "b"]).await.unwrap();

    assert!(!store.exists("a").await.unwrap());
    assert!(!store.exists("b").await.unwrap());
    assert!(store.exists("c").await.unwrap());
}

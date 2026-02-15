use super::*;
use crate::service::CacheService;
use crate::model::PutOptions;

/// These tests require a running MongoDB instance.
/// Set `MONGODB_TEST_URL` (defaults to `mongodb://localhost:27017`) and
/// `MONGODB_TEST_DB` (defaults to `harana_cache_test`).
///
/// Run with:
/// ```sh
/// cargo test -p harana-components-cache --features mongodb -- --ignored
/// ```

fn test_url() -> String {
    std::env::var("MONGODB_TEST_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string())
}

fn test_db() -> String {
    std::env::var("MONGODB_TEST_DB").unwrap_or_else(|_| "harana_cache_test".to_string())
}

async fn setup() -> MongoCacheService {
    let client_options = mongodb::options::ClientOptions::parse(&test_url())
        .await
        .expect("Failed to parse MongoDB URL");
    let client =
        mongodb::Client::with_options(client_options).expect("Failed to create MongoDB client");
    let db = client.database(&test_db());

    // Drop the collection to start fresh.
    db.collection::<Document>("cache")
        .drop()
        .await
        .ok();

    MongoCacheService::new(&db).await.expect("Failed to create MongoCacheService")
}

#[tokio::test]
#[ignore]
async fn test_put_and_get_text() {
    let cache = setup().await;
    cache.put("key1", "value1", PutOptions::new()).await.unwrap();
    let val = cache.get_text("key1").await.unwrap();
    assert_eq!(val, Some("value1".to_string()));
}

#[tokio::test]
#[ignore]
async fn test_put_overwrite() {
    let cache = setup().await;
    cache.put("key1", "v1", PutOptions::new()).await.unwrap();
    cache.put("key1", "v2", PutOptions::new()).await.unwrap();
    let val = cache.get_text("key1").await.unwrap();
    assert_eq!(val, Some("v2".to_string()));
}

#[tokio::test]
#[ignore]
async fn test_delete() {
    let cache = setup().await;
    cache.put("key1", "value1", PutOptions::new()).await.unwrap();
    cache.delete("key1").await.unwrap();
    let val = cache.get_text("key1").await.unwrap();
    assert_eq!(val, None);
}

#[tokio::test]
#[ignore]
async fn test_get_missing_key() {
    let cache = setup().await;
    let val = cache.get_text("nonexistent").await.unwrap();
    assert_eq!(val, None);
}

#[tokio::test]
#[ignore]
async fn test_ttl_expiry() {
    let cache = setup().await;
    // Use a TTL of 1 second.
    cache
        .put("ephemeral", "gone", PutOptions::new().with_ttl(1))
        .await
        .unwrap();

    // Should still be present immediately.
    let val = cache.get_text("ephemeral").await.unwrap();
    assert_eq!(val, Some("gone".to_string()));

    // Wait for the TTL to pass (our reads filter expired docs even before the
    // TTL monitor kicks in).
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    let val = cache.get_text("ephemeral").await.unwrap();
    assert_eq!(val, None);
}

#[tokio::test]
#[ignore]
async fn test_put_and_get_json() {
    let cache = setup().await;
    let data = serde_json::json!({"hello": "world", "num": 42});
    cache
        .put_json("json_key", &data, PutOptions::new())
        .await
        .unwrap();
    let val: Option<serde_json::Value> = cache.get_json("json_key").await.unwrap();
    assert_eq!(val, Some(data));
}

#[tokio::test]
#[ignore]
async fn test_put_and_get_bytes() {
    let cache = setup().await;
    let bytes = vec![0u8, 1, 2, 3, 255];
    cache
        .put_bytes("bin_key", &bytes, PutOptions::new())
        .await
        .unwrap();
    let val = cache.get_bytes("bin_key").await.unwrap();
    assert_eq!(val, Some(bytes));
}

#[tokio::test]
#[ignore]
async fn test_metadata() {
    let cache = setup().await;
    let meta = serde_json::json!({"author": "test"});
    let opts = PutOptions::new().with_metadata(meta.clone()).unwrap();
    cache.put("meta_key", "meta_val", opts).await.unwrap();

    let (val, m): (Option<String>, Option<serde_json::Value>) =
        cache.get_text_with_metadata("meta_key").await.unwrap();
    assert_eq!(val, Some("meta_val".to_string()));
    assert_eq!(m, Some(meta));
}

#[tokio::test]
#[ignore]
async fn test_list() {
    let cache = setup().await;
    cache.put("a/1", "v1", PutOptions::new()).await.unwrap();
    cache.put("a/2", "v2", PutOptions::new()).await.unwrap();
    cache.put("b/1", "v3", PutOptions::new()).await.unwrap();

    // List all.
    let resp = cache
        .list(ListOptions::new())
        .await
        .unwrap();
    assert_eq!(resp.keys.len(), 3);
    assert!(resp.list_complete);

    // List with prefix.
    let resp = cache
        .list(ListOptions::new().with_prefix("a/"))
        .await
        .unwrap();
    assert_eq!(resp.keys.len(), 2);
    assert!(resp.keys.iter().all(|k| k.name.starts_with("a/")));
}

#[tokio::test]
#[ignore]
async fn test_list_pagination() {
    let cache = setup().await;
    for i in 0..5 {
        cache
            .put(&format!("page/{i}"), &format!("v{i}"), PutOptions::new())
            .await
            .unwrap();
    }

    let resp = cache
        .list(ListOptions::new().with_limit(2))
        .await
        .unwrap();
    assert_eq!(resp.keys.len(), 2);
    assert!(!resp.list_complete);
    assert!(resp.cursor.is_some());

    let resp2 = cache
        .list(ListOptions::new().with_limit(2).with_cursor(resp.cursor.unwrap()))
        .await
        .unwrap();
    assert_eq!(resp2.keys.len(), 2);
}

#[tokio::test]
#[ignore]
async fn test_exists() {
    let cache = setup().await;
    assert!(!cache.exists("nope").await.unwrap());
    cache.put("yes", "1", PutOptions::new()).await.unwrap();
    assert!(cache.exists("yes").await.unwrap());
}

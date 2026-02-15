use super::*;
use crate::service::CacheService;
use crate::model::PutOptions;

#[tokio::test]
async fn test_put_and_get_text() {
    let cache = MemoryCacheService::new();
    cache.put("key1", "value1", PutOptions::new()).await.unwrap();
    let val = cache.get_text("key1").await.unwrap();
    assert_eq!(val, Some("value1".to_string()));
}

#[tokio::test]
async fn test_delete() {
    let cache = MemoryCacheService::new();
    cache.put("key1", "value1", PutOptions::new()).await.unwrap();
    cache.delete("key1").await.unwrap();
    let val = cache.get_text("key1").await.unwrap();
    assert_eq!(val, None);
}

#[tokio::test]
async fn test_ttl_expiry() {
    let cache = MemoryCacheService::new();
    // TTL of 0 seconds should expire immediately.
    cache
        .put("ephemeral", "gone", PutOptions::new().with_ttl(0))
        .await
        .unwrap();
    // Tiny sleep to ensure Instant moves forward.
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let val = cache.get_text("ephemeral").await.unwrap();
    assert_eq!(val, None);
}

#[tokio::test]
async fn test_put_and_get_json() {
    let cache = MemoryCacheService::new();
    let data = serde_json::json!({"hello": "world"});
    cache.put_json("json_key", &data, PutOptions::new()).await.unwrap();
    let val: Option<serde_json::Value> = cache.get_json("json_key").await.unwrap();
    assert_eq!(val, Some(data));
}

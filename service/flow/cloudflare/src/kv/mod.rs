// Harana Actions - Cloudflare KV Module
// This module provides Cloudflare Workers KV actions for key-value storage operations.

pub mod output;

use output::*;
use worker::Env;
use worker::kv::KvStore;

fn to_err(e: impl std::fmt::Display) -> String {
    format!("KV error: {e}")
}

fn get_kv(env: &Env, namespace: &str) -> Result<KvStore, String> {
    env.kv(namespace).map_err(to_err)
}

/// Get KV Value
#[allow(unused_variables)]
pub async fn get(
    env: &Env,
    namespace: &str,
    key: &str,
    value_type: Option<&str>,
    cache_ttl: Option<i32>,
) -> Result<GetOutput, String> {
    let kv = get_kv(env, namespace)?;

    match kv.get(key).text().await.map_err(to_err)? {
        Some(value) => {
            let json_value = serde_json::from_str(&value).unwrap_or(serde_json::Value::String(value));
            Ok(GetOutput {
                found: true,
                metadata: serde_json::Value::Null,
                value: json_value,
            })
        }
        None => Ok(GetOutput {
            found: false,
            metadata: serde_json::Value::Null,
            value: serde_json::Value::Null,
        }),
    }
}

/// Get KV Value With Metadata
#[allow(unused_variables)]
pub async fn get_with_metadata(
    env: &Env,
    namespace: &str,
    key: &str,
    value_type: Option<&str>,
    cache_ttl: Option<i32>,
) -> Result<GetWithMetadataOutput, String> {
    let kv = get_kv(env, namespace)?;

    match kv
        .get(key)
        .text_with_metadata::<serde_json::Value>()
        .await
        .map_err(to_err)?
    {
        (Some(value), metadata) => {
            let json_value = serde_json::from_str(&value).unwrap_or(serde_json::Value::String(value));
            Ok(GetWithMetadataOutput {
                found: true,
                metadata: metadata.unwrap_or(serde_json::Value::Null),
                value: json_value,
            })
        }
        (None, _) => Ok(GetWithMetadataOutput {
            found: false,
            metadata: serde_json::Value::Null,
            value: serde_json::Value::Null,
        }),
    }
}

/// Put KV Value
pub async fn put(
    env: &Env,
    namespace: &str,
    key: &str,
    value: serde_json::Value,
    expiration: Option<i32>,
    expiration_ttl: Option<i32>,
    metadata: Option<serde_json::Value>,
) -> Result<PutOutput, String> {
    let kv = get_kv(env, namespace)?;
    let value_str = serde_json::to_string(&value).map_err(|e| format!("KV serialization error: {e}"))?;

    let mut builder = kv.put(key, value_str).map_err(to_err)?;
    if let Some(exp) = expiration {
        builder = builder.expiration(exp as u64);
    }
    if let Some(ttl) = expiration_ttl {
        builder = builder.expiration_ttl(ttl as u64);
    }
    if let Some(meta) = metadata {
        builder = builder.metadata(meta).map_err(to_err)?;
    }
    builder.execute().await.map_err(to_err)?;

    Ok(PutOutput { success: true })
}

/// Put KV Value With Metadata
pub async fn put_with_metadata(
    env: &Env,
    namespace: &str,
    key: &str,
    value: serde_json::Value,
    metadata: serde_json::Value,
    expiration: Option<i32>,
    expiration_ttl: Option<i32>,
) -> Result<PutWithMetadataOutput, String> {
    let kv = get_kv(env, namespace)?;
    let value_str = serde_json::to_string(&value).map_err(|e| format!("KV serialization error: {e}"))?;

    let mut builder = kv.put(key, value_str).map_err(to_err)?;
    builder = builder.metadata(metadata).map_err(to_err)?;
    if let Some(exp) = expiration {
        builder = builder.expiration(exp as u64);
    }
    if let Some(ttl) = expiration_ttl {
        builder = builder.expiration_ttl(ttl as u64);
    }
    builder.execute().await.map_err(to_err)?;

    Ok(PutWithMetadataOutput { success: true })
}

/// Delete KV Key
pub async fn delete(env: &Env, namespace: &str, key: &str) -> Result<DeleteOutput, String> {
    let kv = get_kv(env, namespace)?;
    kv.delete(key).await.map_err(to_err)?;
    Ok(DeleteOutput { success: true })
}

/// List KV Keys
pub async fn list(
    env: &Env,
    namespace: &str,
    prefix: Option<&str>,
    cursor: Option<&str>,
    limit: Option<i32>,
) -> Result<ListOutput, String> {
    let kv = get_kv(env, namespace)?;
    let mut builder = kv.list();

    if let Some(p) = prefix {
        builder = builder.prefix(p.to_string());
    }
    if let Some(c) = cursor {
        builder = builder.cursor(c.to_string());
    }
    if let Some(l) = limit {
        builder = builder.limit(l as u64);
    }

    let response = builder.execute().await.map_err(to_err)?;

    let keys = response
        .keys
        .into_iter()
        .map(|k| KvKey {
            name: k.name,
            expiration: k.expiration.map(|e| e as i64),
            metadata: k.metadata,
        })
        .collect();

    Ok(ListOutput {
        cursor: response.cursor.unwrap_or_default(),
        keys,
        list_complete: response.list_complete,
    })
}

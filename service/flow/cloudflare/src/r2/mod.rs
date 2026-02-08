// Harana Actions - Cloudflare R2 Module
// This module provides Cloudflare R2 object storage actions for managing
// buckets, objects, and multipart uploads.

pub mod output;

use output::*;
use std::collections::HashMap;
use worker::Env;

fn to_err(e: worker::Error) -> String {
    format!("R2 error: {e}")
}

fn bucket(env: &Env, name: &str) -> Result<worker::Bucket, String> {
    env.bucket(name).map_err(to_err)
}

fn object_to_http_metadata(http: &worker::HttpMetadata) -> R2HttpMetadata {
    R2HttpMetadata {
        content_type: http.content_type.clone(),
        content_language: http.content_language.clone(),
        content_disposition: http.content_disposition.clone(),
        content_encoding: http.content_encoding.clone(),
        cache_control: http.cache_control.clone(),
        cache_expiry: http.cache_expiry.as_ref().map(|d| d.to_string()),
    }
}

fn worker_http_metadata(meta: &R2HttpMetadata) -> worker::HttpMetadata {
    let mut http = worker::HttpMetadata::default();
    http.content_type = meta.content_type.clone();
    http.content_language = meta.content_language.clone();
    http.content_disposition = meta.content_disposition.clone();
    http.content_encoding = meta.content_encoding.clone();
    http.cache_control = meta.cache_control.clone();
    http
}

fn date_to_string(d: &worker::Date) -> String {
    d.to_string()
}

/// Get R2 Object
pub async fn get(
    env: &Env,
    bucket_name: &str,
    key: &str,
    _only_if: Option<R2Conditional>,
    _range: Option<R2Range>,
) -> Result<GetOutput, String> {
    let b = bucket(env, bucket_name)?;
    let obj = b
        .get(key)
        .execute()
        .await
        .map_err(to_err)?
        .ok_or_else(|| format!("R2 object '{key}' not found"))?;

    let key_str = obj.key();
    let version = obj.version();
    let size = obj.size() as i32;
    let etag = obj.etag();
    let http_etag = obj.http_etag();
    let uploaded = date_to_string(&obj.uploaded());
    let http_metadata = object_to_http_metadata(&obj.http_metadata());
    let custom_metadata = serde_json::to_value(obj.custom_metadata().map_err(to_err)?)
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

    let body = match obj.body() {
        Some(body) => body.bytes().await.map_err(to_err)?,
        None => vec![],
    };

    Ok(GetOutput {
        body,
        custom_metadata,
        etag,
        http_etag,
        http_metadata,
        key: key_str,
        size,
        uploaded,
        version,
    })
}

/// Head R2 Object
pub async fn head(env: &Env, bucket_name: &str, key: &str) -> Result<HeadOutput, String> {
    let b = bucket(env, bucket_name)?;
    let obj = b
        .head(key)
        .await
        .map_err(to_err)?
        .ok_or_else(|| format!("R2 object '{key}' not found"))?;

    Ok(HeadOutput {
        custom_metadata: serde_json::to_value(obj.custom_metadata().map_err(to_err)?)
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
        etag: obj.etag(),
        http_etag: obj.http_etag(),
        http_metadata: object_to_http_metadata(&obj.http_metadata()),
        key: obj.key(),
        size: obj.size() as i32,
        uploaded: date_to_string(&obj.uploaded()),
        version: obj.version(),
    })
}

/// Put R2 Object
#[allow(clippy::too_many_arguments)]
pub async fn put(
    env: &Env,
    bucket_name: &str,
    key: &str,
    value: Vec<u8>,
    custom_metadata: Option<serde_json::Value>,
    http_metadata: Option<R2HttpMetadata>,
    _md5: Option<&str>,
    _only_if: Option<R2Conditional>,
    _sha1: Option<&str>,
    _sha256: Option<&str>,
    _sha384: Option<&str>,
    _sha512: Option<&str>,
    _storage_class: Option<&str>,
) -> Result<PutOutput, String> {
    let b = bucket(env, bucket_name)?;
    let mut builder = b.put(key, worker::Data::Bytes(value));

    if let Some(http) = http_metadata {
        builder = builder.http_metadata(worker_http_metadata(&http));
    }
    if let Some(custom) = custom_metadata {
        if let Some(obj) = custom.as_object() {
            let map: HashMap<String, String> = obj
                .iter()
                .map(|(k, v)| {
                    let val = match v {
                        serde_json::Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    };
                    (k.clone(), val)
                })
                .collect();
            builder = builder.custom_metadata(map);
        }
    }

    let obj = builder.execute().await.map_err(to_err)?;

    Ok(PutOutput {
        etag: obj.etag(),
        key: obj.key(),
        size: obj.size() as i32,
        success: true,
        uploaded: date_to_string(&obj.uploaded()),
        version: obj.version(),
    })
}

/// Delete R2 Object
pub async fn delete(env: &Env, bucket_name: &str, key: &str) -> Result<DeleteOutput, String> {
    let b = bucket(env, bucket_name)?;
    b.delete(key).await.map_err(to_err)?;
    Ok(DeleteOutput { success: true })
}

/// Delete R2 Objects
pub async fn delete_many(env: &Env, bucket_name: &str, keys: Vec<String>) -> Result<DeleteManyOutput, String> {
    let b = bucket(env, bucket_name)?;
    for key in &keys {
        b.delete(key).await.map_err(to_err)?;
    }
    Ok(DeleteManyOutput { success: true })
}

/// List R2 Objects
#[allow(unused_variables)]
pub async fn list(
    env: &Env,
    bucket_name: &str,
    prefix: Option<&str>,
    cursor: Option<&str>,
    delimiter: Option<&str>,
    include: Option<Vec<String>>,
    limit: Option<i32>,
    start_after: Option<&str>,
) -> Result<ListOutput, String> {
    let b = bucket(env, bucket_name)?;
    let mut builder = b.list();

    if let Some(p) = prefix {
        builder = builder.prefix(p);
    }
    if let Some(c) = cursor {
        builder = builder.cursor(c);
    }
    if let Some(d) = delimiter {
        builder = builder.delimiter(d);
    }
    if let Some(l) = limit {
        builder = builder.limit(l as u32);
    }

    let objects_result = builder.execute().await.map_err(to_err)?;

    let objects = objects_result
        .objects()
        .into_iter()
        .map(|obj| R2Object {
            key: obj.key(),
            size: obj.size() as i32,
            etag: obj.etag(),
            http_etag: obj.http_etag(),
            uploaded: date_to_string(&obj.uploaded()),
            version: obj.version(),
            custom_metadata: Some(
                serde_json::to_value(obj.custom_metadata().unwrap_or_default())
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
            ),
            http_metadata: Some(object_to_http_metadata(&obj.http_metadata())),
        })
        .collect();

    Ok(ListOutput {
        cursor: objects_result.cursor().unwrap_or_default(),
        delimited_prefixes: objects_result.delimited_prefixes(),
        objects,
        truncated: objects_result.truncated(),
    })
}

/// Create R2 Multipart Upload
pub async fn create_multipart_upload(
    env: &Env,
    bucket_name: &str,
    key: &str,
    custom_metadata: Option<serde_json::Value>,
    http_metadata: Option<R2HttpMetadata>,
    _storage_class: Option<&str>,
) -> Result<CreateMultipartUploadOutput, String> {
    let b = bucket(env, bucket_name)?;
    let mut builder = b.create_multipart_upload(key);

    if let Some(http) = http_metadata {
        builder = builder.http_metadata(worker_http_metadata(&http));
    }
    if let Some(custom) = custom_metadata {
        if let Some(obj) = custom.as_object() {
            let map: HashMap<String, String> = obj
                .iter()
                .map(|(k, v)| {
                    let val = match v {
                        serde_json::Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    };
                    (k.clone(), val)
                })
                .collect();
            builder = builder.custom_metadata(map);
        }
    }

    let upload = builder.execute().await.map_err(to_err)?;

    Ok(CreateMultipartUploadOutput {
        key: key.to_string(),
        upload_id: upload.upload_id().await,
    })
}

/// Upload R2 Multipart Part
pub async fn upload_part(
    env: &Env,
    bucket_name: &str,
    key: &str,
    upload_id: &str,
    part_number: i32,
    value: Vec<u8>,
) -> Result<UploadPartOutput, String> {
    let b = bucket(env, bucket_name)?;
    let upload = b.resume_multipart_upload(key, upload_id).map_err(to_err)?;

    let part = upload.upload_part(part_number as u16, value).await.map_err(to_err)?;

    Ok(UploadPartOutput {
        etag: part.etag(),
        part_number: part.part_number() as i32,
    })
}

/// Complete R2 Multipart Upload
pub async fn complete_multipart_upload(
    env: &Env,
    bucket_name: &str,
    key: &str,
    upload_id: &str,
    parts: Vec<R2UploadedPart>,
) -> Result<CompleteMultipartUploadOutput, String> {
    let b = bucket(env, bucket_name)?;
    let upload = b.resume_multipart_upload(key, upload_id).map_err(to_err)?;

    let worker_parts: Vec<worker::UploadedPart> = parts
        .iter()
        .map(|p| worker::UploadedPart::new(p.part_number as u16, p.etag.clone()))
        .collect();

    let obj = upload.complete(worker_parts).await.map_err(to_err)?;

    Ok(CompleteMultipartUploadOutput {
        etag: obj.etag(),
        key: obj.key(),
        size: obj.size() as i32,
        success: true,
        uploaded: date_to_string(&obj.uploaded()),
        version: obj.version(),
    })
}

/// Abort R2 Multipart Upload
pub async fn abort_multipart_upload(
    env: &Env,
    bucket_name: &str,
    key: &str,
    upload_id: &str,
) -> Result<AbortMultipartUploadOutput, String> {
    let b = bucket(env, bucket_name)?;
    let upload = b.resume_multipart_upload(key, upload_id).map_err(to_err)?;

    upload.abort().await.map_err(to_err)?;

    Ok(AbortMultipartUploadOutput { success: true })
}

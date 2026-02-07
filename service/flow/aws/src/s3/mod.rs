// Harana Actions - AWS S3 Module
// This module provides AWS Simple Storage Service (S3) actions for managing
// buckets, objects, and related configurations.

pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client,
    config::Region,
    presigning::PresigningConfig,
    primitives::ByteStream,
    types::{
        BucketLocationConstraint, BucketVersioningStatus, CompletedMultipartUpload, CompletedPart, CorsRule,
        CreateBucketConfiguration, Delete, LifecycleRule, ObjectIdentifier, ServerSideEncryption,
        ServerSideEncryptionByDefault, ServerSideEncryptionConfiguration, ServerSideEncryptionRule, Tag, Tagging,
    },
};
use output::*;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// Creates an S3 client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let s3_config = if let Some(region_str) = region {
        aws_sdk_s3::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_s3::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(s3_config))
}

/// Converts a HashMap of tags to AWS SDK Tag format
fn tags_to_aws_tags(tags: &HashMap<String, Value>) -> Vec<Tag> {
    tags.iter()
        .map(|(k, v)| {
            Tag::builder()
                .key(k)
                .value(v.as_str().unwrap_or(&v.to_string()))
                .build()
                .expect("Failed to build tag")
        })
        .collect()
}

/// Converts a HashMap of metadata to AWS-compatible metadata
fn metadata_to_aws_metadata(metadata: &HashMap<String, Value>) -> HashMap<String, String> {
    metadata
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or(&v.to_string()).to_string()))
        .collect()
}

/// Abort Multipart Upload
///
/// Aborts a multipart upload. After a multipart upload is aborted, no additional
/// parts can be uploaded using that upload ID.
///
pub async fn abort_multipart_upload(
    bucket: &str,
    upload_id: &str,
    key: &str,
    region: Option<&str>,
) -> Result<AbortMultipartUploadOutput, String> {
    let client = create_client(region).await?;

    client
        .abort_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .send()
        .await
        .map_err(|e| format!("Failed to abort multipart upload: {}", e))?;

    Ok(AbortMultipartUploadOutput { success: true })
}

/// Complete Multipart Upload
///
/// Completes a multipart upload by assembling previously uploaded parts.
///
pub async fn complete_multipart_upload(
    parts: Vec<HashMap<String, Value>>,
    bucket: &str,
    upload_id: &str,
    key: &str,
    region: Option<&str>,
) -> Result<CompleteMultipartUploadOutput, String> {
    let client = create_client(region).await?;

    let completed_parts: Vec<CompletedPart> = parts
        .iter()
        .filter_map(|p| {
            let part_number = p.get("part_number")?.as_i64()? as i32;
            let etag = p.get("etag")?.as_str()?.to_string();
            Some(CompletedPart::builder().part_number(part_number).e_tag(etag).build())
        })
        .collect();

    let completed_upload = CompletedMultipartUpload::builder()
        .set_parts(Some(completed_parts))
        .build();

    let response = client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .multipart_upload(completed_upload)
        .send()
        .await
        .map_err(|e| format!("Failed to complete multipart upload: {}", e))?;

    Ok(CompleteMultipartUploadOutput {
        success: true,
        key: response.key().unwrap_or_default().to_string(),
        bucket: response.bucket().unwrap_or_default().to_string(),
        etag: response.e_tag().unwrap_or_default().to_string(),
        location: response.location().unwrap_or_default().to_string(),
    })
}

/// Copy S3 Object
///
/// Creates a copy of an object that is already stored in Amazon S3.
///
#[allow(clippy::too_many_arguments)]
pub async fn copy_object(
    source_bucket: &str,
    source_key: &str,
    destination_bucket: &str,
    destination_key: &str,
    metadata_directive: Option<&str>,
    source_version_id: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    storage_class: Option<&str>,
    metadata: Option<HashMap<String, Value>>,
    region: Option<&str>,
) -> Result<CopyObjectOutput, String> {
    let client = create_client(region).await?;

    let copy_source = if let Some(version) = source_version_id {
        format!("{}/{}?versionId={}", source_bucket, source_key, version)
    } else {
        format!("{}/{}", source_bucket, source_key)
    };

    let mut request = client
        .copy_object()
        .bucket(destination_bucket)
        .key(destination_key)
        .copy_source(&copy_source);

    if let Some(directive) = metadata_directive {
        request = request.metadata_directive(directive.parse().map_err(|_| "Invalid metadata directive")?);
    }

    if let Some(class) = storage_class {
        request = request.storage_class(class.parse().map_err(|_| "Invalid storage class")?);
    }

    if let Some(ref meta) = metadata {
        request = request.set_metadata(Some(metadata_to_aws_metadata(meta)));
    }

    if let Some(ref tags_data) = tags {
        let tag_string = tags_data
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or(&v.to_string())))
            .collect::<Vec<_>>()
            .join("&");
        request = request.tagging(tag_string);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to copy object: {}", e))?;

    let copy_result = response.copy_object_result();

    Ok(CopyObjectOutput {
        success: true,
        etag: copy_result
            .map(|r| r.e_tag().unwrap_or_default().to_string())
            .unwrap_or_default(),
        version_id: response.version_id().unwrap_or_default().to_string(),
    })
}

/// Create S3 Bucket
///
/// Creates a new S3 bucket.
///
pub async fn create_bucket(
    bucket: &str,
    region: Option<&str>,
    acl: Option<&str>,
    tags: Option<HashMap<String, Value>>,
) -> Result<CreateBucketOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_bucket().bucket(bucket);

    // Only set location constraint if region is not us-east-1 (it's the default)
    if let Some(region_str) = region {
        if region_str != "us-east-1" {
            let constraint = BucketLocationConstraint::from(region_str);
            let config = CreateBucketConfiguration::builder()
                .location_constraint(constraint)
                .build();
            request = request.create_bucket_configuration(config);
        }
    }

    if let Some(acl_str) = acl {
        request = request.acl(acl_str.parse().map_err(|_| "Invalid ACL")?);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create bucket: {}", e))?;

    // Apply tags if provided
    if let Some(ref tags_data) = tags {
        let tagging = Tagging::builder()
            .set_tag_set(Some(tags_to_aws_tags(tags_data)))
            .build()
            .map_err(|e| format!("Failed to build tagging: {}", e))?;

        client
            .put_bucket_tagging()
            .bucket(bucket)
            .tagging(tagging)
            .send()
            .await
            .map_err(|e| format!("Failed to set bucket tags: {}", e))?;
    }

    Ok(CreateBucketOutput {
        success: true,
        bucket: bucket.to_string(),
        location: response.location().unwrap_or_default().to_string(),
    })
}

/// Create Multipart Upload
///
/// Initiates a multipart upload and returns an upload ID.
///
pub async fn create_multipart_upload(
    bucket: &str,
    key: &str,
    metadata: Option<HashMap<String, Value>>,
    content_type: Option<&str>,
    storage_class: Option<&str>,
    region: Option<&str>,
) -> Result<CreateMultipartUploadOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.create_multipart_upload().bucket(bucket).key(key);

    if let Some(ct) = content_type {
        request = request.content_type(ct);
    }

    if let Some(class) = storage_class {
        request = request.storage_class(class.parse().map_err(|_| "Invalid storage class")?);
    }

    if let Some(ref meta) = metadata {
        request = request.set_metadata(Some(metadata_to_aws_metadata(meta)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create multipart upload: {}", e))?;

    Ok(CreateMultipartUploadOutput {
        success: true,
        upload_id: response.upload_id().unwrap_or_default().to_string(),
    })
}

/// Delete S3 Bucket
///
/// Deletes the specified bucket. The bucket must be empty.
///
pub async fn delete_bucket(bucket: &str, region: Option<&str>) -> Result<DeleteBucketOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_bucket()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to delete bucket: {}", e))?;

    Ok(DeleteBucketOutput { success: true })
}

/// Delete S3 Bucket CORS
///
/// Deletes the CORS configuration information for the bucket.
///
pub async fn delete_bucket_cors(bucket: &str, region: Option<&str>) -> Result<DeleteBucketCorsOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_bucket_cors()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to delete bucket CORS: {}", e))?;

    Ok(DeleteBucketCorsOutput { success: true })
}

/// Delete S3 Bucket Policy
///
/// Deletes the policy from the bucket.
///
pub async fn delete_bucket_policy(bucket: &str, region: Option<&str>) -> Result<DeleteBucketPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_bucket_policy()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to delete bucket policy: {}", e))?;

    Ok(DeleteBucketPolicyOutput { success: true })
}

/// Delete S3 Object
///
/// Removes an object from a bucket.
///
pub async fn delete_object(
    key: &str,
    bucket: &str,
    region: Option<&str>,
    version_id: Option<&str>,
) -> Result<DeleteObjectOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.delete_object().bucket(bucket).key(key);

    if let Some(version) = version_id {
        request = request.version_id(version);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to delete object: {}", e))?;

    Ok(DeleteObjectOutput {
        success: true,
        version_id: response.version_id().unwrap_or_default().to_string(),
    })
}

/// Delete S3 Objects
///
/// Removes multiple objects from a bucket.
///
pub async fn delete_objects(
    keys: Vec<String>,
    bucket: &str,
    region: Option<&str>,
) -> Result<DeleteObjectsOutput, String> {
    let client = create_client(region).await?;

    let objects: Vec<ObjectIdentifier> = keys
        .iter()
        .map(|k| ObjectIdentifier::builder().key(k).build().unwrap())
        .collect();

    let delete = Delete::builder()
        .set_objects(Some(objects))
        .build()
        .map_err(|e| format!("Failed to build delete request: {}", e))?;

    let response = client
        .delete_objects()
        .bucket(bucket)
        .delete(delete)
        .send()
        .await
        .map_err(|e| format!("Failed to delete objects: {}", e))?;

    let deleted: Vec<HashMap<String, Value>> = response
        .deleted()
        .iter()
        .map(|d| {
            let mut map = HashMap::new();
            map.insert(
                "key".to_string(),
                Value::String(d.key().unwrap_or_default().to_string()),
            );
            if let Some(version) = d.version_id() {
                map.insert("version_id".to_string(), Value::String(version.to_string()));
            }
            map
        })
        .collect();

    let errors: Vec<HashMap<String, Value>> = response
        .errors()
        .iter()
        .map(|e| {
            let mut map = HashMap::new();
            map.insert(
                "key".to_string(),
                Value::String(e.key().unwrap_or_default().to_string()),
            );
            map.insert(
                "code".to_string(),
                Value::String(e.code().unwrap_or_default().to_string()),
            );
            map.insert(
                "message".to_string(),
                Value::String(e.message().unwrap_or_default().to_string()),
            );
            map
        })
        .collect();

    Ok(DeleteObjectsOutput {
        success: errors.is_empty(),
        deleted,
        errors,
    })
}

/// Get S3 Bucket CORS
///
/// Returns the CORS configuration information set for the bucket.
///
pub async fn get_bucket_cors(bucket: &str, region: Option<&str>) -> Result<GetBucketCorsOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_bucket_cors()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to get bucket CORS: {}", e))?;

    let cors_rules: Vec<HashMap<String, Value>> = response
        .cors_rules()
        .iter()
        .map(|rule| {
            let mut map = HashMap::new();
            map.insert(
                "allowed_headers".to_string(),
                Value::Array(
                    rule.allowed_headers()
                        .iter()
                        .map(|h| Value::String(h.to_string()))
                        .collect(),
                ),
            );
            map.insert(
                "allowed_methods".to_string(),
                Value::Array(
                    rule.allowed_methods()
                        .iter()
                        .map(|m| Value::String(m.to_string()))
                        .collect(),
                ),
            );
            map.insert(
                "allowed_origins".to_string(),
                Value::Array(
                    rule.allowed_origins()
                        .iter()
                        .map(|o| Value::String(o.to_string()))
                        .collect(),
                ),
            );
            map.insert(
                "expose_headers".to_string(),
                Value::Array(
                    rule.expose_headers()
                        .iter()
                        .map(|h| Value::String(h.to_string()))
                        .collect(),
                ),
            );
            if let Some(max_age) = rule.max_age_seconds() {
                map.insert("max_age_seconds".to_string(), Value::Number(max_age.into()));
            }
            map
        })
        .collect();

    Ok(GetBucketCorsOutput { cors_rules })
}

/// Get S3 Bucket Encryption
///
/// Returns the default encryption configuration for an Amazon S3 bucket.
///
pub async fn get_bucket_encryption(bucket: &str, region: Option<&str>) -> Result<GetBucketEncryptionOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_bucket_encryption()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to get bucket encryption: {}", e))?;

    let rules: Vec<HashMap<String, Value>> = response
        .server_side_encryption_configuration()
        .map(|config| {
            config
                .rules()
                .iter()
                .map(|rule| {
                    let mut map = HashMap::new();
                    if let Some(default) = rule.apply_server_side_encryption_by_default() {
                        map.insert(
                            "sse_algorithm".to_string(),
                            Value::String(default.sse_algorithm().as_str().to_string()),
                        );
                        if let Some(key_id) = default.kms_master_key_id() {
                            map.insert("kms_master_key_id".to_string(), Value::String(key_id.to_string()));
                        }
                    }
                    if let Some(bucket_key) = rule.bucket_key_enabled() {
                        map.insert("bucket_key_enabled".to_string(), Value::Bool(bucket_key));
                    }
                    map
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(GetBucketEncryptionOutput { rules })
}

/// Get S3 Bucket Lifecycle
///
/// Returns the lifecycle configuration information set on the bucket.
///
pub async fn get_bucket_lifecycle(bucket: &str, region: Option<&str>) -> Result<GetBucketLifecycleOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_bucket_lifecycle_configuration()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to get bucket lifecycle: {}", e))?;

    let rules: Vec<HashMap<String, Value>> = response
        .rules()
        .iter()
        .map(|rule| {
            let mut map = HashMap::new();
            if let Some(id) = rule.id() {
                map.insert("id".to_string(), Value::String(id.to_string()));
            }
            map.insert("status".to_string(), Value::String(rule.status().as_str().to_string()));
            if let Some(expiration) = rule.expiration() {
                let mut exp_map = serde_json::Map::new();
                if let Some(days) = expiration.days() {
                    exp_map.insert("days".to_string(), Value::Number(days.into()));
                }
                if let Some(date) = expiration.date() {
                    exp_map.insert(
                        "date".to_string(),
                        Value::String(
                            chrono::DateTime::from_timestamp(date.secs(), date.subsec_nanos())
                                .map(|dt| dt.to_rfc3339())
                                .unwrap_or_default(),
                        ),
                    );
                }
                map.insert("expiration".to_string(), Value::Object(exp_map));
            }
            map
        })
        .collect();

    Ok(GetBucketLifecycleOutput { rules })
}

/// Get S3 Bucket Policy
///
/// Returns the policy of a specified bucket.
///
pub async fn get_bucket_policy(bucket: &str, region: Option<&str>) -> Result<GetBucketPolicyOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_bucket_policy()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to get bucket policy: {}", e))?;

    Ok(GetBucketPolicyOutput {
        policy: response.policy().unwrap_or_default().to_string(),
    })
}

/// Get S3 Bucket Versioning
///
/// Returns the versioning state of a bucket.
///
pub async fn get_bucket_versioning(bucket: &str, region: Option<&str>) -> Result<GetBucketVersioningOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_bucket_versioning()
        .bucket(bucket)
        .send()
        .await
        .map_err(|e| format!("Failed to get bucket versioning: {}", e))?;

    Ok(GetBucketVersioningOutput {
        status: response.status().map(|s| s.as_str().to_string()).unwrap_or_default(),
        mfa_delete: response
            .mfa_delete()
            .map(|m| m.as_str().to_string())
            .unwrap_or_default(),
    })
}

/// Get S3 Object
///
/// Retrieves an object from Amazon S3.
///
pub async fn get_object(
    key: &str,
    bucket: &str,
    region: Option<&str>,
    version_id: Option<&str>,
) -> Result<GetObjectOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_object().bucket(bucket).key(key);

    if let Some(version) = version_id {
        request = request.version_id(version);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get object: {}", e))?;

    // Extract metadata before consuming the body
    let content_type = response.content_type().unwrap_or_default().to_string();
    let etag = response.e_tag().unwrap_or_default().to_string();
    let last_modified = response
        .last_modified()
        .map(|d| {
            chrono::DateTime::from_timestamp(d.secs(), d.subsec_nanos())
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default()
        })
        .unwrap_or_default();
    let metadata: HashMap<String, Value> = response
        .metadata()
        .map(|m| m.iter().map(|(k, v)| (k.clone(), Value::String(v.clone()))).collect())
        .unwrap_or_default();
    let version_id_out = response.version_id().unwrap_or_default().to_string();
    let storage_class = response
        .storage_class()
        .map(|s| s.as_str().to_string())
        .unwrap_or_default();
    let size = response.content_length().unwrap_or(0) as i32;

    // Now consume the body
    let content = response
        .body
        .collect()
        .await
        .map_err(|e| format!("Failed to read object body: {}", e))?
        .into_bytes()
        .to_vec();

    Ok(GetObjectOutput {
        content,
        content_type,
        etag,
        last_modified,
        metadata,
        version_id: version_id_out,
        storage_class,
        size,
    })
}

/// Get S3 Object Tagging
///
/// Returns the tag-set of an object.
///
pub async fn get_object_tagging(
    key: &str,
    bucket: &str,
    version_id: Option<&str>,
    region: Option<&str>,
) -> Result<GetObjectTaggingOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_object_tagging().bucket(bucket).key(key);

    if let Some(version) = version_id {
        request = request.version_id(version);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get object tagging: {}", e))?;

    let tags: HashMap<String, Value> = response
        .tag_set()
        .iter()
        .map(|t| (t.key().to_string(), Value::String(t.value().to_string())))
        .collect();

    Ok(GetObjectTaggingOutput {
        tags,
        version_id: response.version_id().unwrap_or_default().to_string(),
    })
}

/// Get S3 Object Presigned URL
///
/// Generates a presigned URL for the specified S3 object.
///
pub async fn get_presigned_url(
    bucket: &str,
    key: &str,
    operation: Option<&str>,
    expires_in: Option<i32>,
    region: Option<&str>,
) -> Result<GetPresignedUrlOutput, String> {
    let client = create_client(region).await?;

    let expiration_secs = expires_in.unwrap_or(3600) as u64;
    let presigning_config = PresigningConfig::expires_in(Duration::from_secs(expiration_secs))
        .map_err(|e| format!("Failed to create presigning config: {}", e))?;

    let op = operation.unwrap_or("get");

    let presigned_request = match op.to_lowercase().as_str() {
        "put" => client
            .put_object()
            .bucket(bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| format!("Failed to generate presigned URL: {}", e))?,
        _ => client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| format!("Failed to generate presigned URL: {}", e))?,
    };

    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expiration_secs as i64);

    Ok(GetPresignedUrlOutput {
        url: presigned_request.uri().to_string(),
        expires_at: expires_at.to_rfc3339(),
    })
}

/// Head S3 Object
///
/// Retrieves metadata from an object without returning the object itself.
///
pub async fn head_object(
    key: &str,
    bucket: &str,
    region: Option<&str>,
    version_id: Option<&str>,
) -> Result<HeadObjectOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.head_object().bucket(bucket).key(key);

    if let Some(version) = version_id {
        request = request.version_id(version);
    }

    let response = request.send().await;

    match response {
        Ok(resp) => {
            let metadata: HashMap<String, Value> = resp
                .metadata()
                .map(|m| m.iter().map(|(k, v)| (k.clone(), Value::String(v.clone()))).collect())
                .unwrap_or_default();

            Ok(HeadObjectOutput {
                exists: true,
                content_length: resp.content_length().unwrap_or(0) as i32,
                content_type: resp.content_type().unwrap_or_default().to_string(),
                etag: resp.e_tag().unwrap_or_default().to_string(),
                last_modified: resp
                    .last_modified()
                    .map(|d| {
                        chrono::DateTime::from_timestamp(d.secs(), d.subsec_nanos())
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default()
                    })
                    .unwrap_or_default(),
                metadata,
                storage_class: resp.storage_class().map(|s| s.as_str().to_string()).unwrap_or_default(),
                version_id: resp.version_id().unwrap_or_default().to_string(),
            })
        }
        Err(_) => Ok(HeadObjectOutput {
            exists: false,
            content_length: 0,
            content_type: String::new(),
            etag: String::new(),
            last_modified: String::new(),
            metadata: HashMap::new(),
            storage_class: String::new(),
            version_id: String::new(),
        }),
    }
}

/// List S3 Buckets
///
/// Returns a list of all buckets owned by the authenticated sender of the request.
///
pub async fn list_buckets(region: Option<&str>) -> Result<ListBucketsOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .list_buckets()
        .send()
        .await
        .map_err(|e| format!("Failed to list buckets: {}", e))?;

    let buckets: Vec<HashMap<String, Value>> = response
        .buckets()
        .iter()
        .map(|b| {
            let mut map = HashMap::new();
            map.insert(
                "name".to_string(),
                Value::String(b.name().unwrap_or_default().to_string()),
            );
            if let Some(date) = b.creation_date() {
                map.insert(
                    "creation_date".to_string(),
                    Value::String(
                        chrono::DateTime::from_timestamp(date.secs(), date.subsec_nanos())
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    ),
                );
            }
            map
        })
        .collect();

    Ok(ListBucketsOutput { buckets })
}

/// List S3 Objects
///
/// Returns some or all of the objects in a bucket.
///
pub async fn list_objects(
    bucket: &str,
    region: Option<&str>,
    prefix: Option<&str>,
    max_keys: Option<i32>,
    continuation_token: Option<&str>,
    delimiter: Option<&str>,
    start_after: Option<&str>,
) -> Result<ListObjectsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_objects_v2().bucket(bucket);

    if let Some(p) = prefix {
        request = request.prefix(p);
    }

    if let Some(max) = max_keys {
        request = request.max_keys(max);
    }

    if let Some(token) = continuation_token {
        request = request.continuation_token(token);
    }

    if let Some(d) = delimiter {
        request = request.delimiter(d);
    }

    if let Some(start) = start_after {
        request = request.start_after(start);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list objects: {}", e))?;

    let contents: Vec<HashMap<String, Value>> = response
        .contents()
        .iter()
        .map(|obj| {
            let mut map = HashMap::new();
            map.insert(
                "key".to_string(),
                Value::String(obj.key().unwrap_or_default().to_string()),
            );
            if let Some(date) = obj.last_modified() {
                map.insert(
                    "last_modified".to_string(),
                    Value::String(
                        chrono::DateTime::from_timestamp(date.secs(), date.subsec_nanos())
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    ),
                );
            }
            if let Some(etag) = obj.e_tag() {
                map.insert("etag".to_string(), Value::String(etag.to_string()));
            }
            if let Some(size) = obj.size() {
                map.insert("size".to_string(), Value::Number(size.into()));
            }
            if let Some(class) = obj.storage_class() {
                map.insert("storage_class".to_string(), Value::String(class.as_str().to_string()));
            }
            map
        })
        .collect();

    Ok(ListObjectsOutput {
        contents,
        is_truncated: response.is_truncated().unwrap_or(false),
        key_count: response.key_count().unwrap_or(0),
        prefix: response.prefix().unwrap_or_default().to_string(),
        continuation_token: response.continuation_token().unwrap_or_default().to_string(),
        next_continuation_token: response.next_continuation_token().unwrap_or_default().to_string(),
    })
}

/// Put S3 Bucket CORS
///
/// Sets the CORS configuration for a bucket.
///
pub async fn put_bucket_cors(
    bucket: &str,
    cors_rules: Vec<HashMap<String, Value>>,
    region: Option<&str>,
) -> Result<PutBucketCorsOutput, String> {
    let client = create_client(region).await?;

    let rules: Vec<CorsRule> = cors_rules
        .iter()
        .map(|rule| {
            let allowed_headers: Vec<String> = rule
                .get("allowed_headers")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            let allowed_methods: Vec<String> = rule
                .get("allowed_methods")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            let allowed_origins: Vec<String> = rule
                .get("allowed_origins")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            let expose_headers: Vec<String> = rule
                .get("expose_headers")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();

            let max_age_seconds = rule.get("max_age_seconds").and_then(|v| v.as_i64()).map(|v| v as i32);

            let mut builder = CorsRule::builder()
                .set_allowed_headers(if allowed_headers.is_empty() {
                    None
                } else {
                    Some(allowed_headers)
                })
                .set_allowed_methods(Some(allowed_methods))
                .set_allowed_origins(Some(allowed_origins))
                .set_expose_headers(if expose_headers.is_empty() {
                    None
                } else {
                    Some(expose_headers)
                });

            if let Some(max_age) = max_age_seconds {
                builder = builder.max_age_seconds(max_age);
            }

            builder.build().expect("Failed to build CORS rule")
        })
        .collect();

    let cors_config = aws_sdk_s3::types::CorsConfiguration::builder()
        .set_cors_rules(Some(rules))
        .build()
        .map_err(|e| format!("Failed to build CORS configuration: {}", e))?;

    client
        .put_bucket_cors()
        .bucket(bucket)
        .cors_configuration(cors_config)
        .send()
        .await
        .map_err(|e| format!("Failed to put bucket CORS: {}", e))?;

    Ok(PutBucketCorsOutput { success: true })
}

/// Put S3 Bucket Encryption
///
/// Configures default encryption for a bucket using AES-256 or KMS.
///
pub async fn put_bucket_encryption(
    sse_algorithm: &str,
    bucket: &str,
    kms_master_key_id: Option<&str>,
    region: Option<&str>,
) -> Result<PutBucketEncryptionOutput, String> {
    let client = create_client(region).await?;

    let algorithm = match sse_algorithm.to_uppercase().as_str() {
        "AES256" => ServerSideEncryption::Aes256,
        "AWS:KMS" | "KMS" => ServerSideEncryption::AwsKms,
        _ => return Err(format!("Invalid SSE algorithm: {}", sse_algorithm)),
    };

    let mut default_builder = ServerSideEncryptionByDefault::builder().sse_algorithm(algorithm);

    if let Some(key_id) = kms_master_key_id {
        default_builder = default_builder.kms_master_key_id(key_id);
    }

    let default_encryption = default_builder
        .build()
        .map_err(|e| format!("Failed to build encryption default: {}", e))?;

    let rule = ServerSideEncryptionRule::builder()
        .apply_server_side_encryption_by_default(default_encryption)
        .build();

    let config = ServerSideEncryptionConfiguration::builder()
        .rules(rule)
        .build()
        .map_err(|e| format!("Failed to build encryption configuration: {}", e))?;

    client
        .put_bucket_encryption()
        .bucket(bucket)
        .server_side_encryption_configuration(config)
        .send()
        .await
        .map_err(|e| format!("Failed to put bucket encryption: {}", e))?;

    Ok(PutBucketEncryptionOutput { success: true })
}

/// Put S3 Bucket Lifecycle
///
/// Creates or replaces a lifecycle configuration for a bucket.
///
pub async fn put_bucket_lifecycle(
    bucket: &str,
    lifecycle_rules: Vec<HashMap<String, Value>>,
    region: Option<&str>,
) -> Result<PutBucketLifecycleOutput, String> {
    let client = create_client(region).await?;

    let rules: Vec<LifecycleRule> = lifecycle_rules
        .iter()
        .filter_map(|rule| {
            let id = rule.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());
            let status = rule.get("status").and_then(|v| v.as_str()).unwrap_or("Enabled");

            let status_enum = match status.to_lowercase().as_str() {
                "enabled" => aws_sdk_s3::types::ExpirationStatus::Enabled,
                "disabled" => aws_sdk_s3::types::ExpirationStatus::Disabled,
                _ => return None,
            };

            let prefix = rule.get("prefix").and_then(|v| v.as_str()).map(|s| s.to_string());

            let mut builder = LifecycleRule::builder().status(status_enum);

            if let Some(rule_id) = id {
                builder = builder.id(rule_id);
            }

            if let Some(prefix_str) = prefix {
                let filter = aws_sdk_s3::types::LifecycleRuleFilter::builder()
                    .prefix(prefix_str)
                    .build();
                builder = builder.filter(filter);
            }

            // Handle expiration
            if let Some(expiration) = rule.get("expiration") {
                let mut exp_builder = aws_sdk_s3::types::LifecycleExpiration::builder();
                if let Some(days) = expiration.get("days").and_then(|v| v.as_i64()) {
                    exp_builder = exp_builder.days(days as i32);
                }
                builder = builder.expiration(exp_builder.build());
            }

            builder.build().ok()
        })
        .collect();

    let lifecycle_config = aws_sdk_s3::types::BucketLifecycleConfiguration::builder()
        .set_rules(Some(rules))
        .build()
        .map_err(|e| format!("Failed to build lifecycle configuration: {}", e))?;

    client
        .put_bucket_lifecycle_configuration()
        .bucket(bucket)
        .lifecycle_configuration(lifecycle_config)
        .send()
        .await
        .map_err(|e| format!("Failed to put bucket lifecycle: {}", e))?;

    Ok(PutBucketLifecycleOutput { success: true })
}

/// Set S3 Bucket Policy
///
/// Applies an S3 bucket policy to an Amazon S3 bucket.
///
pub async fn put_bucket_policy(
    policy: &str,
    bucket: &str,
    region: Option<&str>,
) -> Result<PutBucketPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .put_bucket_policy()
        .bucket(bucket)
        .policy(policy)
        .send()
        .await
        .map_err(|e| format!("Failed to put bucket policy: {}", e))?;

    Ok(PutBucketPolicyOutput { success: true })
}

/// Put S3 Bucket Versioning
///
/// Sets the versioning state of an existing bucket.
///
pub async fn put_bucket_versioning(
    status: &str,
    bucket: &str,
    region: Option<&str>,
) -> Result<PutBucketVersioningOutput, String> {
    let client = create_client(region).await?;

    let versioning_status = match status.to_lowercase().as_str() {
        "enabled" => BucketVersioningStatus::Enabled,
        "suspended" => BucketVersioningStatus::Suspended,
        _ => return Err(format!("Invalid versioning status: {}", status)),
    };

    let versioning_config = aws_sdk_s3::types::VersioningConfiguration::builder()
        .status(versioning_status)
        .build();

    client
        .put_bucket_versioning()
        .bucket(bucket)
        .versioning_configuration(versioning_config)
        .send()
        .await
        .map_err(|e| format!("Failed to put bucket versioning: {}", e))?;

    Ok(PutBucketVersioningOutput { success: true })
}

/// Put S3 Object
///
/// Adds an object to a bucket.
///
#[allow(clippy::too_many_arguments)]
pub async fn put_object(
    bucket: &str,
    key: &str,
    content: &[u8],
    tags: Option<HashMap<String, Value>>,
    content_type: Option<&str>,
    server_side_encryption: Option<&str>,
    region: Option<&str>,
    metadata: Option<HashMap<String, Value>>,
    storage_class: Option<&str>,
    acl: Option<&str>,
) -> Result<PutObjectOutput, String> {
    let client = create_client(region).await?;

    let body = ByteStream::from(content.to_vec());

    let mut request = client.put_object().bucket(bucket).key(key).body(body);

    if let Some(ct) = content_type {
        request = request.content_type(ct);
    }

    if let Some(sse) = server_side_encryption {
        request = request.server_side_encryption(sse.parse().map_err(|_| "Invalid server-side encryption")?);
    }

    if let Some(ref meta) = metadata {
        request = request.set_metadata(Some(metadata_to_aws_metadata(meta)));
    }

    if let Some(class) = storage_class {
        request = request.storage_class(class.parse().map_err(|_| "Invalid storage class")?);
    }

    if let Some(acl_str) = acl {
        request = request.acl(acl_str.parse().map_err(|_| "Invalid ACL")?);
    }

    if let Some(ref tags_data) = tags {
        let tag_string = tags_data
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or(&v.to_string())))
            .collect::<Vec<_>>()
            .join("&");
        request = request.tagging(tag_string);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to put object: {}", e))?;

    Ok(PutObjectOutput {
        success: true,
        etag: response.e_tag().unwrap_or_default().to_string(),
        version_id: response.version_id().unwrap_or_default().to_string(),
    })
}

/// Put S3 Object Tagging
///
/// Sets the supplied tag-set to an object that already exists in a bucket.
///
pub async fn put_object_tagging(
    key: &str,
    tags: HashMap<String, Value>,
    bucket: &str,
    region: Option<&str>,
    version_id: Option<&str>,
) -> Result<PutObjectTaggingOutput, String> {
    let client = create_client(region).await?;

    let tagging = Tagging::builder()
        .set_tag_set(Some(tags_to_aws_tags(&tags)))
        .build()
        .map_err(|e| format!("Failed to build tagging: {}", e))?;

    let mut request = client.put_object_tagging().bucket(bucket).key(key).tagging(tagging);

    if let Some(version) = version_id {
        request = request.version_id(version);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to put object tagging: {}", e))?;

    Ok(PutObjectTaggingOutput {
        success: true,
        version_id: response.version_id().unwrap_or_default().to_string(),
    })
}

/// Upload Part
///
/// Uploads a part in a multipart upload.
///
pub async fn upload_part(
    upload_id: &str,
    key: &str,
    body: &[u8],
    bucket: &str,
    part_number: i32,
    region: Option<&str>,
) -> Result<UploadPartOutput, String> {
    let client = create_client(region).await?;

    let body_stream = ByteStream::from(body.to_vec());

    let response = client
        .upload_part()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .part_number(part_number)
        .body(body_stream)
        .send()
        .await
        .map_err(|e| format!("Failed to upload part: {}", e))?;

    Ok(UploadPartOutput {
        success: true,
        etag: response.e_tag().unwrap_or_default().to_string(),
    })
}

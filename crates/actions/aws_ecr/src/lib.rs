//! Harana Actions - AWS ECR Module
//!
//! This module provides AWS ECR actions for container registry management.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
use aws_config::BehaviorVersion;
use aws_sdk_ecr::{
    config::Region,
    types::{
        ImageIdentifier, Tag, EncryptionConfiguration, EncryptionType,
        ImageScanningConfiguration, ImageTagMutability, TagStatus,
    },
    Client,
};

/// Creates an ECR client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    
    let ecr_config = if let Some(region_str) = region {
        aws_sdk_ecr::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_ecr::config::Builder::from(&config).build()
    };
    
    Ok(Client::from_conf(ecr_config))
}

/// Converts HashMap<String, Value> to ImageIdentifier
fn hash_map_to_image_id(map: &HashMap<String, Value>) -> ImageIdentifier {
    let mut builder = ImageIdentifier::builder();
    
    if let Some(Value::String(digest)) = map.get("imageDigest") {
        builder = builder.image_digest(digest);
    }
    if let Some(Value::String(tag)) = map.get("imageTag") {
        builder = builder.image_tag(tag);
    }
    
    builder.build()
}

/// Converts ImageIdentifier to HashMap<String, Value>
fn image_id_to_hash_map(image_id: &ImageIdentifier) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    if let Some(digest) = image_id.image_digest() {
        map.insert("imageDigest".to_string(), Value::String(digest.to_string()));
    }
    if let Some(tag) = image_id.image_tag() {
        map.insert("imageTag".to_string(), Value::String(tag.to_string()));
    }
    map
}

/// Batch Delete ECR Image
pub async fn batch_delete_image(
    image_ids: Vec<HashMap<String, Value>>,
    repository_name: &str,
    region: Option<&str>,
) -> Result<BatchDeleteImageOutput, String> {
    let client = create_client(region).await?;
    
    let aws_image_ids: Vec<ImageIdentifier> = image_ids
        .iter()
        .map(hash_map_to_image_id)
        .collect();
    
    let response = client
        .batch_delete_image()
        .repository_name(repository_name)
        .set_image_ids(Some(aws_image_ids))
        .send()
        .await
        .map_err(|e| format!("Failed to batch delete images: {}", e))?;
    
    let deleted_image_ids: Vec<HashMap<String, Value>> = response
        .image_ids()
        .iter()
        .map(image_id_to_hash_map)
        .collect();
    
    let failures: Vec<HashMap<String, Value>> = response
        .failures()
        .iter()
        .map(|f| {
            let mut map = HashMap::new();
            if let Some(image_id) = f.image_id() {
                map.insert("imageId".to_string(), serde_json::to_value(image_id_to_hash_map(image_id)).unwrap_or(Value::Null));
            }
            if let Some(code) = f.failure_code() {
                map.insert("failureCode".to_string(), Value::String(code.as_str().to_string()));
            }
            if let Some(reason) = f.failure_reason() {
                map.insert("failureReason".to_string(), Value::String(reason.to_string()));
            }
            map
        })
        .collect();
    
    Ok(BatchDeleteImageOutput {
        image_ids: deleted_image_ids,
        failures,
        success: true,
    })
}

/// Create ECR Repository
pub async fn create_repository(
    repository_name: &str,
    scan_on_push: Option<bool>,
    tags: Option<HashMap<String, Value>>,
    kms_key: Option<&str>,
    image_tag_mutability: Option<&str>,
    encryption_type: Option<&str>,
    region: Option<&str>,
) -> Result<CreateRepositoryOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .create_repository()
        .repository_name(repository_name);
    
    if let Some(scan) = scan_on_push {
        request = request.image_scanning_configuration(
            ImageScanningConfiguration::builder()
                .scan_on_push(scan)
                .build()
        );
    }
    
    if let Some(tags_map) = tags {
        for (key, value) in tags_map {
            if let Value::String(v) = value {
                if let Ok(tag) = Tag::builder()
                    .key(key)
                    .value(v)
                    .build() {
                    request = request.tags(tag);
                }
            }
        }
    }
    
    if let Some(mutability) = image_tag_mutability {
        let tag_mutability = match mutability.to_uppercase().as_str() {
            "MUTABLE" => ImageTagMutability::Mutable,
            "IMMUTABLE" => ImageTagMutability::Immutable,
            _ => ImageTagMutability::Mutable,
        };
        request = request.image_tag_mutability(tag_mutability);
    }
    
    if encryption_type.is_some() || kms_key.is_some() {
        let enc_type = match encryption_type.map(|e| e.to_uppercase()).as_deref() {
            Some("KMS") => EncryptionType::Kms,
            _ => EncryptionType::Aes256,
        };
        
        let mut enc_config = EncryptionConfiguration::builder()
            .encryption_type(enc_type);
        
        if let Some(key) = kms_key {
            enc_config = enc_config.kms_key(key);
        }
        
        if let Ok(config) = enc_config.build() {
            request = request.encryption_configuration(config);
        }
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create repository: {}", e))?;
    
    let repo = response.repository().ok_or("No repository in response")?;
    
    Ok(CreateRepositoryOutput {
        success: true,
        repository_arn: repo.repository_arn().unwrap_or_default().to_string(),
        registry_id: repo.registry_id().unwrap_or_default().to_string(),
        repository_uri: repo.repository_uri().unwrap_or_default().to_string(),
    })
}

/// Delete ECR Repository
pub async fn delete_repository(
    repository_name: &str,
    region: Option<&str>,
    force: Option<bool>,
) -> Result<DeleteRepositoryOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .delete_repository()
        .repository_name(repository_name);
    
    if let Some(f) = force {
        request = request.force(f);
    }
    
    request
        .send()
        .await
        .map_err(|e| format!("Failed to delete repository: {}", e))?;
    
    Ok(DeleteRepositoryOutput { success: true })
}

/// Delete Repository Policy
pub async fn delete_repository_policy(
    repository_name: &str,
    region: Option<&str>,
) -> Result<DeleteRepositoryPolicyOutput, String> {
    let client = create_client(region).await?;
    
    client
        .delete_repository_policy()
        .repository_name(repository_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete repository policy: {}", e))?;
    
    Ok(DeleteRepositoryPolicyOutput { success: true })
}

/// Describe ECR Images
pub async fn describe_images(
    repository_name: &str,
    image_ids: Option<Vec<HashMap<String, Value>>>,
    region: Option<&str>,
) -> Result<DescribeImagesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .describe_images()
        .repository_name(repository_name);
    
    if let Some(ids) = image_ids {
        let aws_image_ids: Vec<ImageIdentifier> = ids
            .iter()
            .map(hash_map_to_image_id)
            .collect();
        request = request.set_image_ids(Some(aws_image_ids));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe images: {}", e))?;
    
    let image_details: Vec<HashMap<String, Value>> = response
        .image_details()
        .iter()
        .map(|detail| {
            let mut map = HashMap::new();
            if let Some(digest) = detail.image_digest() {
                map.insert("imageDigest".to_string(), Value::String(digest.to_string()));
            }
            if let Some(registry_id) = detail.registry_id() {
                map.insert("registryId".to_string(), Value::String(registry_id.to_string()));
            }
            if let Some(repo_name) = detail.repository_name() {
                map.insert("repositoryName".to_string(), Value::String(repo_name.to_string()));
            }
            let tags: Vec<Value> = detail.image_tags()
                .iter()
                .map(|t| Value::String(t.to_string()))
                .collect();
            map.insert("imageTags".to_string(), Value::Array(tags));
            if let Some(size) = detail.image_size_in_bytes() {
                map.insert("imageSizeInBytes".to_string(), Value::Number(serde_json::Number::from(size)));
            }
            if let Some(pushed_at) = detail.image_pushed_at() {
                map.insert("imagePushedAt".to_string(), Value::String(pushed_at.to_string()));
            }
            if let Some(scan_status) = detail.image_scan_status() {
                let mut scan_map = HashMap::new();
                if let Some(status) = scan_status.status() {
                    scan_map.insert("status".to_string(), Value::String(status.as_str().to_string()));
                }
                if let Some(desc) = scan_status.description() {
                    scan_map.insert("description".to_string(), Value::String(desc.to_string()));
                }
                map.insert("imageScanStatus".to_string(), serde_json::to_value(scan_map).unwrap_or(Value::Null));
            }
            map
        })
        .collect();
    
    Ok(DescribeImagesOutput { image_details })
}

/// Describe ECR Repositories
pub async fn describe_repositories(
    region: Option<&str>,
    repository_names: Option<Vec<String>>,
) -> Result<DescribeRepositoriesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.describe_repositories();
    
    if let Some(names) = repository_names {
        request = request.set_repository_names(Some(names));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe repositories: {}", e))?;
    
    let repositories: Vec<HashMap<String, Value>> = response
        .repositories()
        .iter()
        .map(|repo| {
            let mut map = HashMap::new();
            if let Some(arn) = repo.repository_arn() {
                map.insert("repositoryArn".to_string(), Value::String(arn.to_string()));
            }
            if let Some(registry_id) = repo.registry_id() {
                map.insert("registryId".to_string(), Value::String(registry_id.to_string()));
            }
            if let Some(name) = repo.repository_name() {
                map.insert("repositoryName".to_string(), Value::String(name.to_string()));
            }
            if let Some(uri) = repo.repository_uri() {
                map.insert("repositoryUri".to_string(), Value::String(uri.to_string()));
            }
            if let Some(created_at) = repo.created_at() {
                map.insert("createdAt".to_string(), Value::String(created_at.to_string()));
            }
            if let Some(mutability) = repo.image_tag_mutability() {
                map.insert("imageTagMutability".to_string(), Value::String(mutability.as_str().to_string()));
            }
            if let Some(scan_config) = repo.image_scanning_configuration() {
                map.insert("scanOnPush".to_string(), Value::Bool(scan_config.scan_on_push()));
            }
            if let Some(enc_config) = repo.encryption_configuration() {
                let mut enc_map = HashMap::new();
                enc_map.insert("encryptionType".to_string(), Value::String(enc_config.encryption_type().as_str().to_string()));
                if let Some(kms_key) = enc_config.kms_key() {
                    enc_map.insert("kmsKey".to_string(), Value::String(kms_key.to_string()));
                }
                map.insert("encryptionConfiguration".to_string(), serde_json::to_value(enc_map).unwrap_or(Value::Null));
            }
            map
        })
        .collect();
    
    Ok(DescribeRepositoriesOutput { repositories })
}

/// Describe Image Scan Findings
pub async fn describe_scan_findings(
    image_id: HashMap<String, Value>,
    repository_name: &str,
    max_results: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeScanFindingsOutput, String> {
    let client = create_client(region).await?;
    
    let aws_image_id = hash_map_to_image_id(&image_id);
    
    let mut request = client
        .describe_image_scan_findings()
        .repository_name(repository_name)
        .image_id(aws_image_id);
    
    if let Some(max) = max_results {
        request = request.max_results(max);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to describe scan findings: {}", e))?;
    
    let findings: Vec<HashMap<String, Value>> = response
        .image_scan_findings()
        .map(|scan_findings| {
            scan_findings.findings()
                .iter()
                .map(|finding| {
                    let mut map = HashMap::new();
                    if let Some(name) = finding.name() {
                        map.insert("name".to_string(), Value::String(name.to_string()));
                    }
                    if let Some(desc) = finding.description() {
                        map.insert("description".to_string(), Value::String(desc.to_string()));
                    }
                    if let Some(uri) = finding.uri() {
                        map.insert("uri".to_string(), Value::String(uri.to_string()));
                    }
                    if let Some(severity) = finding.severity() {
                        map.insert("severity".to_string(), Value::String(severity.as_str().to_string()));
                    }
                    let attributes: Vec<Value> = finding.attributes()
                        .iter()
                        .map(|attr| {
                            let mut attr_map = HashMap::new();
                            attr_map.insert("key".to_string(), Value::String(attr.key.to_string()));
                            if let Some(value) = &attr.value {
                                attr_map.insert("value".to_string(), Value::String(value.to_string()));
                            }
                            serde_json::to_value(attr_map).unwrap_or(Value::Null)
                        })
                        .collect();
                    map.insert("attributes".to_string(), Value::Array(attributes));
                    map
                })
                .collect()
        })
        .unwrap_or_default();
    
    let mut image_scan_status = HashMap::new();
    if let Some(status) = response.image_scan_status() {
        if let Some(s) = status.status() {
            image_scan_status.insert("status".to_string(), Value::String(s.as_str().to_string()));
        }
        if let Some(desc) = status.description() {
            image_scan_status.insert("description".to_string(), Value::String(desc.to_string()));
        }
    }
    
    Ok(DescribeScanFindingsOutput {
        findings,
        image_scan_status,
        next_token: response.next_token().unwrap_or_default().to_string(),
    })
}

/// Get ECR Authorization Token
#[allow(deprecated)]
pub async fn get_auth_token(
    registry_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<GetAuthTokenOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client.get_authorization_token();
    
    // Note: registry_ids is deprecated by AWS but kept for backwards compatibility
    if let Some(ids) = registry_ids {
        request = request.set_registry_ids(Some(ids));
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get authorization token: {}", e))?;
    
    let auth_data = response
        .authorization_data()
        .first()
        .ok_or("No authorization data returned")?;
    
    Ok(GetAuthTokenOutput {
        authorization_token: auth_data.authorization_token().unwrap_or_default().to_string(),
        proxy_endpoint: auth_data.proxy_endpoint().unwrap_or_default().to_string(),
        expires_at: auth_data.expires_at().map(|t| t.to_string()).unwrap_or_default(),
        success: true,
    })
}

/// Get ECR Download URL
pub async fn get_download_url(
    repository_name: &str,
    image_digest: &str,
    region: Option<&str>,
) -> Result<GetDownloadUrlOutput, String> {
    let client = create_client(region).await?;
    
    let response = client
        .get_download_url_for_layer()
        .repository_name(repository_name)
        .layer_digest(image_digest)
        .send()
        .await
        .map_err(|e| format!("Failed to get download URL: {}", e))?;
    
    Ok(GetDownloadUrlOutput {
        download_url: response.download_url().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Get Lifecycle Policy
pub async fn get_lifecycle_policy(
    repository_name: &str,
    region: Option<&str>,
) -> Result<GetLifecyclePolicyOutput, String> {
    let client = create_client(region).await?;
    
    let response = client
        .get_lifecycle_policy()
        .repository_name(repository_name)
        .send()
        .await
        .map_err(|e| format!("Failed to get lifecycle policy: {}", e))?;
    
    Ok(GetLifecyclePolicyOutput {
        last_evaluated_at: response.last_evaluated_at().map(|t| t.to_string()).unwrap_or_default(),
        lifecycle_policy_text: response.lifecycle_policy_text().unwrap_or_default().to_string(),
        repository_name: response.repository_name().unwrap_or_default().to_string(),
        registry_id: response.registry_id().unwrap_or_default().to_string(),
    })
}

/// Get Repository Policy
pub async fn get_repository_policy(
    repository_name: &str,
    region: Option<&str>,
) -> Result<GetRepositoryPolicyOutput, String> {
    let client = create_client(region).await?;
    
    let response = client
        .get_repository_policy()
        .repository_name(repository_name)
        .send()
        .await
        .map_err(|e| format!("Failed to get repository policy: {}", e))?;
    
    Ok(GetRepositoryPolicyOutput {
        policy_text: response.policy_text().unwrap_or_default().to_string(),
        registry_id: response.registry_id().unwrap_or_default().to_string(),
        repository_name: response.repository_name().unwrap_or_default().to_string(),
    })
}

/// List ECR Images
pub async fn list_images(
    repository_name: &str,
    filter: Option<HashMap<String, Value>>,
    region: Option<&str>,
    max_results: Option<i32>,
) -> Result<ListImagesOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .list_images()
        .repository_name(repository_name);
    
    if let Some(filter_map) = filter {
        if let Some(Value::String(tag_status)) = filter_map.get("tagStatus") {
            let status = match tag_status.to_uppercase().as_str() {
                "TAGGED" => TagStatus::Tagged,
                "UNTAGGED" => TagStatus::Untagged,
                "ANY" => TagStatus::Any,
                _ => TagStatus::Any,
            };
            request = request.filter(
                aws_sdk_ecr::types::ListImagesFilter::builder()
                    .tag_status(status)
                    .build()
            );
        }
    }
    
    if let Some(max) = max_results {
        request = request.max_results(max);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list images: {}", e))?;
    
    let image_ids: Vec<HashMap<String, Value>> = response
        .image_ids()
        .iter()
        .map(image_id_to_hash_map)
        .collect();
    
    Ok(ListImagesOutput {
        image_ids,
        next_token: response.next_token().unwrap_or_default().to_string(),
    })
}

/// Put ECR Image
pub async fn put_image(
    repository_name: &str,
    image_manifest: &str,
    image_tag: &str,
    region: Option<&str>,
) -> Result<PutImageOutput, String> {
    let client = create_client(region).await?;
    
    let response = client
        .put_image()
        .repository_name(repository_name)
        .image_manifest(image_manifest)
        .image_tag(image_tag)
        .send()
        .await
        .map_err(|e| format!("Failed to put image: {}", e))?;
    
    let mut image = HashMap::new();
    if let Some(img) = response.image() {
        if let Some(registry_id) = img.registry_id() {
            image.insert("registryId".to_string(), Value::String(registry_id.to_string()));
        }
        if let Some(repo_name) = img.repository_name() {
            image.insert("repositoryName".to_string(), Value::String(repo_name.to_string()));
        }
        if let Some(image_id) = img.image_id() {
            image.insert("imageId".to_string(), serde_json::to_value(image_id_to_hash_map(image_id)).unwrap_or(Value::Null));
        }
        if let Some(manifest) = img.image_manifest() {
            image.insert("imageManifest".to_string(), Value::String(manifest.to_string()));
        }
        if let Some(manifest_media_type) = img.image_manifest_media_type() {
            image.insert("imageManifestMediaType".to_string(), Value::String(manifest_media_type.to_string()));
        }
    }
    
    Ok(PutImageOutput {
        image,
        success: true,
    })
}

/// Put Lifecycle Policy
pub async fn put_lifecycle_policy(
    lifecycle_policy_text: &str,
    repository_name: &str,
    region: Option<&str>,
) -> Result<PutLifecyclePolicyOutput, String> {
    let client = create_client(region).await?;
    
    let response = client
        .put_lifecycle_policy()
        .repository_name(repository_name)
        .lifecycle_policy_text(lifecycle_policy_text)
        .send()
        .await
        .map_err(|e| format!("Failed to put lifecycle policy: {}", e))?;
    
    Ok(PutLifecyclePolicyOutput {
        repository_name: response.repository_name().unwrap_or_default().to_string(),
        lifecycle_policy_text: response.lifecycle_policy_text().unwrap_or_default().to_string(),
        success: true,
        registry_id: response.registry_id().unwrap_or_default().to_string(),
    })
}

/// Set Repository Policy
pub async fn set_repository_policy(
    repository_name: &str,
    policy_text: &str,
    region: Option<&str>,
    force: Option<bool>,
) -> Result<SetRepositoryPolicyOutput, String> {
    let client = create_client(region).await?;
    
    let mut request = client
        .set_repository_policy()
        .repository_name(repository_name)
        .policy_text(policy_text);
    
    if let Some(f) = force {
        request = request.force(f);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to set repository policy: {}", e))?;
    
    Ok(SetRepositoryPolicyOutput {
        registry_id: response.registry_id().unwrap_or_default().to_string(),
        repository_name: response.repository_name().unwrap_or_default().to_string(),
        policy_text: response.policy_text().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Start Image Scan
pub async fn start_image_scan(
    image_id: HashMap<String, Value>,
    repository_name: &str,
    region: Option<&str>,
) -> Result<StartImageScanOutput, String> {
    let client = create_client(region).await?;
    
    let aws_image_id = hash_map_to_image_id(&image_id);
    
    let response = client
        .start_image_scan()
        .repository_name(repository_name)
        .image_id(aws_image_id.clone())
        .send()
        .await
        .map_err(|e| format!("Failed to start image scan: {}", e))?;
    
    let scan_status = response
        .image_scan_status()
        .map(|s| s.status().map(|st| st.as_str().to_string()).unwrap_or_default())
        .unwrap_or_default();
    
    let returned_image_id = response
        .image_id()
        .map(image_id_to_hash_map)
        .unwrap_or_else(|| image_id.clone());
    
    Ok(StartImageScanOutput {
        scan_status,
        success: true,
        image_id: returned_image_id,
    })
}

/// Tag ECR Image
pub async fn tag_image(
    repository_name: &str,
    image_digest: &str,
    image_tag: &str,
    region: Option<&str>,
) -> Result<TagImageOutput, String> {
    let client = create_client(region).await?;
    
    // To tag an image in ECR, we need to get the image manifest and re-put it with a new tag
    // First, get the image manifest using batch_get_image
    let image_id = ImageIdentifier::builder()
        .image_digest(image_digest)
        .build();
    
    let get_response = client
        .batch_get_image()
        .repository_name(repository_name)
        .image_ids(image_id)
        .send()
        .await
        .map_err(|e| format!("Failed to get image for tagging: {}", e))?;
    
    let image = get_response
        .images()
        .first()
        .ok_or("Image not found")?;
    
    let manifest = image
        .image_manifest()
        .ok_or("Image manifest not found")?;
    
    // Put the image with the new tag
    client
        .put_image()
        .repository_name(repository_name)
        .image_manifest(manifest)
        .image_tag(image_tag)
        .send()
        .await
        .map_err(|e| format!("Failed to tag image: {}", e))?;
    
    Ok(TagImageOutput { success: true })
}

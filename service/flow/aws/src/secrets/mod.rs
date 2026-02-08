// Harana Actions - AWS Secrets Manager Module
//
// This module provides AWS Secrets Manager actions for managing secrets,
// secret versions, rotation, and replication.

pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::{
    config::Region,
    types::{Filter, FilterNameStringType, ReplicaRegionType, RotationRulesType, Tag},
    Client,
};
use output::*;
use serde_json::Value;
use std::collections::HashMap;

/// Creates a Secrets Manager client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let sm_config = if let Some(region_str) = region {
        aws_sdk_secretsmanager::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_secretsmanager::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(sm_config))
}

/// Converts a HashMap of tags to AWS SDK Tag format
fn tags_to_aws_tags(tags: &HashMap<String, Value>) -> Vec<Tag> {
    tags.iter()
        .map(|(k, v)| {
            Tag::builder()
                .key(k)
                .value(v.as_str().unwrap_or(&v.to_string()))
                .build()
        })
        .collect()
}

/// Cancel Rotate Secret
///
/// Cancels a scheduled rotation for a secret.
///
pub async fn cancel_rotate(
    secret_id: &str,
    _version_id: &str,
    region: Option<&str>,
) -> Result<CancelRotateOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .cancel_rotate_secret()
        .secret_id(secret_id)
        .send()
        .await
        .map_err(|e| format!("Failed to cancel rotate secret: {}", e))?;

    Ok(CancelRotateOutput {
        arn: response.arn().unwrap_or_default().to_string(),
        name: response.name().unwrap_or_default().to_string(),
        version_id: response.version_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Create Secret
///
/// Creates a new secret with the specified name and value.
///
pub async fn create(
    name: &str,
    secret_value: &str,
    description: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    kms_key_id: Option<&str>,
    region: Option<&str>,
) -> Result<CreateOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .create_secret()
        .name(name)
        .secret_string(secret_value);

    if let Some(desc) = description {
        request = request.description(desc);
    }

    if let Some(key_id) = kms_key_id {
        request = request.kms_key_id(key_id);
    }

    if let Some(ref tags_data) = tags {
        request = request.set_tags(Some(tags_to_aws_tags(tags_data)));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to create secret: {}", e))?;

    Ok(CreateOutput {
        success: true,
        arn: response.arn().unwrap_or_default().to_string(),
        version_id: response.version_id().unwrap_or_default().to_string(),
        name: response.name().unwrap_or_default().to_string(),
    })
}

/// Delete Secret
///
/// Deletes a secret and all of its versions. You can optionally specify a
/// recovery window during which you can restore the secret.
///
pub async fn delete(
    secret_id: &str,
    recovery_window_days: Option<i32>,
    force_delete: Option<bool>,
    region: Option<&str>,
) -> Result<DeleteOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.delete_secret().secret_id(secret_id);

    if let Some(force) = force_delete {
        if force {
            request = request.force_delete_without_recovery(true);
        }
    } else if let Some(days) = recovery_window_days {
        request = request.recovery_window_in_days(days as i64);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to delete secret: {}", e))?;

    let deletion_date = response
        .deletion_date()
        .map(|d| d.secs())
        .unwrap_or(0);

    Ok(DeleteOutput {
        arn: response.arn().unwrap_or_default().to_string(),
        deletion_date,
        name: response.name().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Describe Secret
///
/// Retrieves metadata about a secret, but not the secret value.
///
pub async fn describe(
    secret_id: &str,
    region: Option<&str>,
) -> Result<DescribeOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .describe_secret()
        .secret_id(secret_id)
        .send()
        .await
        .map_err(|e| format!("Failed to describe secret: {}", e))?;

    let tags: HashMap<String, Value> = response
        .tags()
        .iter()
        .filter_map(|tag| {
            let key = tag.key()?;
            let value = tag.value()?;
            Some((key.to_string(), Value::String(value.to_string())))
        })
        .collect();

    Ok(DescribeOutput {
        arn: response.arn().unwrap_or_default().to_string(),
        kms_key_id: response.kms_key_id().unwrap_or_default().to_string(),
        last_rotated_date: response
            .last_rotated_date()
            .map(|d| d.secs())
            .unwrap_or(0),
        last_changed_date: response
            .last_changed_date()
            .map(|d| d.secs())
            .unwrap_or(0),
        description: response.description().unwrap_or_default().to_string(),
        rotation_enabled: response.rotation_enabled().unwrap_or(false),
        name: response.name().unwrap_or_default().to_string(),
        tags,
    })
}

/// Get Secret Value
///
/// Retrieves the contents of the secret value.
///
pub async fn get_value(
    secret_id: &str,
    version_id: Option<&str>,
    version_stage: Option<&str>,
    region: Option<&str>,
) -> Result<GetValueOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_secret_value().secret_id(secret_id);

    if let Some(vid) = version_id {
        request = request.version_id(vid);
    }

    if let Some(stage) = version_stage {
        request = request.version_stage(stage);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get secret value: {}", e))?;

    let secret_value = response
        .secret_string()
        .map(|s| s.to_string())
        .or_else(|| {
            response.secret_binary().map(|b| {
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    b.as_ref(),
                )
            })
        })
        .unwrap_or_default();

    Ok(GetValueOutput {
        version_id: response.version_id().unwrap_or_default().to_string(),
        name: response.name().unwrap_or_default().to_string(),
        secret_value,
        created_date: response
            .created_date()
            .map(|d| d.secs())
            .unwrap_or(0),
        arn: response.arn().unwrap_or_default().to_string(),
    })
}

/// List Secrets
///
/// Lists all of the secrets that are stored by Secrets Manager.
///
pub async fn lists(
    filters: Option<Vec<HashMap<String, Value>>>,
    max_results: Option<i32>,
    sort_order: Option<&str>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_secrets();

    if let Some(max) = max_results {
        request = request.max_results(max);
    }

    if let Some(order) = sort_order {
        request = request.sort_order(order.parse().map_err(|_| "Invalid sort order")?);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    if let Some(ref filter_list) = filters {
        let aws_filters: Vec<Filter> = filter_list
            .iter()
            .filter_map(|f| {
                let key = f.get("key")?.as_str()?;
                let values: Vec<String> = f
                    .get("values")?
                    .as_array()?
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();

                let filter_key = match key {
                    "name" => FilterNameStringType::Name,
                    "description" => FilterNameStringType::Description,
                    "tag-key" => FilterNameStringType::TagKey,
                    "tag-value" => FilterNameStringType::TagValue,
                    "primary-region" => FilterNameStringType::PrimaryRegion,
                    "owning-service" => FilterNameStringType::OwningService,
                    "all" => FilterNameStringType::All,
                    _ => return None,
                };

                Some(
                    Filter::builder()
                        .key(filter_key)
                        .set_values(Some(values))
                        .build(),
                )
            })
            .collect();

        request = request.set_filters(Some(aws_filters));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list secrets: {}", e))?;

    let secrets: Vec<HashMap<String, Value>> = response
        .secret_list()
        .iter()
        .map(|s| {
            let mut map = HashMap::new();
            if let Some(arn) = s.arn() {
                map.insert("arn".to_string(), Value::String(arn.to_string()));
            }
            if let Some(name) = s.name() {
                map.insert("name".to_string(), Value::String(name.to_string()));
            }
            if let Some(desc) = s.description() {
                map.insert("description".to_string(), Value::String(desc.to_string()));
            }
            if let Some(kms) = s.kms_key_id() {
                map.insert("kms_key_id".to_string(), Value::String(kms.to_string()));
            }
            map.insert(
                "rotation_enabled".to_string(),
                Value::Bool(s.rotation_enabled().unwrap_or(false)),
            );
            if let Some(created) = s.created_date() {
                map.insert("created_date".to_string(), Value::Number(created.secs().into()));
            }
            if let Some(changed) = s.last_changed_date() {
                map.insert("last_changed_date".to_string(), Value::Number(changed.secs().into()));
            }
            if let Some(accessed) = s.last_accessed_date() {
                map.insert("last_accessed_date".to_string(), Value::Number(accessed.secs().into()));
            }
            if let Some(deleted) = s.deleted_date() {
                map.insert("deleted_date".to_string(), Value::Number(deleted.secs().into()));
            }
            if let Some(region) = s.primary_region() {
                map.insert("primary_region".to_string(), Value::String(region.to_string()));
            }
            map
        })
        .collect();

    Ok(ListsOutput {
        next_token: response.next_token().unwrap_or_default().to_string(),
        secrets,
    })
}

/// Put Secret Value
///
/// Stores a new encrypted secret value in the specified version of a secret.
///
pub async fn put_value(
    secret_id: &str,
    secret_value: &str,
    version_stages: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<PutValueOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .put_secret_value()
        .secret_id(secret_id)
        .secret_string(secret_value);

    if let Some(stages) = version_stages {
        request = request.set_version_stages(Some(stages));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to put secret value: {}", e))?;

    Ok(PutValueOutput {
        version_id: response.version_id().unwrap_or_default().to_string(),
        arn: response.arn().unwrap_or_default().to_string(),
        name: response.name().unwrap_or_default().to_string(),
        version_stages: response
            .version_stages()
            .iter()
            .map(|s| s.to_string())
            .collect(),
    })
}

/// Remove Regions From Replication
///
/// Removes one or more regions from replication for a secret.
///
pub async fn remove_regions_from_replication(
    secret_id: &str,
    regions: Vec<String>,
    region: Option<&str>,
) -> Result<RemoveRegionsFromReplicationOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .remove_regions_from_replication()
        .secret_id(secret_id)
        .set_remove_replica_regions(Some(regions))
        .send()
        .await
        .map_err(|e| format!("Failed to remove regions from replication: {}", e))?;

    let replication_status: Vec<HashMap<String, Value>> = response
        .replication_status()
        .iter()
        .map(|s| {
            let mut map = HashMap::new();
            if let Some(region) = s.region() {
                map.insert("region".to_string(), Value::String(region.to_string()));
            }
            if let Some(kms) = s.kms_key_id() {
                map.insert("kms_key_id".to_string(), Value::String(kms.to_string()));
            }
            if let Some(status) = s.status() {
                map.insert("status".to_string(), Value::String(status.as_str().to_string()));
            }
            if let Some(msg) = s.status_message() {
                map.insert("status_message".to_string(), Value::String(msg.to_string()));
            }
            map
        })
        .collect();

    Ok(RemoveRegionsFromReplicationOutput {
        arn: response.arn().unwrap_or_default().to_string(),
        success: true,
        replication_status,
    })
}

/// Replicate Secret To Regions
///
/// Replicates the secret to one or more new AWS regions.
///
pub async fn replicate(
    secret_id: &str,
    regions: Vec<String>,
    kms_key_ids: Option<HashMap<String, Value>>,
    region: Option<&str>,
) -> Result<ReplicateOutput, String> {
    let client = create_client(region).await?;

    let replica_regions: Vec<ReplicaRegionType> = regions
        .iter()
        .map(|r| {
            let mut builder = ReplicaRegionType::builder().region(r);
            if let Some(ref keys) = kms_key_ids {
                if let Some(key) = keys.get(r).and_then(|v| v.as_str()) {
                    builder = builder.kms_key_id(key);
                }
            }
            builder.build()
        })
        .collect();

    let response = client
        .replicate_secret_to_regions()
        .secret_id(secret_id)
        .set_add_replica_regions(Some(replica_regions))
        .send()
        .await
        .map_err(|e| format!("Failed to replicate secret: {}", e))?;

    let replication_status: Vec<HashMap<String, Value>> = response
        .replication_status()
        .iter()
        .map(|s| {
            let mut map = HashMap::new();
            if let Some(region) = s.region() {
                map.insert("region".to_string(), Value::String(region.to_string()));
            }
            if let Some(kms) = s.kms_key_id() {
                map.insert("kms_key_id".to_string(), Value::String(kms.to_string()));
            }
            if let Some(status) = s.status() {
                map.insert("status".to_string(), Value::String(status.as_str().to_string()));
            }
            if let Some(msg) = s.status_message() {
                map.insert("status_message".to_string(), Value::String(msg.to_string()));
            }
            map
        })
        .collect();

    Ok(ReplicateOutput {
        arn: response.arn().unwrap_or_default().to_string(),
        replication_status,
        success: true,
    })
}

/// Restore Secret
///
/// Cancels the scheduled deletion of a secret.
///
pub async fn restore(
    secret_id: &str,
    region: Option<&str>,
) -> Result<RestoreOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .restore_secret()
        .secret_id(secret_id)
        .send()
        .await
        .map_err(|e| format!("Failed to restore secret: {}", e))?;

    Ok(RestoreOutput {
        arn: response.arn().unwrap_or_default().to_string(),
        success: true,
        name: response.name().unwrap_or_default().to_string(),
    })
}

/// Rotate Secret
///
/// Configures and starts the asynchronous process of rotating the secret.
///
pub async fn rotate(
    rotation_lambda_arn: &str,
    secret_id: &str,
    rotation_rules: Option<HashMap<String, Value>>,
    region: Option<&str>,
) -> Result<RotateOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .rotate_secret()
        .secret_id(secret_id)
        .rotation_lambda_arn(rotation_lambda_arn);

    if let Some(ref rules) = rotation_rules {
        let mut rules_builder = RotationRulesType::builder();

        if let Some(days) = rules.get("automatically_after_days").and_then(|v| v.as_i64()) {
            rules_builder = rules_builder.automatically_after_days(days);
        }
        if let Some(duration) = rules.get("duration").and_then(|v| v.as_str()) {
            rules_builder = rules_builder.duration(duration);
        }
        if let Some(expression) = rules.get("schedule_expression").and_then(|v| v.as_str()) {
            rules_builder = rules_builder.schedule_expression(expression);
        }

        request = request.rotation_rules(rules_builder.build());
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to rotate secret: {}", e))?;

    Ok(RotateOutput {
        arn: response.arn().unwrap_or_default().to_string(),
        name: response.name().unwrap_or_default().to_string(),
        version_id: response.version_id().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Tag Resource
///
/// Attaches tags to a secret.
///
pub async fn tag(
    secret_id: &str,
    tags: HashMap<String, Value>,
    region: Option<&str>,
) -> Result<TagOutput, String> {
    let client = create_client(region).await?;

    client
        .tag_resource()
        .secret_id(secret_id)
        .set_tags(Some(tags_to_aws_tags(&tags)))
        .send()
        .await
        .map_err(|e| format!("Failed to tag secret: {}", e))?;

    Ok(TagOutput { success: true })
}

/// Untag Resource
///
/// Removes one or more tags from a secret.
///
pub async fn untag(
    tag_keys: Vec<String>,
    secret_id: &str,
    region: Option<&str>,
) -> Result<UntagOutput, String> {
    let client = create_client(region).await?;

    client
        .untag_resource()
        .secret_id(secret_id)
        .set_tag_keys(Some(tag_keys))
        .send()
        .await
        .map_err(|e| format!("Failed to untag secret: {}", e))?;

    Ok(UntagOutput { success: true })
}

/// Update Secret
///
/// Updates the metadata or the secret value for a secret.
///
pub async fn update(
    secret_id: &str,
    secret_value: &str,
    description: Option<&str>,
    kms_key_id: Option<&str>,
    region: Option<&str>,
) -> Result<UpdateOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .update_secret()
        .secret_id(secret_id)
        .secret_string(secret_value);

    if let Some(desc) = description {
        request = request.description(desc);
    }

    if let Some(key_id) = kms_key_id {
        request = request.kms_key_id(key_id);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to update secret: {}", e))?;

    Ok(UpdateOutput {
        success: true,
        name: response.name().unwrap_or_default().to_string(),
        arn: response.arn().unwrap_or_default().to_string(),
        version_id: response.version_id().unwrap_or_default().to_string(),
    })
}

/// Validate Secret
///
/// Validates that a secret exists and is accessible.
///
pub async fn validate(
    secret_id: &str,
    region: Option<&str>,
) -> Result<ValidateOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .describe_secret()
        .secret_id(secret_id)
        .send()
        .await;

    match response {
        Ok(secret) => {
            let scheduled_for_deletion = secret.deleted_date().is_some();
            Ok(ValidateOutput {
                scheduled_for_deletion,
                exists: true,
                valid: !scheduled_for_deletion,
            })
        }
        Err(e) => {
            let error_str = e.to_string();
            if error_str.contains("ResourceNotFoundException") {
                Ok(ValidateOutput {
                    scheduled_for_deletion: false,
                    exists: false,
                    valid: false,
                })
            } else {
                Err(format!("Failed to validate secret: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests;

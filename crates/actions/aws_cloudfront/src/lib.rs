//! Harana Actions - AWS CloudFront Module
//!
//! This module provides AWS CloudFront CDN actions for managing distributions,
//! invalidations, and cache behaviors.

#![warn(missing_docs)]

/// Output types for AWS CloudFront actions
pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_cloudfront::{
    config::Region,
    types::{
        Aliases, AllowedMethods, CacheBehavior as AwsCacheBehavior, CacheBehaviors,
        CachedMethods, CustomErrorResponse as AwsCustomErrorResponse, CustomErrorResponses,
        DefaultCacheBehavior, Distribution, DistributionConfig, InvalidationBatch, Method,
        Origin, Origins, Paths, PriceClass, S3OriginConfig as AwsS3OriginConfig, Tag, Tags,
        ViewerCertificate, ViewerProtocolPolicy,
    },
    Client,
};
use chrono::{DateTime, Utc};
use output::*;

async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let cf_config = if let Some(region_str) = region {
        aws_sdk_cloudfront::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_cloudfront::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(cf_config))
}

fn parse_viewer_protocol_policy(policy: &str) -> ViewerProtocolPolicy {
    match policy.to_lowercase().as_str() {
        "allow-all" => ViewerProtocolPolicy::AllowAll,
        "https-only" => ViewerProtocolPolicy::HttpsOnly,
        "redirect-to-https" => ViewerProtocolPolicy::RedirectToHttps,
        _ => ViewerProtocolPolicy::AllowAll,
    }
}

fn parse_price_class(price_class: &str) -> PriceClass {
    match price_class.to_lowercase().as_str() {
        "priceclass_100" | "price_class_100" => PriceClass::PriceClass100,
        "priceclass_200" | "price_class_200" => PriceClass::PriceClass200,
        "priceclass_all" | "price_class_all" => PriceClass::PriceClassAll,
        _ => PriceClass::PriceClassAll,
    }
}

fn parse_method(method: &str) -> Method {
    match method.to_uppercase().as_str() {
        "GET" => Method::Get,
        "HEAD" => Method::Head,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "PATCH" => Method::Patch,
        "OPTIONS" => Method::Options,
        "DELETE" => Method::Delete,
        _ => Method::Get,
    }
}

fn to_default_cache_behavior(cb: &CacheBehavior) -> Result<DefaultCacheBehavior, String> {
    let allowed_methods: Vec<Method> = cb.allowed_methods.iter().map(|m| parse_method(m)).collect();
    let cached_methods: Vec<Method> = cb.cached_methods.iter().map(|m| parse_method(m)).collect();

    let cached_methods_obj = CachedMethods::builder()
        .quantity(cached_methods.len() as i32)
        .set_items(Some(cached_methods))
        .build()
        .map_err(|e| format!("Failed to build cached methods: {}", e))?;

    let allowed_methods_obj = AllowedMethods::builder()
        .quantity(allowed_methods.len() as i32)
        .set_items(Some(allowed_methods))
        .cached_methods(cached_methods_obj)
        .build()
        .map_err(|e| format!("Failed to build allowed methods: {}", e))?;

    let mut builder = DefaultCacheBehavior::builder()
        .target_origin_id(&cb.target_origin_id)
        .viewer_protocol_policy(parse_viewer_protocol_policy(&cb.viewer_protocol_policy))
        .compress(cb.compress)
        .allowed_methods(allowed_methods_obj);

    if let Some(policy_id) = &cb.cache_policy_id {
        builder = builder.cache_policy_id(policy_id);
    }
    if let Some(policy_id) = &cb.origin_request_policy_id {
        builder = builder.origin_request_policy_id(policy_id);
    }

    builder.build().map_err(|e| format!("Failed to build default cache behavior: {}", e))
}

fn to_aws_cache_behavior(cb: &CacheBehavior) -> Result<AwsCacheBehavior, String> {
    let allowed_methods: Vec<Method> = cb.allowed_methods.iter().map(|m| parse_method(m)).collect();
    let cached_methods: Vec<Method> = cb.cached_methods.iter().map(|m| parse_method(m)).collect();
    let path_pattern = cb.path_pattern.as_deref().unwrap_or("/*");

    let cached_methods_obj = CachedMethods::builder()
        .quantity(cached_methods.len() as i32)
        .set_items(Some(cached_methods))
        .build()
        .map_err(|e| format!("Failed to build cached methods: {}", e))?;

    let allowed_methods_obj = AllowedMethods::builder()
        .quantity(allowed_methods.len() as i32)
        .set_items(Some(allowed_methods))
        .cached_methods(cached_methods_obj)
        .build()
        .map_err(|e| format!("Failed to build allowed methods: {}", e))?;

    let mut builder = AwsCacheBehavior::builder()
        .path_pattern(path_pattern)
        .target_origin_id(&cb.target_origin_id)
        .viewer_protocol_policy(parse_viewer_protocol_policy(&cb.viewer_protocol_policy))
        .compress(cb.compress)
        .allowed_methods(allowed_methods_obj);

    if let Some(policy_id) = &cb.cache_policy_id {
        builder = builder.cache_policy_id(policy_id);
    }
    if let Some(policy_id) = &cb.origin_request_policy_id {
        builder = builder.origin_request_policy_id(policy_id);
    }

    builder.build().map_err(|e| format!("Failed to build cache behavior: {}", e))
}

fn aws_datetime_to_chrono(dt: &aws_sdk_cloudfront::primitives::DateTime) -> DateTime<Utc> {
    DateTime::from_timestamp(dt.secs(), 0).unwrap_or_else(Utc::now)
}

fn distribution_to_output(dist: &Distribution, etag: &str) -> GetDistributionOutput {
    let config = dist.distribution_config();

    let aliases: Vec<String> = config
        .and_then(|c| c.aliases())
        .map(|a| a.items().iter().map(|s| s.to_string()).collect())
        .unwrap_or_default();

    let origins: Vec<CloudFrontOrigin> = config
        .and_then(|c| c.origins())
        .map(|o| {
            o.items()
                .iter()
                .map(|origin| CloudFrontOrigin {
                    id: origin.id().to_string(),
                    domain_name: origin.domain_name().to_string(),
                    origin_path: origin.origin_path().map(|s| s.to_string()),
                    custom_headers: origin.custom_headers().map(|h| {
                        h.items()
                            .iter()
                            .map(|header| (header.header_name().to_string(), header.header_value().to_string()))
                            .collect()
                    }),
                    s3_origin_config: origin.s3_origin_config().map(|s3| S3OriginConfig {
                        origin_access_identity: s3.origin_access_identity().to_string(),
                    }),
                    custom_origin_config: origin.custom_origin_config().map(|custom| CustomOriginConfig {
                        http_port: custom.http_port(),
                        https_port: custom.https_port(),
                        origin_protocol_policy: custom.origin_protocol_policy().as_str().to_string(),
                        origin_ssl_protocols: custom
                            .origin_ssl_protocols()
                            .map(|p| p.items().iter().map(|s| s.as_str().to_string()).collect())
                            .unwrap_or_default(),
                    }),
                })
                .collect()
        })
        .unwrap_or_default();

    let default_cache_behavior = config.and_then(|c| c.default_cache_behavior()).map(|dcb| CacheBehavior {
        path_pattern: None,
        target_origin_id: dcb.target_origin_id().to_string(),
        viewer_protocol_policy: dcb.viewer_protocol_policy().as_str().to_string(),
        allowed_methods: dcb.allowed_methods()
            .map(|m| m.items().iter().map(|method| method.as_str().to_string()).collect())
            .unwrap_or_default(),
        cached_methods: dcb.allowed_methods()
            .and_then(|m| m.cached_methods())
            .map(|cm| cm.items().iter().map(|method| method.as_str().to_string()).collect())
            .unwrap_or_default(),
        compress: dcb.compress().unwrap_or(false),
        default_ttl: None,
        max_ttl: None,
        min_ttl: None,
        cache_policy_id: dcb.cache_policy_id().map(|s| s.to_string()),
        origin_request_policy_id: dcb.origin_request_policy_id().map(|s| s.to_string()),
    });

    let cache_behaviors: Vec<CacheBehavior> = config
        .and_then(|c| c.cache_behaviors())
        .map(|cbs| {
            cbs.items()
                .iter()
                .map(|cb| CacheBehavior {
                    path_pattern: Some(cb.path_pattern().to_string()),
                    target_origin_id: cb.target_origin_id().to_string(),
                    viewer_protocol_policy: cb.viewer_protocol_policy().as_str().to_string(),
                    allowed_methods: cb.allowed_methods()
                        .map(|m| m.items().iter().map(|method| method.as_str().to_string()).collect())
                        .unwrap_or_default(),
                    cached_methods: cb.allowed_methods()
                        .and_then(|m| m.cached_methods())
                        .map(|cm| cm.items().iter().map(|method| method.as_str().to_string()).collect())
                        .unwrap_or_default(),
                    compress: cb.compress().unwrap_or(false),
                    default_ttl: None,
                    max_ttl: None,
                    min_ttl: None,
                    cache_policy_id: cb.cache_policy_id().map(|s| s.to_string()),
                    origin_request_policy_id: cb.origin_request_policy_id().map(|s| s.to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    let last_modified = aws_datetime_to_chrono(dist.last_modified_time());

    GetDistributionOutput {
        distribution_id: dist.id().to_string(),
        domain_name: dist.domain_name().to_string(),
        arn: dist.arn().to_string(),
        status: dist.status().to_string(),
        enabled: config.map(|c| c.enabled()).unwrap_or(false),
        aliases,
        origins,
        default_cache_behavior,
        cache_behaviors,
        price_class: config.and_then(|c| c.price_class()).map(|p| p.as_str().to_string()).unwrap_or_else(|| "PriceClass_All".to_string()),
        last_modified_time: last_modified,
        etag: etag.to_string(),
        comment: config.map(|c| c.comment().to_string()).unwrap_or_default(),
    }
}

/// Create CloudFront Distribution
pub async fn create_distribution(
    origin_domain_name: &str,
    origin_id: &str,
    aliases: Option<Vec<String>>,
    cache_behaviors: Option<Vec<CacheBehavior>>,
    certificate_arn: Option<&str>,
    comment: Option<&str>,
    custom_error_responses: Option<Vec<CustomErrorResponse>>,
    default_cache_behavior: Option<CacheBehavior>,
    default_root_object: Option<&str>,
    enabled: Option<bool>,
    _minimum_protocol_version: Option<&str>,
    price_class: Option<&str>,
    _ssl_support_method: Option<&str>,
    tags: Option<CloudFrontTags>,
    region: Option<&str>,
) -> Result<CreateDistributionOutput, String> {
    let client = create_client(region).await?;
    let caller_reference = uuid::Uuid::new_v4().to_string();

    let s3_origin_config = AwsS3OriginConfig::builder().origin_access_identity("").build();
    let origin = Origin::builder()
        .id(origin_id)
        .domain_name(origin_domain_name)
        .s3_origin_config(s3_origin_config)
        .build()
        .map_err(|e| format!("Failed to build origin: {}", e))?;

    let origins = Origins::builder()
        .quantity(1)
        .items(origin)
        .build()
        .map_err(|e| format!("Failed to build origins: {}", e))?;

    let default_cb = if let Some(ref cb) = default_cache_behavior {
        to_default_cache_behavior(cb)?
    } else {
        let cached_methods = CachedMethods::builder()
            .quantity(2)
            .items(Method::Get)
            .items(Method::Head)
            .build()
            .map_err(|e| format!("Failed to build cached methods: {}", e))?;

        let allowed_methods = AllowedMethods::builder()
            .quantity(2)
            .items(Method::Get)
            .items(Method::Head)
            .cached_methods(cached_methods)
            .build()
            .map_err(|e| format!("Failed to build allowed methods: {}", e))?;

        DefaultCacheBehavior::builder()
            .target_origin_id(origin_id)
            .viewer_protocol_policy(ViewerProtocolPolicy::AllowAll)
            .cache_policy_id("658327ea-f89d-4fab-a63d-7e88639e58f6")
            .allowed_methods(allowed_methods)
            .compress(true)
            .build()
            .map_err(|e| format!("Failed to build default cache behavior: {}", e))?
    };

    let mut config_builder = DistributionConfig::builder()
        .caller_reference(&caller_reference)
        .origins(origins)
        .default_cache_behavior(default_cb)
        .comment(comment.unwrap_or(""))
        .enabled(enabled.unwrap_or(true));

    if let Some(ref alias_list) = aliases {
        config_builder = config_builder.aliases(
            Aliases::builder()
                .quantity(alias_list.len() as i32)
                .set_items(Some(alias_list.clone()))
                .build()
                .map_err(|e| format!("Failed to build aliases: {}", e))?
        );
    } else {
        config_builder = config_builder.aliases(
            Aliases::builder().quantity(0).build().map_err(|e| format!("Failed to build empty aliases: {}", e))?
        );
    }

    if let Some(root_obj) = default_root_object {
        config_builder = config_builder.default_root_object(root_obj);
    }

    if let Some(pc) = price_class {
        config_builder = config_builder.price_class(parse_price_class(pc));
    }

    if let Some(ref cbs) = cache_behaviors {
        let aws_cbs: Result<Vec<AwsCacheBehavior>, String> = cbs.iter().map(to_aws_cache_behavior).collect();
        config_builder = config_builder.cache_behaviors(
            CacheBehaviors::builder()
                .quantity(cbs.len() as i32)
                .set_items(Some(aws_cbs?))
                .build()
                .map_err(|e| format!("Failed to build cache behaviors: {}", e))?
        );
    } else {
        config_builder = config_builder.cache_behaviors(
            CacheBehaviors::builder().quantity(0).build().map_err(|e| format!("Failed to build empty cache behaviors: {}", e))?
        );
    }

    if let Some(ref errors) = custom_error_responses {
        let aws_errors: Vec<AwsCustomErrorResponse> = errors
            .iter()
            .filter_map(|e| {
                let mut builder = AwsCustomErrorResponse::builder().error_code(e.error_code);
                if let Some(code) = e.response_code { builder = builder.response_code(code.to_string()); }
                if let Some(ref path) = e.response_page_path { builder = builder.response_page_path(path); }
                if let Some(ttl) = e.error_caching_min_ttl { builder = builder.error_caching_min_ttl(ttl); }
                builder.build().ok()
            })
            .collect();
        config_builder = config_builder.custom_error_responses(
            CustomErrorResponses::builder()
                .quantity(errors.len() as i32)
                .set_items(Some(aws_errors))
                .build()
                .map_err(|e| format!("Failed to build custom error responses: {}", e))?
        );
    }

    if let Some(cert_arn) = certificate_arn {
        config_builder = config_builder.viewer_certificate(
            ViewerCertificate::builder()
                .acm_certificate_arn(cert_arn)
                .ssl_support_method(aws_sdk_cloudfront::types::SslSupportMethod::SniOnly)
                .minimum_protocol_version(aws_sdk_cloudfront::types::MinimumProtocolVersion::TlSv122021)
                .build()
        );
    }

    let distribution_config = config_builder.build().map_err(|e| format!("Failed to build distribution config: {}", e))?;

    let (distribution, etag) = if let Some(ref tag_data) = tags {
        let aws_tags: Vec<Tag> = tag_data.tags.iter().filter_map(|(k, v)| Tag::builder().key(k).value(v).build().ok()).collect();
        let dist_config_with_tags = aws_sdk_cloudfront::types::DistributionConfigWithTags::builder()
            .distribution_config(distribution_config)
            .tags(Tags::builder().set_items(Some(aws_tags)).build())
            .build();
        let response = client.create_distribution_with_tags().distribution_config_with_tags(dist_config_with_tags).send().await.map_err(|e| format!("Failed to create distribution with tags: {}", e))?;
        (response.distribution().ok_or("No distribution in response")?.clone(), response.e_tag().unwrap_or_default().to_string())
    } else {
        let response = client.create_distribution().distribution_config(distribution_config).send().await.map_err(|e| format!("Failed to create distribution: {}", e))?;
        (response.distribution().ok_or("No distribution in response")?.clone(), response.e_tag().unwrap_or_default().to_string())
    };

    Ok(CreateDistributionOutput {
        distribution_id: distribution.id().to_string(),
        domain_name: distribution.domain_name().to_string(),
        arn: distribution.arn().to_string(),
        status: distribution.status().to_string(),
        etag,
        success: true,
    })
}

/// Get CloudFront Distribution
pub async fn get_distribution(distribution_id: &str, region: Option<&str>) -> Result<GetDistributionOutput, String> {
    let client = create_client(region).await?;
    let response = client.get_distribution().id(distribution_id).send().await.map_err(|e| format!("Failed to get distribution: {}", e))?;
    let distribution = response.distribution().ok_or("No distribution in response")?;
    let etag = response.e_tag().unwrap_or_default();
    Ok(distribution_to_output(distribution, etag))
}

/// Update CloudFront Distribution
pub async fn update_distribution(
    distribution_id: &str,
    if_match: &str,
    aliases: Option<Vec<String>>,
    cache_behaviors: Option<Vec<CacheBehavior>>,
    comment: Option<&str>,
    custom_error_responses: Option<Vec<CustomErrorResponse>>,
    default_cache_behavior: Option<CacheBehavior>,
    default_root_object: Option<&str>,
    enabled: Option<bool>,
    region: Option<&str>,
) -> Result<UpdateDistributionOutput, String> {
    let client = create_client(region).await?;

    let current = client.get_distribution().id(distribution_id).send().await.map_err(|e| format!("Failed to get current distribution: {}", e))?;
    let current_dist = current.distribution().ok_or("No distribution in response")?;
    let current_config = current_dist.distribution_config().ok_or("No distribution config in response")?;

    let mut config_builder = DistributionConfig::builder()
        .caller_reference(current_config.caller_reference())
        .origins(current_config.origins().cloned().ok_or("No origins")?)
        .enabled(enabled.unwrap_or(current_config.enabled()))
        .comment(comment.unwrap_or(current_config.comment()));

    if let Some(ref cb) = default_cache_behavior {
        config_builder = config_builder.default_cache_behavior(to_default_cache_behavior(cb)?);
    } else if let Some(dcb) = current_config.default_cache_behavior() {
        config_builder = config_builder.default_cache_behavior(dcb.clone());
    }

    if let Some(ref alias_list) = aliases {
        config_builder = config_builder.aliases(Aliases::builder().quantity(alias_list.len() as i32).set_items(Some(alias_list.clone())).build().map_err(|e| format!("Failed to build aliases: {}", e))?);
    } else if let Some(a) = current_config.aliases() {
        config_builder = config_builder.aliases(a.clone());
    } else {
        config_builder = config_builder.aliases(Aliases::builder().quantity(0).build().map_err(|e| format!("Failed to build empty aliases: {}", e))?);
    }

    if let Some(ref cbs) = cache_behaviors {
        let aws_cbs: Result<Vec<AwsCacheBehavior>, String> = cbs.iter().map(to_aws_cache_behavior).collect();
        config_builder = config_builder.cache_behaviors(CacheBehaviors::builder().quantity(cbs.len() as i32).set_items(Some(aws_cbs?)).build().map_err(|e| format!("Failed to build cache behaviors: {}", e))?);
    } else if let Some(cbs) = current_config.cache_behaviors() {
        config_builder = config_builder.cache_behaviors(cbs.clone());
    } else {
        config_builder = config_builder.cache_behaviors(CacheBehaviors::builder().quantity(0).build().map_err(|e| format!("Failed to build empty cache behaviors: {}", e))?);
    }

    if let Some(root_obj) = default_root_object {
        config_builder = config_builder.default_root_object(root_obj);
    } else if let Some(obj) = current_config.default_root_object() {
        config_builder = config_builder.default_root_object(obj);
    }

    if let Some(ref errors) = custom_error_responses {
        let aws_errors: Vec<AwsCustomErrorResponse> = errors.iter().filter_map(|e| {
            let mut builder = AwsCustomErrorResponse::builder().error_code(e.error_code);
            if let Some(code) = e.response_code { builder = builder.response_code(code.to_string()); }
            if let Some(ref path) = e.response_page_path { builder = builder.response_page_path(path); }
            if let Some(ttl) = e.error_caching_min_ttl { builder = builder.error_caching_min_ttl(ttl); }
            builder.build().ok()
        }).collect();
        config_builder = config_builder.custom_error_responses(CustomErrorResponses::builder().quantity(errors.len() as i32).set_items(Some(aws_errors)).build().map_err(|e| format!("Failed to build custom error responses: {}", e))?);
    } else if let Some(errs) = current_config.custom_error_responses() {
        config_builder = config_builder.custom_error_responses(errs.clone());
    }

    if let Some(pc) = current_config.price_class() { config_builder = config_builder.price_class(pc.clone()); }
    if let Some(vc) = current_config.viewer_certificate() { config_builder = config_builder.viewer_certificate(vc.clone()); }
    if let Some(r) = current_config.restrictions() { config_builder = config_builder.restrictions(r.clone()); }
    if let Some(l) = current_config.logging() { config_builder = config_builder.logging(l.clone()); }
    if let Some(w) = current_config.web_acl_id() { config_builder = config_builder.web_acl_id(w); }
    if let Some(h) = current_config.http_version() { config_builder = config_builder.http_version(h.clone()); }
    if let Some(ipv6) = current_config.is_ipv6_enabled() { config_builder = config_builder.is_ipv6_enabled(ipv6); }
    if let Some(s) = current_config.staging() { config_builder = config_builder.staging(s); }

    let distribution_config = config_builder.build().map_err(|e| format!("Failed to build distribution config: {}", e))?;
    let response = client.update_distribution().id(distribution_id).if_match(if_match).distribution_config(distribution_config).send().await.map_err(|e| format!("Failed to update distribution: {}", e))?;
    let distribution = response.distribution().ok_or("No distribution in response")?;

    Ok(UpdateDistributionOutput {
        distribution_id: distribution.id().to_string(),
        status: distribution.status().to_string(),
        etag: response.e_tag().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete CloudFront Distribution
pub async fn delete_distribution(distribution_id: &str, if_match: &str, region: Option<&str>) -> Result<DeleteDistributionOutput, String> {
    let client = create_client(region).await?;
    client.delete_distribution().id(distribution_id).if_match(if_match).send().await.map_err(|e| format!("Failed to delete distribution: {}", e))?;
    Ok(DeleteDistributionOutput { success: true })
}

/// List CloudFront Distributions
pub async fn list_distributions(marker: Option<&str>, max_items: Option<i32>, region: Option<&str>) -> Result<ListDistributionsOutput, String> {
    let client = create_client(region).await?;
    let mut request = client.list_distributions();
    if let Some(m) = marker { request = request.marker(m); }
    if let Some(max) = max_items { request = request.max_items(max); }

    let response = request.send().await.map_err(|e| format!("Failed to list distributions: {}", e))?;
    let distribution_list = response.distribution_list();

    let distributions: Vec<CloudFrontDistribution> = distribution_list.map(|dl| {
        dl.items().iter().map(|summary| {
            CloudFrontDistribution {
                id: summary.id().to_string(),
                arn: summary.arn().to_string(),
                domain_name: summary.domain_name().to_string(),
                status: summary.status().to_string(),
                enabled: summary.enabled(),
                aliases: summary.aliases().map(|a| a.items().iter().map(|s| s.to_string()).collect()).unwrap_or_default(),
                comment: Some(summary.comment().to_string()),
                price_class: summary.price_class().as_str().to_string(),
                last_modified_time: aws_datetime_to_chrono(summary.last_modified_time()),
            }
        }).collect()
    }).unwrap_or_default();

    let next_marker = distribution_list.and_then(|dl| dl.next_marker().map(|s| s.to_string()));
    let is_truncated = distribution_list.map(|dl| dl.is_truncated()).unwrap_or(false);

    Ok(ListDistributionsOutput { distributions, next_marker, is_truncated })
}

/// Create Invalidation
pub async fn create_invalidation(distribution_id: &str, paths: Vec<String>, caller_reference: Option<&str>, region: Option<&str>) -> Result<CreateInvalidationOutput, String> {
    let client = create_client(region).await?;
    let caller_ref = caller_reference.map(|s| s.to_string()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let paths_config = Paths::builder().quantity(paths.len() as i32).set_items(Some(paths)).build().map_err(|e| format!("Failed to build paths: {}", e))?;
    let invalidation_batch = InvalidationBatch::builder().paths(paths_config).caller_reference(&caller_ref).build().map_err(|e| format!("Failed to build invalidation batch: {}", e))?;
    let response = client.create_invalidation().distribution_id(distribution_id).invalidation_batch(invalidation_batch).send().await.map_err(|e| format!("Failed to create invalidation: {}", e))?;
    let invalidation = response.invalidation().ok_or("No invalidation in response")?;

    Ok(CreateInvalidationOutput {
        invalidation_id: invalidation.id().to_string(),
        status: invalidation.status().to_string(),
        create_time: aws_datetime_to_chrono(invalidation.create_time()),
        success: true,
    })
}

/// Get Invalidation
pub async fn get_invalidation(distribution_id: &str, invalidation_id: &str, region: Option<&str>) -> Result<GetInvalidationOutput, String> {
    let client = create_client(region).await?;
    let response = client.get_invalidation().distribution_id(distribution_id).id(invalidation_id).send().await.map_err(|e| format!("Failed to get invalidation: {}", e))?;
    let invalidation = response.invalidation().ok_or("No invalidation in response")?;
    let paths: Vec<String> = invalidation.invalidation_batch()
        .and_then(|batch| batch.paths())
        .map(|p| p.items().iter().map(|s| s.to_string()).collect())
        .unwrap_or_default();

    Ok(GetInvalidationOutput {
        invalidation_id: invalidation.id().to_string(),
        status: invalidation.status().to_string(),
        create_time: aws_datetime_to_chrono(invalidation.create_time()),
        paths,
    })
}

/// List Invalidations
pub async fn list_invalidations(distribution_id: &str, marker: Option<&str>, max_items: Option<i32>, region: Option<&str>) -> Result<ListInvalidationsOutput, String> {
    let client = create_client(region).await?;
    let mut request = client.list_invalidations().distribution_id(distribution_id);
    if let Some(m) = marker { request = request.marker(m); }
    if let Some(max) = max_items { request = request.max_items(max); }

    let response = request.send().await.map_err(|e| format!("Failed to list invalidations: {}", e))?;
    let invalidation_list = response.invalidation_list();

    let invalidations: Vec<InvalidationSummary> = invalidation_list.map(|il| {
        il.items().iter().map(|summary| InvalidationSummary {
            id: summary.id().to_string(),
            create_time: aws_datetime_to_chrono(summary.create_time()),
            status: summary.status().to_string(),
        }).collect()
    }).unwrap_or_default();

    let next_marker = invalidation_list.and_then(|il| il.next_marker().map(|s| s.to_string()));
    let is_truncated = invalidation_list.map(|il| il.is_truncated()).unwrap_or(false);

    Ok(ListInvalidationsOutput { invalidations, next_marker, is_truncated })
}

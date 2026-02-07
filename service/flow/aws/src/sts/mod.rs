// Harana Actions - AWS STS Module
//
// This module provides AWS Security Token Service (STS) actions for managing
// temporary security credentials.

pub mod output;


use aws_config::BehaviorVersion;
use aws_sdk_sts::{
    config::Region,
    types::{PolicyDescriptorType, Tag},
    Client,
};
use chrono::{DateTime, Utc};
use output::*;
use std::collections::HashMap;
use serde_json::Value;

/// Creates an STS client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let sts_config = if let Some(region_str) = region {
        aws_sdk_sts::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_sts::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(sts_config))
}

/// Converts AWS SDK credentials to output credentials
fn convert_credentials(creds: &aws_sdk_sts::types::Credentials) -> Credentials {
    let exp = creds.expiration();
    Credentials {
        access_key_id: creds.access_key_id().to_string(),
        secret_access_key: creds.secret_access_key().to_string(),
        session_token: creds.session_token().to_string(),
        expiration: DateTime::<Utc>::from_timestamp(exp.secs(), exp.subsec_nanos()),
    }
}

/// Converts AWS SDK assumed role user to output type
fn convert_assumed_role_user(user: &aws_sdk_sts::types::AssumedRoleUser) -> AssumedRoleUser {
    AssumedRoleUser {
        assumed_role_id: user.assumed_role_id().to_string(),
        arn: user.arn().to_string(),
    }
}

/// Converts AWS SDK federated user to output type
fn convert_federated_user(user: &aws_sdk_sts::types::FederatedUser) -> FederatedUser {
    FederatedUser {
        federated_user_id: user.federated_user_id().to_string(),
        arn: user.arn().to_string(),
    }
}

/// Parse policy ARNs from a list of HashMaps
fn parse_policy_arns(policy_arns: Option<Vec<HashMap<String, Value>>>) -> Option<Vec<PolicyDescriptorType>> {
    policy_arns.map(|arns| {
        arns.into_iter()
            .filter_map(|map| {
                map.get("arn")
                    .and_then(|v| v.as_str())
                    .map(|arn| PolicyDescriptorType::builder().arn(arn).build())
            })
            .collect()
    })
}

/// Parse tags from a HashMap
fn parse_tags(tags: Option<HashMap<String, Value>>) -> Option<Vec<Tag>> {
    tags.map(|tag_map| {
        tag_map
            .into_iter()
            .filter_map(|(key, value)| {
                value.as_str().map(|v| {
                    Tag::builder()
                        .key(key)
                        .value(v)
                        .build()
                        .ok()
                })
            })
            .flatten()
            .collect()
    })
}

/// Assume STS Role
///
/// Returns a set of temporary security credentials that you can use to access
/// AWS resources that you might not normally have access to.
///
pub async fn assume_role(
    role_session_name: &str,
    role_arn: &str,
    policy: Option<&str>,
    serial_number: Option<&str>,
    source_identity: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    external_id: Option<&str>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    transitive_tag_keys: Option<Vec<String>>,
    token_code: Option<&str>,
    duration_seconds: Option<i32>,
    region: Option<&str>,
) -> Result<AssumeRoleOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .assume_role()
        .role_session_name(role_session_name)
        .role_arn(role_arn);

    if let Some(p) = policy {
        request = request.policy(p);
    }

    if let Some(sn) = serial_number {
        request = request.serial_number(sn);
    }

    if let Some(si) = source_identity {
        request = request.source_identity(si);
    }

    if let Some(parsed_tags) = parse_tags(tags) {
        request = request.set_tags(Some(parsed_tags));
    }

    if let Some(ext_id) = external_id {
        request = request.external_id(ext_id);
    }

    if let Some(parsed_arns) = parse_policy_arns(policy_arns) {
        request = request.set_policy_arns(Some(parsed_arns));
    }

    if let Some(ttk) = transitive_tag_keys {
        request = request.set_transitive_tag_keys(Some(ttk));
    }

    if let Some(tc) = token_code {
        request = request.token_code(tc);
    }

    if let Some(ds) = duration_seconds {
        request = request.duration_seconds(ds);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to assume role: {}", e))?;

    Ok(AssumeRoleOutput {
        credentials: response.credentials().map(convert_credentials),
        assumed_role_user: response.assumed_role_user().map(convert_assumed_role_user),
        packed_policy_size: response.packed_policy_size(),
        source_identity: response.source_identity().map(|s| s.to_string()),
        success: true,
    })
}

/// Assume STS Role With SAML
///
/// Returns a set of temporary security credentials for users who have been
/// authenticated via a SAML authentication response.
///
pub async fn assume_role_with_saml(
    role_arn: &str,
    saml_assertion: &str,
    principal_arn: &str,
    region: Option<&str>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    duration_seconds: Option<i32>,
    policy: Option<&str>,
) -> Result<AssumeRoleWithSamlOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .assume_role_with_saml()
        .role_arn(role_arn)
        .saml_assertion(saml_assertion)
        .principal_arn(principal_arn);

    if let Some(parsed_arns) = parse_policy_arns(policy_arns) {
        request = request.set_policy_arns(Some(parsed_arns));
    }

    if let Some(ds) = duration_seconds {
        request = request.duration_seconds(ds);
    }

    if let Some(p) = policy {
        request = request.policy(p);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to assume role with SAML: {}", e))?;

    Ok(AssumeRoleWithSamlOutput {
        credentials: response.credentials().map(convert_credentials),
        assumed_role_user: response.assumed_role_user().map(convert_assumed_role_user),
        packed_policy_size: response.packed_policy_size(),
        subject: response.subject().map(|s| s.to_string()),
        subject_type: response.subject_type().map(|s| s.to_string()),
        issuer: response.issuer().map(|s| s.to_string()),
        audience: response.audience().map(|s| s.to_string()),
        name_qualifier: response.name_qualifier().map(|s| s.to_string()),
        source_identity: response.source_identity().map(|s| s.to_string()),
        success: true,
    })
}

/// Assume STS Role With Web Identity
///
/// Returns a set of temporary security credentials for users who have been
/// authenticated in a mobile or web application with a web identity provider.
///
pub async fn assume_role_with_web_identity(
    role_session_name: &str,
    role_arn: &str,
    web_identity_token: &str,
    duration_seconds: Option<i32>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    provider_id: Option<&str>,
    policy: Option<&str>,
    region: Option<&str>,
) -> Result<AssumeRoleWithWebIdentityOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .assume_role_with_web_identity()
        .role_session_name(role_session_name)
        .role_arn(role_arn)
        .web_identity_token(web_identity_token);

    if let Some(ds) = duration_seconds {
        request = request.duration_seconds(ds);
    }

    if let Some(parsed_arns) = parse_policy_arns(policy_arns) {
        request = request.set_policy_arns(Some(parsed_arns));
    }

    if let Some(pid) = provider_id {
        request = request.provider_id(pid);
    }

    if let Some(p) = policy {
        request = request.policy(p);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to assume role with web identity: {}", e))?;

    Ok(AssumeRoleWithWebIdentityOutput {
        credentials: response.credentials().map(convert_credentials),
        assumed_role_user: response.assumed_role_user().map(convert_assumed_role_user),
        packed_policy_size: response.packed_policy_size(),
        provider: response.provider().map(|s| s.to_string()),
        audience: response.audience().map(|s| s.to_string()),
        subject_from_web_identity_token: response.subject_from_web_identity_token().map(|s| s.to_string()),
        source_identity: response.source_identity().map(|s| s.to_string()),
        success: true,
    })
}

/// Decode STS Authorization Message
///
/// Decodes additional information about the authorization status of a request
/// from an encoded message returned in response to an AWS request.
///
pub async fn decode_authorization_message(
    encoded_message: &str,
    region: Option<&str>,
) -> Result<DecodeAuthorizationMessageOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .decode_authorization_message()
        .encoded_message(encoded_message)
        .send()
        .await
        .map_err(|e| format!("Failed to decode authorization message: {}", e))?;

    Ok(DecodeAuthorizationMessageOutput {
        decoded_message: response.decoded_message().map(|s| s.to_string()),
        success: true,
    })
}

/// Get STS Access Key Info
///
/// Returns the account identifier for the specified access key ID.
///
pub async fn get_access_key_info(
    access_key_id: &str,
    region: Option<&str>,
) -> Result<GetAccessKeyInfoOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_access_key_info()
        .access_key_id(access_key_id)
        .send()
        .await
        .map_err(|e| format!("Failed to get access key info: {}", e))?;

    Ok(GetAccessKeyInfoOutput {
        account: response.account().map(|s| s.to_string()),
        success: true,
    })
}

/// Get STS Caller Identity
///
/// Returns details about the IAM user or role whose credentials are used to call the operation.
///
pub async fn get_caller_identity(
    region: Option<&str>,
) -> Result<GetCallerIdentityOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_caller_identity()
        .send()
        .await
        .map_err(|e| format!("Failed to get caller identity: {}", e))?;

    Ok(GetCallerIdentityOutput {
        user_id: response.user_id().map(|s| s.to_string()),
        account: response.account().map(|s| s.to_string()),
        arn: response.arn().map(|s| s.to_string()),
        success: true,
    })
}

/// Get STS Federation Token
///
/// Returns a set of temporary security credentials for a federated user.
///
pub async fn get_federation_token(
    name: &str,
    tags: Option<HashMap<String, Value>>,
    policy: Option<&str>,
    region: Option<&str>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    duration_seconds: Option<i32>,
) -> Result<GetFederationTokenOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_federation_token().name(name);

    if let Some(parsed_tags) = parse_tags(tags) {
        request = request.set_tags(Some(parsed_tags));
    }

    if let Some(p) = policy {
        request = request.policy(p);
    }

    if let Some(parsed_arns) = parse_policy_arns(policy_arns) {
        request = request.set_policy_arns(Some(parsed_arns));
    }

    if let Some(ds) = duration_seconds {
        request = request.duration_seconds(ds);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get federation token: {}", e))?;

    Ok(GetFederationTokenOutput {
        credentials: response.credentials().map(convert_credentials),
        federated_user: response.federated_user().map(convert_federated_user),
        packed_policy_size: response.packed_policy_size(),
        success: true,
    })
}

/// Get STS Session Token
///
/// Returns a set of temporary credentials for an AWS account or IAM user.
///
pub async fn get_session_token(
    token_code: Option<&str>,
    serial_number: Option<&str>,
    region: Option<&str>,
    duration_seconds: Option<i32>,
) -> Result<GetSessionTokenOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.get_session_token();

    if let Some(tc) = token_code {
        request = request.token_code(tc);
    }

    if let Some(sn) = serial_number {
        request = request.serial_number(sn);
    }

    if let Some(ds) = duration_seconds {
        request = request.duration_seconds(ds);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to get session token: {}", e))?;

    Ok(GetSessionTokenOutput {
        credentials: response.credentials().map(convert_credentials),
        success: true,
    })
}

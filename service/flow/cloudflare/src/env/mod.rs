// Harana Actions - Cloudflare Environment Module
// This module provides Cloudflare Workers environment actions for accessing
// variables, secrets, and version metadata.

pub mod output;

use output::*;
use worker::Env;

fn to_err(e: impl std::fmt::Display) -> String {
    format!("Env error: {e}")
}

/// Get Environment Variable
pub async fn get_var(env: &Env, name: &str) -> Result<GetVarOutput, String> {
    match env.var(name) {
        Ok(var) => Ok(GetVarOutput {
            found: true,
            value: var.to_string(),
        }),
        Err(_) => Ok(GetVarOutput {
            found: false,
            value: String::new(),
        }),
    }
}

/// Get Secret
pub async fn get_secret(env: &Env, name: &str) -> Result<GetSecretOutput, String> {
    let secret = env.secret(name).map_err(to_err)?;
    Ok(GetSecretOutput {
        value: secret.to_string(),
    })
}

/// Get Secret From Secrets Store
pub async fn get_store_secret(env: &Env, store_binding: &str, name: &str) -> Result<GetStoreSecretOutput, String> {
    let store: worker::SecretStore = env.get_binding(store_binding).map_err(to_err)?;
    let secret_value = store.get().await.map_err(to_err)?;

    match secret_value {
        Some(value) => Ok(GetStoreSecretOutput { value }),
        None => Err(format!("Secret '{}' not found in store '{}'", name, store_binding)),
    }
}

/// List Environment Variables
pub async fn list_vars(_env: &Env) -> Result<ListVarsOutput, String> {
    // The worker crate does not provide a way to enumerate all variables.
    // Variables must be accessed by name. Return empty list.
    Ok(ListVarsOutput { variables: vec![] })
}

/// Get Version Metadata
pub async fn get_version(env: &Env, binding: &str) -> Result<GetVersionOutput, String> {
    let version: worker::WorkerVersionMetadata = env.get_binding(binding).map_err(to_err)?;

    Ok(GetVersionOutput {
        id: version.id(),
        message: String::new(),
        tag: version.tag(),
        timestamp: version.timestamp(),
    })
}

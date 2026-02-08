// Harana Actions - Cloudflare Hyperdrive Module
// This module provides Cloudflare Hyperdrive actions for connecting to databases
// through Cloudflare's connection pooling and caching infrastructure.

pub mod output;

use output::*;
use worker::Env;

fn to_err(e: worker::Error) -> String {
    format!("Hyperdrive error: {e}")
}

/// Hyperdrive Connect
pub async fn connect(env: &Env, binding: &str) -> Result<ConnectOutput, String> {
    let hyperdrive = env.hyperdrive(binding).map_err(to_err)?;

    Ok(ConnectOutput {
        connection_string: hyperdrive.connection_string(),
        host: hyperdrive.host(),
        port: hyperdrive.port() as i32,
        user: hyperdrive.user(),
        password: hyperdrive.password(),
    })
}

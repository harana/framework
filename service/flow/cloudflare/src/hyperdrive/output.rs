// Harana Actions - Cloudflare Hyperdrive Module Output Types

use serde::{Deserialize, Serialize};

// connect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectOutput {
    pub connection_string: String,
    pub host: String,
    pub password: String,
    pub port: i32,
    pub user: String,
}

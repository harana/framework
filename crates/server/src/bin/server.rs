//! Example binary to run the HTTP server

use harana_components_http_server::{HttpServer, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    HttpServer::init_tracing();

    // Create server configuration
    let config = ServerConfig::default();
    
    tracing::info!("Starting Harana HTTP server...");
    tracing::info!("Features enabled:");
    tracing::info!("  ✅ Passkey/WebAuthn authentication");
    tracing::info!("  ✅ OAuth 2.0 / OpenID Connect");
    tracing::info!("  ✅ JWT token management");
    tracing::info!("  ✅ Datastar real-time SSE updates");
    tracing::info!("  ✅ Session management");

    // Create and run server
    let server = HttpServer::new(config);
    tracing::info!("Server listening on http://{}", server.address());
    
    server.run().await
}

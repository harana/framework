#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.address(), "127.0.0.1:3000");
    }

    #[cfg(feature = "standalone")]
    #[test]
    fn test_server_creation() {
        // Use a minimal in-memory cache for the test
        let cache = crate::auth::session::tests::memory_cache();
        let config = ServerConfig::default();
        let server = HttpServer::new(config, cache);
        assert_eq!(server.address(), "127.0.0.1:3000");
    }

}

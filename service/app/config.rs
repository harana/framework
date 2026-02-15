//use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
use harana_components_http_server::ServerConfig;plication configuration structures and builders.

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use harana_components_server::ServerConfig;

/// Main application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub name: String,
    
    /// Application version
    pub version: String,
    
    /// Application description
    pub description: String,
    
    /// Environment (e.g., "development", "staging", "production")
    pub environment: String,
    
    /// HTTP service configuration (if http feature is enabled)
    #[cfg(feature = "http")]
    pub http: Option<ServerConfig>,
    
    /// Additional custom configuration as key-value pairs
    #[serde(flatten)]
    pub custom: serde_json::Value,
}

impl AppConfig {
    /// Create a new configuration builder
    pub fn builder() -> AppConfigBuilder {
        AppConfigBuilder::new()
    }

    /// Load configuration from a YAML file
    pub fn from_yaml_file(path: &str) -> Result<Self, crate::error::AppError> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            crate::error::AppError::ConfigError(format!("Failed to read config file: {}", e))
        })?;
        
        Self::from_yaml(&content)
    }

    /// Load configuration from YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self, crate::error::AppError> {
        serde_yaml::from_str(yaml).map_err(|e| {
            crate::error::AppError::ConfigError(format!("Failed to parse YAML config: {}", e))
        })
    }

    /// Load configuration from JSON string
    pub fn from_json(json: &str) -> Result<Self, crate::error::AppError> {
        serde_json::from_str(json).map_err(|e| {
            crate::error::AppError::ConfigError(format!("Failed to parse JSON config: {}", e))
        })
    }

    /// Get a custom configuration value by key
    pub fn get_custom<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.custom.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

/// Builder for AppConfig
pub struct AppConfigBuilder {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    environment: Option<String>,
    #[cfg(feature = "http")]
    http: Option<ServerConfig>,
    custom: serde_json::Map<String, serde_json::Value>,
}

impl AppConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            name: None,
            version: None,
            description: None,
            environment: Some("development".to_string()),
            #[cfg(feature = "http")]
            http: None,
            custom: serde_json::Map::new(),
        }
    }

    /// Set the application name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the application version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the application description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the environment
    pub fn environment(mut self, environment: impl Into<String>) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// Set HTTP service configuration
    #[cfg(feature = "http")]
    pub fn http(mut self, http: ServerConfig) -> Self {
        self.http = Some(http);
        self
    }

    /// Add a custom configuration value
    pub fn custom<T: Serialize>(mut self, key: impl Into<String>, value: T) -> Self {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.custom.insert(key.into(), json_value);
        }
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<AppConfig, crate::error::AppError> {
        Ok(AppConfig {
            name: self.name.ok_or_else(|| {
                crate::error::AppError::ConfigError("Application name is required".to_string())
            })?,
            version: self.version.ok_or_else(|| {
                crate::error::AppError::ConfigError("Application version is required".to_string())
            })?,
            description: self.description.unwrap_or_else(|| "".to_string()),
            environment: self.environment.unwrap_or_else(|| "development".to_string()),
            #[cfg(feature = "http")]
            http: self.http,
            custom: serde_json::Value::Object(self.custom),
        })
    }
}

impl Default for AppConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = AppConfig::builder()
            .name("test-app")
            .version("1.0.0")
            .description("Test application")
            .environment("test")
            .custom("debug", true)
            .build()
            .unwrap();

        assert_eq!(config.name, "test-app");
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.description, "Test application");
        assert_eq!(config.environment, "test");
        assert_eq!(config.get_custom::<bool>("debug"), Some(true));
    }

    #[test]
    fn test_config_builder_missing_required() {
        let result = AppConfig::builder().version("1.0.0").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_json() {
        let json = r#"{
            "name": "json-app",
            "version": "2.0.0",
            "description": "From JSON",
            "environment": "production"
        }"#;

        let config = AppConfig::from_json(json).unwrap();
        assert_eq!(config.name, "json-app");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.environment, "production");
    }
}

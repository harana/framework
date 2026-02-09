use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub max_events: Option<usize>,
    pub buffer_size: usize,
    pub persistent: bool,
    pub default_ttl_seconds: Option<u64>,
    pub allow_duplicates: bool,
    pub allowed_event_types: HashSet<String>,
    pub durable: bool,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            max_events: None,
            buffer_size: 1024,
            persistent: false,
            default_ttl_seconds: None,
            allow_duplicates: false,
            allowed_event_types: HashSet::new(),
            durable: false,
        }
    }
}

impl ChannelConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration for a persistent channel
    pub fn persistent() -> Self {
        Self {
            persistent: true,
            durable: true,
            ..Default::default()
        }
    }

    /// Create a configuration for an in-memory only channel
    pub fn in_memory() -> Self {
        Self {
            persistent: false,
            durable: false,
            ..Default::default()
        }
    }

    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events = Some(max);
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_persistence(mut self, enabled: bool) -> Self {
        self.persistent = enabled;
        self
    }

    pub fn with_default_ttl(mut self, ttl_seconds: u64) -> Self {
        self.default_ttl_seconds = Some(ttl_seconds);
        self
    }

    pub fn with_allowed_types(mut self, types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allowed_event_types = types.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_durability(mut self, durable: bool) -> Self {
        self.durable = durable;
        self
    }
    pub fn is_event_type_allowed(&self, event_type: &str) -> bool {
        self.allowed_event_types.is_empty() || self.allowed_event_types.contains(event_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub config: ChannelConfig,
    pub subscriber_count: usize,
    pub total_events: u64,
    pub pending_events: u64,
    pub active: bool,
}

impl Channel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            config: ChannelConfig::default(),
            subscriber_count: 0,
            total_events: 0,
            pending_events: 0,
            active: true,
        }
    }

    pub fn with_config(name: impl Into<String>, config: ChannelConfig) -> Self {
        Self {
            name: name.into(),
            config,
            subscriber_count: 0,
            total_events: 0,
            pending_events: 0,
            active: true,
        }
    }
}

// Implement Entity trait for storage compatibility
impl harana_components_storage::Entity for Channel {
    fn id(&self) -> &str {
        &self.name
    }

    fn entity_type() -> &'static str {
        "channel"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_config_defaults() {
        let config = ChannelConfig::default();
        assert!(!config.persistent);
        assert!(!config.durable);
        assert!(config.allowed_event_types.is_empty());
    }

    #[test]
    fn test_channel_config_persistent() {
        let config = ChannelConfig::persistent();
        assert!(config.persistent);
        assert!(config.durable);
    }

    #[test]
    fn test_allowed_event_types() {
        let config = ChannelConfig::new().with_allowed_types(vec!["user.created", "user.updated"]);

        assert!(config.is_event_type_allowed("user.created"));
        assert!(config.is_event_type_allowed("user.updated"));
        assert!(!config.is_event_type_allowed("user.deleted"));
    }

    #[test]
    fn test_empty_allowed_types_allows_all() {
        let config = ChannelConfig::new();
        assert!(config.is_event_type_allowed("any.event.type"));
    }
}

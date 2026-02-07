
// Harana Components - SQL Text Search Implementation
// Provides full-text search support for SQL databases.

use async_trait::async_trait;
use serde_json::Value as JsonValue;

use crate::{Entity, StorageResult, Store};

/// SQL-specific text search filter that includes both column specification
/// and additional filter conditions.
#[derive(Debug, Clone, Default)]
pub struct SqlTextSearchFilter {
        pub search_columns: Vec<String>,
        pub filters: Option<JsonValue>,
}

impl SqlTextSearchFilter {
    /// Creates a new SQL text search filter with the specified columns.
    pub fn new(search_columns: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            search_columns: search_columns.into_iter().map(Into::into).collect(),
            filters: None,
        }
    }

    /// Creates a filter from a slice of column names.
    pub fn from_columns(columns: &[&str]) -> Self {
        Self {
            search_columns: columns.iter().map(|s| s.to_string()).collect(),
            filters: None,
        }
    }

    /// Adds additional filter conditions.
    pub fn with_filters(mut self, filters: JsonValue) -> Self {
        self.filters = Some(filters);
        self
    }
}

/// Trait for SQL-specific text search operations.
/// This extends the generic `Store` with SQL-specific convenience methods.
/// Different SQL databases have different full-text search implementations:
/// - PostgreSQL: Uses tsvector/tsquery with GIN indexes
/// - MySQL: Uses FULLTEXT indexes with MATCH AGAINST
/// - SQLite: Uses FTS5 virtual tables
#[async_trait]
pub trait SqlTextSearchBackend<T: Entity>: Store<T, Filter = SqlTextSearchFilter> {
    /// Perform a full-text search on specific columns.
    async fn text_search_columns(
        &self,
        text: &str,
        search_columns: &[&str],
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        let filter = SqlTextSearchFilter::from_columns(search_columns);
        self.text_search_with_filter(text, filter, limit, offset.map(|o| o as u64)).await
    }

    /// Perform a full-text search with additional filter conditions.
    async fn text_search_columns_with_filter(
        &self,
        text: &str,
        search_columns: &[&str],
        filters: JsonValue,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        let filter = SqlTextSearchFilter::from_columns(search_columns).with_filters(filters);
        self.text_search_with_filter(text, filter, limit, offset.map(|o| o as u64)).await
    }
}

/// Configuration for text search behavior
#[derive(Debug, Clone)]
pub struct TextSearchConfig {
        pub language: String,
        pub prefix_match: bool,
        pub min_word_length: Option<usize>,
}

impl Default for TextSearchConfig {
    fn default() -> Self {
        Self {
            language: "english".to_string(),
            prefix_match: false,
            min_word_length: None,
        }
    }
}

impl TextSearchConfig {
    /// Creates a new text search configuration with the specified language.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            ..Default::default()
        }
    }

    /// Enables prefix matching for autocomplete-style searches.
    pub fn with_prefix_match(mut self) -> Self {
        self.prefix_match = true;
        self
    }

    /// Sets the minimum word length for search terms.
    pub fn with_min_word_length(mut self, len: usize) -> Self {
        self.min_word_length = Some(len);
        self
    }
}

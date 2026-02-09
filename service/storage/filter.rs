use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct QueryOptions {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_desc: bool,
}

impl QueryOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_sort(mut self, field: impl Into<String>, desc: bool) -> Self {
        self.sort_by = Some(field.into());
        self.sort_desc = desc;
        self
    }
}

#[derive(Debug, Clone)]
pub enum FilterCondition {
    Eq(String, Value),
    Ne(String, Value),
    Gt(String, Value),
    Gte(String, Value),
    Lt(String, Value),
    Lte(String, Value),
    In(String, Vec<Value>),
    NotIn(String, Vec<Value>),
    Contains(String, String),
    StartsWith(String, String),
    EndsWith(String, String),
    IsNull(String),
    IsNotNull(String),
    And(Vec<FilterCondition>),
    Or(Vec<FilterCondition>),
}

impl FilterCondition {
    /// Helper to serialize a value, falling back to null on error.
    fn to_json(value: impl Serialize) -> Value {
        serde_json::to_value(value).unwrap_or(Value::Null)
    }

    pub fn eq(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Eq(field.into(), Self::to_json(value))
    }

    pub fn ne(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Ne(field.into(), Self::to_json(value))
    }

    pub fn gt(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Gt(field.into(), Self::to_json(value))
    }

    pub fn gte(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Gte(field.into(), Self::to_json(value))
    }

    pub fn lt(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Lt(field.into(), Self::to_json(value))
    }

    pub fn lte(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Lte(field.into(), Self::to_json(value))
    }

    pub fn is_in(field: impl Into<String>, values: impl IntoIterator<Item = impl Serialize>) -> Self {
        Self::In(field.into(), values.into_iter().map(Self::to_json).collect())
    }

    pub fn not_in(field: impl Into<String>, values: impl IntoIterator<Item = impl Serialize>) -> Self {
        Self::NotIn(field.into(), values.into_iter().map(Self::to_json).collect())
    }

    pub fn contains(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Contains(field.into(), value.into())
    }

    pub fn starts_with(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::StartsWith(field.into(), value.into())
    }

    pub fn ends_with(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::EndsWith(field.into(), value.into())
    }

    pub fn is_null(field: impl Into<String>) -> Self {
        Self::IsNull(field.into())
    }

    pub fn is_not_null(field: impl Into<String>) -> Self {
        Self::IsNotNull(field.into())
    }

    pub fn and(conditions: Vec<FilterCondition>) -> Self {
        Self::And(conditions)
    }

    pub fn or(conditions: Vec<FilterCondition>) -> Self {
        Self::Or(conditions)
    }
}

// Harana Components - D1 Query Builder
//
// Translates `FilterCondition` into SQLite-compatible WHERE clauses with `?`
// placeholders. Parameters are collected in order and later bound to the
// query via `bind()`.

use serde_json::Value;

use crate::{FilterCondition, QueryOptions, StorageResult};

pub struct D1QueryBuilder {
    params: Vec<Value>,
}

impl D1QueryBuilder {
    /// Creates a new, empty query builder.
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }

    /// Returns the collected bind parameters in order.
    pub fn params(&self) -> &[Value] {
        &self.params
    }

    /// Consumes the builder and returns the owned parameter list.
    pub fn into_params(self) -> Vec<Value> {
        self.params
    }

    /// Builds a SQL WHERE clause (without the `WHERE` keyword) from a
    /// [`FilterCondition`].  Appends bind parameters to `self.params`.
    pub fn build_where(&mut self, filter: &FilterCondition) -> StorageResult<String> {
        match filter {
            FilterCondition::Eq(field, value) => {
                self.params.push(value.clone());
                Ok(format!("\"{}\" = ?", field))
            }
            FilterCondition::Ne(field, value) => {
                self.params.push(value.clone());
                Ok(format!("\"{}\" != ?", field))
            }
            FilterCondition::Gt(field, value) => {
                self.params.push(value.clone());
                Ok(format!("\"{}\" > ?", field))
            }
            FilterCondition::Gte(field, value) => {
                self.params.push(value.clone());
                Ok(format!("\"{}\" >= ?", field))
            }
            FilterCondition::Lt(field, value) => {
                self.params.push(value.clone());
                Ok(format!("\"{}\" < ?", field))
            }
            FilterCondition::Lte(field, value) => {
                self.params.push(value.clone());
                Ok(format!("\"{}\" <= ?", field))
            }
            FilterCondition::In(field, values) => {
                if values.is_empty() {
                    // IN () is invalid SQL; return a falsy condition
                    return Ok("0 = 1".to_string());
                }
                let placeholders = values.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                self.params.extend(values.iter().cloned());
                Ok(format!("\"{}\" IN ({})", field, placeholders))
            }
            FilterCondition::NotIn(field, values) => {
                if values.is_empty() {
                    // NOT IN () is always true
                    return Ok("1 = 1".to_string());
                }
                let placeholders = values.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                self.params.extend(values.iter().cloned());
                Ok(format!("\"{}\" NOT IN ({})", field, placeholders))
            }
            FilterCondition::Contains(field, text) => {
                self.params.push(Value::String(format!("%{}%", text)));
                Ok(format!("\"{}\" LIKE ?", field))
            }
            FilterCondition::StartsWith(field, text) => {
                self.params.push(Value::String(format!("{}%", text)));
                Ok(format!("\"{}\" LIKE ?", field))
            }
            FilterCondition::EndsWith(field, text) => {
                self.params.push(Value::String(format!("%{}", text)));
                Ok(format!("\"{}\" LIKE ?", field))
            }
            FilterCondition::IsNull(field) => Ok(format!("\"{}\" IS NULL", field)),
            FilterCondition::IsNotNull(field) => Ok(format!("\"{}\" IS NOT NULL", field)),
            FilterCondition::And(conditions) => {
                if conditions.is_empty() {
                    return Ok("1 = 1".to_string());
                }
                let clauses: Vec<String> = conditions
                    .iter()
                    .map(|c| self.build_where(c))
                    .collect::<Result<_, _>>()?;
                Ok(format!("({})", clauses.join(" AND ")))
            }
            FilterCondition::Or(conditions) => {
                if conditions.is_empty() {
                    return Ok("0 = 1".to_string());
                }
                let clauses: Vec<String> = conditions
                    .iter()
                    .map(|c| self.build_where(c))
                    .collect::<Result<_, _>>()?;
                Ok(format!("({})", clauses.join(" OR ")))
            }
        }
    }

    /// Generates an `ORDER BY` clause from query options.
    pub fn build_order_by(options: &QueryOptions) -> Option<String> {
        options.sort_by.as_ref().map(|field| {
            let dir = if options.sort_desc { "DESC" } else { "ASC" };
            format!("ORDER BY \"{}\" {}", field, dir)
        })
    }

    /// Generates a `LIMIT` clause from query options.
    pub fn build_limit(options: &QueryOptions) -> Option<String> {
        options.limit.map(|l| format!("LIMIT {}", l))
    }

    /// Generates an `OFFSET` clause from query options.
    pub fn build_offset(options: &QueryOptions) -> Option<String> {
        options.offset.map(|o| format!("OFFSET {}", o))
    }
}

#[cfg(test)]
mod tests;

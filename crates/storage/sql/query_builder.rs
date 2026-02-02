// Harana Components - SQL Query Builder

use serde_json::Value;

use crate::{FilterCondition, QueryOptions, StorageResult};

pub struct SqlQueryBuilder {
    params: Vec<Value>,
    param_counter: usize,
}

impl SqlQueryBuilder {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            param_counter: 0,
        }
    }

    fn next_param(&mut self) -> String {
        self.param_counter += 1;
        format!("${}", self.param_counter)
    }

    /// Builds a comparison clause (e.g., "field" = $1)
    fn build_comparison(&mut self, field: &str, op: &str, value: &Value) -> String {
        let param = self.next_param();
        self.params.push(value.clone());
        format!("\"{field}\" {op} {param}")
    }

    pub fn build_where(&mut self, filter: &FilterCondition) -> StorageResult<String> {
        Ok(match filter {
            FilterCondition::Eq(field, value) => self.build_comparison(field, "=", value),
            FilterCondition::Ne(field, value) => self.build_comparison(field, "!=", value),
            FilterCondition::Gt(field, value) => self.build_comparison(field, ">", value),
            FilterCondition::Gte(field, value) => self.build_comparison(field, ">=", value),
            FilterCondition::Lt(field, value) => self.build_comparison(field, "<", value),
            FilterCondition::Lte(field, value) => self.build_comparison(field, "<=", value),

            FilterCondition::In(field, values) => {
                let placeholders = self.build_value_list(values);
                format!("\"{field}\" IN ({placeholders})")
            }
            FilterCondition::NotIn(field, values) => {
                let placeholders = self.build_value_list(values);
                format!("\"{field}\" NOT IN ({placeholders})")
            }

            FilterCondition::Contains(field, value) => {
                let param = self.next_param();
                self.params.push(Value::String(format!("%{value}%")));
                format!("\"{field}\" ILIKE {param}")
            }
            FilterCondition::StartsWith(field, value) => {
                let param = self.next_param();
                self.params.push(Value::String(format!("{value}%")));
                format!("\"{field}\" LIKE {param}")
            }
            FilterCondition::EndsWith(field, value) => {
                let param = self.next_param();
                self.params.push(Value::String(format!("%{value}")));
                format!("\"{field}\" LIKE {param}")
            }

            FilterCondition::IsNull(field) => format!("\"{field}\" IS NULL"),
            FilterCondition::IsNotNull(field) => format!("\"{field}\" IS NOT NULL"),

            FilterCondition::And(conditions) => self.build_logical("AND", conditions)?,
            FilterCondition::Or(conditions) => self.build_logical("OR", conditions)?,
        })
    }

    fn build_value_list(&mut self, values: &[Value]) -> String {
        values
            .iter()
            .map(|v| {
                let param = self.next_param();
                self.params.push(v.clone());
                param
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn build_logical(&mut self, op: &str, conditions: &[FilterCondition]) -> StorageResult<String> {
        let clauses: Result<Vec<_>, _> = conditions.iter().map(|c| self.build_where(c)).collect();
        Ok(format!("({})", clauses?.join(&format!(" {op} "))))
    }

    pub fn params(&self) -> &[Value] {
        &self.params
    }

    pub fn build_order_by(options: &QueryOptions) -> Option<String> {
        options.sort_by.as_ref().map(|field| {
            let direction = if options.sort_desc { "DESC" } else { "ASC" };
            format!("ORDER BY \"{field}\" {direction}")
        })
    }

    pub fn build_limit(options: &QueryOptions) -> Option<String> {
        options.limit.map(|limit| format!("LIMIT {limit}"))
    }

    pub fn build_offset(options: &QueryOptions) -> Option<String> {
        options.offset.map(|offset| format!("OFFSET {offset}"))
    }
}

impl Default for SqlQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_query_builder_eq() {
        let mut builder = SqlQueryBuilder::new();
        let filter = FilterCondition::eq("name", "John");
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "\"name\" = $1");
        assert_eq!(builder.params().len(), 1);
    }

    #[test]
    fn test_sql_query_builder_and() {
        let mut builder = SqlQueryBuilder::new();
        let filter = FilterCondition::and(vec![
            FilterCondition::eq("active", true),
            FilterCondition::gt("age", 18),
        ]);
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "(\"active\" = $1 AND \"age\" > $2)");
        assert_eq!(builder.params().len(), 2);
    }

    #[test]
    fn test_sql_query_builder_or() {
        let mut builder = SqlQueryBuilder::new();
        let filter = FilterCondition::or(vec![
            FilterCondition::eq("status", "active"),
            FilterCondition::eq("status", "pending"),
        ]);
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "(\"status\" = $1 OR \"status\" = $2)");
    }

    #[test]
    fn test_build_order_by() {
        let options = QueryOptions::new().with_sort("created_at", true);
        let order_by = SqlQueryBuilder::build_order_by(&options);
        assert_eq!(order_by, Some("ORDER BY \"created_at\" DESC".to_string()));
    }

    #[test]
    fn test_build_limit_offset() {
        let options = QueryOptions::new().with_limit(10).with_offset(20);
        assert_eq!(SqlQueryBuilder::build_limit(&options), Some("LIMIT 10".to_string()));
        assert_eq!(SqlQueryBuilder::build_offset(&options), Some("OFFSET 20".to_string()));
    }
}

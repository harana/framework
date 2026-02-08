#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_eq() {
        let mut builder = D1QueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::eq("name", "Alice")).unwrap();
        assert_eq!(clause, "\"name\" = ?");
        assert_eq!(builder.params(), &[json!("Alice")]);
    }

    #[test]
    fn test_compound_and() {
        let filter = FilterCondition::and(vec![
            FilterCondition::eq("status", "active"),
            FilterCondition::gte("age", 18),
        ]);
        let mut builder = D1QueryBuilder::new();
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "(\"status\" = ? AND \"age\" >= ?)");
        assert_eq!(builder.params().len(), 2);
    }

    #[test]
    fn test_contains() {
        let mut builder = D1QueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::contains("bio", "rust")).unwrap();
        assert_eq!(clause, "\"bio\" LIKE ?");
        assert_eq!(builder.params(), &[json!("%rust%")]);
    }

    #[test]
    fn test_in_values() {
        let filter = FilterCondition::is_in("id", vec!["a", "b", "c"]);
        let mut builder = D1QueryBuilder::new();
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "\"id\" IN (?, ?, ?)");
        assert_eq!(builder.params().len(), 3);
    }

    #[test]
    fn test_empty_in() {
        let filter = FilterCondition::In("id".to_string(), vec![]);
        let mut builder = D1QueryBuilder::new();
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "0 = 1");
        assert!(builder.params().is_empty());
    }

    #[test]
    fn test_is_null() {
        let mut builder = D1QueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::is_null("email")).unwrap();
        assert_eq!(clause, "\"email\" IS NULL");
        assert!(builder.params().is_empty());
    }

    #[test]
    fn test_nested_or_and() {
        let filter = FilterCondition::or(vec![
            FilterCondition::and(vec![
                FilterCondition::eq("status", "active"),
                FilterCondition::gt("age", 21),
            ]),
            FilterCondition::eq("admin", true),
        ]);
        let mut builder = D1QueryBuilder::new();
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "((\"status\" = ? AND \"age\" > ?) OR \"admin\" = ?)");
        assert_eq!(builder.params().len(), 3);
    }

    #[test]
    fn test_order_by() {
        let options = QueryOptions::new().with_sort("created_at", true);
        assert_eq!(
            D1QueryBuilder::build_order_by(&options),
            Some("ORDER BY \"created_at\" DESC".to_string())
        );
    }

    #[test]
    fn test_limit_offset() {
        let options = QueryOptions::new().with_limit(10).with_offset(20);
        assert_eq!(D1QueryBuilder::build_limit(&options), Some("LIMIT 10".to_string()));
        assert_eq!(D1QueryBuilder::build_offset(&options), Some("OFFSET 20".to_string()));
    }

}

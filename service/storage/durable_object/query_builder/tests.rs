#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_eq() {
        let mut builder = DOQueryBuilder::new();
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
        let mut builder = DOQueryBuilder::new();
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "(\"status\" = ? AND \"age\" >= ?)");
        assert_eq!(builder.params().len(), 2);
    }

    #[test]
    fn test_contains() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::contains("bio", "rust")).unwrap();
        assert_eq!(clause, "\"bio\" LIKE ?");
        assert_eq!(builder.params(), &[json!("%rust%")]);
    }

    #[test]
    fn test_starts_with() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder
            .build_where(&FilterCondition::starts_with("name", "Jo"))
            .unwrap();
        assert_eq!(clause, "\"name\" LIKE ?");
        assert_eq!(builder.params(), &[json!("Jo%")]);
    }

    #[test]
    fn test_ends_with() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder
            .build_where(&FilterCondition::ends_with("email", ".com"))
            .unwrap();
        assert_eq!(clause, "\"email\" LIKE ?");
        assert_eq!(builder.params(), &[json!("%.com")]);
    }

    #[test]
    fn test_is_null() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::is_null("deleted_at")).unwrap();
        assert_eq!(clause, "\"deleted_at\" IS NULL");
        assert!(builder.params().is_empty());
    }

    #[test]
    fn test_is_not_null() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::is_not_null("email")).unwrap();
        assert_eq!(clause, "\"email\" IS NOT NULL");
        assert!(builder.params().is_empty());
    }

    #[test]
    fn test_in_filter() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder
            .build_where(&FilterCondition::is_in("status", vec!["a", "b", "c"]))
            .unwrap();
        assert_eq!(clause, "\"status\" IN (?, ?, ?)");
        assert_eq!(builder.params().len(), 3);
    }

    #[test]
    fn test_in_empty() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder
            .build_where(&FilterCondition::is_in::<&str, &str>("status", vec![]))
            .unwrap();
        assert_eq!(clause, "0 = 1");
    }

    #[test]
    fn test_not_in_empty() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder
            .build_where(&FilterCondition::not_in::<&str, &str>("status", vec![]))
            .unwrap();
        assert_eq!(clause, "1 = 1");
    }

    #[test]
    fn test_or_filter() {
        let filter = FilterCondition::or(vec![
            FilterCondition::eq("role", "admin"),
            FilterCondition::eq("role", "superadmin"),
        ]);
        let mut builder = DOQueryBuilder::new();
        let clause = builder.build_where(&filter).unwrap();
        assert_eq!(clause, "(\"role\" = ? OR \"role\" = ?)");
        assert_eq!(builder.params().len(), 2);
    }

    #[test]
    fn test_empty_and() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::and(vec![])).unwrap();
        assert_eq!(clause, "1 = 1");
    }

    #[test]
    fn test_empty_or() {
        let mut builder = DOQueryBuilder::new();
        let clause = builder.build_where(&FilterCondition::or(vec![])).unwrap();
        assert_eq!(clause, "0 = 1");
    }

    #[test]
    fn test_order_by() {
        let options = QueryOptions::new().with_sort("name", false);
        assert_eq!(
            DOQueryBuilder::build_order_by(&options),
            Some("ORDER BY \"name\" ASC".to_string())
        );
    }

    #[test]
    fn test_order_by_desc() {
        let options = QueryOptions::new().with_sort("created_at", true);
        assert_eq!(
            DOQueryBuilder::build_order_by(&options),
            Some("ORDER BY \"created_at\" DESC".to_string())
        );
    }

    #[test]
    fn test_limit() {
        let options = QueryOptions::new().with_limit(10);
        assert_eq!(DOQueryBuilder::build_limit(&options), Some("LIMIT 10".to_string()));
    }

    #[test]
    fn test_offset() {
        let options = QueryOptions::new().with_offset(20);
        assert_eq!(DOQueryBuilder::build_offset(&options), Some("OFFSET 20".to_string()));
    }

    #[test]
    fn test_no_order_by() {
        let options = QueryOptions::new();
        assert_eq!(DOQueryBuilder::build_order_by(&options), None);
    }

    #[test]
    fn test_no_limit() {
        let options = QueryOptions::new();
        assert_eq!(DOQueryBuilder::build_limit(&options), None);
    }

    #[test]
    fn test_no_offset() {
        let options = QueryOptions::new();
        assert_eq!(DOQueryBuilder::build_offset(&options), None);
    }
}

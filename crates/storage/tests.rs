#[cfg(test)]
mod tests {
    use crate::{Entity, FilterCondition, QueryOptions, StorageError};

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[allow(dead_code)]
    struct TestEntity {
        id: String,
        name: String,
    }

    impl Entity for TestEntity {
        fn id(&self) -> &str {
            &self.id
        }

        fn entity_type() -> &'static str {
            "test_entity"
        }
    }

    #[test]
    fn test_query_options_builder() {
        let options = QueryOptions::new()
            .with_limit(10)
            .with_offset(20)
            .with_sort("name", true);

        assert_eq!(options.limit, Some(10));
        assert_eq!(options.offset, Some(20));
        assert_eq!(options.sort_by, Some("name".to_string()));
        assert!(options.sort_desc);
    }

    #[test]
    fn test_filter_condition_builders() {
        let eq_filter = FilterCondition::eq("name", "John");
        matches!(eq_filter, FilterCondition::Eq(field, _) if field == "name");

        let and_filter = FilterCondition::and(vec![
            FilterCondition::eq("active", true),
            FilterCondition::gt("age", 18),
        ]);
        matches!(and_filter, FilterCondition::And(_));
    }

    #[test]
    fn test_storage_error_display() {
        let error = StorageError::NotFound {
            entity_type: "User".to_string(),
            id: "123".to_string(),
        };
        assert_eq!(error.to_string(), "User with id '123' not found");

        let error = StorageError::DuplicateKey {
            entity_type: "User".to_string(),
            id: "123".to_string(),
        };
        assert_eq!(error.to_string(), "User with id '123' already exists");
    }
}

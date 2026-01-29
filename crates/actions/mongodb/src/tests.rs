#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_insert_and_find_one() {
        let doc = vec![
            ("name".to_string(), json!("Alice")),
            ("age".to_string(), json!(30)),
        ]
        .into_iter()
        .collect();

        let insert_result = insert("users", "test_db_insert_find_one", doc).await.unwrap();
        assert!(insert_result.success);
        assert!(!insert_result.inserted_id.is_empty());

        let filter = vec![("name".to_string(), json!("Alice"))]
            .into_iter()
            .collect();
        let find_result = find_one("users", "test_db_insert_find_one", filter, None).await.unwrap();
        assert!(find_result.found);
        assert_eq!(
            find_result.document.as_ref().unwrap().get("name").unwrap(),
            &json!("Alice")
        );
    }

    #[tokio::test]
    async fn test_insert_many() {
        let docs = vec![
            vec![("name".to_string(), json!("Bob")), ("age".to_string(), json!(25))]
                .into_iter()
                .collect(),
            vec![("name".to_string(), json!("Charlie")), ("age".to_string(), json!(35))]
                .into_iter()
                .collect(),
        ];

        let result = insert_many("users", "test_db_insert_many", docs, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.inserted_count, 2);
        assert_eq!(result.inserted_ids.len(), 2);
    }

    #[tokio::test]
    async fn test_find_with_filter() {
        // Insert test data
        insert(
            "users",
            "test_db_find_filter",
            vec![("name".to_string(), json!("Dave")), ("age".to_string(), json!(40))]
                .into_iter()
                .collect(),
        )
        .await
        .unwrap();
        
        insert(
            "users",
            "test_db_find_filter",
            vec![("name".to_string(), json!("Eve")), ("age".to_string(), json!(28))]
                .into_iter()
                .collect(),
        )
        .await
        .unwrap();

        let filter = vec![("name".to_string(), json!("Dave"))]
            .into_iter()
            .collect();
        let result = find("users", "test_db_find_filter", Some(filter), None, None, None, None)
            .await
            .unwrap();
        assert_eq!(result.count, 1);
        assert_eq!(result.documents.len(), 1);
    }

    #[tokio::test]
    async fn test_find_with_limit_and_skip() {
        // Insert multiple documents
        for i in 0..10 {
            insert(
                "users",
                "test_db_limit_skip",
                vec![("index".to_string(), json!(i))]
                    .into_iter()
                    .collect(),
            )
            .await
            .unwrap();
        }

        let result = find("users", "test_db_limit_skip", None, Some(3), None, Some(2), None)
            .await
            .unwrap();
        assert_eq!(result.count, 3);
        assert_eq!(result.documents.len(), 3);
    }

    #[tokio::test]
    async fn test_update_one() {
        insert(
            "users",
            "test_db_update_one",
            vec![("name".to_string(), json!("Frank")), ("age".to_string(), json!(45))]
                .into_iter()
                .collect(),
        )
        .await
        .unwrap();

        let filter: HashMap<String, Value> = vec![("name".to_string(), json!("Frank"))]
            .into_iter()
            .collect();
        let update = vec![(
            "$set".to_string(),
            json!({"age": 46}),
        )]
        .into_iter()
        .collect();

        let result = update_one("users", "test_db_update_one", filter.clone(), update, None)
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.matched_count, 1);
        assert_eq!(result.modified_count, 1);

        let find_result = find_one("users", "test_db_update_one", filter, None).await.unwrap();
        assert_eq!(
            find_result.document.as_ref().unwrap().get("age").unwrap(),
            &json!(46)
        );
    }

    #[tokio::test]
    async fn test_update_many() {
        insert_many(
            "users",
            "test_db_update_many",
            vec![
                vec![("category".to_string(), json!("A")), ("score".to_string(), json!(10))]
                    .into_iter()
                    .collect(),
                vec![("category".to_string(), json!("A")), ("score".to_string(), json!(20))]
                    .into_iter()
                    .collect(),
            ],
            None,
        )
        .await
        .unwrap();

        let filter = vec![("category".to_string(), json!("A"))]
            .into_iter()
            .collect();
        let update = vec![(
            "$inc".to_string(),
            json!({"score": 5}),
        )]
        .into_iter()
        .collect();

        let result = update_many("users", "test_db_update_many", filter, update, None)
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.matched_count, 2);
        assert_eq!(result.modified_count, 2);
    }

    #[tokio::test]
    async fn test_upsert() {
        let filter: HashMap<String, Value> = vec![("name".to_string(), json!("Ghost"))]
            .into_iter()
            .collect();
        let update = vec![(
            "$set".to_string(),
            json!({"age": 999}),
        )]
        .into_iter()
        .collect();

        let result = update_one("users", "test_db_upsert", filter.clone(), update, Some(true))
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.matched_count, 0);
        assert!(result.upserted_id.is_some());

        let find_result = find_one("users", "test_db_upsert", filter, None).await.unwrap();
        assert!(find_result.found);
    }

    #[tokio::test]
    async fn test_replace_one() {
        insert(
            "users",
            "test_db_replace_one",
            vec![("name".to_string(), json!("Henry")), ("age".to_string(), json!(50))]
                .into_iter()
                .collect(),
        )
        .await
        .unwrap();

        let filter: HashMap<String, Value> = vec![("name".to_string(), json!("Henry"))]
            .into_iter()
            .collect();
        let replacement = vec![
            ("name".to_string(), json!("Henry")),
            ("age".to_string(), json!(51)),
            ("city".to_string(), json!("NYC")),
        ]
        .into_iter()
        .collect();

        let result = replace_one("users", "test_db_replace_one", filter.clone(), replacement, None)
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.matched_count, 1);

        let find_result = find_one("users", "test_db_replace_one", filter, None).await.unwrap();
        assert_eq!(
            find_result.document.as_ref().unwrap().get("city").unwrap(),
            &json!("NYC")
        );
    }

    #[tokio::test]
    async fn test_delete_one() {
        let db = "test_db_delete_one";
        let coll = "users";
        
        insert(
            coll,
            db,
            vec![("name".to_string(), json!("Ivy"))]
                .into_iter()
                .collect(),
        )
        .await
        .unwrap();

        let filter: HashMap<String, Value> = vec![("name".to_string(), json!("Ivy"))]
            .into_iter()
            .collect();
        let result = delete_one(coll, db, filter.clone())
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.deleted_count, 1);

        let find_result = find_one(coll, db, filter, None).await.unwrap();
        assert!(!find_result.found);
    }

    #[tokio::test]
    async fn test_delete_many() {
        insert_many(
            "users",
            "test_db_delete_many",
            vec![
                vec![("status".to_string(), json!("inactive"))]
                    .into_iter()
                    .collect(),
                vec![("status".to_string(), json!("inactive"))]
                    .into_iter()
                    .collect(),
                vec![("status".to_string(), json!("active"))]
                    .into_iter()
                    .collect(),
            ],
            None,
        )
        .await
        .unwrap();

        let filter = vec![("status".to_string(), json!("inactive"))]
            .into_iter()
            .collect();
        let result = delete_many("users", "test_db_delete_many", filter)
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.deleted_count, 2);
    }

    #[tokio::test]
    async fn test_count() {
        insert_many(
            "users",
            "test_db_count",
            vec![
                vec![("type".to_string(), json!("premium"))]
                    .into_iter()
                    .collect(),
                vec![("type".to_string(), json!("premium"))]
                    .into_iter()
                    .collect(),
                vec![("type".to_string(), json!("free"))]
                    .into_iter()
                    .collect(),
            ],
            None,
        )
        .await
        .unwrap();

        let filter = vec![("type".to_string(), json!("premium"))]
            .into_iter()
            .collect();
        let result = count("users", "test_db_count", Some(filter))
            .await
            .unwrap();
        assert_eq!(result.count, 2);
    }

    #[tokio::test]
    async fn test_aggregate() {
        insert_many(
            "users",
            "test_db_aggregate",
            vec![
                vec![("name".to_string(), json!("Jack")), ("age".to_string(), json!(25))]
                    .into_iter()
                    .collect(),
                vec![("name".to_string(), json!("Jill")), ("age".to_string(), json!(30))]
                    .into_iter()
                    .collect(),
                vec![("name".to_string(), json!("John")), ("age".to_string(), json!(35))]
                    .into_iter()
                    .collect(),
            ],
            None,
        )
        .await
        .unwrap();

        let pipeline = vec![
            vec![("$match".to_string(), json!({"age": {"$gte": 30}}))]
                .into_iter()
                .collect(),
            vec![("$limit".to_string(), json!(1))]
                .into_iter()
                .collect(),
        ];

        let result = aggregate("users", "test_db_aggregate", pipeline)
            .await
            .unwrap();
        assert_eq!(result.documents.len(), 1);
    }

    #[tokio::test]
    async fn test_create_and_drop_index() {
        let keys = vec![("email".to_string(), json!(1))]
            .into_iter()
            .collect();
        let result = create_index("users", "test_db_index", keys, Some("email_idx"), Some(true))
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.index_name, "email_idx");

        let drop_result = drop_index("users", "test_db_index", "email_idx")
            .await
            .unwrap();
        assert!(drop_result.success);
    }

    #[tokio::test]
    async fn test_list_collections() {
        insert("coll1", "test_db_list_coll", vec![("test".to_string(), json!("data1"))].into_iter().collect())
            .await
            .unwrap();
        insert("coll2", "test_db_list_coll", vec![("test".to_string(), json!("data2"))].into_iter().collect())
            .await
            .unwrap();

        let result = list_collections("test_db_list_coll").await.unwrap();
        assert!(result.collections.contains(&"coll1".to_string()));
        assert!(result.collections.contains(&"coll2".to_string()));
    }

    #[tokio::test]
    async fn test_drop_collection() {
        insert("temp_coll", "test_db_drop_coll", vec![("test".to_string(), json!("data"))].into_iter().collect())
            .await
            .unwrap();

        let result = drop_collection("temp_coll", "test_db_drop_coll")
            .await
            .unwrap();
        assert!(result.success);

        let list_result = list_collections("test_db_drop_coll").await.unwrap();
        assert!(!list_result.collections.contains(&"temp_coll".to_string()));
    }

    #[tokio::test]
    async fn test_bulk_write() {
        let operations = vec![
            vec![(
                "insertOne".to_string(),
                json!({"name": "Alice", "age": 25}),
            )]
            .into_iter()
            .collect(),
            vec![(
                "insertOne".to_string(),
                json!({"name": "Bob", "age": 30}),
            )]
            .into_iter()
            .collect(),
        ];

        let result = bulk_write("users", "test_db_bulk", operations, None)
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.inserted_count, 2);
    }

    #[tokio::test]
    async fn test_query_operators() {
        insert_many(
            "users",
            "test_db_query_ops",
            vec![
                vec![("name".to_string(), json!("Alice")), ("score".to_string(), json!(85))]
                    .into_iter()
                    .collect(),
                vec![("name".to_string(), json!("Bob")), ("score".to_string(), json!(92))]
                    .into_iter()
                    .collect(),
                vec![("name".to_string(), json!("Charlie")), ("score".to_string(), json!(78))]
                    .into_iter()
                    .collect(),
            ],
            None,
        )
        .await
        .unwrap();

        // Test $gt operator
        let filter = vec![("score".to_string(), json!({"$gt": 80}))]
            .into_iter()
            .collect();
        let result = find("users", "test_db_query_ops", Some(filter), None, None, None, None)
            .await
            .unwrap();
        assert_eq!(result.count, 2);

        // Test $lte operator
        let filter = vec![("score".to_string(), json!({"$lte": 85}))]
            .into_iter()
            .collect();
        let result = find("users", "test_db_query_ops", Some(filter), None, None, None, None)
            .await
            .unwrap();
        assert_eq!(result.count, 2);
    }

    #[tokio::test]
    async fn test_projection() {
        insert(
            "users",
            "test_db_projection",
            vec![
                ("name".to_string(), json!("Test User")),
                ("email".to_string(), json!("test@example.com")),
                ("password".to_string(), json!("secret")),
            ]
            .into_iter()
            .collect(),
        )
        .await
        .unwrap();

        let filter = vec![("name".to_string(), json!("Test User"))]
            .into_iter()
            .collect();
        let projection = vec![
            ("name".to_string(), 1),
            ("email".to_string(), 1),
        ]
        .into_iter()
        .collect();

        let result = find_one("users", "test_db_projection", filter, Some(projection))
            .await
            .unwrap();
        assert!(result.found);
        let doc = result.document.unwrap();
        assert!(doc.contains_key("name"));
        assert!(doc.contains_key("email"));
        assert!(!doc.contains_key("password"));
    }

    #[tokio::test]
    async fn test_sort() {
        insert_many(
            "users",
            "test_db_sort",
            vec![
                vec![("name".to_string(), json!("Charlie")), ("order".to_string(), json!(3))]
                    .into_iter()
                    .collect(),
                vec![("name".to_string(), json!("Alice")), ("order".to_string(), json!(1))]
                    .into_iter()
                    .collect(),
                vec![("name".to_string(), json!("Bob")), ("order".to_string(), json!(2))]
                    .into_iter()
                    .collect(),
            ],
            None,
        )
        .await
        .unwrap();

        let sort = vec![("order".to_string(), 1)] // ascending
            .into_iter()
            .collect();
        let result = find("users", "test_db_sort", None, None, None, None, Some(sort))
            .await
            .unwrap();
        
        assert_eq!(result.documents[0].get("name").unwrap(), &json!("Alice"));
        assert_eq!(result.documents[1].get("name").unwrap(), &json!("Bob"));
        assert_eq!(result.documents[2].get("name").unwrap(), &json!("Charlie"));
    }
}

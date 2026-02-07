use super::*;
use serde_json::json;

#[tokio::test]
async fn test_create_table() {
    let columns = vec![
        {
            let mut col = HashMap::new();
            col.insert("name".to_string(), Value::String("id".to_string()));
            col.insert("type".to_string(), Value::String("INTEGER".to_string()));
            col.insert("primary_key".to_string(), Value::Bool(true));
            col
        },
        {
            let mut col = HashMap::new();
            col.insert("name".to_string(), Value::String("name".to_string()));
            col.insert("type".to_string(), Value::String("TEXT".to_string()));
            col
        },
    ];

    let result = create_table("test_users", columns, None, Some(true)).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);
}

#[tokio::test]
async fn test_create_table_already_exists() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("id".to_string()));
        col
    }];

    create_table("dup_table", columns.clone(), None, None).await.unwrap();

    // Without IF NOT EXISTS should fail
    let result = create_table("dup_table", columns.clone(), None, Some(false)).await;
    assert!(result.is_err());

    // With IF NOT EXISTS should succeed
    let result = create_table("dup_table", columns, None, Some(true)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_insert_and_select() {
    // Create table first
    let columns = vec![
        {
            let mut col = HashMap::new();
            col.insert("name".to_string(), Value::String("id".to_string()));
            col.insert("type".to_string(), Value::String("INTEGER".to_string()));
            col
        },
        {
            let mut col = HashMap::new();
            col.insert("name".to_string(), Value::String("username".to_string()));
            col.insert("type".to_string(), Value::String("TEXT".to_string()));
            col
        },
    ];
    create_table("users_test", columns, None, Some(true)).await.unwrap();

    // Insert a row
    let insert_result = insert(
        "INSERT INTO users_test (username) VALUES ('alice')",
        None,
        None,
    ).await;

    assert!(insert_result.is_ok());
    let insert_out = insert_result.unwrap();
    assert!(insert_out.success);
    assert_eq!(insert_out.affected_rows, 1);

    // Select the row
    let select_result = select(
        "SELECT * FROM users_test",
        None,
        None,
        None,
        None,
    ).await;

    assert!(select_result.is_ok());
    let select_out = select_result.unwrap();
    assert!(select_out.count >= 1);
}

#[tokio::test]
async fn test_update() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("status".to_string()));
        col
    }];
    create_table("update_test", columns, None, Some(true)).await.unwrap();

    insert("INSERT INTO update_test (status) VALUES ('pending')", None, None).await.unwrap();

    let update_result = update(
        "UPDATE update_test SET status = 'completed'",
        None,
        None,
    ).await;

    assert!(update_result.is_ok());
    let out = update_result.unwrap();
    assert!(out.success);
    assert!(out.affected_rows >= 1);
}

#[tokio::test]
async fn test_delete() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("name".to_string()));
        col
    }];
    create_table("delete_test", columns, None, Some(true)).await.unwrap();

    insert("INSERT INTO delete_test (name) VALUES ('toDelete')", None, None).await.unwrap();

    let delete_result = delete(
        "DELETE FROM delete_test WHERE name = 'toDelete'",
        None,
        None,
    ).await;

    assert!(delete_result.is_ok());
    assert!(delete_result.unwrap().success);
}

#[tokio::test]
async fn test_drop_table() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("id".to_string()));
        col
    }];
    create_table("drop_me", columns, None, None).await.unwrap();

    let result = drop_table("drop_me", None, None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().success);

    // Should fail now
    let result2 = drop_table("drop_me", None, Some(false)).await;
    assert!(result2.is_err());

    // With IF EXISTS should succeed
    let result3 = drop_table("drop_me", None, Some(true)).await;
    assert!(result3.is_ok());
}

#[tokio::test]
async fn test_truncate_table() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("data".to_string()));
        col
    }];
    create_table("truncate_test", columns, None, Some(true)).await.unwrap();

    insert("INSERT INTO truncate_test (data) VALUES ('test1')", None, None).await.unwrap();
    insert("INSERT INTO truncate_test (data) VALUES ('test2')", None, None).await.unwrap();

    let result = truncate_table("truncate_test", None).await;
    assert!(result.is_ok());

    let select_result = select("SELECT * FROM truncate_test", None, None, None, None).await.unwrap();
    assert_eq!(select_result.count, 0);
}

#[tokio::test]
async fn test_get_schema() {
    let columns = vec![
        {
            let mut col = HashMap::new();
            col.insert("name".to_string(), Value::String("id".to_string()));
            col.insert("type".to_string(), Value::String("INTEGER".to_string()));
            col.insert("primary_key".to_string(), Value::Bool(true));
            col
        },
        {
            let mut col = HashMap::new();
            col.insert("name".to_string(), Value::String("email".to_string()));
            col.insert("type".to_string(), Value::String("VARCHAR".to_string()));
            col
        },
    ];
    create_table("schema_test", columns, None, Some(true)).await.unwrap();

    let result = get_schema("schema_test", None).await;
    assert!(result.is_ok());
    let schema = result.unwrap();
    assert_eq!(schema.columns.len(), 2);
    assert!(schema.primary_key.contains(&"id".to_string()));
}

#[tokio::test]
async fn test_create_and_drop_index() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("email".to_string()));
        col
    }];
    create_table("index_test", columns, None, Some(true)).await.unwrap();

    let create_result = create_index(
        vec!["email".to_string()],
        "idx_email",
        "index_test",
        Some(true),
        None,
    ).await;

    assert!(create_result.is_ok());

    // Verify index in schema
    let schema = get_schema("index_test", None).await.unwrap();
    assert!(!schema.indexes.is_empty());

    // Drop the index
    let drop_result = drop_index("idx_email", Some("index_test"), None, None).await;
    assert!(drop_result.is_ok());
}

#[tokio::test]
async fn test_begin_commit_transaction() {
    let begin_result = begin_transaction(Some("SERIALIZABLE"), None).await;
    assert!(begin_result.is_ok());
    let tx = begin_result.unwrap();
    assert!(!tx.transaction_id.is_empty());

    let commit_result = commit(&tx.transaction_id, None).await;
    assert!(commit_result.is_ok());
}

#[tokio::test]
async fn test_rollback_transaction() {
    let begin_result = begin_transaction(None, None).await;
    let tx = begin_result.unwrap();

    let rollback_result = rollback(&tx.transaction_id, None).await;
    assert!(rollback_result.is_ok());
}

#[tokio::test]
async fn test_bulk_insert() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("name".to_string()));
        col
    }];
    create_table("bulk_test", columns, None, Some(true)).await.unwrap();

    let rows = vec![
        json!(["Alice"]),
        json!(["Bob"]),
        json!(["Charlie"]),
    ];

    let result = bulk_insert(
        vec!["name".to_string()],
        "bulk_test",
        rows,
        None,
    ).await;

    assert!(result.is_ok());
    let out = result.unwrap();
    assert!(out.success);
    assert_eq!(out.affected_rows, 3);
}

#[tokio::test]
async fn test_call_procedure() {
    let result = call_procedure("get_version", None, None).await;
    assert!(result.is_ok());
    let out = result.unwrap();
    assert!(out.success);
    assert!(out.out_parameters.contains_key("version"));
}

#[tokio::test]
async fn test_batch_queries() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("value".to_string()));
        col
    }];
    create_table("batch_test", columns, None, Some(true)).await.unwrap();

    let queries = vec![
        {
            let mut q = HashMap::new();
            q.insert("query".to_string(), Value::String("INSERT INTO batch_test (value) VALUES ('q1')".to_string()));
            q
        },
        {
            let mut q = HashMap::new();
            q.insert("query".to_string(), Value::String("INSERT INTO batch_test (value) VALUES ('q2')".to_string()));
            q
        },
    ];

    let result = batch(queries, None).await;
    assert!(result.is_ok());
    let out = result.unwrap();
    assert_eq!(out.success_count, 2);
    assert_eq!(out.failed_count, 0);
}

#[tokio::test]
async fn test_prepared_statement() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("category".to_string()));
        col
    }];
    create_table("prep_test", columns, None, Some(true)).await.unwrap();

    let result = prepared_statement(
        "INSERT INTO prep_test (category) VALUES ($1)",
        None,
        Some(vec![Value::String("test_category".to_string())]),
    ).await;

    assert!(result.is_ok());
    assert!(result.unwrap().success);
}

#[tokio::test]
async fn test_execute() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("field".to_string()));
        col
    }];
    create_table("exec_test", columns, None, Some(true)).await.unwrap();

    let result = execute(
        "INSERT INTO exec_test (field) VALUES ('data')",
        None,
        None,
    ).await;

    assert!(result.is_ok());
    let out = result.unwrap();
    assert!(out.success);
    assert_eq!(out.affected_rows, 1);
}

#[tokio::test]
async fn test_list_tables() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("id".to_string()));
        col
    }];
    create_table("list_test_1", columns.clone(), None, Some(true)).await.unwrap();
    create_table("list_test_2", columns, None, Some(true)).await.unwrap();

    let result = list_tables(None, None).await;
    assert!(result.is_ok());
    let tables = result.unwrap().tables;
    assert!(tables.iter().any(|t| t.contains("list_test")));
}

#[tokio::test]
async fn test_select_with_where() {
    let columns = vec![
        {
            let mut col = HashMap::new();
            col.insert("name".to_string(), Value::String("status".to_string()));
            col
        },
    ];
    create_table("where_test", columns, None, Some(true)).await.unwrap();

    insert("INSERT INTO where_test (status) VALUES ('active')", None, None).await.unwrap();
    insert("INSERT INTO where_test (status) VALUES ('inactive')", None, None).await.unwrap();

    let result = select(
        "SELECT * FROM where_test WHERE status = 'active'",
        None,
        None,
        None,
        None,
    ).await;

    assert!(result.is_ok());
    let out = result.unwrap();
    // Should only return active rows
    for row in &out.rows {
        if let Some(Value::String(s)) = row.get("status") {
            assert_eq!(s, "active");
        }
    }
}

#[tokio::test]
async fn test_select_with_limit_offset() {
    let columns = vec![{
        let mut col = HashMap::new();
        col.insert("name".to_string(), Value::String("num".to_string()));
        col
    }];
    create_table("limit_test", columns, None, Some(true)).await.unwrap();

    for i in 1..=10 {
        insert(&format!("INSERT INTO limit_test (num) VALUES ({})", i), None, None).await.unwrap();
    }

    let result = select(
        "SELECT * FROM limit_test",
        Some(3),
        Some(2),
        None,
        None,
    ).await;

    assert!(result.is_ok());
    let out = result.unwrap();
    assert!(out.count <= 3);
}

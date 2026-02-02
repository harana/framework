use super::*;

#[tokio::test]
async fn test_create_db_instance() {
    clear_all_data();

    let result = create_db_instance(
        "test-db".to_string(),
        "db.t3.micro".to_string(),
        "mysql".to_string(),
        Some("8.0.33".to_string()),
        Some("admin".to_string()),
        Some("password123".to_string()),
        Some("testdb".to_string()),
        Some(20),
        None,
        Some(7),
        None,
        Some(false),
        Some(true),
        Some(false),
        Some(output::StorageType::Gp2),
        Some(false),
        None,
        None,
        Some(3306),
        None,
        None,
        Some(vec!["sg-12345".to_string()]),
        None,
        Some(true),
        Some(vec![output::Tag {
            key: "Environment".to_string(),
            value: "test".to_string(),
        }]),
        Some("us-east-1".to_string()),
    ).await;

    assert!(result.success);
    assert_eq!(result.db_instance.db_instance_identifier, "test-db");
    assert_eq!(result.db_instance.db_instance_class, "db.t3.micro");
    assert_eq!(result.db_instance.engine, "mysql");
    assert_eq!(result.db_instance.db_instance_status, "available");
    assert_eq!(result.db_instance.allocated_storage, 20);
    assert!(result.db_instance.deletion_protection);
}

#[tokio::test]
async fn test_describe_db_instances() {
    clear_all_data();

    // Create multiple instances
    create_db_instance(
        "test-db-1".to_string(),
        "db.t3.micro".to_string(),
        "mysql".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    create_db_instance(
        "test-db-2".to_string(),
        "db.t3.small".to_string(),
        "postgres".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    // Describe all instances
    let result = describe_db_instances(
        None,
        None,
        None,
        None,
        None,
    ).await;

    assert_eq!(result.db_instances.len(), 2);

    // Describe specific instance
    let result = describe_db_instances(
        Some("test-db-1".to_string()),
        None,
        None,
        None,
        None,
    ).await;

    assert_eq!(result.db_instances.len(), 1);
    assert_eq!(result.db_instances[0].db_instance_identifier, "test-db-1");
}

#[tokio::test]
async fn test_modify_db_instance() {
    clear_all_data();

    create_db_instance(
        "modify-test".to_string(),
        "db.t3.micro".to_string(),
        "mysql".to_string(),
        None,
        None,
        None,
        None,
        Some(20),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = modify_db_instance(
        "modify-test".to_string(),
        Some("db.t3.medium".to_string()),
        Some(50),
        None,
        Some(14),
        Some(true),
        None,
        None,
        None,
        None,
        Some(false),
        None,
        None,
    ).await;

    assert!(result.success);
    assert_eq!(result.db_instance.db_instance_class, "db.t3.medium");
    assert_eq!(result.db_instance.allocated_storage, 50);
    assert!(result.db_instance.multi_az);
    assert!(!result.db_instance.deletion_protection);
}

#[tokio::test]
async fn test_start_stop_db_instance() {
    clear_all_data();

    create_db_instance(
        "stop-start-test".to_string(),
        "db.t3.micro".to_string(),
        "mysql".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    // Stop the instance
    let stop_result = stop_db_instance(
        "stop-start-test".to_string(),
        None,
        None,
    ).await;

    assert!(stop_result.success);
    assert_eq!(stop_result.db_instance.db_instance_status, "stopped");

    // Start the instance
    let start_result = start_db_instance(
        "stop-start-test".to_string(),
        None,
    ).await;

    assert!(start_result.success);
    assert_eq!(start_result.db_instance.db_instance_status, "available");
}

#[tokio::test]
async fn test_reboot_db_instance() {
    clear_all_data();

    create_db_instance(
        "reboot-test".to_string(),
        "db.t3.micro".to_string(),
        "postgres".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = reboot_db_instance(
        "reboot-test".to_string(),
        Some(false),
        None,
    ).await;

    assert!(result.success);
    assert_eq!(result.db_instance.db_instance_status, "available");
}

#[tokio::test]
async fn test_delete_db_instance() {
    clear_all_data();

    create_db_instance(
        "delete-test".to_string(),
        "db.t3.micro".to_string(),
        "mysql".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = delete_db_instance(
        "delete-test".to_string(),
        Some(true),
        None,
        Some(true),
        None,
    ).await;

    assert!(result.success);
    assert_eq!(result.db_instance.db_instance_status, "deleting");

    // Verify instance is gone
    let describe_result = describe_db_instances(
        Some("delete-test".to_string()),
        None,
        None,
        None,
        None,
    ).await;

    assert!(describe_result.db_instances.is_empty());
}

#[tokio::test]
async fn test_create_db_cluster() {
    clear_all_data();

    let result = create_db_cluster(
        "test-cluster".to_string(),
        "aurora-mysql".to_string(),
        Some("5.7.mysql_aurora.2.11.2".to_string()),
        Some("provisioned".to_string()),
        Some("admin".to_string()),
        Some("password123".to_string()),
        Some("testdb".to_string()),
        Some(3306),
        None,
        Some(vec!["us-east-1a".to_string(), "us-east-1b".to_string()]),
        Some(7),
        None,
        Some(vec!["sg-12345".to_string()]),
        Some(true),
        Some(true),
        None,
        Some(vec![output::Tag {
            key: "Environment".to_string(),
            value: "test".to_string(),
        }]),
        Some("us-east-1".to_string()),
    ).await;

    assert!(result.success);
    assert_eq!(result.db_cluster.db_cluster_identifier, "test-cluster");
    assert_eq!(result.db_cluster.engine, "aurora-mysql");
    assert_eq!(result.db_cluster.status, "available");
    assert!(result.db_cluster.deletion_protection);
    assert!(result.db_cluster.storage_encrypted);
}

#[tokio::test]
async fn test_describe_db_clusters() {
    clear_all_data();

    create_db_cluster(
        "cluster-1".to_string(),
        "aurora-mysql".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    create_db_cluster(
        "cluster-2".to_string(),
        "aurora-postgresql".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    // Describe all clusters
    let result = describe_db_clusters(
        None,
        None,
        None,
        None,
        None,
    ).await;

    assert_eq!(result.db_clusters.len(), 2);

    // Describe specific cluster
    let result = describe_db_clusters(
        Some("cluster-1".to_string()),
        None,
        None,
        None,
        None,
    ).await;

    assert_eq!(result.db_clusters.len(), 1);
    assert_eq!(result.db_clusters[0].db_cluster_identifier, "cluster-1");
}

#[tokio::test]
async fn test_delete_db_cluster() {
    clear_all_data();

    create_db_cluster(
        "delete-cluster".to_string(),
        "aurora-mysql".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = delete_db_cluster(
        "delete-cluster".to_string(),
        Some(true),
        None,
        None,
    ).await;

    assert!(result.success);
    assert_eq!(result.db_cluster.status, "deleting");

    // Verify cluster is gone
    let describe_result = describe_db_clusters(
        Some("delete-cluster".to_string()),
        None,
        None,
        None,
        None,
    ).await;

    assert!(describe_result.db_clusters.is_empty());
}

#[tokio::test]
async fn test_different_engine_ports() {
    clear_all_data();

    // Test MySQL default port
    let mysql_result = create_db_instance(
        "mysql-port-test".to_string(),
        "db.t3.micro".to_string(),
        "mysql".to_string(),
        None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None,
    ).await;
    assert_eq!(mysql_result.db_instance.endpoint.as_ref().unwrap().port, 3306);

    // Test PostgreSQL default port
    let pg_result = create_db_instance(
        "postgres-port-test".to_string(),
        "db.t3.micro".to_string(),
        "postgres".to_string(),
        None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None,
    ).await;
    assert_eq!(pg_result.db_instance.endpoint.as_ref().unwrap().port, 5432);

    // Test Oracle default port
    let oracle_result = create_db_instance(
        "oracle-port-test".to_string(),
        "db.t3.micro".to_string(),
        "oracle-se2".to_string(),
        None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None,
    ).await;
    assert_eq!(oracle_result.db_instance.endpoint.as_ref().unwrap().port, 1521);

    // Test SQL Server default port
    let sqlserver_result = create_db_instance(
        "sqlserver-port-test".to_string(),
        "db.t3.micro".to_string(),
        "sqlserver-se".to_string(),
        None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None,
    ).await;
    assert_eq!(sqlserver_result.db_instance.endpoint.as_ref().unwrap().port, 1433);
}

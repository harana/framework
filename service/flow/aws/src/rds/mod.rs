pub mod output;

#[cfg(test)]
mod tests;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use chrono::Utc;
use uuid::Uuid;

use output::*;

// In-memory storage
static DB_INSTANCES: Lazy<DashMap<String, StoredDbInstance>> = Lazy::new(DashMap::new);
static DB_CLUSTERS: Lazy<DashMap<String, StoredDbCluster>> = Lazy::new(DashMap::new);

fn generate_resource_id() -> String {
    format!("db-{}", Uuid::new_v4().to_string().replace("-", "")[..20].to_string().to_uppercase())
}

fn generate_endpoint(identifier: &str, region: &str, port: i32) -> Endpoint {
    Endpoint {
        address: format!("{}.{}.{}.rds.amazonaws.com", identifier, generate_resource_id().to_lowercase()[..8].to_string(), region),
        port,
        hosted_zone_id: Some(format!("Z{}", &generate_resource_id()[..14])),
    }
}

fn storage_type_to_string(st: StorageType) -> String {
    match st {
        StorageType::Standard => "standard".to_string(),
        StorageType::Gp2 => "gp2".to_string(),
        StorageType::Gp3 => "gp3".to_string(),
        StorageType::Io1 => "io1".to_string(),
        StorageType::Io2 => "io2".to_string(),
    }
}

fn get_default_port(engine: &str) -> i32 {
    match engine.to_lowercase().as_str() {
        "mysql" | "mariadb" | "aurora" | "aurora-mysql" => 3306,
        "postgres" | "aurora-postgresql" => 5432,
        "oracle-se" | "oracle-se1" | "oracle-se2" | "oracle-ee" => 1521,
        "sqlserver-ee" | "sqlserver-se" | "sqlserver-ex" | "sqlserver-web" => 1433,
        _ => 3306,
    }
}

fn stored_to_db_instance(stored: &StoredDbInstance) -> DBInstance {
    DBInstance {
        db_instance_identifier: stored.db_instance_identifier.clone(),
        db_instance_class: stored.db_instance_class.clone(),
        engine: stored.engine.clone(),
        engine_version: stored.engine_version.clone(),
        db_instance_status: stored.status.clone(),
        master_username: stored.master_username.clone(),
        db_name: stored.db_name.clone(),
        endpoint: stored.endpoint_address.as_ref().map(|addr| Endpoint {
            address: addr.clone(),
            port: stored.endpoint_port.unwrap_or(stored.port),
            hosted_zone_id: None,
        }),
        allocated_storage: stored.allocated_storage,
        instance_create_time: Some(stored.created_at),
        preferred_backup_window: Some("03:00-04:00".to_string()),
        backup_retention_period: stored.backup_retention_period,
        db_security_groups: Vec::new(),
        vpc_security_groups: stored.vpc_security_group_ids.iter().map(|id| VpcSecurityGroupMembership {
            vpc_security_group_id: id.clone(),
            status: "active".to_string(),
        }).collect(),
        db_parameter_groups: stored.db_parameter_group_name.as_ref().map(|name| vec![DBParameterGroupStatus {
            db_parameter_group_name: name.clone(),
            parameter_apply_status: "in-sync".to_string(),
        }]).unwrap_or_default(),
        availability_zone: stored.availability_zone.clone(),
        db_subnet_group: stored.db_subnet_group_name.as_ref().map(|name| DBSubnetGroup {
            db_subnet_group_name: name.clone(),
            db_subnet_group_description: None,
            vpc_id: None,
            subnet_group_status: "Complete".to_string(),
            subnets: Vec::new(),
        }),
        preferred_maintenance_window: Some("sun:05:00-sun:06:00".to_string()),
        pending_modified_values: Some(PendingModifiedValues::default()),
        latest_restorable_time: Some(Utc::now()),
        multi_az: stored.multi_az,
        auto_minor_version_upgrade: stored.auto_minor_version_upgrade,
        read_replica_source_db_instance_identifier: None,
        read_replica_db_instance_identifiers: Vec::new(),
        license_model: Some("general-public-license".to_string()),
        iops: stored.iops,
        option_group_memberships: Vec::new(),
        publicly_accessible: stored.publicly_accessible,
        storage_type: Some(storage_type_to_string(stored.storage_type)),
        db_cluster_identifier: stored.db_cluster_identifier.clone(),
        storage_encrypted: stored.storage_encrypted,
        kms_key_id: stored.kms_key_id.clone(),
        dbi_resource_id: generate_resource_id(),
        ca_certificate_identifier: Some("rds-ca-2019".to_string()),
        deletion_protection: stored.deletion_protection,
        tags: stored.tags.clone(),
    }
}

/// Create a DB instance
pub async fn create_db_instance(
    db_instance_identifier: String,
    db_instance_class: String,
    engine: String,
    engine_version: Option<String>,
    master_username: Option<String>,
    master_user_password: Option<String>,
    db_name: Option<String>,
    allocated_storage: Option<i32>,
    max_allocated_storage: Option<i32>,
    backup_retention_period: Option<i32>,
    availability_zone: Option<String>,
    multi_az: Option<bool>,
    auto_minor_version_upgrade: Option<bool>,
    publicly_accessible: Option<bool>,
    storage_type: Option<StorageType>,
    storage_encrypted: Option<bool>,
    kms_key_id: Option<String>,
    iops: Option<i32>,
    port: Option<i32>,
    db_cluster_identifier: Option<String>,
    db_subnet_group_name: Option<String>,
    vpc_security_group_ids: Option<Vec<String>>,
    db_parameter_group_name: Option<String>,
    deletion_protection: Option<bool>,
    tags: Option<Vec<Tag>>,
    region: Option<String>,
) -> CreateDbInstanceOutput {
    let region = region.unwrap_or_else(|| "us-east-1".to_string());
    let port = port.unwrap_or_else(|| get_default_port(&engine));
    let now = Utc::now();

    let endpoint = generate_endpoint(&db_instance_identifier, &region, port);

    let stored = StoredDbInstance {
        db_instance_identifier: db_instance_identifier.clone(),
        db_instance_class,
        engine,
        engine_version,
        status: "available".to_string(),
        master_username,
        master_user_password,
        db_name,
        allocated_storage: allocated_storage.unwrap_or(20),
        max_allocated_storage,
        backup_retention_period: backup_retention_period.unwrap_or(1),
        availability_zone,
        multi_az: multi_az.unwrap_or(false),
        auto_minor_version_upgrade: auto_minor_version_upgrade.unwrap_or(true),
        publicly_accessible: publicly_accessible.unwrap_or(false),
        storage_type: storage_type.unwrap_or(StorageType::Gp2),
        storage_encrypted: storage_encrypted.unwrap_or(false),
        kms_key_id,
        iops,
        port,
        db_cluster_identifier,
        db_subnet_group_name,
        vpc_security_group_ids: vpc_security_group_ids.unwrap_or_default(),
        db_parameter_group_name,
        deletion_protection: deletion_protection.unwrap_or(false),
        endpoint_address: Some(endpoint.address),
        endpoint_port: Some(endpoint.port),
        tags: tags.unwrap_or_default(),
        created_at: now,
        region,
    };

    let db_instance = stored_to_db_instance(&stored);
    DB_INSTANCES.insert(db_instance_identifier, stored);

    CreateDbInstanceOutput {
        db_instance,
        success: true,
    }
}

/// Delete a DB instance
pub async fn delete_db_instance(
    db_instance_identifier: String,
    _skip_final_snapshot: Option<bool>,
    _final_db_snapshot_identifier: Option<String>,
    _delete_automated_backups: Option<bool>,
    _region: Option<String>,
) -> DeleteDbInstanceOutput {
    if let Some((_, stored)) = DB_INSTANCES.remove(&db_instance_identifier) {
        let mut db_instance = stored_to_db_instance(&stored);
        db_instance.db_instance_status = "deleting".to_string();
        
        DeleteDbInstanceOutput {
            db_instance,
            success: true,
        }
    } else {
        DeleteDbInstanceOutput {
            db_instance: DBInstance::default(),
            success: false,
        }
    }
}

/// Describe DB instances
pub async fn describe_db_instances(
    db_instance_identifier: Option<String>,
    _filters: Option<Vec<RdsFilter>>,
    max_records: Option<i32>,
    _marker: Option<String>,
    _region: Option<String>,
) -> DescribeDbInstancesOutput {
    let max_records = max_records.unwrap_or(100) as usize;

    let db_instances: Vec<DBInstance> = if let Some(identifier) = db_instance_identifier {
        DB_INSTANCES.get(&identifier)
            .map(|entry| vec![stored_to_db_instance(entry.value())])
            .unwrap_or_default()
    } else {
        DB_INSTANCES
            .iter()
            .take(max_records)
            .map(|entry| stored_to_db_instance(entry.value()))
            .collect()
    };

    DescribeDbInstancesOutput {
        db_instances,
        marker: None,
    }
}

/// Modify a DB instance
pub async fn modify_db_instance(
    db_instance_identifier: String,
    db_instance_class: Option<String>,
    allocated_storage: Option<i32>,
    master_user_password: Option<String>,
    backup_retention_period: Option<i32>,
    multi_az: Option<bool>,
    engine_version: Option<String>,
    iops: Option<i32>,
    storage_type: Option<String>,
    vpc_security_group_ids: Option<Vec<String>>,
    deletion_protection: Option<bool>,
    _apply_immediately: Option<bool>,
    _region: Option<String>,
) -> ModifyDbInstanceOutput {
    if let Some(mut entry) = DB_INSTANCES.get_mut(&db_instance_identifier) {
        if let Some(class) = db_instance_class {
            entry.db_instance_class = class;
        }
        if let Some(storage) = allocated_storage {
            entry.allocated_storage = storage;
        }
        if let Some(password) = master_user_password {
            entry.master_user_password = Some(password);
        }
        if let Some(retention) = backup_retention_period {
            entry.backup_retention_period = retention;
        }
        if let Some(az) = multi_az {
            entry.multi_az = az;
        }
        if let Some(version) = engine_version {
            entry.engine_version = Some(version);
        }
        if let Some(i) = iops {
            entry.iops = Some(i);
        }
        if let Some(ids) = vpc_security_group_ids {
            entry.vpc_security_group_ids = ids;
        }
        if let Some(protection) = deletion_protection {
            entry.deletion_protection = protection;
        }

        let db_instance = stored_to_db_instance(&entry);
        
        ModifyDbInstanceOutput {
            db_instance,
            success: true,
        }
    } else {
        ModifyDbInstanceOutput {
            db_instance: DBInstance::default(),
            success: false,
        }
    }
}

/// Start a DB instance
pub async fn start_db_instance(
    db_instance_identifier: String,
    _region: Option<String>,
) -> StartDbInstanceOutput {
    if let Some(mut entry) = DB_INSTANCES.get_mut(&db_instance_identifier) {
        entry.status = "starting".to_string();
        
        // Simulate quick start
        entry.status = "available".to_string();

        let db_instance = stored_to_db_instance(&entry);
        
        StartDbInstanceOutput {
            db_instance,
            success: true,
        }
    } else {
        StartDbInstanceOutput {
            db_instance: DBInstance::default(),
            success: false,
        }
    }
}

/// Stop a DB instance
pub async fn stop_db_instance(
    db_instance_identifier: String,
    _db_snapshot_identifier: Option<String>,
    _region: Option<String>,
) -> StopDbInstanceOutput {
    if let Some(mut entry) = DB_INSTANCES.get_mut(&db_instance_identifier) {
        entry.status = "stopped".to_string();

        let db_instance = stored_to_db_instance(&entry);
        
        StopDbInstanceOutput {
            db_instance,
            success: true,
        }
    } else {
        StopDbInstanceOutput {
            db_instance: DBInstance::default(),
            success: false,
        }
    }
}

/// Reboot a DB instance
pub async fn reboot_db_instance(
    db_instance_identifier: String,
    _force_failover: Option<bool>,
    _region: Option<String>,
) -> RebootDbInstanceOutput {
    if let Some(mut entry) = DB_INSTANCES.get_mut(&db_instance_identifier) {
        entry.status = "rebooting".to_string();
        
        // Simulate quick reboot
        entry.status = "available".to_string();

        let db_instance = stored_to_db_instance(&entry);
        
        RebootDbInstanceOutput {
            db_instance,
            success: true,
        }
    } else {
        RebootDbInstanceOutput {
            db_instance: DBInstance::default(),
            success: false,
        }
    }
}

/// Create a DB cluster
pub async fn create_db_cluster(
    db_cluster_identifier: String,
    engine: String,
    engine_version: Option<String>,
    engine_mode: Option<String>,
    master_username: Option<String>,
    _master_user_password: Option<String>,
    database_name: Option<String>,
    port: Option<i32>,
    allocated_storage: Option<i32>,
    availability_zones: Option<Vec<String>>,
    backup_retention_period: Option<i32>,
    db_subnet_group_name: Option<String>,
    vpc_security_group_ids: Option<Vec<String>>,
    deletion_protection: Option<bool>,
    storage_encrypted: Option<bool>,
    kms_key_id: Option<String>,
    tags: Option<Vec<Tag>>,
    region: Option<String>,
) -> CreateDbClusterOutput {
    let region = region.unwrap_or_else(|| "us-east-1".to_string());
    let port = port.unwrap_or_else(|| get_default_port(&engine));
    let now = Utc::now();

    let cluster_arn = format!(
        "arn:aws:rds:{}:123456789012:cluster:{}",
        region, db_cluster_identifier
    );

    let endpoint = format!(
        "{}.cluster-{}.{}.rds.amazonaws.com",
        db_cluster_identifier,
        generate_resource_id().to_lowercase()[..12].to_string(),
        region
    );

    let reader_endpoint = format!(
        "{}.cluster-ro-{}.{}.rds.amazonaws.com",
        db_cluster_identifier,
        generate_resource_id().to_lowercase()[..12].to_string(),
        region
    );

    let stored = StoredDbCluster {
        db_cluster_identifier: db_cluster_identifier.clone(),
        db_cluster_arn: cluster_arn.clone(),
        engine,
        engine_version,
        engine_mode,
        status: "available".to_string(),
        master_username,
        database_name,
        port,
        allocated_storage: allocated_storage.unwrap_or(1),
        availability_zones: availability_zones.unwrap_or_else(|| vec![
            format!("{}a", region),
            format!("{}b", region),
            format!("{}c", region),
        ]),
        backup_retention_period: backup_retention_period.unwrap_or(1),
        db_subnet_group_name,
        vpc_security_group_ids: vpc_security_group_ids.unwrap_or_default(),
        deletion_protection: deletion_protection.unwrap_or(false),
        storage_encrypted: storage_encrypted.unwrap_or(false),
        kms_key_id,
        endpoint: Some(endpoint.clone()),
        reader_endpoint: Some(reader_endpoint.clone()),
        members: Vec::new(),
        tags: tags.unwrap_or_default(),
        created_at: now,
        region,
    };

    let db_cluster = DBCluster {
        db_cluster_identifier: stored.db_cluster_identifier.clone(),
        db_cluster_arn: stored.db_cluster_arn.clone(),
        engine: stored.engine.clone(),
        engine_version: stored.engine_version.clone(),
        engine_mode: stored.engine_mode.clone(),
        status: stored.status.clone(),
        master_username: stored.master_username.clone(),
        database_name: stored.database_name.clone(),
        endpoint: stored.endpoint.clone(),
        reader_endpoint: stored.reader_endpoint.clone(),
        port: Some(stored.port),
        allocated_storage: stored.allocated_storage,
        availability_zones: stored.availability_zones.clone(),
        backup_retention_period: stored.backup_retention_period,
        db_cluster_members: Vec::new(),
        vpc_security_groups: stored.vpc_security_group_ids.iter().map(|id| VpcSecurityGroupMembership {
            vpc_security_group_id: id.clone(),
            status: "active".to_string(),
        }).collect(),
        db_subnet_group: stored.db_subnet_group_name.clone(),
        deletion_protection: stored.deletion_protection,
        storage_encrypted: stored.storage_encrypted,
        kms_key_id: stored.kms_key_id.clone(),
        cluster_create_time: Some(stored.created_at),
        tags: stored.tags.clone(),
    };

    DB_CLUSTERS.insert(db_cluster_identifier, stored);

    CreateDbClusterOutput {
        db_cluster,
        success: true,
    }
}

/// Delete a DB cluster
pub async fn delete_db_cluster(
    db_cluster_identifier: String,
    _skip_final_snapshot: Option<bool>,
    _final_db_snapshot_identifier: Option<String>,
    _region: Option<String>,
) -> DeleteDbClusterOutput {
    if let Some((_, stored)) = DB_CLUSTERS.remove(&db_cluster_identifier) {
        let db_cluster = DBCluster {
            db_cluster_identifier: stored.db_cluster_identifier,
            db_cluster_arn: stored.db_cluster_arn,
            engine: stored.engine,
            engine_version: stored.engine_version,
            engine_mode: stored.engine_mode,
            status: "deleting".to_string(),
            master_username: stored.master_username,
            database_name: stored.database_name,
            endpoint: stored.endpoint,
            reader_endpoint: stored.reader_endpoint,
            port: Some(stored.port),
            allocated_storage: stored.allocated_storage,
            availability_zones: stored.availability_zones,
            backup_retention_period: stored.backup_retention_period,
            db_cluster_members: Vec::new(),
            vpc_security_groups: Vec::new(),
            db_subnet_group: stored.db_subnet_group_name,
            deletion_protection: stored.deletion_protection,
            storage_encrypted: stored.storage_encrypted,
            kms_key_id: stored.kms_key_id,
            cluster_create_time: Some(stored.created_at),
            tags: stored.tags,
        };
        
        DeleteDbClusterOutput {
            db_cluster,
            success: true,
        }
    } else {
        DeleteDbClusterOutput {
            db_cluster: DBCluster::default(),
            success: false,
        }
    }
}

/// Describe DB clusters
pub async fn describe_db_clusters(
    db_cluster_identifier: Option<String>,
    _filters: Option<Vec<RdsFilter>>,
    max_records: Option<i32>,
    _marker: Option<String>,
    _region: Option<String>,
) -> DescribeDbClustersOutput {
    let max_records = max_records.unwrap_or(100) as usize;

    let db_clusters: Vec<DBCluster> = if let Some(identifier) = db_cluster_identifier {
        DB_CLUSTERS.get(&identifier)
            .map(|entry| {
                let stored = entry.value();
                vec![DBCluster {
                    db_cluster_identifier: stored.db_cluster_identifier.clone(),
                    db_cluster_arn: stored.db_cluster_arn.clone(),
                    engine: stored.engine.clone(),
                    engine_version: stored.engine_version.clone(),
                    engine_mode: stored.engine_mode.clone(),
                    status: stored.status.clone(),
                    master_username: stored.master_username.clone(),
                    database_name: stored.database_name.clone(),
                    endpoint: stored.endpoint.clone(),
                    reader_endpoint: stored.reader_endpoint.clone(),
                    port: Some(stored.port),
                    allocated_storage: stored.allocated_storage,
                    availability_zones: stored.availability_zones.clone(),
                    backup_retention_period: stored.backup_retention_period,
                    db_cluster_members: Vec::new(),
                    vpc_security_groups: Vec::new(),
                    db_subnet_group: stored.db_subnet_group_name.clone(),
                    deletion_protection: stored.deletion_protection,
                    storage_encrypted: stored.storage_encrypted,
                    kms_key_id: stored.kms_key_id.clone(),
                    cluster_create_time: Some(stored.created_at),
                    tags: stored.tags.clone(),
                }]
            })
            .unwrap_or_default()
    } else {
        DB_CLUSTERS
            .iter()
            .take(max_records)
            .map(|entry| {
                let stored = entry.value();
                DBCluster {
                    db_cluster_identifier: stored.db_cluster_identifier.clone(),
                    db_cluster_arn: stored.db_cluster_arn.clone(),
                    engine: stored.engine.clone(),
                    engine_version: stored.engine_version.clone(),
                    engine_mode: stored.engine_mode.clone(),
                    status: stored.status.clone(),
                    master_username: stored.master_username.clone(),
                    database_name: stored.database_name.clone(),
                    endpoint: stored.endpoint.clone(),
                    reader_endpoint: stored.reader_endpoint.clone(),
                    port: Some(stored.port),
                    allocated_storage: stored.allocated_storage,
                    availability_zones: stored.availability_zones.clone(),
                    backup_retention_period: stored.backup_retention_period,
                    db_cluster_members: Vec::new(),
                    vpc_security_groups: Vec::new(),
                    db_subnet_group: stored.db_subnet_group_name.clone(),
                    deletion_protection: stored.deletion_protection,
                    storage_encrypted: stored.storage_encrypted,
                    kms_key_id: stored.kms_key_id.clone(),
                    cluster_create_time: Some(stored.created_at),
                    tags: stored.tags.clone(),
                }
            })
            .collect()
    };

    DescribeDbClustersOutput {
        db_clusters,
        marker: None,
    }
}

// Utility functions for testing
#[cfg(test)]
pub fn clear_all_data() {
    DB_INSTANCES.clear();
    DB_CLUSTERS.clear();
}

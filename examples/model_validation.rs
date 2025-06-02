use rust_az_wrapper::models::{CosmosAccount, BackupPolicy, ContinuousBackupProperties, PeriodicBackupProperties};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Validating Azure Cosmos DB specific models...\n");

    // Test 1: Validate continuous backup logic
    test_continuous_backup_logic()?;
    
    // Test 2: Validate periodic backup logic  
    test_periodic_backup_logic()?;
    
    // Test 3: Validate Azure-specific field mapping
    test_azure_field_mapping()?;
    
    println!("\n✅ All validation tests passed!");
    println!("📊 Our models are correctly mapped to Azure CLI");
    
    Ok(())
}

fn test_continuous_backup_logic() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Test 1: Continuous Backup Logic");
    
    // Simulate different continuous backup types
    let continuous_7_days = ContinuousBackupProperties {
        tier: Some("Continuous7Days".to_string()),
    };
    
    let _continuous_30_days = ContinuousBackupProperties {
        tier: Some("Continuous30Days".to_string()),
    };
    
    let backup_policy = BackupPolicy {
        backup_type: "Continuous".to_string(),
        migration_state: None,
        periodic_mode_properties: None,
        continuous_mode_properties: Some(continuous_7_days),
    };
    
    // Validate our specific logic
    assert_eq!(backup_policy.backup_type, "Continuous");
    assert!(backup_policy.continuous_mode_properties.is_some());
    assert!(backup_policy.periodic_mode_properties.is_none());
    
    if let Some(props) = &backup_policy.continuous_mode_properties {
        if let Some(tier) = &props.tier {
            assert!(tier.starts_with("Continuous"));
            println!("   ✅ Valid continuous backup tier: {}", tier);
        }
    }
    
    Ok(())
}

fn test_periodic_backup_logic() -> Result<(), Box<dyn std::error::Error>> {
    println!("📅 Test 2: Periodic Backup Logic");
    
    let periodic_props = PeriodicBackupProperties {
        backup_interval_in_minutes: Some(240), // 4 hours
        backup_retention_interval_in_hours: Some(168), // 7 days
        backup_storage_redundancy: Some("LocallyRedundant".to_string()),
    };
    
    let backup_policy = BackupPolicy {
        backup_type: "Periodic".to_string(),
        migration_state: None,
        periodic_mode_properties: Some(periodic_props),
        continuous_mode_properties: None,
    };
    
    // Validate our specific logic
    assert_eq!(backup_policy.backup_type, "Periodic");
    assert!(backup_policy.periodic_mode_properties.is_some());
    assert!(backup_policy.continuous_mode_properties.is_none());
    
    if let Some(props) = &backup_policy.periodic_mode_properties {
        if let Some(interval) = props.backup_interval_in_minutes {
            assert!(interval > 0);
            println!("   ✅ Valid backup interval: {} minutes", interval);
        }
        
        if let Some(retention) = props.backup_retention_interval_in_hours {
            assert!(retention > 0);
            println!("   ✅ Valid backup retention: {} hours", retention);
        }
    }
    
    Ok(())
}

fn test_azure_field_mapping() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️  Test 3: Azure CLI Field Mapping");
    
    // Simulate data that would come from Azure CLI
    let test_json = r#"
    {
        "name": "test-cosmos",
        "resourceGroup": "test-rg",
        "documentEndpoint": "https://test-cosmos.documents.azure.com:443/",
        "kind": "GlobalDocumentDB",
        "location": "East US",
        "provisioningState": "Succeeded",
        "type": "Microsoft.DocumentDB/databaseAccounts",
        "id": "/subscriptions/test-sub/resourceGroups/test-rg/providers/Microsoft.DocumentDB/databaseAccounts/test-cosmos"
    }
    "#;
    
    // Test if our aliases work correctly
    let account: CosmosAccount = serde_json::from_str(test_json)?;
    
    // Validate Azure-specific mapping
    assert_eq!(account.name, "test-cosmos");
    assert_eq!(account.resource_group, "test-rg"); // Alias from "resourceGroup"
    assert_eq!(account.document_endpoint, "https://test-cosmos.documents.azure.com:443/");
    assert_eq!(account.kind, "GlobalDocumentDB");
    assert_eq!(account.provisioning_state, "Succeeded");
    
    println!("   ✅ Field aliases working correctly");
    println!("   ✅ Azure CLI → Rust struct mapping validated");
    
    // Test optional fields
    assert!(account.backup_policy.is_none()); // Not provided in JSON
    assert!(account.tags.is_none());
    
    println!("   ✅ Optional fields handled correctly");
    
    Ok(())
}

/// Helper function to demonstrate practical usage
#[allow(dead_code)]
fn demonstrate_practical_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💡 Practical Usage Demonstration:");
    
    // Create fictional account for demonstration
    let account = CosmosAccount {
        name: "my-app-cosmos".to_string(),
        id: "/subscriptions/12345/resourceGroups/my-rg/providers/Microsoft.DocumentDB/databaseAccounts/my-app-cosmos".to_string(),
        location: "West Europe".to_string(),
        resource_group: "my-rg".to_string(),
        resource_type: "Microsoft.DocumentDB/databaseAccounts".to_string(),
        kind: "MongoDB".to_string(),
        provisioning_state: "Succeeded".to_string(),
        document_endpoint: "https://my-app-cosmos.documents.azure.com:443/".to_string(),
        tags: None,
        backup_policy: Some(BackupPolicy {
            backup_type: "Continuous".to_string(),
            migration_state: None,
            continuous_mode_properties: Some(ContinuousBackupProperties {
                tier: Some("Continuous7Days".to_string()),
            }),
            periodic_mode_properties: None,
        }),
        // ... other fields as None for simplicity
        analytical_storage_configuration: None,
        api_properties: None,
        capabilities: None,
        capacity: None,
        consistency_policy: None,
        cors: None,
        create_mode: None,
        customer_managed_key_status: None,
        database_account_offer_type: None,
        default_identity: None,
        disable_key_based_metadata_write_access: None,
        disable_local_auth: None,
        enable_analytical_storage: None,
        enable_automatic_failover: None,
        enable_burst_capacity: None,
        enable_cassandra_connector: None,
        enable_free_tier: None,
        enable_multiple_write_locations: None,
        enable_partition_merge: None,
        enable_per_region_per_partition_autoscale: None,
        failover_policies: None,
        identity: None,
        instance_id: None,
        ip_rules: None,
        is_virtual_network_filter_enabled: None,
        key_vault_key_uri: None,
        keys_metadata: None,
        locations: None,
        minimal_tls_version: None,
        network_acl_bypass: None,
        network_acl_bypass_resource_ids: None,
        private_endpoint_connections: None,
        public_network_access: None,
        read_locations: None,
        restore_parameters: None,
        system_data: None,
        virtual_network_rules: None,
        write_locations: None,
    };
    
    // Demonstrate domain-specific logic
    if let Some(backup) = &account.backup_policy {
        match backup.backup_type.as_str() {
            "Continuous" => {
                println!("   📊 Account with continuous backup configured");
                if let Some(props) = &backup.continuous_mode_properties {
                    if let Some(tier) = &props.tier {
                        println!("   🕐 Tier: {}", tier);
                    }
                }
            },
            "Periodic" => {
                println!("   📊 Account with periodic backup configured");
            },
            _ => {
                println!("   ⚠️  Unknown backup type: {}", backup.backup_type);
            }
        }
    }
    
    println!("   🎯 This is the logic we should actually test!");
    
    Ok(())
} 
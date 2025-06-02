//! Integration tests for Azure CLI wrapper

use rust_az_wrapper::{AzureClient, Result};

#[tokio::test]
#[ignore] // Ignored by default as it requires configured Azure CLI
async fn test_show_current_subscription() -> Result<()> {
    let client = AzureClient::new()?;
    let subscription = client.show_current_subscription().await?;
    
    // Check if mandatory fields are present
    assert!(!subscription.id.is_empty());
    assert!(!subscription.display_name.is_empty());
    assert!(!subscription.state.is_empty());
    assert!(!subscription.tenant_id.is_empty());
    
    println!("Current subscription: {}", subscription.display_name);
    
    Ok(())
}

#[tokio::test]
#[ignore] // Ignored by default as it requires configured Azure CLI
async fn test_list_subscriptions() -> Result<()> {
    let client = AzureClient::new()?;
    let subscriptions = client.list_subscriptions().await?;
    
    // Check if mandatory fields are present
    for subscription in &subscriptions {
        assert!(!subscription.id.is_empty());
        assert!(!subscription.display_name.is_empty());
        assert!(!subscription.state.is_empty());
        assert!(!subscription.tenant_id.is_empty());
    }
    
    println!("Found {} subscriptions", subscriptions.len());
    
    Ok(())
}

#[tokio::test]
#[ignore] // Ignored by default as it requires configured Azure CLI
async fn test_list_cosmos_accounts() -> Result<()> {
    let client = AzureClient::new()?;
    let accounts = client.list_cosmos_accounts(None).await?;
    
    // May not have accounts, but if it does, must have valid fields
    for account in &accounts {
        assert!(!account.name.is_empty());
        assert!(!account.id.is_empty());
        assert!(!account.location.is_empty());
        assert!(!account.resource_group.is_empty());
        assert!(!account.resource_type.is_empty());
        assert!(!account.kind.is_empty());
        assert!(!account.provisioning_state.is_empty());
        assert!(!account.document_endpoint.is_empty());
    }
    
    println!("Found {} Cosmos accounts", accounts.len());
    
    Ok(())
}

#[tokio::test]
#[ignore] // Ignored by default as it requires configured Azure CLI
async fn test_json_serialization() -> Result<()> {
    let client = AzureClient::new()?;
    
    // Test subscription serialization
    let subscriptions = client.list_subscriptions().await?;
    let _json = serde_json::to_string(&subscriptions)?;
    
    // Test deserialization
    let _parsed: Vec<rust_az_wrapper::models::Subscription> = serde_json::from_str(&_json)?;
    
    // Test Cosmos accounts serialization
    let accounts = client.list_cosmos_accounts(None).await?;
    let _json = serde_json::to_string(&accounts)?;
    
    // Test deserialization
    let _parsed: Vec<rust_az_wrapper::models::CosmosAccount> = serde_json::from_str(&_json)?;
    
    println!("✅ JSON serialization test passed");
    
    Ok(())
}

#[tokio::test]
#[ignore] // Ignored by default as it requires configured Azure CLI
async fn test_cosmos_operations_if_account_exists() -> Result<()> {
    let client = AzureClient::new()?;
    let accounts = client.list_cosmos_accounts(None).await?;
    
    if let Some(account) = accounts.first() {
        // Test key retrieval
        let keys = client.list_cosmos_keys(&account.name, &account.resource_group).await?;
        assert!(!keys.primary_master_key.is_empty());
        assert!(!keys.secondary_master_key.is_empty());
        
        // Test database listing
        let databases = client.list_sql_databases(&account.name, &account.resource_group).await?;
        
        // If there are databases, test container listing
        if let Some(database) = databases.first() {
            let containers = client.list_sql_containers(&account.name, &account.resource_group, &database.name).await?;
            
            // Check if containers have valid fields
            for container in &containers {
                assert!(!container.name.is_empty());
                assert!(!container.id.is_empty());
            }
        }
        
        println!("✅ Cosmos operations test passed");
    } else {
        println!("⚠️ No Cosmos account found, skipping operations test");
    }
    
    Ok(())
} 
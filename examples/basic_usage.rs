//! Basic usage example of the Azure CLI wrapper for Cosmos DB

use rust_az_wrapper::{AzureClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create Azure client
    let client = AzureClient::new()?;

    println!("🔍 Listing subscriptions...");
    
    // Get current subscription
    let subscriptions = client.list_subscriptions().await?;
    
    // Show subscriptions
    for (i, sub) in subscriptions.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, sub.display_name, sub.id);
    }

    let current_subscription = subscriptions.first().unwrap();
    println!("🔍 Current subscription: {} ({})", current_subscription.display_name, current_subscription.id);
    
    println!("\n🗂️ Listing resource groups...");
    
    // List all resource groups from current subscription
    let resource_groups = client.list_resource_groups(None).await?;
    println!("📁 Resource groups found: {}", resource_groups.len());
    
    if resource_groups.is_empty() {
        println!("⚠️ No resource groups found.");
        return Ok(());
    }
    
    // Show resource groups
    for (i, rg) in resource_groups.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, rg.name, rg.location);
        println!("     Status: {}", rg.properties.provisioning_state);
    }
    
    // Get the first resource group
    let first_rg = &resource_groups[0];
    println!("\n🌍 Listing Cosmos DB accounts in resource group '{}'...", first_rg.name);
    
    // List Cosmos DB accounts in the first resource group
    let cosmos_accounts = client.list_cosmos_accounts(Some(&first_rg.name)).await?;
    println!("🗄️ Cosmos accounts found: {}", cosmos_accounts.len());
    
    if cosmos_accounts.is_empty() {
        println!("⚠️ No Cosmos accounts found in resource group '{}'.", first_rg.name);
        
        // If not found in the first, try other resource groups
        println!("\n🔍 Searching in other resource groups...");
        let mut found_any = false;
        
        for rg in resource_groups.iter().take(5) {
            let accounts = client.list_cosmos_accounts(Some(&rg.name)).await?;
            if !accounts.is_empty() {
                println!("✅ Found {} accounts in resource group '{}'", accounts.len(), rg.name);
                
                // Process the first account found
                let account = &accounts[0];
                println!("  📍 Account: {} ({})", account.name, account.location);
                println!("  🌐 Endpoint: {}", account.document_endpoint);
                println!("  🏷️ Type: {}", account.kind);
                
                found_any = true;
                break;
            }
        }
        
        if !found_any {
            println!("⚠️ No Cosmos accounts found in the first resource groups.");
        }
    } else {
        // Process found accounts
        for account in &cosmos_accounts {
            println!("  - {} ({})", account.name, account.location);
            println!("    📍 Endpoint: {}", account.document_endpoint);
            println!("    🏷️ Type: {}", account.kind);
        }
        
        // If accounts found, show more details of the first one
        let account = &cosmos_accounts[0];
        let account_name = &account.name;
        let resource_group = &account.resource_group;
        
        println!("\n🔑 Getting details for account '{}'...", account_name);
        
        // List account keys
        match client.list_cosmos_keys(account_name, resource_group).await {
            Ok(keys) => {
                println!("  ✅ Keys retrieved successfully");
                println!("  🔐 Primary key: {}...", &keys.primary_master_key[..20]);
            }
            Err(e) => {
                println!("  ❌ Error getting keys: {}", e);
            }
        }
        
        println!("\n📊 Listing SQL databases...");
        
        // List SQL databases
        match client.list_sql_databases(account_name, resource_group).await {
            Ok(databases) => {
                println!("  📚 Databases found: {}", databases.len());
                for db in &databases {
                    println!("    - {}", db.name);
                }
                
                // If there are databases, list containers of the first one
                if let Some(database) = databases.first() {
                    let db_name = &database.name;
                    println!("\n📦 Listing containers for database '{}'...", db_name);
                    
                    match client.list_sql_containers(account_name, resource_group, db_name).await {
                        Ok(containers) => {
                            println!("      📋 Containers found: {}", containers.len());
                            for container in &containers {
                                println!("        - {}", container.name);
                            }
                        }
                        Err(e) => {
                            println!("      ❌ Error listing containers: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("  ❌ Error listing databases: {}", e);
            }
        }
    }

    println!("\n✅ Example completed!");
    Ok(())
} 
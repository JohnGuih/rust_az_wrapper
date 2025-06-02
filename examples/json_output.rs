//! Example demonstrating JSON serialization and deserialization capabilities

use rust_az_wrapper::{AzureClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔄 Demonstrating JSON conversion capabilities...");

    // Create Azure client
    let client = AzureClient::new()?;

    println!("\n📋 Getting subscription information...");
    
    // Get current subscription
    let subscription = client.show_current_subscription().await?;
    
    // Convert to pretty JSON
    let subscription_json = serde_json::to_string_pretty(&subscription)?;
    println!("✨ Subscription as JSON:");
    println!("{}", subscription_json);
    
    // Convert back from JSON to verify serialization works both ways
    let parsed_subscription: rust_az_wrapper::Subscription = serde_json::from_str(&subscription_json)?;
    println!("\n✅ JSON parsing successful!");
    println!("📝 Parsed subscription name: {}", parsed_subscription.display_name);
    
    println!("\n🗂️ Getting resource groups...");
    
    // List resource groups
    let resource_groups = client.list_resource_groups(None).await?;
    
    if !resource_groups.is_empty() {
        // Get the first resource group and convert to JSON
        let first_rg = &resource_groups[0];
        let rg_json = serde_json::to_string_pretty(first_rg)?;
        
        println!("✨ First resource group as JSON:");
        println!("{}", rg_json);
        
        // Verify round-trip JSON conversion
        let parsed_rg: rust_az_wrapper::commands::account::ResourceGroup = serde_json::from_str(&rg_json)?;
        println!("\n✅ Resource group JSON parsing successful!");
        println!("📍 Parsed RG name: {}", parsed_rg.name);
        println!("🌍 Parsed RG location: {}", parsed_rg.location);
    }
    
    println!("\n🌍 Searching for Cosmos DB accounts...");
    
    // Try to find Cosmos accounts
    let cosmos_accounts = client.list_cosmos_accounts(None).await?;
    
    if !cosmos_accounts.is_empty() {
        let first_account = &cosmos_accounts[0];
        
        // Convert account to JSON
        let account_json = serde_json::to_string_pretty(first_account)?;
        println!("✨ Cosmos account as JSON:");
        println!("{}", account_json);
        
        // Verify parsing
        let parsed_account: rust_az_wrapper::CosmosAccount = serde_json::from_str(&account_json)?;
        println!("\n✅ Cosmos account JSON parsing successful!");
        println!("📍 Parsed account name: {}", parsed_account.name);
        println!("🌐 Parsed endpoint: {}", parsed_account.document_endpoint);
        
        // Try to get keys and convert to JSON
        match client.list_cosmos_keys(&first_account.name, &first_account.resource_group).await {
            Ok(keys) => {
                let keys_json = serde_json::to_string_pretty(&keys)?;
                println!("\n✨ Cosmos keys as JSON:");
                println!("{}", keys_json);
                
                // Verify keys parsing
                let parsed_keys: rust_az_wrapper::CosmosKeys = serde_json::from_str(&keys_json)?;
                println!("\n✅ Keys JSON parsing successful!");
                println!("🔐 Has primary key: {}", !parsed_keys.primary_master_key.is_empty());
            }
            Err(e) => {
                println!("\n⚠️ Could not get keys (this is normal in some environments): {}", e);
            }
        }
    } else {
        println!("📄 No Cosmos accounts found, demonstrating with resource groups instead");
        
        // Convert all resource groups to JSON
        let all_rgs_json = serde_json::to_string_pretty(&resource_groups)?;
        println!("✨ All resource groups as JSON:");
        println!("{}", all_rgs_json);
        
        // Parse back
        let parsed_rgs: Vec<rust_az_wrapper::commands::account::ResourceGroup> = serde_json::from_str(&all_rgs_json)?;
        println!("\n✅ All resource groups JSON parsing successful!");
        println!("📊 Parsed {} resource groups", parsed_rgs.len());
    }
    
    println!("\n🎉 JSON conversion demonstration completed!");
    println!("💡 All Azure resources can be serialized to/from JSON automatically");
    println!("🔄 This enables easy integration with REST APIs, databases, and configuration files");
    
    Ok(())
} 
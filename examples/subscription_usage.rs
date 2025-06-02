//! Example of using the Azure CLI wrapper with specific subscriptions

use rust_az_wrapper::{AzureClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Demonstrating specific subscription usage...");

    // 1. Default client (uses current Azure CLI subscription)
    let default_client = AzureClient::new()?;
    
    println!("\n📋 Listing all available subscriptions...");
    let subscriptions = default_client.list_subscriptions().await?;
    
    for (i, sub) in subscriptions.iter().enumerate().take(3) {
        println!("  {}. {} ({})", i + 1, sub.display_name, sub.id);
    }

    if subscriptions.is_empty() {
        println!("⚠️ No subscriptions found.");
        return Ok(());
    }

    // 2. Client configured for a specific subscription
    let first_subscription = &subscriptions[0];
    println!("\n🎯 Using specific subscription: {}", first_subscription.display_name);
    
    let specific_client = AzureClient::with_subscription(first_subscription.id.clone())?;
    
    // 3. Check which subscription is configured
    println!("📍 Configured subscription: {:?}", specific_client.get_subscription());
    
    // 4. Use client with specific subscription
    println!("\n🗂️ Listing resource groups from specific subscription...");
    let resource_groups = specific_client.list_resource_groups(None).await?;
    
    println!("📁 Resource groups found: {}", resource_groups.len());
    for (i, rg) in resource_groups.iter().enumerate().take(3) {
        println!("  {}. {} ({})", i + 1, rg.name, rg.location);
    }

    // 5. Mutable client to switch subscriptions
    if subscriptions.len() > 1 {
        let mut mutable_client = AzureClient::new()?;
        
        println!("\n🔄 Demonstrating subscription switching...");
        println!("Initial subscription: {:?}", mutable_client.get_subscription());
        
        // Switch to second subscription
        let second_subscription = &subscriptions[1];
        mutable_client.use_subscription(second_subscription.id.clone());
        println!("After switching: {:?}", mutable_client.get_subscription());
        
        // Return to default subscription
        mutable_client.clear_subscription();
        println!("After clearing: {:?}", mutable_client.get_subscription());
    }

    // 6. Check differences between subscriptions
    if subscriptions.len() > 1 {
        println!("\n🔍 Comparing resource groups between subscriptions...");
        
        let first_sub_client = AzureClient::with_subscription(subscriptions[0].id.clone())?;
        let second_sub_client = AzureClient::with_subscription(subscriptions[1].id.clone())?;
        
        let first_rgs = first_sub_client.list_resource_groups(None).await?;
        let second_rgs = second_sub_client.list_resource_groups(None).await?;
        
        println!("  Subscription '{}': {} resource groups", 
                 subscriptions[0].display_name, first_rgs.len());
        println!("  Subscription '{}': {} resource groups", 
                 subscriptions[1].display_name, second_rgs.len());
    }

    // 7. Example with Cosmos DB accounts in specific subscription
    if !resource_groups.is_empty() {
        println!("\n🌍 Searching for Cosmos accounts in specific subscription...");
        
        let cosmos_accounts = specific_client.list_cosmos_accounts(None).await?;
        println!("🗄️ Total Cosmos accounts in subscription: {}", cosmos_accounts.len());
        
        for account in cosmos_accounts.iter().take(2) {
            println!("  - {} ({})", account.name, account.location);
        }
    }

    println!("\n✅ Subscription example completed!");
    Ok(())
} 
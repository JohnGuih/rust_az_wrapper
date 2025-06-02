//! Example showing how to work with different Azure subscriptions

use rust_az_wrapper::{AzureClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Azure Subscriptions Example");
    
    // Create basic client (uses default subscription)
    let basic_client = AzureClient::new()?;
    
    // List all available subscriptions
    println!("\n📋 Listing all available subscriptions...");
    let subscriptions = basic_client.list_subscriptions().await?;
    
    for (i, subscription) in subscriptions.iter().enumerate() {
        println!("  {}. {} ({})", 
            i + 1, 
            subscription.display_name, 
            &subscription.id[..8] // Show only first 8 chars for privacy
        );
        println!("     State: {}", subscription.state);
        println!("     Tenant: {}", &subscription.tenant_id[..8]);
    }
    
    if subscriptions.len() < 2 {
        println!("⚠️ Need at least 2 subscriptions to demonstrate switching");
        return Ok(());
    }
    
    // Create client with specific subscription
    let first_subscription = &subscriptions[0];
    let specific_client = AzureClient::with_subscription(first_subscription.id.clone())?;
    println!("\n🎯 Created client for subscription: {}", first_subscription.display_name);
    println!("📍 Configured subscription: {:?}", specific_client.subscription_id());
    
    // Show current subscription using the basic client
    println!("\n🔍 Current subscription details:");
    let current = basic_client.show_current_subscription().await?;
    println!("  Name: {}", current.display_name);
    println!("  State: {}", current.state);
    
    // Demonstrate dynamic subscription switching with mutable client
    println!("\n🔄 Demonstrating subscription switching...");
    let mut mutable_client = AzureClient::new()?;
    
    if let Some(second_subscription) = subscriptions.get(1) {
        println!("Initial subscription: {:?}", mutable_client.subscription_id());
        
        // Switch to different subscription
        mutable_client.set_subscription(second_subscription.id.clone());
        println!("After switching to '{}': {:?}", 
                second_subscription.display_name,
                mutable_client.subscription_id());
        
        // Reset to no specific subscription (uses default)
        mutable_client.set_subscription("".to_string()); // Empty means default
        println!("After resetting: {:?}", mutable_client.subscription_id());
    }
    
    // List resource groups using different subscriptions
    println!("\n🗂️ Comparing resource groups across subscriptions...");
    
    for (i, subscription) in subscriptions.iter().take(2).enumerate() {
        let client = AzureClient::with_subscription(subscription.id.clone())?;
        
        match client.list_resource_groups(None).await {
            Ok(resource_groups) => {
                println!("  Subscription {}: {} resource groups", 
                        i + 1, 
                        resource_groups.len());
                
                for rg in resource_groups.iter().take(3) {
                    println!("    - {} ({})", rg.name, rg.location);
                }
                
                if resource_groups.len() > 3 {
                    println!("    ... and {} more", resource_groups.len() - 3);
                }
            }
            Err(e) => {
                println!("  Subscription {}: Error listing resource groups: {}", i + 1, e);
            }
        }
    }
    
    println!("\n✅ Subscription example completed!");
    Ok(())
} 
# 🔍 Rust Azure CLI Wrapper - Read-Only Cosmos DB

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Azure CLI](https://img.shields.io/badge/Azure_CLI-Required-blue.svg)](https://docs.microsoft.com/en-us/cli/azure/)

## 🎯 About

This library provides a **safe, read-only interface** to Azure Cosmos DB via the Azure CLI. It's designed for exploration, monitoring, and data discovery without the risk of accidental modifications.

## ✨ Key Features

- 🔒 **Read-Only Design** - Zero risk of accidental resource changes
- 📊 **Type-Safe Models** - All Azure responses as Rust structs
- 🧭 **Cosmos DB Focused** - SQL API, MongoDB, keys, throughput
- ⚡ **Async/Await** - Modern async Rust with Tokio
- 🌍 **Multi-Subscription** - Work across Azure subscriptions
- 📄 **JSON Serializable** - Easy integration with other tools

## 🚀 Quick Start

### Prerequisites

- **Azure CLI** installed and authenticated (`az login`)
- **Rust 1.70+**

### Basic Usage

```rust
use rust_az_wrapper::AzureClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AzureClient::new()?;
    
    // Verify Azure CLI authentication
    client.verify_authentication().await?;
    
    // Explore your Cosmos DB accounts
    let accounts = client.list_cosmos_accounts(None).await?;
    
    for account in accounts {
        println!("🌍 Account: {} ({})", account.name, account.location);
        
        // Get account details
        let keys = client.list_cosmos_keys(&account.name, &account.resource_group).await?;
        println!("🔑 Has keys: Yes");
        
        // List databases
        let databases = client.list_sql_databases(&account.name, &account.resource_group).await?;
        println!("📚 Databases: {}", databases.len());
        
        for db in databases {
            // List containers in each database
            let containers = client.list_sql_containers(
                &account.name, 
                &account.resource_group, 
                &db.name
            ).await?;
            println!("  📦 {}: {} containers", db.name, containers.len());
        }
    }
    
    Ok(())
}
```

## 📋 Available Operations

### Subscriptions & Resource Groups
- `list_subscriptions()` - List all Azure subscriptions
- `show_current_subscription()` - Show current subscription details
- `list_resource_groups()` - List resource groups
- `show_resource_group()` - Show resource group details

### Cosmos DB Accounts
- `list_cosmos_accounts()` - List all Cosmos DB accounts
- `show_cosmos_account()` - Show account details
- `list_cosmos_keys()` - Get master keys
- `list_cosmos_read_only_keys()` - Get read-only keys
- `list_cosmos_connection_strings()` - Get connection strings

### SQL API
- `list_sql_databases()` - List SQL databases
- `show_sql_database()` - Show database details
- `list_sql_containers()` - List containers
- `show_sql_container()` - Show container details
- `get_database_throughput()` - Get database throughput
- `get_container_throughput()` - Get container throughput

### MongoDB API
- `list_mongodb_databases()` - List MongoDB databases
- `list_mongodb_collections()` - List collections

## 🏗️ Architecture

```
src/
├── lib.rs          # Main API and exports
├── client.rs       # Unified Azure client
├── commands/
│   ├── account.rs  # Subscription/resource group operations
│   └── cosmos.rs   # Cosmos DB operations
├── models.rs       # Azure resource models
├── utils.rs        # CLI utilities
└── error.rs        # Error handling
```

## 🛡️ Safety by Design

This library is **intentionally read-only**. It provides no methods to:
- Create resources
- Modify existing resources  
- Delete resources
- Change configurations
- Regenerate keys

This makes it perfect for:
- 🔍 **Discovery** - Explore your Azure environment safely
- 📊 **Monitoring** - Build dashboards and reports
- 🧾 **Inventory** - Catalog your Cosmos DB resources
- 🔄 **Integration** - Feed data to other systems


## 🤝 Contributing

Contributions are welcome!

## 📄 License

MIT License - see LICENSE file for details
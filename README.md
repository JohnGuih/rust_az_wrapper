# 🌍 Rust Azure CLI Wrapper for Cosmos DB

> **A type-safe and ergonomic Rust wrapper for Azure CLI, with specific focus on Cosmos DB**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Azure CLI](https://img.shields.io/badge/Azure_CLI-Required-blue.svg)](https://docs.microsoft.com/en-us/cli/azure/)

## 🎯 About the Project

This project provides a **type-safe and ergonomic Rust interface** for Azure CLI operations related to **Cosmos DB**. Instead of dealing with command line commands and manual JSON parsing, you get:

- ✅ **Typed structs** for all Azure resources
- ✅ **Automatic serialization/deserialization** to/from JSON  
- ✅ **Robust error handling** with specific types
- ✅ **Asynchronous operations** with Tokio
- ✅ **Complete multi-subscription support**
- ✅ **Ergonomic API** that abstracts Azure CLI complexity

## ✨ Features

- 🔐 **Complete abstractions** for Azure CLI commands related to Cosmos DB
- 📦 **Serializable models** for all resources (JSON in/out)
- 🛡️ **Robust error handling** with specific types
- ⚡ **Asynchronous operations** with Tokio
- 🎯 **Cosmos DB focused** - SQL API, MongoDB, throughput, keys
- 🔄 **Multi-subscription support** - work with specific subscriptions
- 📋 **Resource Groups** - complete management
- 🧪 **Extensively tested** with real Azure data

## 🚀 Quick Start

### ⚙️ Prerequisites

- **Rust 1.70+** 
- **Azure CLI** installed and configured (`az --version`)
- **Valid Azure authentication** (`az login`)

```bash
# Check prerequisites
az --version
az account show  # Should show active subscription
```

### Basic Client

```rust
use rust_az_wrapper::{AzureClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Default client (uses current Azure CLI subscription)
    let client = AzureClient::new()?;
    
    // List available subscriptions
    let subscriptions = client.list_subscriptions().await?;
    
    // List resource groups
    let resource_groups = client.list_resource_groups(None).await?;
    
    // List Cosmos DB accounts
    let cosmos_accounts = client.list_cosmos_accounts(None).await?;
    
    Ok(())
}
```

### Working with Specific Subscriptions

```rust
use rust_az_wrapper::{AzureClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Client configured for specific subscription
    let client = AzureClient::with_subscription("your-subscription-id".to_string())?;
    
    // 2. Mutable client to switch subscriptions
    let mut client = AzureClient::new()?;
    client.use_subscription("subscription-id".to_string());
    
    // All commands now use this subscription
    let rgs = client.list_resource_groups(None).await?;
    let cosmos = client.list_cosmos_accounts(None).await?;
    
    // Return to default subscription
    client.clear_subscription();
    
    // 3. Check configured subscription
    println!("Current subscription: {:?}", client.get_subscription());
    
    Ok(())
}
```

### Complete Example - Cosmos DB

```rust
use rust_az_wrapper::{AzureClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = AzureClient::new()?;
    
    // Find a Cosmos account
    let accounts = client.list_cosmos_accounts(None).await?;
    let account = &accounts[0];
    
    // Get access keys
    let keys = client.list_cosmos_keys(&account.name, &account.resource_group).await?;
    println!("🔑 Primary Key: {}...", &keys.primary_master_key[..20]);
    
    // List databases
    let databases = client.list_sql_databases(&account.name, &account.resource_group).await?;
    println!("📚 Databases: {}", databases.len());
    
    // For each database, list containers
    for db in &databases {
        let containers = client.list_sql_containers(
            &account.name, 
            &account.resource_group, 
            &db.name
        ).await?;
        println!("  📦 Database '{}': {} containers", db.name, containers.len());
    }
    
    // Convert to JSON
    let json = serde_json::to_string_pretty(&account)?;
    println!("📄 Account as JSON:\n{}", json);
    
    Ok(())
}
```

## 🏗️ Project Architecture

```
rust_az_wrapper/
├── src/
│   ├── lib.rs              # 📚 Main API and public exports
│   ├── client.rs           # 🔧 Unified Azure client 
│   ├── commands/           # 📝 Azure CLI specific commands
│   │   ├── account.rs      #   └─ Subscriptions and Resource Groups
│   │   └── cosmos.rs       #   └─ Cosmos DB specific
│   ├── models.rs           # 📊 Typed models for Azure resources
│   ├── utils.rs            # 🛠️ Utilities and command builder
│   └── error.rs            # ⚠️ Typed error system
├── examples/               # 🎯 Practical examples
│   ├── basic_usage.rs      #   └─ Basic wrapper usage
│   ├── json_output.rs      #   └─ JSON conversion
│   └── subscription_usage.rs   └─ Working with subscriptions
├── tests/                  # 🧪 Integration tests
└── README.md, Cargo.toml   # 📋 Documentation and configuration
```

## 🔧 Implemented Features

### ✅ **Subscriptions & Resource Groups**
```rust
// List and manage subscriptions
let subs = client.list_subscriptions().await?;
let current = client.show_current_subscription().await?;

// Resource Groups with specific subscription
let rgs = client.list_resource_groups(Some("subscription-id")).await?;
let rg = client.show_resource_group("rg-name", Some("subscription-id")).await?;
```

### ✅ **Cosmos DB Accounts**
```rust
// Complete CRUD for accounts
let accounts = client.list_cosmos_accounts(Some("resource-group")).await?;
let account = client.show_cosmos_account("account-name", "rg").await?;
let new_account = client.create_cosmos_account(
    "name", "rg", "eastus", Some("GlobalDocumentDB"), /*...*/ None
).await?;
```

### ✅ **Keys and Authentication**
```rust
// Keys and connection strings
let keys = client.list_cosmos_keys("account", "rg").await?;
let readonly_keys = client.list_cosmos_read_only_keys("account", "rg").await?;
let conn_strings = client.list_cosmos_connection_strings("account", "rg").await?;
```

### ✅ **SQL API - Databases & Containers**
```rust
// SQL Databases
let databases = client.list_sql_databases("account", "rg").await?;
let new_db = client.create_sql_database("account", "rg", "db-name", Some(400), None).await?;

// SQL Containers
let containers = client.list_sql_containers("account", "rg", "db").await?;
let new_container = client.create_sql_container(
    "account", "rg", "db", "container", "/id", Some(400), None
).await?;
```

### ✅ **MongoDB API**
```rust
// MongoDB databases and collections
let mongo_dbs = client.list_mongodb_databases("account", "rg").await?;
let collections = client.list_mongodb_collections("account", "rg", "db").await?;
```

### ✅ **Throughput Management**
```rust
// Performance management
let db_throughput = client.get_database_throughput("account", "rg", "db").await?;
let updated = client.update_database_throughput("account", "rg", "db", Some(800), None).await?;
```

### Clone and Test

```bash
git clone https://github.com/your-user/rust_az_wrapper.git
cd rust_az_wrapper

# Run tests (requires configured Azure CLI)
cargo test

# Run examples
cargo run --example basic_usage
cargo run --example subscription_usage
```

## 🧪 **Test Status**

The project has **extensive test coverage** with real Azure data:

```bash
# Unit and integration tests
cargo test

# Functional examples  
cargo run --example basic_usage
cargo run --example subscription_usage
```

## 🔄 **Automatic JSON Conversion**

All types implement `Serialize` and `Deserialize`:

```rust
use rust_az_wrapper::{AzureClient, Result};

#[tokio::main] 
async fn main() -> Result<()> {
    let client = AzureClient::new()?;
    let accounts = client.list_cosmos_accounts(None).await?;
    
    // Rust structs → JSON
    let json = serde_json::to_string_pretty(&accounts)?;
    println!("{}", json);
    
    // JSON → Rust structs  
    let parsed: Vec<CosmosAccount> = serde_json::from_str(&json)?;
    
    // Work with typed data
    for account in parsed {
        println!("Account: {} in {}", account.name, account.location);
        println!("Endpoint: {}", account.document_endpoint);
    }
    
    Ok(())
}
```

## 🎯 **Use Cases**

### **DevOps & Automation**
```rust
// Automate Cosmos DB deployment
let account = client.create_cosmos_account(/* params */).await?;
let database = client.create_sql_database(/* params */).await?;
let container = client.create_sql_container(/* params */).await?;
```

### **Monitoring & Inventory**
```rust
// Discover resources across multiple subscriptions
for sub in subscriptions {
    let client = AzureClient::with_subscription(sub.id)?;
    let accounts = client.list_cosmos_accounts(None).await?;
    inventory.extend(accounts);
}
```

### **Migration & Backup**
```rust
// Get configurations for migration
let account = client.show_cosmos_account("source", "rg").await?;
let databases = client.list_sql_databases("source", "rg").await?;
let keys = client.list_cosmos_keys("source", "rg").await?;
```

## 🤝 **Contributing**

1. Fork the project
2. Create a branch (`git checkout -b feature/new-feature`)
3. Commit changes (`git commit -am 'Add new feature'`)
4. Push to branch (`git push origin feature/new-feature`) 
5. Open a Pull Request
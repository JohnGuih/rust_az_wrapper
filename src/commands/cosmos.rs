//! Cosmos DB specific Azure CLI commands
//! 
//! This module provides read-only access to Azure Cosmos DB resources.
//! Only query operations are supported - no creation, modification, or deletion.

use crate::models::*;
use crate::utils::AzCommandBuilder;
use crate::Result;

// === ACCOUNT COMMANDS (READ-ONLY) ===

/// Lists all Cosmos DB accounts in a resource group or subscription
pub async fn list_accounts(
    resource_group: Option<&str>,
    subscription_id: Option<&str>
) -> Result<Vec<CosmosAccount>> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("list");
    
    if let Some(rg) = resource_group {
        builder = builder.param("--resource-group", rg);
    }
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

/// Shows details of a specific Cosmos DB account
pub async fn show_account(
    name: &str, 
    resource_group: &str,
    subscription_id: Option<&str>
) -> Result<CosmosAccount> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("show")
        .param("--name", name)
        .param("--resource-group", resource_group);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

// === KEYS COMMANDS (READ-ONLY) ===

/// Lists master keys for a Cosmos DB account
pub async fn list_keys(
    name: &str,
    resource_group: &str,
    subscription_id: Option<&str>
) -> Result<CosmosKeys> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("keys")
        .subcommand("list")
        .param("--name", name)
        .param("--resource-group", resource_group);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

/// Lists read-only keys for a Cosmos DB account
pub async fn list_read_only_keys(
    name: &str,
    resource_group: &str,
    subscription_id: Option<&str>
) -> Result<CosmosKeys> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("keys")
        .subcommand("list")
        .param("--name", name)
        .param("--resource-group", resource_group)
        .param("--type", "read-only-keys");
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

/// Lists connection strings for a Cosmos DB account
pub async fn list_connection_strings(
    name: &str,
    resource_group: &str,
    subscription_id: Option<&str>
) -> Result<CosmosConnectionStrings> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("keys")
        .subcommand("list")
        .param("--name", name)
        .param("--resource-group", resource_group)
        .param("--type", "connection-strings");
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

// === SQL DATABASE COMMANDS (READ-ONLY) ===

/// Lists SQL databases in a Cosmos DB account
pub async fn list_sql_databases(
    account_name: &str,
    resource_group: &str,
    subscription_id: Option<&str>
) -> Result<Vec<CosmosDatabase>> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("sql")
        .subcommand("database")
        .subcommand("list")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

/// Shows details of a specific SQL database
pub async fn show_sql_database(
    account_name: &str,
    resource_group: &str,
    database_name: &str,
    subscription_id: Option<&str>
) -> Result<CosmosDatabase> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("sql")
        .subcommand("database")
        .subcommand("show")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group)
        .param("--name", database_name);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

// === SQL CONTAINER COMMANDS (READ-ONLY) ===

/// Lists SQL containers in a database
pub async fn list_sql_containers(
    account_name: &str,
    resource_group: &str,
    database_name: &str,
    subscription_id: Option<&str>
) -> Result<Vec<CosmosContainer>> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("sql")
        .subcommand("container")
        .subcommand("list")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group)
        .param("--database-name", database_name);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

/// Shows details of a specific SQL container
pub async fn show_sql_container(
    account_name: &str,
    resource_group: &str,
    database_name: &str,
    container_name: &str,
    subscription_id: Option<&str>
) -> Result<CosmosContainer> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("sql")
        .subcommand("container")
        .subcommand("show")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group)
        .param("--database-name", database_name)
        .param("--name", container_name);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

// === MONGODB COMMANDS (READ-ONLY) ===

/// Lists MongoDB databases in a Cosmos DB account
pub async fn list_mongodb_databases(
    account_name: &str,
    resource_group: &str,
    subscription_id: Option<&str>
) -> Result<Vec<CosmosDatabase>> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("mongodb")
        .subcommand("database")
        .subcommand("list")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

/// Lists MongoDB collections in a database
pub async fn list_mongodb_collections(
    account_name: &str,
    resource_group: &str,
    database_name: &str,
    subscription_id: Option<&str>
) -> Result<Vec<CosmosContainer>> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("mongodb")
        .subcommand("collection")
        .subcommand("list")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group)
        .param("--database-name", database_name);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

// === THROUGHPUT COMMANDS (READ-ONLY) ===

/// Gets the throughput settings of a database
pub async fn get_database_throughput(
    account_name: &str,
    resource_group: &str,
    database_name: &str,
    subscription_id: Option<&str>
) -> Result<ThroughputSettings> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("sql")
        .subcommand("database")
        .subcommand("throughput")
        .subcommand("show")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group)
        .param("--name", database_name);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
}

/// Gets the throughput settings of a container
pub async fn get_container_throughput(
    account_name: &str,
    resource_group: &str,
    database_name: &str,
    container_name: &str,
    subscription_id: Option<&str>
) -> Result<ThroughputSettings> {
    let mut builder = AzCommandBuilder::new()
        .subcommand("cosmosdb")
        .subcommand("sql")
        .subcommand("container")
        .subcommand("throughput")
        .subcommand("show")
        .param("--account-name", account_name)
        .param("--resource-group", resource_group)
        .param("--database-name", database_name)
        .param("--name", container_name);
    
    if let Some(sub) = subscription_id {
        builder = builder.subscription(Some(sub));
    }
    
    builder.execute().await
} 
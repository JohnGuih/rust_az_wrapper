//! Cosmos DB specific commands

use crate::error::Result;
use crate::models::*;
use crate::utils::AzCommandBuilder;

/// Commands for managing Cosmos DB
pub struct CosmosCommands;

impl CosmosCommands {
    // === ACCOUNT COMMANDS ===

    /// Lists all Cosmos DB accounts
    pub async fn list_accounts(resource_group: Option<&str>, subscription_id: Option<&str>) -> Result<Vec<CosmosAccount>> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("list")
            .subscription(subscription_id);

        if let Some(rg) = resource_group {
            builder = builder.param("--resource-group", rg);
        }

        builder.execute().await
    }

    /// Shows details of a specific Cosmos DB account
    pub async fn show_account(name: &str, resource_group: &str, subscription_id: Option<&str>) -> Result<CosmosAccount> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("show")
            .param("--name", name)
            .param("--resource-group", resource_group)
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Creates a new Cosmos DB account
    pub async fn create_account(
        name: &str,
        resource_group: &str,
        location: &str,
        kind: Option<&str>,
        capabilities: Option<Vec<&str>>,
        enable_automatic_failover: Option<bool>,
        enable_multiple_write_locations: Option<bool>,
        subscription_id: Option<&str>,
    ) -> Result<CosmosAccount> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("create")
            .param("--name", name)
            .param("--resource-group", resource_group)
            .param("--location", location)
            .subscription(subscription_id);

        if let Some(k) = kind {
            builder = builder.param("--kind", k);
        }

        if let Some(caps) = capabilities {
            for cap in caps {
                builder = builder.param("--capabilities", cap);
            }
        }

        if let Some(auto_failover) = enable_automatic_failover {
            builder = builder.conditional_flag("--enable-automatic-failover", auto_failover);
        }

        if let Some(multi_write) = enable_multiple_write_locations {
            builder = builder.conditional_flag("--enable-multiple-write-locations", multi_write);
        }

        builder.execute().await
    }

    /// Updates an existing Cosmos DB account
    pub async fn update_account(
        name: &str,
        resource_group: &str,
        enable_automatic_failover: Option<bool>,
        enable_multiple_write_locations: Option<bool>,
        subscription_id: Option<&str>,
    ) -> Result<CosmosAccount> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("update")
            .param("--name", name)
            .param("--resource-group", resource_group)
            .subscription(subscription_id);

        if let Some(auto_failover) = enable_automatic_failover {
            builder = builder.conditional_flag("--enable-automatic-failover", auto_failover);
        }

        if let Some(multi_write) = enable_multiple_write_locations {
            builder = builder.conditional_flag("--enable-multiple-write-locations", multi_write);
        }

        builder.execute().await
    }

    /// Deletes a Cosmos DB account
    pub async fn delete_account(name: &str, resource_group: &str, subscription_id: Option<&str>) -> Result<String> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("delete")
            .param("--name", name)
            .param("--resource-group", resource_group)
            .flag("--yes")
            .subscription(subscription_id)
            .execute_raw()
            .await
    }

    // === KEY COMMANDS ===

    /// Lists the access keys of a Cosmos DB account
    pub async fn list_keys(account_name: &str, resource_group: &str, subscription_id: Option<&str>) -> Result<CosmosKeys> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("keys")
            .subcommand("list")
            .param("--name", account_name)
            .param("--resource-group", resource_group)
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Lists the read-only access keys
    pub async fn list_read_only_keys(account_name: &str, resource_group: &str, subscription_id: Option<&str>) -> Result<CosmosKeys> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("keys")
            .subcommand("list")
            .param("--name", account_name)
            .param("--resource-group", resource_group)
            .param("--type", "read-only-keys")
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Regenerates a specific key
    pub async fn regenerate_key(
        account_name: &str,
        resource_group: &str,
        key_kind: &str, // primary, secondary, primaryReadonly, secondaryReadonly
        subscription_id: Option<&str>,
    ) -> Result<String> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("keys")
            .subcommand("regenerate")
            .param("--name", account_name)
            .param("--resource-group", resource_group)
            .param("--key-kind", key_kind)
            .subscription(subscription_id)
            .execute_raw()
            .await
    }

    // === CONNECTION STRING COMMANDS ===

    /// Lists the connection strings of a Cosmos DB account
    pub async fn list_connection_strings(
        account_name: &str,
        resource_group: &str,
        subscription_id: Option<&str>,
    ) -> Result<CosmosConnectionStrings> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("keys")
            .subcommand("list")
            .param("--name", account_name)
            .param("--resource-group", resource_group)
            .param("--type", "connection-strings")
            .subscription(subscription_id)
            .execute()
            .await
    }

    // === DATABASE COMMANDS ===

    /// Lists all SQL databases of a Cosmos DB account
    pub async fn list_sql_databases(
        account_name: &str,
        resource_group: &str,
        subscription_id: Option<&str>,
    ) -> Result<Vec<CosmosDatabase>> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("database")
            .subcommand("list")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Shows details of a specific SQL database
    pub async fn show_sql_database(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        subscription_id: Option<&str>,
    ) -> Result<CosmosDatabase> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("database")
            .subcommand("show")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--name", database_name)
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Creates a new SQL database
    pub async fn create_sql_database(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
        subscription_id: Option<&str>,
    ) -> Result<CosmosDatabase> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("database")
            .subcommand("create")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--name", database_name)
            .subscription(subscription_id);

        if let Some(tp) = throughput {
            builder = builder.param("--throughput", &tp.to_string());
        }

        if let Some(max_tp) = max_throughput {
            builder = builder.param("--max-throughput", &max_tp.to_string());
        }

        builder.execute().await
    }

    /// Deletes a SQL database
    pub async fn delete_sql_database(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        subscription_id: Option<&str>,
    ) -> Result<String> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("database")
            .subcommand("delete")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--name", database_name)
            .flag("--yes")
            .subscription(subscription_id)
            .execute_raw()
            .await
    }

    // === CONTAINER COMMANDS ===

    /// Lists all containers of a SQL database
    pub async fn list_sql_containers(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<Vec<CosmosContainer>> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("container")
            .subcommand("list")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--database-name", database_name)
            .execute()
            .await
    }

    /// Shows details of a specific SQL container
    pub async fn show_sql_container(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<CosmosContainer> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("container")
            .subcommand("show")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--database-name", database_name)
            .param("--name", container_name)
            .execute()
            .await
    }

    /// Creates a new SQL container
    pub async fn create_sql_container(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
        partition_key_path: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
    ) -> Result<CosmosContainer> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("container")
            .subcommand("create")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--database-name", database_name)
            .param("--name", container_name)
            .param("--partition-key-path", partition_key_path);

        if let Some(t) = throughput {
            builder = builder.param("--throughput", &t.to_string());
        }

        if let Some(max_t) = max_throughput {
            builder = builder.param("--max-throughput", &max_t.to_string());
        }

        builder.execute().await
    }

    /// Deletes a SQL container
    pub async fn delete_sql_container(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<String> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("container")
            .subcommand("delete")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--database-name", database_name)
            .param("--name", container_name)
            .flag("--yes")
            .execute_raw()
            .await
    }

    // === THROUGHPUT COMMANDS ===

    /// Gets the throughput of a database
    pub async fn get_database_throughput(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<ThroughputSettings> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("database")
            .subcommand("throughput")
            .subcommand("show")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--name", database_name)
            .execute()
            .await
    }

    /// Updates the throughput of a database
    pub async fn update_database_throughput(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
    ) -> Result<ThroughputSettings> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("database")
            .subcommand("throughput")
            .subcommand("update")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--name", database_name);

        if let Some(t) = throughput {
            builder = builder.param("--throughput", &t.to_string());
        }

        if let Some(max_t) = max_throughput {
            builder = builder.param("--max-throughput", &max_t.to_string());
        }

        builder.execute().await
    }

    /// Gets the throughput of a container
    pub async fn get_container_throughput(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<ThroughputSettings> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("container")
            .subcommand("throughput")
            .subcommand("show")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--database-name", database_name)
            .param("--name", container_name)
            .execute()
            .await
    }

    /// Updates the throughput of a container
    pub async fn update_container_throughput(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
    ) -> Result<ThroughputSettings> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("sql")
            .subcommand("container")
            .subcommand("throughput")
            .subcommand("update")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--database-name", database_name)
            .param("--name", container_name);

        if let Some(t) = throughput {
            builder = builder.param("--throughput", &t.to_string());
        }

        if let Some(max_t) = max_throughput {
            builder = builder.param("--max-throughput", &max_t.to_string());
        }

        builder.execute().await
    }

    // === MONGODB COMMANDS ===

    /// Lists all MongoDB databases of a Cosmos DB account
    pub async fn list_mongodb_databases(
        account_name: &str,
        resource_group: &str,
    ) -> Result<Vec<CosmosDatabase>> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("mongodb")
            .subcommand("database")
            .subcommand("list")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .execute()
            .await
    }

    /// Lists all MongoDB collections of a database
    pub async fn list_mongodb_collections(
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<Vec<CosmosContainer>> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("mongodb")
            .subcommand("collection")
            .subcommand("list")
            .param("--account-name", account_name)
            .param("--resource-group", resource_group)
            .param("--database-name", database_name)
            .execute()
            .await
    }

    // === LOCATION COMMANDS ===

    /// Lists all regions where an account is available
    pub async fn list_account_locations(
        account_name: &str,
        resource_group: &str,
    ) -> Result<Vec<AccountLocation>> {
        AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("locations")
            .subcommand("list")
            .param("--name", account_name)
            .param("--resource-group", resource_group)
            .execute()
            .await
    }

    // === FAILOVER COMMANDS ===

    /// Initiates a manual failover to another region
    pub async fn failover_priority_change(
        account_name: &str,
        resource_group: &str,
        failover_policies: Vec<(String, i32)>, // (location, priority)
    ) -> Result<String> {
        let mut builder = AzCommandBuilder::new()
            .subcommand("cosmosdb")
            .subcommand("failover-priority-change")
            .param("--name", account_name)
            .param("--resource-group", resource_group);

        for (location, priority) in failover_policies {
            builder = builder.param("--failover-policies", &format!("{}={}", location, priority));
        }

        builder.execute_raw().await
    }
} 
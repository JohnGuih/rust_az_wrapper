//! Main client for Azure CLI wrapper focused on Cosmos DB

use crate::commands::{AccountCommands, CosmosCommands};
use crate::error::Result;
use crate::models::*;
use crate::utils::{check_authentication, check_az_cli};

/// Main client for interacting with Azure CLI
#[derive(Debug, Clone)]
pub struct AzureClient {
    /// Whether to automatically check authentication
    auto_check_auth: bool,
    /// Specific subscription to use (if None, uses default)
    subscription_id: Option<String>,
}

impl AzureClient {
    /// Creates a new Azure client
    pub fn new() -> Result<Self> {
        Ok(Self {
            auto_check_auth: true,
            subscription_id: None,
        })
    }

    /// Creates a client without automatic authentication check
    pub fn new_no_auth_check() -> Self {
        Self {
            auto_check_auth: false,
            subscription_id: None,
        }
    }

    /// Creates a client configured for a specific subscription
    pub fn with_subscription(subscription_id: String) -> Result<Self> {
        Ok(Self {
            auto_check_auth: true,
            subscription_id: Some(subscription_id),
        })
    }

    /// Sets the subscription to use in all commands
    pub fn use_subscription(&mut self, subscription_id: String) {
        self.subscription_id = Some(subscription_id);
    }

    /// Removes the specific subscription (returns to using default)
    pub fn clear_subscription(&mut self) {
        self.subscription_id = None;
    }

    /// Gets the currently configured subscription
    pub fn get_subscription(&self) -> Option<&str> {
        self.subscription_id.as_deref()
    }

    /// Checks if Azure CLI is installed and the user is authenticated
    pub async fn verify_setup(&self) -> Result<()> {
        check_az_cli().await?;
        
        if self.auto_check_auth {
            check_authentication().await?;
        }
        
        Ok(())
    }

    // === ACCOUNT/SUBSCRIPTION METHODS ===

    /// Lists all available subscriptions
    pub async fn list_subscriptions(&self) -> Result<Vec<Subscription>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        AccountCommands::list_subscriptions().await
    }

    /// Gets information about the current or specific subscription
    pub async fn show_current_subscription(&self) -> Result<Subscription> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        AccountCommands::show_subscription(self.subscription_id.as_deref()).await
    }

    /// Sets a subscription as default in Azure CLI
    pub async fn set_subscription(&self, subscription_id: &str) -> Result<String> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        AccountCommands::set_subscription(subscription_id).await
    }

    /// Lists all available locations
    pub async fn list_locations(&self) -> Result<Vec<serde_json::Value>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        AccountCommands::list_locations(self.subscription_id.as_deref()).await
    }

    /// Lists all resource groups
    pub async fn list_resource_groups(&self, subscription_id: Option<&str>) -> Result<Vec<crate::commands::account::ResourceGroup>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        if let Some(subscription_id) = subscription_id {
            AccountCommands::list_resource_groups(Some(subscription_id)).await
        } else {
            AccountCommands::list_resource_groups(self.subscription_id.as_deref()).await
        }
    }

    /// Shows details of a specific resource group
    pub async fn show_resource_group(&self, name: &str) -> Result<crate::commands::account::ResourceGroup> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        AccountCommands::show_resource_group(name, self.subscription_id.as_deref()).await
    }

    /// Creates a new resource group
    pub async fn create_resource_group(&self, name: &str, location: &str) -> Result<crate::commands::account::ResourceGroup> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        AccountCommands::create_resource_group(name, location, self.subscription_id.as_deref()).await
    }

    /// Deletes a resource group
    pub async fn delete_resource_group(&self, name: &str) -> Result<String> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        AccountCommands::delete_resource_group(name, self.subscription_id.as_deref()).await
    }

    // === COSMOS ACCOUNT METHODS ===

    /// Lists all Cosmos DB accounts
    pub async fn list_cosmos_accounts(&self, resource_group: Option<&str>) -> Result<Vec<CosmosAccount>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_accounts(resource_group, self.subscription_id.as_deref()).await
    }

    /// Shows details of a specific Cosmos DB account
    pub async fn show_cosmos_account(&self, name: &str, resource_group: &str) -> Result<CosmosAccount> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::show_account(name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Creates a new Cosmos DB account
    pub async fn create_cosmos_account(
        &self,
        name: &str,
        resource_group: &str,
        location: &str,
        kind: Option<&str>,
        capabilities: Option<Vec<&str>>,
        enable_automatic_failover: Option<bool>,
        enable_multiple_write_locations: Option<bool>,
    ) -> Result<CosmosAccount> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::create_account(
            name,
            resource_group,
            location,
            kind,
            capabilities,
            enable_automatic_failover,
            enable_multiple_write_locations,
            self.subscription_id.as_deref(),
        ).await
    }

    /// Updates an existing Cosmos DB account
    pub async fn update_cosmos_account(
        &self,
        name: &str,
        resource_group: &str,
        enable_automatic_failover: Option<bool>,
        enable_multiple_write_locations: Option<bool>,
    ) -> Result<CosmosAccount> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::update_account(
            name,
            resource_group,
            enable_automatic_failover,
            enable_multiple_write_locations,
            self.subscription_id.as_deref(),
        ).await
    }

    /// Deletes a Cosmos DB account
    pub async fn delete_cosmos_account(&self, name: &str, resource_group: &str) -> Result<String> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::delete_account(name, resource_group, self.subscription_id.as_deref()).await
    }

    // === KEYS AND CONNECTION STRINGS METHODS ===

    /// Lists the access keys of a Cosmos DB account
    pub async fn list_cosmos_keys(&self, account_name: &str, resource_group: &str) -> Result<CosmosKeys> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_keys(account_name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Lists the read-only access keys
    pub async fn list_cosmos_read_only_keys(&self, account_name: &str, resource_group: &str) -> Result<CosmosKeys> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_read_only_keys(account_name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Regenerates a specific key
    pub async fn regenerate_cosmos_key(
        &self,
        account_name: &str,
        resource_group: &str,
        key_kind: &str,
    ) -> Result<String> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::regenerate_key(account_name, resource_group, key_kind, self.subscription_id.as_deref()).await
    }

    /// Lists the connection strings of a Cosmos DB account
    pub async fn list_cosmos_connection_strings(
        &self,
        account_name: &str,
        resource_group: &str,
    ) -> Result<CosmosConnectionStrings> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_connection_strings(account_name, resource_group, self.subscription_id.as_deref()).await
    }

    // === SQL DATABASE METHODS ===

    /// Lists all SQL databases of a Cosmos DB account
    pub async fn list_sql_databases(
        &self,
        account_name: &str,
        resource_group: &str,
    ) -> Result<Vec<CosmosDatabase>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_sql_databases(account_name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Shows details of a specific SQL database
    pub async fn show_sql_database(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<CosmosDatabase> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::show_sql_database(account_name, resource_group, database_name, self.subscription_id.as_deref()).await
    }

    /// Creates a new SQL database
    pub async fn create_sql_database(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
    ) -> Result<CosmosDatabase> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::create_sql_database(
            account_name,
            resource_group,
            database_name,
            throughput,
            max_throughput,
            self.subscription_id.as_deref(),
        ).await
    }

    /// Deletes a SQL database
    pub async fn delete_sql_database(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<String> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::delete_sql_database(account_name, resource_group, database_name, self.subscription_id.as_deref()).await
    }

    // === SQL CONTAINER METHODS ===

    /// Lists all containers of a SQL database
    pub async fn list_sql_containers(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<Vec<CosmosContainer>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_sql_containers(account_name, resource_group, database_name).await
    }

    /// Shows details of a specific SQL container
    pub async fn show_sql_container(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<CosmosContainer> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::show_sql_container(account_name, resource_group, database_name, container_name).await
    }

    /// Creates a new SQL container
    pub async fn create_sql_container(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
        partition_key_path: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
    ) -> Result<CosmosContainer> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::create_sql_container(
            account_name,
            resource_group,
            database_name,
            container_name,
            partition_key_path,
            throughput,
            max_throughput,
        ).await
    }

    /// Deletes a SQL container
    pub async fn delete_sql_container(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<String> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::delete_sql_container(account_name, resource_group, database_name, container_name).await
    }

    // === THROUGHPUT METHODS ===

    /// Gets the throughput of a database
    pub async fn get_database_throughput(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<ThroughputSettings> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::get_database_throughput(account_name, resource_group, database_name).await
    }

    /// Updates the throughput of a database
    pub async fn update_database_throughput(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
    ) -> Result<ThroughputSettings> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::update_database_throughput(
            account_name,
            resource_group,
            database_name,
            throughput,
            max_throughput,
        ).await
    }

    /// Gets the throughput of a container
    pub async fn get_container_throughput(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<ThroughputSettings> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::get_container_throughput(account_name, resource_group, database_name, container_name).await
    }

    /// Updates the throughput of a container
    pub async fn update_container_throughput(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
        container_name: &str,
        throughput: Option<i32>,
        max_throughput: Option<i32>,
    ) -> Result<ThroughputSettings> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::update_container_throughput(
            account_name,
            resource_group,
            database_name,
            container_name,
            throughput,
            max_throughput,
        ).await
    }

    // === MONGODB METHODS ===

    /// Lists all MongoDB databases of a Cosmos DB account
    pub async fn list_mongodb_databases(
        &self,
        account_name: &str,
        resource_group: &str,
    ) -> Result<Vec<CosmosDatabase>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_mongodb_databases(account_name, resource_group).await
    }

    /// Lists all MongoDB collections of a database
    pub async fn list_mongodb_collections(
        &self,
        account_name: &str,
        resource_group: &str,
        database_name: &str,
    ) -> Result<Vec<CosmosContainer>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_mongodb_collections(account_name, resource_group, database_name).await
    }

    // === LOCATIONS AND FAILOVER METHODS ===

    /// Lists all regions where an account is available
    pub async fn list_cosmos_account_locations(
        &self,
        account_name: &str,
        resource_group: &str,
    ) -> Result<Vec<AccountLocation>> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::list_account_locations(account_name, resource_group).await
    }

    /// Initiates a manual failover to another region
    pub async fn cosmos_failover_priority_change(
        &self,
        account_name: &str,
        resource_group: &str,
        failover_policies: Vec<(String, i32)>,
    ) -> Result<String> {
        if self.auto_check_auth {
            self.verify_setup().await?;
        }
        CosmosCommands::failover_priority_change(account_name, resource_group, failover_policies).await
    }
}

impl Default for AzureClient {
    fn default() -> Self {
        Self {
            auto_check_auth: true,
            subscription_id: None,
        }
    }
} 
//! Azure CLI client wrapper

use crate::commands::{account, cosmos};
use crate::models::*;
use crate::utils::check_authentication;
use crate::Result;

/// Main client for Azure CLI operations
pub struct AzureClient {
    subscription_id: Option<String>,
}

impl AzureClient {
    /// Creates a new Azure client
    /// 
    /// This will verify that Azure CLI is available and the user is authenticated
    pub fn new() -> Result<Self> {
        // Note: We can't use async in constructor, so authentication check is deferred
        Ok(Self {
            subscription_id: None,
        })
    }

    /// Creates a new Azure client with a specific subscription
    pub fn with_subscription(subscription_id: String) -> Result<Self> {
        Ok(Self {
            subscription_id: Some(subscription_id),
        })
    }

    /// Verifies authentication status
    pub async fn verify_authentication(&self) -> Result<()> {
        check_authentication().await
    }

    /// Sets the default subscription for all operations
    pub fn set_subscription(&mut self, subscription_id: String) {
        self.subscription_id = Some(subscription_id);
    }

    /// Gets the current subscription ID
    pub fn subscription_id(&self) -> Option<&str> {
        self.subscription_id.as_deref()
    }

    // === SUBSCRIPTION OPERATIONS (READ-ONLY) ===

    /// Lists all available subscriptions
    pub async fn list_subscriptions(&self) -> Result<Vec<Subscription>> {
        account::AccountCommands::list_subscriptions().await
    }

    /// Shows the current subscription
    pub async fn show_current_subscription(&self) -> Result<Subscription> {
        account::AccountCommands::show_subscription(None).await
    }

    /// Shows details of a specific subscription
    pub async fn show_subscription(&self, subscription_id: &str) -> Result<Subscription> {
        account::AccountCommands::show_subscription(Some(subscription_id)).await
    }

    /// Lists available locations
    pub async fn list_locations(&self) -> Result<Vec<serde_json::Value>> {
        account::AccountCommands::list_locations(self.subscription_id.as_deref()).await
    }

    /// Lists resource groups in current or specified subscription
    pub async fn list_resource_groups(&self, subscription_id: Option<&str>) -> Result<Vec<account::ResourceGroup>> {
        let sub_id = subscription_id.or(self.subscription_id.as_deref());
        account::AccountCommands::list_resource_groups(sub_id).await
    }

    /// Shows details of a specific resource group
    pub async fn show_resource_group(&self, name: &str) -> Result<account::ResourceGroup> {
        account::AccountCommands::show_resource_group(name, self.subscription_id.as_deref()).await
    }

    // === COSMOS DB OPERATIONS (READ-ONLY) ===

    /// Lists all Cosmos DB accounts
    pub async fn list_cosmos_accounts(&self, resource_group: Option<&str>) -> Result<Vec<CosmosAccount>> {
        cosmos::list_accounts(resource_group, self.subscription_id.as_deref()).await
    }

    /// Shows details of a specific Cosmos DB account
    pub async fn show_cosmos_account(&self, name: &str, resource_group: &str) -> Result<CosmosAccount> {
        cosmos::show_account(name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Lists master keys for a Cosmos DB account
    pub async fn list_cosmos_keys(&self, name: &str, resource_group: &str) -> Result<CosmosKeys> {
        cosmos::list_keys(name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Lists read-only keys for a Cosmos DB account
    pub async fn list_cosmos_read_only_keys(&self, name: &str, resource_group: &str) -> Result<CosmosKeys> {
        cosmos::list_read_only_keys(name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Lists connection strings for a Cosmos DB account
    pub async fn list_cosmos_connection_strings(&self, name: &str, resource_group: &str) -> Result<CosmosConnectionStrings> {
        cosmos::list_connection_strings(name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Lists SQL databases in a Cosmos DB account
    pub async fn list_sql_databases(&self, account_name: &str, resource_group: &str) -> Result<Vec<CosmosDatabase>> {
        cosmos::list_sql_databases(account_name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Shows details of a specific SQL database
    pub async fn show_sql_database(&self, account_name: &str, resource_group: &str, database_name: &str) -> Result<CosmosDatabase> {
        cosmos::show_sql_database(account_name, resource_group, database_name, self.subscription_id.as_deref()).await
    }

    /// Lists SQL containers in a database
    pub async fn list_sql_containers(&self, account_name: &str, resource_group: &str, database_name: &str) -> Result<Vec<CosmosContainer>> {
        cosmos::list_sql_containers(account_name, resource_group, database_name, self.subscription_id.as_deref()).await
    }

    /// Shows details of a specific SQL container
    pub async fn show_sql_container(&self, account_name: &str, resource_group: &str, database_name: &str, container_name: &str) -> Result<CosmosContainer> {
        cosmos::show_sql_container(account_name, resource_group, database_name, container_name, self.subscription_id.as_deref()).await
    }

    /// Lists MongoDB databases in a Cosmos DB account
    pub async fn list_mongodb_databases(&self, account_name: &str, resource_group: &str) -> Result<Vec<CosmosDatabase>> {
        cosmos::list_mongodb_databases(account_name, resource_group, self.subscription_id.as_deref()).await
    }

    /// Lists MongoDB collections in a database
    pub async fn list_mongodb_collections(&self, account_name: &str, resource_group: &str, database_name: &str) -> Result<Vec<CosmosContainer>> {
        cosmos::list_mongodb_collections(account_name, resource_group, database_name, self.subscription_id.as_deref()).await
    }

    /// Gets throughput settings of a database
    pub async fn get_database_throughput(&self, account_name: &str, resource_group: &str, database_name: &str) -> Result<ThroughputSettings> {
        cosmos::get_database_throughput(account_name, resource_group, database_name, self.subscription_id.as_deref()).await
    }

    /// Gets throughput settings of a container
    pub async fn get_container_throughput(&self, account_name: &str, resource_group: &str, database_name: &str, container_name: &str) -> Result<ThroughputSettings> {
        cosmos::get_container_throughput(account_name, resource_group, database_name, container_name, self.subscription_id.as_deref()).await
    }
} 
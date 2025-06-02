//! Commands related to Azure accounts and subscriptions

use crate::error::Result;
use crate::models::Subscription;
use crate::utils::AzCommandBuilder;
use serde::{Deserialize, Serialize};

/// Resource Group properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroupProperties {
    /// Provisioning status
    #[serde(alias = "provisioningState")]
    pub provisioning_state: String,
}

/// Resource Group information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroup {
    /// Resource group name
    pub name: String,
    /// Resource group location
    pub location: String,
    /// Resource group properties
    pub properties: ResourceGroupProperties,
    /// Associated tags
    pub tags: Option<std::collections::HashMap<String, String>>,
    /// Resource group ID
    pub id: String,
    /// Managed by (if applicable)
    #[serde(alias = "managedBy")]
    pub managed_by: Option<String>,
}

/// Commands for managing accounts and subscriptions
pub struct AccountCommands;

impl AccountCommands {
    /// Lists all available subscriptions
    pub async fn list_subscriptions() -> Result<Vec<Subscription>> {
        AzCommandBuilder::new()
            .subcommand("account")
            .subcommand("list")
            .execute()
            .await
    }

    /// Gets information about the current or specific subscription
    pub async fn show_subscription(subscription_id: Option<&str>) -> Result<Subscription> {
        AzCommandBuilder::new()
            .subcommand("account")
            .subcommand("show")
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Sets a subscription as default
    pub async fn set_subscription(subscription_id: &str) -> Result<String> {
        AzCommandBuilder::new()
            .subcommand("account")
            .subcommand("set")
            .param("--subscription", subscription_id)
            .execute_raw()
            .await
    }

    /// Lists all available locations
    pub async fn list_locations(subscription_id: Option<&str>) -> Result<Vec<serde_json::Value>> {
        AzCommandBuilder::new()
            .subcommand("account")
            .subcommand("list-locations")
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Lists all resource groups
    pub async fn list_resource_groups(subscription_id: Option<&str>) -> Result<Vec<ResourceGroup>> {
        AzCommandBuilder::new()
            .subcommand("group")
            .subcommand("list")
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Shows details of a specific resource group
    pub async fn show_resource_group(name: &str, subscription_id: Option<&str>) -> Result<ResourceGroup> {
        AzCommandBuilder::new()
            .subcommand("group")
            .subcommand("show")
            .param("--name", name)
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Creates a new resource group
    pub async fn create_resource_group(name: &str, location: &str, subscription_id: Option<&str>) -> Result<ResourceGroup> {
        AzCommandBuilder::new()
            .subcommand("group")
            .subcommand("create")
            .param("--name", name)
            .param("--location", location)
            .subscription(subscription_id)
            .execute()
            .await
    }

    /// Deletes a resource group
    pub async fn delete_resource_group(name: &str, subscription_id: Option<&str>) -> Result<String> {
        AzCommandBuilder::new()
            .subcommand("group")
            .subcommand("delete")
            .param("--name", name)
            .flag("--yes")
            .subscription(subscription_id)
            .execute_raw()
            .await
    }
} 
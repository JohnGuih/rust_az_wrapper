//! Azure CLI Wrapper for Rust
//! 
//! This library provides a Rust wrapper around the Azure CLI, focused on **read-only** operations
//! for Azure Cosmos DB resources. It offers a type-safe interface to query and inspect Azure
//! resources without the risk of accidental modifications.
//! 
//! ## Features
//! 
//! - **Cosmos DB Accounts**: List and show account details
//! - **Keys & Connection Strings**: Access read-only and master keys
//! - **SQL API**: List databases and containers 
//! - **MongoDB API**: List databases and collections
//! - **Throughput**: Query throughput settings
//! - **Subscriptions**: List and show subscription information
//! - **Resource Groups**: List and show resource group details
//! 
//! ## Read-Only Design
//! 
//! This library is intentionally designed for **read-only operations only**. It does not provide
//! methods to create, update, or delete Azure resources, ensuring safe exploration and monitoring
//! of your Azure environment without risk of accidental changes.
//! 
//! ## Example
//! 
//! ```rust
//! use rust_az_wrapper::AzureClient;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AzureClient::new()?;
//!     
//!     // Verify authentication
//!     client.verify_authentication().await?;
//!     
//!     // List all Cosmos DB accounts
//!     let accounts = client.list_cosmos_accounts(None).await?;
//!     for account in accounts {
//!         println!("Account: {} in {}", account.name, account.location);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod commands;
pub mod error;
pub mod models;
pub mod utils;

pub use client::AzureClient;
pub use error::{AzureError, Result};
pub use models::*; 